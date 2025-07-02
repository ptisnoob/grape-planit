use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;
use tauri_plugin_sql::{Migration, MigrationKind};


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
        },
        Migration {
            version: 3,
            description: "create_todos_table",
            sql: "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                start_time INTEGER NOT NULL, -- 开始时间戳（秒）
                end_time INTEGER, -- 结束时间戳（秒），可为空
                notes TEXT,
                level INTEGER NOT NULL DEFAULT 0,
                cycle TEXT NOT NULL DEFAULT 'one',
                status INTEGER NOT NULL DEFAULT 0, -- 0: pending, 1: completed, 2: deleted
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            kind: MigrationKind::Up,
        },

    ]
}

// 数据库操作函数
#[tauri::command]
pub async fn load_config_from_db(pool: State<'_, SqlitePool>) -> Result<CountdownConfig, String> {
    load_config_from_db_internal(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

pub async fn load_config_from_db_internal(
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
        // 如果没有配置，返回默认配置
        Ok(get_default_config())
    }
}

#[tauri::command]
pub async fn save_config_to_db(pool: State<'_, SqlitePool>, config: CountdownConfig) -> Result<(), String> {
    println!("🔧 [Rust] save_config_to_db 开始执行，配置: {:?}", config);
    
    // 删除现有配置
    println!("🔧 [Rust] 删除现有配置...");
    sqlx::query("DELETE FROM countdown_config")
        .execute(pool.inner())
        .await
        .map_err(|e| {
            println!("❌ [Rust] 删除现有配置失败: {}", e);
            e.to_string()
        })?;
    println!("🔧 [Rust] 成功删除现有配置");
    
    // 插入新配置
    println!("🔧 [Rust] 插入新配置...");
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
    println!("🔧 [Rust] 成功插入新配置");
    
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
        time_display_mode: "remaining".to_string(), // 保持和数据库默认值一致
    }
}