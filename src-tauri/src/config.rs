use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomCountdown {
    #[serde(rename = "name")]
    #[sqlx(rename = "custom_countdown_name")]
    pub name: String,
    #[serde(rename = "target")]
    #[sqlx(rename = "custom_countdown_target")]
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountdownConfig {
    #[serde(rename = "workEndTime")]
    #[sqlx(rename = "work_end_time")]
    pub work_end_time: String,
    #[serde(rename = "customCountdown")]
    #[sqlx(flatten)]
    pub custom_countdown: CustomCountdown,
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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AISettings {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

// 默认配置函数
pub fn get_default_countdown_config() -> CountdownConfig {
    CountdownConfig {
        work_end_time: String::new(),
        custom_countdown: CustomCountdown {
            name: "自定义事件".to_string(),
            target: String::new(),
        },
        show_seconds: true,
        time_display_mode: "remaining".to_string(),
    }
}

pub fn get_default_window_settings() -> WindowSettings {
    WindowSettings {
        theme: "auto".to_string(),
        window_position: "bottom-right".to_string(),
        opacity: 0.35,
        always_on_top: true,
        accent_color: "#007bff".to_string(), // 默认蓝色主题
    }
}

pub fn get_default_ai_settings() -> AISettings {
    AISettings {
        api_key: String::new(),
        base_url: "https://api.openai.com/v1".to_string(),
        model: "gpt-3.5-turbo".to_string(),
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
        "SELECT work_end_time, custom_countdown_name, custom_countdown_target, show_seconds, time_display_mode FROM countdown_config ORDER BY id DESC LIMIT 1",
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
    
    sqlx::query("INSERT INTO countdown_config (work_end_time, custom_countdown_name, custom_countdown_target, show_seconds, time_display_mode) VALUES (?, ?, ?, ?, ?)")
        .bind(&config.work_end_time)
        .bind(&config.custom_countdown.name)
        .bind(&config.custom_countdown.target)
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
        "SELECT theme, window_position, opacity, always_on_top, accent_color FROM window_settings ORDER BY id DESC LIMIT 1",
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
    
    sqlx::query("INSERT INTO window_settings (theme, window_position, opacity, always_on_top, accent_color) VALUES (?, ?, ?, ?, ?)")
        .bind(&settings.theme)
        .bind(&settings.window_position)
        .bind(settings.opacity)
        .bind(settings.always_on_top)
        .bind(&settings.accent_color)
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