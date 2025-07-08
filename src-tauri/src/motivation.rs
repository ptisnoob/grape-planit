use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MotivationCache {
    pub id: Option<i64>,
    pub content: String,
    pub cache_date: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotivationContent {
    pub content: String,
}

// 获取今日励志文案缓存
#[tauri::command]
pub async fn get_today_motivation_cache(
    pool: State<'_, SqlitePool>,
) -> Result<Option<MotivationContent>, String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    
    let result = sqlx::query_as::<_, MotivationCache>(
        "SELECT id, content, cache_date, created_at, updated_at FROM motivation_cache WHERE cache_date = ?"
    )
    .bind(today)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(result.map(|cache| MotivationContent {
        content: cache.content,
    }))
}

/// 保存今日励志文案缓存
#[tauri::command]
pub async fn save_today_motivation_cache(
    pool: State<'_, SqlitePool>,
    content: String,
) -> Result<(), String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    sqlx::query(
        "INSERT OR REPLACE INTO motivation_cache (id, content, cache_date, created_at, updated_at)
         VALUES (
             (SELECT id FROM motivation_cache WHERE cache_date = ?),
             ?, ?, 
             COALESCE((SELECT created_at FROM motivation_cache WHERE cache_date = ?), ?),
             ?
         )"
    )
    .bind(&today)
    .bind(&content)
    .bind(&today)
    .bind(&today)
    .bind(&now)
    .bind(&now)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// 清理7天前的励志文案缓存
#[tauri::command]
pub async fn cleanup_motivation_cache(
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    let seven_days_ago = Local::now()
        .checked_sub_signed(chrono::Duration::days(7))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    
    sqlx::query(
        "DELETE FROM motivation_cache WHERE cache_date < ?"
    )
    .bind(seven_days_ago)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}