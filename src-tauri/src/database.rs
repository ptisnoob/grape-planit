use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;
use tauri_plugin_sql::{Migration, MigrationKind};


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomCountdown {
    pub name: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CountdownConfig {
    #[serde(rename = "workEndTime")]
    pub work_end_time: String,
    #[sqlx(flatten)]
    pub custom_countdown: CustomCountdown,
    #[serde(rename = "showSeconds")]
    pub show_seconds: bool,
    #[serde(rename = "timeDisplayMode")]
    pub time_display_mode: String,
}

// 数据库迁移配置
pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create_countdown_config_table",
            sql: "CREATE TABLE IF NOT EXISTS countdown_config (
                id INTEGER PRIMARY KEY,
                work_end_time TEXT NOT NULL,
                custom_countdown_name TEXT NOT NULL,
                custom_countdown_target TEXT NOT NULL,
                show_seconds BOOLEAN NOT NULL DEFAULT 1,
                time_display_mode TEXT NOT NULL DEFAULT 'remaining',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_countdown_records_table",
            sql: "CREATE TABLE IF NOT EXISTS countdown_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mode TEXT NOT NULL,
                target_time DATETIME,
                duration INTEGER,
                status TEXT NOT NULL DEFAULT 'running',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                finished_at DATETIME
            );",
            kind: MigrationKind::Up,
        }
    ]
}

// 数据库操作函数
#[tauri::command]
pub async fn load_config_from_db(pool: State<'_, SqlitePool>) -> Result<CountdownConfig, String> {
        let result = sqlx::query_as::<_, CountdownConfig>("SELECT work_end_time, custom_countdown_name as name, custom_countdown_target as target, show_seconds, time_display_mode FROM countdown_config ORDER BY id DESC LIMIT 1")
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    if let Some(config) = result {
        Ok(config)
    } else {
        Ok(get_default_config())
    }
}

#[tauri::command]
pub async fn save_config_to_db(pool: State<'_, SqlitePool>, config: CountdownConfig) -> Result<(), String> {
    sqlx::query("DELETE FROM countdown_config").execute(pool.inner()).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO countdown_config (work_end_time, custom_countdown_name, custom_countdown_target, show_seconds, time_display_mode) VALUES (?, ?, ?, ?, ?)")
        .bind(config.work_end_time)
        .bind(config.custom_countdown.name)
        .bind(config.custom_countdown.target)
        .bind(config.show_seconds)
        .bind(config.time_display_mode)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?; 
    
    Ok(())
}

#[tauri::command]
pub async fn save_countdown_record(pool: State<'_, SqlitePool>, mode: String, target_time: Option<String>, duration: Option<i64>) -> Result<(), String> {
    sqlx::query("INSERT INTO countdown_records (mode, target_time, duration, created_at) VALUES (?, ?, ?, datetime('now'))")
        .bind(mode)
        .bind(target_time)
        .bind(duration)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn get_default_config() -> CountdownConfig {
    CountdownConfig {
        work_end_time: String::new(),
        custom_countdown: CustomCountdown {
            name: "自定义事件".to_string(),
            target: String::new(),
        },
        show_seconds: true,
        time_display_mode: "current".to_string(),
    }
}