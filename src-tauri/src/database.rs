use sqlx::SqlitePool;
use tauri::State;
use tauri_plugin_sql::{Migration, MigrationKind};

// 重新导出config模块的类型
pub use crate::config::*;

// 数据库初始化配置 - 使用最新版本的表结构
pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "initialize_database_schema",
            sql: "
                -- 倒计时配置表
                CREATE TABLE IF NOT EXISTS countdown_config (
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
                
                -- 倒计时记录表
                CREATE TABLE IF NOT EXISTS countdown_records (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    mode TEXT NOT NULL,
                    target_time DATETIME,
                    duration INTEGER,
                    status TEXT NOT NULL DEFAULT 'running',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    finished_at DATETIME
                );
                
                -- 待办事项表
                CREATE TABLE IF NOT EXISTS todos (
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
                );
                
                -- 窗口设置表
                CREATE TABLE IF NOT EXISTS window_settings (
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
                );
                
                -- AI设置表
                CREATE TABLE IF NOT EXISTS ai_settings (
                    id INTEGER PRIMARY KEY,
                    api_key TEXT NOT NULL DEFAULT '',
                    base_url TEXT NOT NULL DEFAULT 'https://open.bigmodel.cn/api/paas/v4',
                    model TEXT NOT NULL DEFAULT 'glm-4-flash-250414',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );
                
                -- 待办事项颜色设置表
                CREATE TABLE IF NOT EXISTS todo_color_settings (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    level_key TEXT NOT NULL UNIQUE,
                    color_value TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );
                
                -- 快捷键设置表
                CREATE TABLE IF NOT EXISTS shortcut_settings (
                    id INTEGER PRIMARY KEY,
                    toggle_window TEXT NOT NULL DEFAULT 'Alt+G',
                    quick_add_todo TEXT NOT NULL DEFAULT 'Alt+N',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );
                
                -- 天气设置表
                CREATE TABLE IF NOT EXISTS weather_settings (
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
                
                -- 节假日年份表
                CREATE TABLE IF NOT EXISTS holiday_years (
                    year INTEGER PRIMARY KEY,
                    papers TEXT NOT NULL,
                    sync_time DATETIME NOT NULL,
                    count INTEGER NOT NULL DEFAULT 0
                );
                
                -- 节假日表
                CREATE TABLE IF NOT EXISTS holidays (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    year INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    date TEXT NOT NULL,
                    is_off_day BOOLEAN NOT NULL,
                    FOREIGN KEY (year) REFERENCES holiday_years (year) ON DELETE CASCADE
                );
                
                -- 代理设置表
                CREATE TABLE IF NOT EXISTS proxy_settings (
                    id INTEGER PRIMARY KEY,
                    enabled BOOLEAN NOT NULL DEFAULT 0,
                    proxy_url TEXT NOT NULL DEFAULT '',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );
                
                -- 励志语缓存表
                CREATE TABLE IF NOT EXISTS motivation_cache (
                    id INTEGER PRIMARY KEY,
                    content TEXT NOT NULL,
                    cache_date TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );
                
                -- 创建索引
                CREATE INDEX IF NOT EXISTS idx_holidays_year ON holidays (year);
                CREATE INDEX IF NOT EXISTS idx_holidays_date ON holidays (date);
                CREATE INDEX IF NOT EXISTS idx_todos_status ON todos (status);
                CREATE INDEX IF NOT EXISTS idx_todos_level ON todos (level);
                CREATE INDEX IF NOT EXISTS idx_countdown_records_mode ON countdown_records (mode);
            ",
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