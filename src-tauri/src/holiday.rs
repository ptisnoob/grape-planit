use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};
use tauri::State;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holiday {
    pub name: String,
    pub date: String, // ISO 8601 格式
    #[serde(rename = "isOffDay")]
    pub is_off_day: bool,
}

#[derive(Debug, Clone, FromRow)]
pub struct HolidayRow {
    pub name: String,
    pub date: String,
    pub is_off_day: bool,
}

impl From<HolidayRow> for Holiday {
    fn from(row: HolidayRow) -> Self {
        Holiday {
            name: row.name,
            date: row.date,
            is_off_day: row.is_off_day,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayYear {
    pub year: i32,
    pub papers: Vec<String>,
    pub days: Vec<Holiday>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoredHolidayYear {
    pub year: i32,
    #[serde(rename = "syncTime")]
    pub sync_time: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProxySettings {
    pub id: i32,
    pub enabled: bool,
    #[serde(rename = "proxyUrl")]
    pub proxy_url: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateProxyParams {
    pub enabled: bool,
    pub proxy_url: String,
}

#[derive(serde::Deserialize)]
pub struct SyncHolidayParams {
    pub year: i32,
}

/// 从GitHub获取节假日数据
async fn fetch_holiday_data(year: i32, proxy_url: Option<String>) -> Result<HolidayYear, String> {
    let base_url = "https://raw.githubusercontent.com/NateScarlet/holiday-cn/master".to_string();
    let file_url = format!("{}/{}.json", base_url, year);
    
    let url = if let Some(proxy) = proxy_url {
        if proxy.trim().is_empty() {
            file_url
        } else {
            let proxy = proxy.trim_end_matches('/');
            format!("{}/{}", proxy, file_url)
        }
    } else {
        file_url
    };
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP错误: {}", response.status()));
    }
    
    let text = response.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    
    if text.trim().is_empty() {
        return Err("数据为空，可能该年份数据尚未发布".to_string());
    }
    
    serde_json::from_str::<HolidayYear>(&text)
        .map_err(|e| format!("解析JSON失败: {}", e))
}

#[tauri::command]
pub async fn sync_holiday_data(pool: State<'_, SqlitePool>, params: SyncHolidayParams) -> Result<(), String> {
    // 获取代理配置
    let proxy_url = match get_proxy_settings_internal(pool.inner()).await? {
        Some(settings) if settings.enabled => Some(settings.proxy_url),
        _ => None,
    };
    
    // 获取节假日数据
    let holiday_data = fetch_holiday_data(params.year, proxy_url).await?;
    
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    
    // 删除已存在的数据
    sqlx::query("DELETE FROM holidays WHERE year = ?")
        .bind(params.year)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    
    sqlx::query("DELETE FROM holiday_years WHERE year = ?")
        .bind(params.year)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    
    // 插入年份信息
    let papers_json = serde_json::to_string(&holiday_data.papers)
        .map_err(|e| format!("序列化papers失败: {}", e))?;
    
    sqlx::query(
        "INSERT INTO holiday_years (year, papers, sync_time, count) VALUES (?, ?, datetime('now'), ?)"
    )
    .bind(params.year)
    .bind(papers_json)
    .bind(holiday_data.days.len() as i32)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    
    // 插入节假日数据
    for holiday in holiday_data.days {
        sqlx::query(
            "INSERT INTO holidays (year, name, date, is_off_day) VALUES (?, ?, ?, ?)"
        )
        .bind(params.year)
        .bind(holiday.name)
        .bind(holiday.date)
        .bind(holiday.is_off_day)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }
    
    tx.commit().await.map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_stored_holiday_years(pool: State<'_, SqlitePool>) -> Result<Vec<StoredHolidayYear>, String> {
    let years = sqlx::query_as::<_, StoredHolidayYear>(
        "SELECT year, sync_time, count FROM holiday_years ORDER BY year DESC"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(years)
}

#[tauri::command]
pub async fn get_holidays_by_year(pool: State<'_, SqlitePool>, year: i32) -> Result<Option<HolidayYear>, String> {
    // 获取年份信息
    let year_info = sqlx::query(
        "SELECT papers FROM holiday_years WHERE year = ?"
    )
    .bind(year)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    if year_info.is_none() {
        return Ok(None);
    }
    
    let papers_json: String = year_info.unwrap().get("papers");
    let papers: Vec<String> = serde_json::from_str(&papers_json)
        .map_err(|e| format!("解析papers失败: {}", e))?;
    
    // 获取节假日数据
    let holiday_rows = sqlx::query_as::<_, HolidayRow>(
        "SELECT name, date, is_off_day FROM holidays WHERE year = ? ORDER BY date"
    )
    .bind(year)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let holidays: Vec<Holiday> = holiday_rows.into_iter().map(Holiday::from).collect();
    
    Ok(Some(HolidayYear {
        year,
        papers,
        days: holidays,
    }))
}

#[tauri::command]
pub async fn delete_holiday_year(pool: State<'_, SqlitePool>, year: i32) -> Result<(), String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    
    sqlx::query("DELETE FROM holidays WHERE year = ?")
        .bind(year)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    
    sqlx::query("DELETE FROM holiday_years WHERE year = ?")
        .bind(year)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    
    tx.commit().await.map_err(|e| e.to_string())?;
    
    Ok(())
}

/// 内部函数：获取代理配置
async fn get_proxy_settings_internal(pool: &SqlitePool) -> Result<Option<ProxySettings>, String> {
    let settings = sqlx::query_as::<_, ProxySettings>(
        "SELECT id, enabled, proxy_url, created_at, updated_at FROM proxy_settings WHERE id = 1"
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(settings)
}

#[tauri::command]
pub async fn get_proxy_settings(pool: State<'_, SqlitePool>) -> Result<Option<ProxySettings>, String> {
    get_proxy_settings_internal(pool.inner()).await
}

#[tauri::command]
pub async fn save_proxy_settings(pool: State<'_, SqlitePool>, params: UpdateProxyParams) -> Result<(), String> {
    // 使用 INSERT OR REPLACE 来确保只有一条记录
    sqlx::query(
        "INSERT OR REPLACE INTO proxy_settings (id, enabled, proxy_url, updated_at) VALUES (1, ?, ?, datetime('now'))"
    )
    .bind(params.enabled)
    .bind(params.proxy_url)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}