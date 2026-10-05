use tauri::{Manager, PhysicalPosition, PhysicalSize};

#[derive(Clone, Copy)]
struct MonitorGeometry {
    left: i32,
    top: i32,
    width: u32,
    height: u32,
    scale: f64,
}

#[tauri::command]
pub fn set_window_mode(
    app: tauri::AppHandle,
    mode: String,
    monitor_mode: String,
    top_gap: i64,
    island_size: String,
) -> Result<(), String> {
    if !["collapsed", "expanded", "quick"].contains(&mode.as_str()) {
        return Err("Invalid window mode".into());
    }
    if !["active", "primary"].contains(&monitor_mode.as_str()) {
        return Err("Invalid monitor mode".into());
    }
    if ![0, 6, 12].contains(&top_gap) {
        return Err("Invalid top gap".into());
    }
    if !["compact", "normal", "large"].contains(&island_size.as_str()) {
        return Err("Invalid island size".into());
    }

    let window = app
        .get_webview_window("main")
        .ok_or("Main window is unavailable")?;
    let monitor = monitor_geometry(&monitor_mode, &window)?;
    let (logical_width, logical_height): (f64, f64) = match mode.as_str() {
        "expanded" => (948.0, 420.0),
        "quick" => (430.0, 64.0),
        _ => match island_size.as_str() {
            "compact" => (202.0, 46.0),
            "large" => (254.0, 58.0),
            _ => (228.0, 52.0),
        },
    };
    let available_logical_width = monitor.width as f64 / monitor.scale;
    let available_logical_height = monitor.height as f64 / monitor.scale;
    let width = logical_width.min((available_logical_width - 16.0).max(180.0));
    let height = logical_height.min((available_logical_height - top_gap as f64 - 8.0).max(42.0));
    let physical_width = (width * monitor.scale).round() as u32;
    let physical_height = (height * monitor.scale).round() as u32;
    let x = monitor.left + ((monitor.width as i64 - physical_width as i64) / 2) as i32;
    let y = monitor.top + (top_gap as f64 * monitor.scale).round() as i32;

    window
        .set_size(PhysicalSize::new(physical_width, physical_height))
        .map_err(|error| format!("Could not resize window: {error}"))?;
    window
        .set_position(PhysicalPosition::new(x, y))
        .map_err(|error| format!("Could not position window: {error}"))?;
    Ok(())
}

#[tauri::command]
pub fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window is unavailable")?;
    window
        .show()
        .map_err(|error| format!("Could not show window: {error}"))?;
    window
        .set_focus()
        .map_err(|error| format!("Could not focus window: {error}"))?;
    Ok(())
}

#[tauri::command]
pub fn begin_window_drag(app: tauri::AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window is unavailable")?;
    window
        .start_dragging()
        .map_err(|error| format!("Could not move window: {error}"))
}

pub fn show_without_error(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg(windows)]
fn monitor_geometry(mode: &str, _window: &tauri::WebviewWindow) -> Result<MonitorGeometry, String> {
    use std::mem::size_of;
    use windows::Win32::{
        Foundation::POINT,
        Graphics::Gdi::{
            GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
            MONITOR_DEFAULTTOPRIMARY,
        },
        UI::{
            HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI},
            WindowsAndMessaging::GetCursorPos,
        },
    };

    unsafe {
        let monitor = if mode == "primary" {
            MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY)
        } else {
            let mut cursor = POINT::default();
            GetCursorPos(&mut cursor)
                .map_err(|error| format!("Could not read cursor position: {error}"))?;
            MonitorFromPoint(cursor, MONITOR_DEFAULTTONEAREST)
        };
        if monitor.is_invalid() {
            return Err("Windows did not return a monitor".into());
        }
        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(monitor, &mut info).as_bool() {
            return Err("Could not read monitor work area".into());
        }
        let mut dpi_x = 96u32;
        let mut dpi_y = 96u32;
        let _ = GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);
        let work = info.rcWork;
        Ok(MonitorGeometry {
            left: work.left,
            top: work.top,
            width: work.right.saturating_sub(work.left) as u32,
            height: work.bottom.saturating_sub(work.top) as u32,
            scale: (dpi_x.max(dpi_y) as f64 / 96.0).clamp(1.0, 4.0),
        })
    }
}

#[cfg(not(windows))]
fn monitor_geometry(mode: &str, window: &tauri::WebviewWindow) -> Result<MonitorGeometry, String> {
    let monitor = if mode == "primary" {
        window.primary_monitor()
    } else {
        let cursor = window
            .cursor_position()
            .map_err(|error| format!("Could not read cursor position: {error}"))?;
        window.monitor_from_point(cursor.x, cursor.y)
    }
    .map_err(|error| format!("Could not query display: {error}"))?
    .ok_or("No display was found for FocusIsland")?;

    let work_area = monitor.work_area();
    let position = work_area.position;
    let size = work_area.size;
    Ok(MonitorGeometry {
        left: position.x,
        top: position.y,
        width: size.width,
        height: size.height,
        scale: monitor.scale_factor().clamp(1.0, 4.0),
    })
}
