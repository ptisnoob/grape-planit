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
                due_date TEXT,
                notes TEXT,
                level INTEGER NOT NULL DEFAULT 0,
                cycle TEXT NOT NULL DEFAULT 'one',
                status INTEGER NOT NULL DEFAULT 0, -- 0: pending, 1: completed, 2: deleted
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            kind: MigrationKind::Up,
        }
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

// ================= Todo 相关 =================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub due_date: String, // 使用字符串存储日期，例如 "YYYY-MM-DD HH:MM"
    pub notes: Option<String>,
    pub level: i64, // 0: 重要不紧急, 1: 重要且紧急, 2: 不重要不紧急, 3: 不重要但紧急
    pub cycle: String, // one, day, week, month, year
    pub status: i64, // 0: pending, 1: completed, 2: deleted
    pub created_at: String,
    pub updated_at: String,
}



#[derive(serde::Deserialize)]
pub struct AddTodoParams {
    title: String,
    #[serde(rename = "due_date")]
    due_date: String,
    notes: Option<String>,
    level: i64,
    cycle: String,
}

#[tauri::command]
pub async fn add_todo(pool: State<'_, SqlitePool>, params: AddTodoParams) -> Result<i64, String> {
    let result = sqlx::query(
        "INSERT INTO todos (title, due_date, notes, level, cycle) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(params.title)
    .bind(params.due_date)
    .bind(params.notes)
    .bind(params.level)
    .bind(params.cycle)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn get_all_todos(pool: State<'_, SqlitePool>) -> Result<Vec<Todo>, String> {
    sqlx::query_as::<_, Todo>("SELECT id, title, due_date, notes, level, cycle, status, created_at, updated_at FROM todos WHERE status != 2 ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_todo(pool: State<'_, SqlitePool>, todo: Todo) -> Result<(), String> {
    sqlx::query("UPDATE todos SET title = ?, due_date = ?, notes = ?, level = ?, cycle = ?, status = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(todo.title)
        .bind(todo.due_date)
        .bind(todo.notes)
        .bind(todo.level)
        .bind(todo.cycle)
        .bind(todo.status)
        .bind(todo.id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_todo(pool: State<'_, SqlitePool>, id: i64) -> Result<(), String> {
    // 逻辑删除
    sqlx::query("UPDATE todos SET status = 2, updated_at = datetime('now') WHERE id = ?")
        .bind(id)
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