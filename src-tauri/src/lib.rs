use std::sync::{Arc, Mutex};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, PhysicalPosition};

// 导入自定义模块
pub mod countdown;
pub mod database;

pub use database::{get_migrations, CountdownConfig};

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
    let window = app.get_webview_window("main").unwrap();
    let is_always_on_top = window.is_always_on_top().unwrap_or(false);
    let is_decorated = window.is_decorated().unwrap_or(true);

    let toggle_top = MenuItem::with_id(
        app,
        "toggle_top",
        format!("切换置顶 {}", if is_always_on_top { "✓" } else { "" }),
        true,
        None::<&str>,
    )?;
    let toggle_shadow = MenuItem::with_id(
        app,
        "toggle_shadow",
        format!("毛玻璃效果 {}", if !is_decorated { "✓" } else { "" }),
        true,
        None::<&str>,
    )?;

    let pos_top_left = MenuItem::with_id(app, "pos_top_left", "左上角", true, None::<&str>)?;
    let pos_top_right = MenuItem::with_id(app, "pos_top_right", "右上角", true, None::<&str>)?;
    let pos_bottom_left = MenuItem::with_id(app, "pos_bottom_left", "左下角", true, None::<&str>)?;
    let pos_bottom_right =
        MenuItem::with_id(app, "pos_bottom_right", "右下角", true, None::<&str>)?;

    let position_submenu = Submenu::with_items(
        app,
        "窗口位置",
        true,
        &[
            &pos_top_left,
            &pos_top_right,
            &pos_bottom_left,
            &pos_bottom_right,
        ],
    )?;

    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &toggle_top,
            &toggle_shadow,
            &PredefinedMenuItem::separator(app)?,
            &position_submenu,
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
    if let Some(window) = app.get_webview_window("main") {
        match event.id.as_ref() {
            "toggle_top" => {
                let _ = window.set_always_on_top(!window.is_always_on_top().unwrap_or(false));
                if let Ok(menu) = create_tray_menu(app) {
                    let _ = app.tray_by_id("tray").unwrap().set_menu(Some(menu));
                }
            }
            "toggle_shadow" => {
                let is_decorated = window.is_decorated().unwrap_or(true);
                let _ = window.set_decorations(!is_decorated);
                if let Ok(menu) = create_tray_menu(app) {
                    let _ = app.tray_by_id("tray").unwrap().set_menu(Some(menu));
                }
            }
            "pos_top_left" => {
                let _ = set_window_position(window.clone(), "top-left".to_string());
            }
            "pos_top_right" => {
                let _ = set_window_position(window.clone(), "top-right".to_string());
            }
            "pos_bottom_left" => {
                let _ = set_window_position(window.clone(), "bottom-left".to_string());
            }
            "pos_bottom_right" => {
                let _ = set_window_position(window.clone(), "bottom-right".to_string());
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        }
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
    let config = database::get_default_config();
    let _config_state: ConfigState = Arc::new(Mutex::new(config));

    tauri::Builder::default()
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

            let menu = create_tray_menu(app.handle()).expect("Failed to create tray menu");
            TrayIconBuilder::with_id("tray")
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
            set_window_position,
            countdown::update_countdown_config,
            countdown::start_countdown_timer,
            database::load_config_from_db,
            database::save_config_to_db,
            database::save_countdown_record,
            database::add_todo,
            database::get_all_todos,
            database::update_todo,
            database::delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
