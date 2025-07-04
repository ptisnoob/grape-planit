use tauri::{Manager, Runtime, WebviewWindowBuilder, WebviewUrl, PhysicalPosition};

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
