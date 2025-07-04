use crate::database::{self, CountdownConfig, save_countdown_record};
use chrono::{DateTime, Local, NaiveTime, TimeZone};
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
    println!("🔧 [Rust] update_countdown_config 被调用，配置: {:?}", config);
    
    // 保存到数据库
    match crate::config::save_countdown_config_to_db(pool.clone(), config.clone()).await {
        Ok(_) => {
            println!("🔧 [Rust] update_countdown_config 成功保存到数据库");
            Ok(())
        }
        Err(e) => {
            println!("❌ [Rust] update_countdown_config 保存失败: {}", e);
            Err(e)
        }
    }
}

// 计算下班倒计时
pub async fn calculate_work_end_countdown(pool: &SqlitePool) -> Option<CountdownData> {
    let config = match crate::config::load_countdown_config_from_db_internal(pool).await {
        Ok(config) => config,
        Err(_) => return None,
    };
    
    if config.work_end_time.is_empty() {
        return None;
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
                target_info: format!("下班时间：{}", config.work_end_time),
                status: "running".to_string(),
            })
        }
    } else {
        None
    }
}

// 计算自定义倒计时
pub async fn calculate_custom_countdown(pool: &SqlitePool) -> Option<CountdownData> {
    let config = match crate::config::load_countdown_config_from_db_internal(pool).await {
        Ok(config) => config,
        Err(_) => return None,
    };
    
    if config.custom_countdown.target.is_empty() {
        return None;
    }
    
    let now = Local::now();
    if let Ok(target_dt) = DateTime::parse_from_rfc3339(&config.custom_countdown.target) {
        let target_local = target_dt.with_timezone(&Local);
        let diff = target_local - now;
        let total_seconds = diff.num_seconds();
        
        if total_seconds <= 0 {
            // 倒计时结束，返回finished状态
            Some(CountdownData {
                mode: "custom".to_string(),
                timestamp: 0,
                target_info: format!(
                    "目标时间：{}",
                    target_local.format("%Y-%m-%d %H:%M:%S")
                ),
                status: "finished".to_string(),
            })
        } else {
            Some(CountdownData {
                mode: "custom".to_string(),
                timestamp: total_seconds,
                target_info: format!(
                    "目标时间：{}",
                    target_local.format("%Y-%m-%d %H:%M:%S")
                ),
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
    
    // 检查自定义倒计时
    if let Some(custom) = calculate_custom_countdown(pool).await {
        countdowns.push(custom);
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
        }
        "custom" => {
            if let Some(countdown) = calculate_custom_countdown(pool).await {
                countdown
            } else {
                CountdownData {
                    mode: "custom".to_string(),
                    timestamp: 0,
                    target_info: "请设置目标时间".to_string(),
                    status: "reset".to_string(),
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
    // 记录倒计时开始到数据库
    let config = crate::config::load_countdown_config_from_db_internal(&pool_clone).await.unwrap_or_else(|_| crate::config::get_default_countdown_config());
    if config.time_display_mode == "workEnd" && !config.work_end_time.is_empty() {
        let _ = save_countdown_record(
            pool.clone(),
            "workEnd".to_string(),
            Some(config.work_end_time.clone()),
            None,
        )
        .await;
    } else if config.time_display_mode == "custom" && !config.custom_countdown.target.is_empty() {
        let _ = save_countdown_record(
            pool.clone(),
            "custom".to_string(),
            Some(config.custom_countdown.target.clone()),
            None,
        )
        .await;
    }

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            // 获取当前配置以确定显示模式
            let config = crate::config::load_countdown_config_from_db_internal(&pool_clone).await
                .unwrap_or_else(|_| crate::config::get_default_countdown_config());
            
            // 根据当前显示模式发送对应的倒计时数据
            let countdown_data = match config.time_display_mode.as_str() {
                "workEnd" => {
                    if let Some(countdown) = calculate_work_end_countdown(&pool_clone).await {
                        countdown
                    } else {
                        CountdownData {
                            mode: "workEnd".to_string(),
                            timestamp: 0,
                            target_info: "请设置下班时间".to_string(),
                            status: "reset".to_string(),
                        }
                    }
                }
                "custom" => {
                    if let Some(countdown) = calculate_custom_countdown(&pool_clone).await {
                        countdown
                    } else {
                        CountdownData {
                            mode: "custom".to_string(),
                            timestamp: 0,
                            target_info: "请设置目标时间".to_string(),
                            status: "reset".to_string(),
                        }
                    }
                }
                _ => {
                    // current模式不需要发送倒计时数据
                    continue;
                }
            };
            
            // 发送倒计时更新事件
            if let Err(e) = app_handle.emit("countdown-update", countdown_data) {
                eprintln!("Failed to emit countdown-update event: {}", e);
            }
        }
    });

    Ok(())
}
