use tauri::{Manager, Runtime, WebviewWindowBuilder, WebviewUrl, PhysicalPosition};
use serde::Serialize;

#[derive(Serialize)]
pub struct MonitorInfo {
    pub name: String,
    pub index: usize,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub is_primary: bool,
}

#[tauri::command]
pub async fn show_settings_window<R: Runtime>(app_handle: tauri::AppHandle<R>) -> Result<(), String> {
    // 获取已配置的 settings 窗口
    if let Some(window) = app_handle.webview_windows().get("settings") {
        // 如果窗口已存在，显示并聚焦
        window.show().map_err(|e| format!("显示窗口失败: {}", e))?;
        window.set_focus().map_err(|e| format!("聚焦窗口失败: {}", e))?;
    } else {
        // 如果窗口不存在，创建新窗口
        WebviewWindowBuilder::new(&app_handle, "settings", WebviewUrl::App("/settings".into()))
            .title("设置")
            .inner_size(570.0, 400.0)
            .resizable(true)
            .build()
            .map_err(|e| format!("创建窗口失败: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn set_main_window_position<R: Runtime>(app_handle: tauri::AppHandle<R>, position: String) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        set_main_window_position_internal(window, position).await
    } else {
        Err("主窗口未找到".to_string())
    }
}

pub async fn set_main_window_position_internal<R: Runtime>(window: tauri::WebviewWindow<R>, position: String) -> Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    let monitor_size = monitor.size();
    let window_size = window.outer_size().map_err(|e| e.to_string())?;

    // 获取任务栏高度（Windows上通常为40像素）
    let taskbar_height = 40;

    let (x, y) = match position.as_str() {
        "top-left" => (0, 0),
        "top-right" => (monitor_size.width as i32, 0),
        "bottom-left" => (
            0,
            monitor_size.height as i32 - window_size.height as i32 - taskbar_height,
        ),
        "bottom-right" => (
            monitor_size.width as i32,
            monitor_size.height as i32 - window_size.height as i32 - taskbar_height,
        ),
        "center" => (
            (monitor_size.width as i32 - window_size.width as i32) / 2,
            (monitor_size.height as i32 - window_size.height as i32) / 2,
        ),
        _ => return Err("Invalid position".to_string()),
    };

    // 调整x坐标，确保窗口完全贴边
    let adjusted_x = if x == monitor_size.width as i32 {
        x - window_size.width as i32
    } else {
        x
    };

    window
        .set_position(PhysicalPosition::new(adjusted_x, y))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn set_window_opacity<R: Runtime>(app_handle: tauri::AppHandle<R>, opacity: f64) -> Result<(), String> {
    // 通过修改CSS变量来实现透明度调整
    if let Some(window) = app_handle.get_webview_window("main") {
        let script = format!(
            r#"
            document.documentElement.style.setProperty('--bg-primary-opacity', '{}');
            document.documentElement.style.setProperty('--bg-secondary-opacity', '{}');
            "#,
            opacity, opacity
        );
        window.eval(&script).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("主窗口未找到".to_string())
    }
}

#[tauri::command]
pub async fn set_always_on_top<R: Runtime>(app_handle: tauri::AppHandle<R>, always_on_top: bool) -> Result<(), String> {
    if let Some(window) = app_handle.webview_windows().get("main") {
        window.set_always_on_top(always_on_top)
            .map_err(|e| format!("设置窗口置顶失败: {}", e))?;
    }
    
    Ok(())
}



#[tauri::command]
pub async fn eval_script_in_main_window<R: Runtime>(app_handle: tauri::AppHandle<R>, script: String) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.eval(&script).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("主窗口未找到".to_string())
    }
}

#[tauri::command]
pub async fn get_monitors<R: Runtime>(app_handle: tauri::AppHandle<R>) -> Result<Vec<MonitorInfo>, String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let monitors = window.available_monitors().map_err(|e| e.to_string())?;
        let primary_monitor = window.primary_monitor().map_err(|e| e.to_string())?;
        
        let mut monitor_infos = Vec::new();
        for (index, monitor) in monitors.iter().enumerate() {
            let is_primary = if let Some(ref primary) = primary_monitor {
                monitor.name() == primary.name()
            } else {
                index == 0
            };
            
            monitor_infos.push(MonitorInfo {
                name: monitor.name().map(|s| s.to_string()).unwrap_or_else(|| format!("显示器 {}", index + 1)),
                index,
                size: (monitor.size().width, monitor.size().height),
                position: (monitor.position().x, monitor.position().y),
                is_primary,
            });
        }
        
        Ok(monitor_infos)
    } else {
        Err("主窗口未找到".to_string())
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_window_monitor<R: Runtime>(app_handle: tauri::AppHandle<R>, monitor_index: usize, position: String) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let monitors = window.available_monitors().map_err(|e| e.to_string())?;
        
        if monitor_index >= monitors.len() {
            return Err("显示器索引超出范围".to_string());
        }
        
        let target_monitor = &monitors[monitor_index];
        let monitor_size = target_monitor.size();
        let monitor_position = target_monitor.position();
        let window_size = window.outer_size().map_err(|e| e.to_string())?;
        
        // 获取任务栏高度（Windows上通常为40像素）
        let taskbar_height = 40;
        
        let (x, y) = match position.as_str() {
            "top-left" => (monitor_position.x, monitor_position.y),
            "top-right" => (
                monitor_position.x + monitor_size.width as i32 - window_size.width as i32,
                monitor_position.y,
            ),
            "bottom-left" => (
                monitor_position.x,
                monitor_position.y + monitor_size.height as i32 - window_size.height as i32 - taskbar_height,
            ),
            "bottom-right" => (
                monitor_position.x + monitor_size.width as i32 - window_size.width as i32,
                monitor_position.y + monitor_size.height as i32 - window_size.height as i32 - taskbar_height,
            ),
            "center" => (
                monitor_position.x + (monitor_size.width as i32 - window_size.width as i32) / 2,
                monitor_position.y + (monitor_size.height as i32 - window_size.height as i32) / 2,
            ),
            _ => return Err("Invalid position".to_string()),
        };
        
        window
            .set_position(PhysicalPosition::new(x, y))
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("主窗口未找到".to_string())
    }
}
