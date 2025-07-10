use std::sync::{Arc, Mutex};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, PhysicalPosition};
// 导入自定义模块
pub mod config;
pub mod countdown;
pub mod database;
pub mod holiday;
pub mod motivation;
pub mod todo;
pub mod window_commands;

pub use database::get_migrations;
pub use config::CountdownConfig;

type ConfigState = Arc<Mutex<CountdownConfig>>;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn toggle_always_on_top(window: tauri::WebviewWindow) -> Result<(), String> {
    let is_always_on_top = window.is_always_on_top().map_err(|e| e.to_string())?;
    window
        .set_always_on_top(!is_always_on_top)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn toggle_shadow(window: tauri::WebviewWindow) -> Result<(), String> {
    let is_decorated = window.is_decorated().map_err(|e| e.to_string())?;
    window
        .set_decorations(!is_decorated)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn hide_to_tray(window: tauri::WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn set_window_position(window: tauri::WebviewWindow, position: String) -> Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    let monitor_size = monitor.size();
    let window_size = window.outer_size().map_err(|e| e.to_string())?;

    // 获取任务栏高度（Windows上通常为40像素）
    let taskbar_height = 40;

    let (x, y) = match position.as_str() {
        "top-left" => (0, 0),
        "top-right" => (monitor_size.width as i32, 0),
        "bottom-left" => (
            0,
            monitor_size.height as i32 - window_size.height as i32 - taskbar_height,
        ),
        "bottom-right" => (
            monitor_size.width as i32,
            monitor_size.height as i32 - window_size.height as i32 - taskbar_height,
        ),
        _ => return Err("Invalid position".to_string()),
    };

    // 调整x坐标，确保窗口完全贴边
    let adjusted_x = if x == monitor_size.width as i32 {
        x - window_size.width as i32
    } else {
        x
    };

    window
        .set_position(PhysicalPosition::new(adjusted_x, y))
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn create_tray_menu(app: &tauri::AppHandle) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let open = MenuItem::with_id(app, "open", "打开", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &open,
            &settings,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;

    Ok(menu)
}

fn handle_tray_event(_tray: &tauri::tray::TrayIcon<tauri::Wry>, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button,
        button_state,
        ..
    } = event
    {
        if button == tauri::tray::MouseButton::Left
            && button_state == tauri::tray::MouseButtonState::Up
        {
            if let Some(window) = _tray.app_handle().get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    }
}

fn handle_menu_event(app: &tauri::AppHandle, event: tauri::menu::MenuEvent) {
    match event.id.as_ref() {
        "open" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "settings" => {
            if let Some(settings_window) = app.get_webview_window("settings") {
                let _ = settings_window.show();
                let _ = settings_window.set_focus();
            } else {
                // 如果设置窗口不存在，创建一个新的
                let _ = window_commands::show_settings_window(app.clone());
            }
        }
        "quit" => {
            std::process::exit(0);
        }
        _ => {}
    }
}

pub async fn db_pool(app: &tauri::AppHandle) -> Result<SqlitePool, sqlx::Error> {
    let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
    }
    let db_url = format!("sqlite:{}", app_data_dir.join("grape_planit.db").to_str().unwrap());

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }

    SqlitePool::connect(&db_url).await
}

pub fn run() {
    let config = config::get_default_countdown_config();
    let _config_state: ConfigState = Arc::new(Mutex::new(config));

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            let rt = tokio::runtime::Runtime::new().unwrap();
            let pool = rt.block_on(async {
                let pool = db_pool(&handle).await.expect("Failed to create database pool.");
                let migrations = get_migrations();
                for migration in migrations {
                    sqlx::query(&migration.sql)
                        .execute(&pool)
                        .await
                        .expect(&format!("Failed to execute migration: {}", migration.description));
                }
                pool
            });

            app.manage(pool);

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let pool_state: tauri::State<SqlitePool> = app_handle.state();
                if let Err(e) = countdown::start_countdown_timer(app_handle.clone(), pool_state).await {
                    eprintln!("Failed to start countdown timer: {}", e);
                }
            });

            // 加载并应用窗口设置
            let app_handle_settings = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let pool_state: tauri::State<SqlitePool> = app_handle_settings.state();
                if let Ok(settings) = config::load_window_settings_from_db_internal(pool_state.inner()).await {
                    if let Some(main_window) = app_handle_settings.get_webview_window("main") {
                        // 应用窗口位置
                        let _ = window_commands::set_main_window_position_internal(main_window.clone(), settings.window_position).await;
                        
                        // 应用置顶设置
                        let _ = main_window.set_always_on_top(settings.always_on_top);
                        
                        // 应用主题
                        let theme_script = format!("document.documentElement.setAttribute('data-theme', '{}')", settings.theme);
                        let _ = main_window.eval(&theme_script);
                        
                        // 应用透明度
                        let opacity_script = format!(
                            "document.documentElement.style.setProperty('--bg-primary-opacity', '{}'); document.documentElement.style.setProperty('--bg-secondary-opacity', '{}')",
                            settings.opacity, settings.opacity
                        );
                        let _ = main_window.eval(&opacity_script);
                        
                        // 应用主题色
                        let accent_color_script = format!(
                            "document.documentElement.style.setProperty('--accent-color', '{}')",
                            settings.accent_color
                        );
                        let _ = main_window.eval(&accent_color_script);
                        
                        // 加载并应用TODO颜色设置
                        if let Ok(todo_colors) = config::load_todo_color_settings_internal(pool_state.inner()).await {
                            let mut color_script = String::new();
                            for (level_key, color_value) in todo_colors {
                                color_script.push_str(&format!(
                                    "document.documentElement.style.setProperty('--{}-color', '{}');",
                                    level_key, color_value
                                ));
                            }
                            if !color_script.is_empty() {
                                let _ = main_window.eval(&color_script);
                            }
                        }
                    }
                }
            });

            // 加载并注册全局快捷键
            let app_handle_shortcuts = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let pool_state: tauri::State<SqlitePool> = app_handle_shortcuts.state();
                if let Ok(shortcut_settings) = config::load_shortcut_settings_from_db_internal(pool_state.inner()).await {
                    if let Err(e) = config::register_global_shortcuts(app_handle_shortcuts.clone(), shortcut_settings).await {
                        eprintln!("Failed to register global shortcuts: {}", e);
                    }
                }
            });

            let menu = create_tray_menu(app.handle()).expect("Failed to create tray menu");
            TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| handle_menu_event(app, event))
                .on_tray_icon_event(handle_tray_event)
                .build(app)
                .expect("Failed to build tray icon");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            toggle_always_on_top,
            toggle_shadow,
            hide_to_tray,
            set_window_position,
            window_commands::show_settings_window,
            window_commands::set_main_window_position,
            window_commands::set_window_opacity,
            window_commands::set_always_on_top,

            window_commands::eval_script_in_main_window,
            window_commands::get_monitors,
            window_commands::set_window_monitor,
            countdown::update_countdown_config,
            countdown::start_countdown_timer,
            countdown::reset_work_end_countdown_to_next_day,
            config::load_countdown_config_from_db,
            config::save_countdown_config_to_db,
            database::save_countdown_record,
            config::load_window_settings_from_db,
            config::save_window_settings_to_db,
            config::load_ai_settings_from_db,
            config::save_ai_settings_to_db,
            config::get_ai_settings_for_test,
            todo::add_todo,
            todo::get_all_todos,
            todo::get_recent_todos,
            todo::get_todo_by_id,
            todo::update_todo,
            todo::delete_todo,
            config::load_todo_color_settings,
            config::save_todo_color_settings,
            config::apply_todo_colors_to_main_window,
            config::load_shortcut_settings_from_db,
            config::save_shortcut_settings_to_db,
            config::register_global_shortcuts,
            config::load_weather_settings_from_db,
            config::save_weather_settings_to_db,
            holiday::sync_holiday_data,
            holiday::get_stored_holiday_years,
            holiday::get_holidays_by_year,
            holiday::delete_holiday_year,
            holiday::get_proxy_settings,
            holiday::save_proxy_settings,
            motivation::get_today_motivation_cache,
            motivation::save_today_motivation_cache,
            motivation::cleanup_motivation_cache
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
