use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, Row};
use tauri::{State, Manager};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct WeatherSettings {
    pub enabled: bool,
    #[serde(rename = "api_key")]
    #[sqlx(rename = "amap_api_key")]
    pub api_key: String,
    pub location_name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub adcode: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountdownConfig {
    #[serde(rename = "workEndTime")]
    #[sqlx(rename = "work_end_time")]
    pub work_end_time: String,
    #[serde(rename = "enableWorkEndCountdown")]
    #[sqlx(rename = "enable_work_end_countdown")]
    pub enable_work_end_countdown: bool,
    #[serde(rename = "finalCountdownMinutes")]
    #[sqlx(rename = "final_countdown_minutes")]
    pub final_countdown_minutes: i32,
    #[serde(rename = "endStateKeepMinutes")]
    #[sqlx(rename = "end_state_keep_minutes")]
    pub end_state_keep_minutes: i32,
    #[serde(rename = "workDays")]
    #[sqlx(rename = "work_days")]
    pub work_days: String, // "single" 单休, "double" 双休
    #[serde(rename = "showSeconds")]
    #[sqlx(rename = "show_seconds")]
    pub show_seconds: bool,
    #[serde(rename = "timeDisplayMode")]
    #[sqlx(rename = "time_display_mode")]
    pub time_display_mode: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct WindowSettings {
    pub theme: String,
    pub window_position: String,
    pub opacity: f64,
    pub always_on_top: bool,
    pub accent_color: String, // 新增主题色配置
    pub recent_days: i64,
    pub default_startup: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AISettings {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ShortcutSettings {
    pub toggle_window: String,
    pub quick_add_todo: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoColorSettings {
    pub colors: HashMap<String, String>,
}

// 默认配置函数
pub fn get_default_countdown_config() -> CountdownConfig {
    CountdownConfig {
        work_end_time: String::new(),
        enable_work_end_countdown: false,
        final_countdown_minutes: 1,
        end_state_keep_minutes: 5,
        work_days: "double".to_string(), // 默认双休
        show_seconds: true,
        time_display_mode: "current".to_string(),
    }
}

pub fn get_default_window_settings() -> WindowSettings {
    WindowSettings {
        theme: "auto".to_string(),
        window_position: "bottom-right".to_string(),
        opacity: 0.35,
        always_on_top: true,
        accent_color: "#007bff".to_string(), // 默认蓝色主题
        recent_days: 5,
        default_startup: "auto".to_string(),
    }
}

pub fn get_default_ai_settings() -> AISettings {
    AISettings {
        api_key: String::new(),
        base_url: "https://api.openai.com/v1".to_string(),
        model: "gpt-3.5-turbo".to_string(),
    }
}

pub fn get_default_todo_color_settings() -> TodoColorSettings {
    let mut colors = HashMap::new();
    colors.insert("level-important-urgent".to_string(), "#ff4757".to_string());
    colors.insert("level-important-not-urgent".to_string(), "#ffa726".to_string());
    colors.insert("level-not-important-urgent".to_string(), "#ffca28".to_string());
    colors.insert("level-not-important-not-urgent".to_string(), "#66bb6a".to_string());
    colors.insert("level-uncategorized".to_string(), "#bdbdbd".to_string());
    
    TodoColorSettings { colors }
}

pub fn get_default_shortcut_settings() -> ShortcutSettings {
    ShortcutSettings {
        toggle_window: "Alt+G".to_string(),
        quick_add_todo: "Alt+N".to_string(),
    }
}

pub fn get_default_weather_settings() -> WeatherSettings {
    WeatherSettings {
        enabled: false,
        api_key: String::new(),
        location_name: String::new(),
        latitude: None,
        longitude: None,
        adcode: None,
        province: None,
        city: None,
        district: None,
    }
}

// 倒计时配置相关函数
#[tauri::command]
pub async fn load_countdown_config_from_db(pool: State<'_, SqlitePool>) -> Result<CountdownConfig, String> {
    load_countdown_config_from_db_internal(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

pub async fn load_countdown_config_from_db_internal(
    pool: &SqlitePool,
) -> Result<CountdownConfig, sqlx::Error> {
    let result = sqlx::query_as::<_, CountdownConfig>(
        "SELECT work_end_time, enable_work_end_countdown, final_countdown_minutes, end_state_keep_minutes, work_days, show_seconds, time_display_mode FROM countdown_config ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(config) = result {
        Ok(config)
    } else {
        Ok(get_default_countdown_config())
    }
}

#[tauri::command]
pub async fn save_countdown_config_to_db(pool: State<'_, SqlitePool>, config: CountdownConfig) -> Result<(), String> {
    println!("🔧 [Rust] save_countdown_config_to_db 开始执行，配置: {:?}", config);
    
    sqlx::query("DELETE FROM countdown_config")
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 删除现有配置失败: {}", e);
            e.to_string()
        })?;
    
    sqlx::query("INSERT INTO countdown_config (work_end_time, enable_work_end_countdown, final_countdown_minutes, end_state_keep_minutes, work_days, show_seconds, time_display_mode) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&config.work_end_time)
        .bind(config.enable_work_end_countdown)
        .bind(config.final_countdown_minutes)
        .bind(config.end_state_keep_minutes)
        .bind(&config.work_days)
        .bind(config.show_seconds)
        .bind(&config.time_display_mode)
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 插入新配置失败: {}", e);
            e.to_string()
        })?;
    
    Ok(())
}

// 快捷键设置相关函数
#[tauri::command]
pub async fn load_shortcut_settings_from_db(pool: State<'_, SqlitePool>) -> Result<ShortcutSettings, String> {
    load_shortcut_settings_from_db_internal(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

pub async fn load_shortcut_settings_from_db_internal(
    pool: &SqlitePool,
) -> Result<ShortcutSettings, sqlx::Error> {
    let result = sqlx::query_as::<_, ShortcutSettings>(
        "SELECT toggle_window, quick_add_todo FROM shortcut_settings ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(settings) = result {
        Ok(settings)
    } else {
        Ok(get_default_shortcut_settings())
    }
}

#[tauri::command]
pub async fn save_shortcut_settings_to_db(pool: State<'_, SqlitePool>, settings: ShortcutSettings) -> Result<(), String> {
    println!("🔧 [Rust] save_shortcut_settings_to_db 开始执行，设置: {:?}", settings);
    
    sqlx::query("DELETE FROM shortcut_settings")
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 删除现有快捷键设置失败: {}", e);
            e.to_string()
        })?;
    
    sqlx::query("INSERT INTO shortcut_settings (toggle_window, quick_add_todo) VALUES (?, ?)")
        .bind(&settings.toggle_window)
        .bind(&settings.quick_add_todo)
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 插入新快捷键设置失败: {}", e);
            e.to_string()
        })?;
    
    println!("✅ [Rust] 快捷键设置保存成功");
    Ok(())
}

#[tauri::command]
pub async fn register_global_shortcuts(app: tauri::AppHandle, settings: ShortcutSettings) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{Code, Modifiers, GlobalShortcutExt};
    use tauri::Emitter;
    
    println!("🔧 [Rust] 注册全局快捷键: {:?}", settings);
    
    // 先取消注册所有现有的快捷键
    let _ = app.global_shortcut().unregister_all();
    
    // 解析并注册显示/隐藏窗口快捷键
    if let Ok(shortcut) = parse_shortcut(&settings.toggle_window) {
        let app_handle = app.clone();
        app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
            // 只在按键按下时触发，避免按下和释放都触发
            if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        }).map_err(|e| format!("注册显示/隐藏窗口快捷键失败: {}", e))?;
        
        println!("✅ [Rust] 显示/隐藏窗口快捷键注册成功: {}", settings.toggle_window);
    } else {
        println!("❌ [Rust] 无法解析显示/隐藏窗口快捷键: {}", settings.toggle_window);
    }
    
    // 解析并注册快速添加待办快捷键
    if let Ok(shortcut) = parse_shortcut(&settings.quick_add_todo) {
        let app_handle = app.clone();
        app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
            // 只在按键按下时触发，避免按下和释放都触发
            if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                if let Some(window) = app_handle.get_webview_window("main") {
                    // 显示窗口并聚焦
                    let _ = window.show();
                    let _ = window.set_focus();
                    
                    // 通过事件系统通知前端打开添加待办页面
                    let _ = window.emit("quick-add-todo", ());
                }
            }
        }).map_err(|e| format!("注册快速添加待办快捷键失败: {}", e))?;
        
        println!("✅ [Rust] 快速添加待办快捷键注册成功: {}", settings.quick_add_todo);
    } else {
        println!("❌ [Rust] 无法解析快速添加待办快捷键: {}", settings.quick_add_todo);
    }
    
    println!("✅ [Rust] 全局快捷键注册完成");
    Ok(())
}

// 解析快捷键字符串为Shortcut对象
fn parse_shortcut(shortcut_str: &str) -> Result<tauri_plugin_global_shortcut::Shortcut, String> {
    use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
    
    let parts: Vec<&str> = shortcut_str.split('+').collect();
    if parts.is_empty() {
        return Err("快捷键格式无效".to_string());
    }
    
    let mut modifiers = Modifiers::empty();
    let mut key_code = None;
    
    for part in parts {
        let part = part.trim();
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "alt" => modifiers |= Modifiers::ALT,
            "shift" => modifiers |= Modifiers::SHIFT,
            "meta" | "cmd" | "super" => modifiers |= Modifiers::SUPER,
            key => {
                key_code = Some(match key.to_lowercase().as_str() {
                    "a" => Code::KeyA,
                    "b" => Code::KeyB,
                    "c" => Code::KeyC,
                    "d" => Code::KeyD,
                    "e" => Code::KeyE,
                    "f" => Code::KeyF,
                    "g" => Code::KeyG,
                    "h" => Code::KeyH,
                    "i" => Code::KeyI,
                    "j" => Code::KeyJ,
                    "k" => Code::KeyK,
                    "l" => Code::KeyL,
                    "m" => Code::KeyM,
                    "n" => Code::KeyN,
                    "o" => Code::KeyO,
                    "p" => Code::KeyP,
                    "q" => Code::KeyQ,
                    "r" => Code::KeyR,
                    "s" => Code::KeyS,
                    "t" => Code::KeyT,
                    "u" => Code::KeyU,
                    "v" => Code::KeyV,
                    "w" => Code::KeyW,
                    "x" => Code::KeyX,
                    "y" => Code::KeyY,
                    "z" => Code::KeyZ,
                    "0" => Code::Digit0,
                    "1" => Code::Digit1,
                    "2" => Code::Digit2,
                    "3" => Code::Digit3,
                    "4" => Code::Digit4,
                    "5" => Code::Digit5,
                    "6" => Code::Digit6,
                    "7" => Code::Digit7,
                    "8" => Code::Digit8,
                    "9" => Code::Digit9,
                    "space" => Code::Space,
                    "enter" => Code::Enter,
                    "escape" => Code::Escape,
                    "tab" => Code::Tab,
                    "backspace" => Code::Backspace,
                    "delete" => Code::Delete,
                    "f1" => Code::F1,
                    "f2" => Code::F2,
                    "f3" => Code::F3,
                    "f4" => Code::F4,
                    "f5" => Code::F5,
                    "f6" => Code::F6,
                    "f7" => Code::F7,
                    "f8" => Code::F8,
                    "f9" => Code::F9,
                    "f10" => Code::F10,
                    "f11" => Code::F11,
                    "f12" => Code::F12,
                    _ => return Err(format!("不支持的按键: {}", key)),
                });
            }
        }
    }
    
    if let Some(code) = key_code {
        Ok(Shortcut::new(Some(modifiers), code))
    } else {
        Err("未找到有效的按键".to_string())
    }
}

// 窗口设置相关函数
#[tauri::command]
pub async fn load_window_settings_from_db(pool: State<'_, SqlitePool>) -> Result<WindowSettings, String> {
    load_window_settings_from_db_internal(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

pub async fn load_window_settings_from_db_internal(
    pool: &SqlitePool,
) -> Result<WindowSettings, sqlx::Error> {
    let result = sqlx::query_as::<_, WindowSettings>(
        "SELECT theme, window_position, opacity, always_on_top, accent_color, recent_days, default_startup FROM window_settings ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(settings) = result {
        Ok(settings)
    } else {
        Ok(get_default_window_settings())
    }
}

#[tauri::command]
pub async fn save_window_settings_to_db(pool: State<'_, SqlitePool>, settings: WindowSettings) -> Result<(), String> {
    println!("🔧 [Rust] save_window_settings_to_db 开始执行，设置: {:?}", settings);
    
    sqlx::query("DELETE FROM window_settings")
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 删除现有窗口设置失败: {}", e);
            e.to_string()
        })?;
    
    sqlx::query("INSERT INTO window_settings (theme, window_position, opacity, always_on_top, accent_color, recent_days, default_startup) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&settings.theme)
        .bind(&settings.window_position)
        .bind(settings.opacity)
        .bind(settings.always_on_top)
        .bind(&settings.accent_color)
        .bind(settings.recent_days)
        .bind(&settings.default_startup)
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 插入新窗口设置失败: {}", e);
            e.to_string()
        })?;
    
    Ok(())
}

// AI设置相关函数
#[tauri::command]
pub async fn load_ai_settings_from_db(pool: State<'_, SqlitePool>) -> Result<AISettings, String> {
    load_ai_settings_from_db_internal(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

pub async fn load_ai_settings_from_db_internal(
    pool: &SqlitePool,
) -> Result<AISettings, sqlx::Error> {
    let result = sqlx::query_as::<_, AISettings>(
        "SELECT api_key, base_url, model FROM ai_settings ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(settings) = result {
        Ok(settings)
    } else {
        Ok(get_default_ai_settings())
    }
}

#[tauri::command]
pub async fn save_ai_settings_to_db(pool: State<'_, SqlitePool>, settings: AISettings) -> Result<(), String> {
    println!("🔧 [Rust] save_ai_settings_to_db 开始执行，设置: {:?}", settings);
    
    sqlx::query("DELETE FROM ai_settings")
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 删除现有AI设置失败: {}", e);
            e.to_string()
        })?;
    
    sqlx::query("INSERT INTO ai_settings (api_key, base_url, model) VALUES (?, ?, ?)")
        .bind(&settings.api_key)
        .bind(&settings.base_url)
        .bind(&settings.model)
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 插入新AI设置失败: {}", e);
            e.to_string()
        })?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_ai_settings_for_test(pool: State<'_, SqlitePool>) -> Result<AISettings, String> {
    load_ai_settings_from_db_internal(pool.inner())
        .await
        .map_err(|e| format!("加载AI设置失败: {}", e))
}

// TODO颜色设置相关函数
#[tauri::command]
pub async fn load_todo_color_settings(pool: State<'_, SqlitePool>) -> Result<HashMap<String, String>, String> {
    load_todo_color_settings_internal(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

pub async fn load_todo_color_settings_internal(
    pool: &SqlitePool,
) -> Result<HashMap<String, String>, sqlx::Error> {
    let rows = sqlx::query("SELECT level_key, color_value FROM todo_color_settings")
        .fetch_all(pool)
        .await?;
    
    let mut colors = HashMap::new();
    for row in rows {
        let level_key: String = row.get("level_key");
        let color_value: String = row.get("color_value");
        colors.insert(level_key, color_value);
    }
    
    // 如果没有设置，返回默认颜色
    if colors.is_empty() {
        colors = get_default_todo_color_settings().colors;
    }
    
    Ok(colors)
}

#[tauri::command]
pub async fn save_todo_color_settings(pool: State<'_, SqlitePool>, colors: HashMap<String, String>) -> Result<(), String> {
    println!("🔧 [Rust] save_todo_color_settings 开始执行，颜色设置: {:?}", colors);
    
    // 删除现有设置
    sqlx::query("DELETE FROM todo_color_settings")
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 删除现有TODO颜色设置失败: {}", e);
            e.to_string()
        })?;
    
    // 插入新设置
    for (level_key, color_value) in colors {
        sqlx::query("INSERT INTO todo_color_settings (level_key, color_value) VALUES (?, ?)")
            .bind(&level_key)
            .bind(&color_value)
            .execute(pool.inner())
            .await
            .map_err(|e| {
                println!("❌ [Rust] 插入TODO颜色设置失败: {}", e);
                e.to_string()
            })?;
    }
    
    println!("✅ [Rust] TODO颜色设置保存成功");
    Ok(())
}

#[tauri::command]
pub async fn apply_todo_colors_to_main_window(
    colors: HashMap<String, String>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    if let Some(main_window) = app.get_webview_window("main") {
        for (level_key, color_value) in colors {
            let script = format!(
                "document.documentElement.style.setProperty('--{}-color', '{}')",
                level_key, color_value
            );
            
            if let Err(e) = main_window.eval(&script) {
                eprintln!("应用颜色到主窗口失败: {}", e);
            }
        }
    }
    
    Ok(())
}

// 天气设置相关函数
#[tauri::command]
pub async fn load_weather_settings_from_db(pool: State<'_, SqlitePool>) -> Result<WeatherSettings, String> {
    load_weather_settings_from_db_internal(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

pub async fn load_weather_settings_from_db_internal(
    pool: &SqlitePool,
) -> Result<WeatherSettings, sqlx::Error> {
    let result = sqlx::query_as::<_, WeatherSettings>(
        "SELECT enabled, amap_api_key, location_name, latitude, longitude, adcode, province, city, district FROM weather_settings ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(settings) = result {
        Ok(settings)
    } else {
        Ok(get_default_weather_settings())
    }
}

#[tauri::command]
pub async fn save_weather_settings_to_db(pool: State<'_, SqlitePool>, settings: WeatherSettings) -> Result<(), String> {
    println!("🔧 [Rust] save_weather_settings_to_db 开始执行，设置: {:?}", settings);
    
    sqlx::query("DELETE FROM weather_settings")
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 删除现有天气设置失败: {}", e);
            e.to_string()
        })?;
    
    sqlx::query("INSERT INTO weather_settings (enabled, amap_api_key, location_name, latitude, longitude, adcode, province, city, district) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(settings.enabled)
        .bind(&settings.api_key)
        .bind(&settings.location_name)
        .bind(settings.latitude)
        .bind(settings.longitude)
        .bind(&settings.adcode)
        .bind(&settings.province)
        .bind(&settings.city)
        .bind(&settings.district)
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 插入新天气设置失败: {}", e);
            e.to_string()
        })?;
    
    println!("✅ [Rust] 天气设置保存成功");
    Ok(())
}