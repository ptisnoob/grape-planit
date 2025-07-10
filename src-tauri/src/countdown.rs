use crate::database::{CountdownConfig, save_countdown_record};
use chrono::{ Local, NaiveTime, TimeZone};
use serde::Serialize;
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter, State};
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Serialize)]
pub struct CountdownData {
    pub mode: String,
    pub timestamp: i64,
    pub target_info: String,
    pub status: String, // "running", "finished", "reset"
}

#[tauri::command]
pub async fn update_countdown_config(
    pool: State<'_, SqlitePool>,
    config: CountdownConfig,
) -> Result<(), String> {
    // 保存到数据库
    match crate::config::save_countdown_config_to_db(pool.clone(), config.clone()).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

// 计算下班倒计时
pub async fn calculate_work_end_countdown(pool: &SqlitePool) -> Option<CountdownData> {
    let config = match crate::config::load_countdown_config_from_db_internal(pool).await {
        Ok(config) => config,
        Err(_) => return None,
    };
    
    if !config.enable_work_end_countdown {
        return None;
    }
    
    if config.work_end_time.is_empty() {
        return None;
    }
    
    // 检查今天是否有重置记录
    let today_start = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
    let today_start_str = today_start.format("%Y-%m-%d %H:%M:%S").to_string();
    
    let reset_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM countdown_records WHERE mode = 'workEnd_reset' AND created_at >= ?"
    )
    .bind(today_start_str)
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    
    // 如果今天有重置记录，返回重置状态
    if reset_count > 0 {
        return Some(CountdownData {
            mode: "workEnd".to_string(),
            timestamp: 0,
            target_info: "已重置到明天".to_string(),
            status: "reset".to_string(),
        });
    }
    
    let now = Local::now();
    
    if let Ok(work_time) = NaiveTime::parse_from_str(&config.work_end_time, "%H:%M") {
        let work_end = now.date_naive().and_time(work_time);
        let work_end_dt = Local.from_local_datetime(&work_end).unwrap();
        
        let diff = work_end_dt - now;
        let total_seconds = diff.num_seconds();
        
        
        if total_seconds <= 0 {
            Some(CountdownData {
                mode: "workEnd".to_string(),
                timestamp: 0,
                target_info: "下班".to_string(),
                status: "finished".to_string(),
            })
        } else {
            Some(CountdownData {
                mode: "workEnd".to_string(),
                timestamp: total_seconds,
                target_info: "下班".to_string(),
                status: "running".to_string(),
            })
        }
    } else {
        None
    }
}



// 获取所有有效的倒计时
pub async fn get_all_countdowns(pool: &SqlitePool) -> Vec<CountdownData> {
    let mut countdowns = Vec::new();
    
    // 检查下班倒计时
    if let Some(work_end) = calculate_work_end_countdown(pool).await {
        countdowns.push(work_end);
    }
    
    countdowns
}

// 保持向后兼容的函数
pub async fn calculate_countdown_timestamp(pool: &SqlitePool) -> CountdownData {
    let config = match crate::config::load_countdown_config_from_db_internal(pool).await {
        Ok(config) => config,
        Err(_) => return CountdownData {
            mode: "error".to_string(),
            timestamp: 0,
            target_info: "无法加载配置".to_string(),
            status: "reset".to_string(),
        },
    };
    
    // 根据当前显示模式返回对应的倒计时
    match config.time_display_mode.as_str() {
        "workEnd" => {
            if config.enable_work_end_countdown {
                if let Some(countdown) = calculate_work_end_countdown(pool).await {
                    countdown
                } else {
                    CountdownData {
                        mode: "workEnd".to_string(),
                        timestamp: 0,
                        target_info: "请设置下班时间".to_string(),
                        status: "reset".to_string(),
                    }
                }
            } else {
                CountdownData {
                    mode: "current".to_string(),
                    timestamp: 0,
                    target_info: String::new(),
                    status: "running".to_string(),
                }
            }
        }
        _ => CountdownData {
            mode: "current".to_string(),
            timestamp: 0,
            target_info: String::new(),
            status: "running".to_string(),
        },
    }
}

#[tauri::command]
pub async fn start_countdown_timer(
    app_handle: AppHandle,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    let pool_clone = pool.inner().clone();
    
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            // 获取当前配置
            let config = crate::config::load_countdown_config_from_db_internal(&pool_clone).await
                .unwrap_or_else(|_| crate::config::get_default_countdown_config());
            
            // 如果启用了下班倒计时，就发送倒计时数据
            if config.enable_work_end_countdown {
                let countdown_data = if let Some(countdown) = calculate_work_end_countdown(&pool_clone).await {
                    countdown
                } else {
                    CountdownData {
                        mode: "workEnd".to_string(),
                        timestamp: 0,
                        target_info: "请设置下班时间".to_string(),
                        status: "reset".to_string(),
                    }
                };
                
                // 发送倒计时更新事件
                if let Err(e) = app_handle.emit("countdown-update", countdown_data) {
                    eprintln!("Failed to emit countdown-update event: {}", e);
                }
            }
        }
    });

    Ok(())
}

// 重置下班倒计时到下一天
#[tauri::command]
pub async fn reset_work_end_countdown_to_next_day(
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    // 加载当前配置
    let config = match crate::config::load_countdown_config_from_db_internal(pool.inner()).await {
        Ok(config) => config,
        Err(e) => return Err(format!("无法加载配置: {}", e)),
    };
    
    // 记录重置事件到数据库
    let _ = save_countdown_record(
        pool.clone(),
        "workEnd_reset".to_string(),
        Some(format!("重置到下一天: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))),
        None,
    )
    .await;
    
    Ok(())
}
