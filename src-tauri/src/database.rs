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
        Migration {
            version: 7,
            description: "create_shortcut_settings_table",
            sql: "CREATE TABLE IF NOT EXISTS shortcut_settings (
                id INTEGER PRIMARY KEY,
                toggle_window TEXT NOT NULL DEFAULT 'Alt+G',
                quick_add_todo TEXT NOT NULL DEFAULT 'Alt+N',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 8,
            description: "create_weather_settings_table",
            sql: "CREATE TABLE IF NOT EXISTS weather_settings (
                id INTEGER PRIMARY KEY,
                amap_api_key TEXT NOT NULL DEFAULT '',
                location_name TEXT NOT NULL DEFAULT '',
                longitude REAL,
                latitude REAL,
                adcode TEXT NOT NULL DEFAULT '',
                province TEXT NOT NULL DEFAULT '',
                city TEXT NOT NULL DEFAULT '',
                district TEXT NOT NULL DEFAULT '',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 9,
            description: "recreate_weather_settings_with_enabled",
            sql: "CREATE TABLE IF NOT EXISTS weather_settings_new (
                      id INTEGER PRIMARY KEY,
                      amap_api_key TEXT NOT NULL DEFAULT '',
                      location_name TEXT NOT NULL DEFAULT '',
                      longitude REAL,
                      latitude REAL,
                      adcode TEXT NOT NULL DEFAULT '',
                      province TEXT NOT NULL DEFAULT '',
                      city TEXT NOT NULL DEFAULT '',
                      district TEXT NOT NULL DEFAULT '',
                      enabled BOOLEAN NOT NULL DEFAULT 0,
                      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                  );
                  INSERT OR IGNORE INTO weather_settings_new (id, amap_api_key, location_name, longitude, latitude, adcode, province, city, district, enabled, created_at, updated_at)
                  SELECT id, amap_api_key, location_name, longitude, latitude, adcode, province, city, district, 0 as enabled, created_at, updated_at
                  FROM weather_settings WHERE EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='weather_settings');
                  DROP TABLE IF EXISTS weather_settings;
                  ALTER TABLE weather_settings_new RENAME TO weather_settings;",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 10,
            description: "update_countdown_config_table_structure",
            sql: "CREATE TABLE IF NOT EXISTS countdown_config_new (
                id INTEGER PRIMARY KEY,
                work_end_time TEXT NOT NULL DEFAULT '',
                enable_work_end_countdown BOOLEAN NOT NULL DEFAULT 0,
                final_countdown_minutes INTEGER NOT NULL DEFAULT 1,
                end_state_keep_minutes INTEGER NOT NULL DEFAULT 5,
                work_days TEXT NOT NULL DEFAULT 'double',
                show_seconds BOOLEAN NOT NULL DEFAULT 1,
                time_display_mode TEXT NOT NULL DEFAULT 'current',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            INSERT OR IGNORE INTO countdown_config_new (id, work_end_time, enable_work_end_countdown, final_countdown_minutes, end_state_keep_minutes, work_days, show_seconds, time_display_mode, created_at, updated_at)
            SELECT id, work_end_time, 
                   COALESCE((SELECT 1 FROM countdown_config c2 WHERE c2.id = countdown_config.id AND c2.work_end_time != ''), 0) as enable_work_end_countdown,
                   1 as final_countdown_minutes,
                   5 as end_state_keep_minutes,
                   'double' as work_days,
                   show_seconds, time_display_mode, created_at, updated_at
            FROM countdown_config WHERE EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='countdown_config');
            DROP TABLE IF EXISTS countdown_config;
            ALTER TABLE countdown_config_new RENAME TO countdown_config;",
            kind: MigrationKind::Up,
        },

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