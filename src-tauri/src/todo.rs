use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;
use chrono::{DateTime, Utc, Duration, Datelike};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    #[serde(rename = "startTime")]
    pub start_time: i64, // 开始时间戳（秒）
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>, // 结束时间戳（秒），可为空
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
    #[serde(rename = "startTime")]
    start_time: i64, // 开始时间戳（秒）
    #[serde(rename = "endTime")]
    end_time: Option<i64>, // 结束时间戳（秒），可为空
    notes: Option<String>,
    level: i64,
    cycle: String,
}

#[tauri::command]
pub async fn add_todo(pool: State<'_, SqlitePool>, params: AddTodoParams) -> Result<i64, String> {
    let result = sqlx::query(
        "INSERT INTO todos (title, start_time, end_time, notes, level, cycle) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(params.title)
    .bind(params.start_time)
    .bind(params.end_time)
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
    sqlx::query_as::<_, Todo>("SELECT id, title, start_time, end_time, notes, level, cycle, status, created_at, updated_at FROM todos WHERE status != 2 ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recent_todos(pool: State<'_, SqlitePool>, days: i64) -> Result<Vec<Todo>, String> {
    let now = chrono::Utc::now().timestamp();
    let cutoff_timestamp = now - (days * 24 * 60 * 60);
    
    // 首先处理到期的循环任务
    process_expired_todos(pool.inner()).await?;
    
    // 获取最近x天内的todos（开始时间在最近x天内），按优先级排序
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, start_time, end_time, notes, level, cycle, status, created_at, updated_at FROM todos WHERE status = 0 AND start_time >= ? AND start_time <= ? ORDER BY CASE level WHEN 1 THEN 1 WHEN 0 THEN 2 WHEN 3 THEN 3 WHEN 2 THEN 4 END, start_time ASC"
    )
    .bind(cutoff_timestamp)
    .bind(now)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(todos)
}

// 处理到期的循环任务
async fn process_expired_todos(pool: &SqlitePool) -> Result<(), String> {
    let now = chrono::Utc::now().timestamp();
    
    // 查找所有到期的循环任务
    let expired_todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, start_time, end_time, notes, level, cycle, status, created_at, updated_at FROM todos WHERE status = 0 AND start_time < ? AND cycle != 'one'"
    )
    .bind(now)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;
    
    for todo in expired_todos {
        // 归档旧的todo（设置为已完成）
        sqlx::query("UPDATE todos SET status = 1, updated_at = datetime('now') WHERE id = ?")
            .bind(todo.id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        
        // 根据cycle生成新的todo
        let new_start_time = calculate_next_cycle_time(todo.start_time, &todo.cycle);
        let new_end_time = todo.end_time.map(|end| {
            let duration = end - todo.start_time;
            new_start_time + duration
        });
        
        sqlx::query(
            "INSERT INTO todos (title, start_time, end_time, notes, level, cycle) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(todo.title)
        .bind(new_start_time)
        .bind(new_end_time)
        .bind(todo.notes)
        .bind(todo.level)
        .bind(todo.cycle)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

// 计算下一个循环时间
fn calculate_next_cycle_time(current_time: i64, cycle: &str) -> i64 {
    let dt = DateTime::<Utc>::from_timestamp(current_time, 0).unwrap_or_else(|| Utc::now());
    
    match cycle {
        "day" => (dt + Duration::days(1)).timestamp(),
        "week" => (dt + Duration::weeks(1)).timestamp(),
        "month" => {
            // 添加一个月，处理月份边界
            let mut year = dt.year();
            let mut month = dt.month() + 1;
            if month > 12 {
                month = 1;
                year += 1;
            }
            dt.with_year(year).unwrap_or(dt)
                .with_month(month).unwrap_or(dt)
                .timestamp()
        },
        "year" => (dt + Duration::days(365)).timestamp(),
        _ => current_time, // 默认不变
    }
}

#[tauri::command]
pub async fn update_todo(pool: State<'_, SqlitePool>, todo: Todo) -> Result<(), String> {
    sqlx::query("UPDATE todos SET title = ?, start_time = ?, end_time = ?, notes = ?, level = ?, cycle = ?, status = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(todo.title)
        .bind(todo.start_time)
        .bind(todo.end_time)
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