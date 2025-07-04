use sqlx::SqlitePool;
use tauri::State;
use tauri_plugin_sql::{Migration, MigrationKind};

// 重新导出config模块的类型
pub use crate::config::*;

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
        Migration {
            version: 4,
            description: "create_window_settings_table",
            sql: "CREATE TABLE IF NOT EXISTS window_settings (
                id INTEGER PRIMARY KEY,
                theme TEXT NOT NULL DEFAULT 'auto',
                window_position TEXT NOT NULL DEFAULT 'bottom-right',
                opacity REAL NOT NULL DEFAULT 0.35,
                always_on_top BOOLEAN NOT NULL DEFAULT 1,
                accent_color TEXT NOT NULL DEFAULT '#007bff',
                recent_days INTEGER NOT NULL DEFAULT 5,
                default_startup TEXT NOT NULL DEFAULT 'auto',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 5,
            description: "create_ai_settings_table",
            sql: "CREATE TABLE IF NOT EXISTS ai_settings (
                id INTEGER PRIMARY KEY,
                api_key TEXT NOT NULL DEFAULT '',
                base_url TEXT NOT NULL DEFAULT 'https://api.openai.com/v1',
                model TEXT NOT NULL DEFAULT 'gpt-3.5-turbo',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 6,
            description: "create_todo_color_settings_table",
            sql: "CREATE TABLE IF NOT EXISTS todo_color_settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                level_key TEXT NOT NULL UNIQUE,
                color_value TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            kind: MigrationKind::Up,
        },
        // Migration version 7 and 8 removed - recent_days and default_startup columns are now included in the initial window_settings table creation

    ]
}

// 数据库操作函数 - 只保留倒计时记录相关

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