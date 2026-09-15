use tauri::{AppHandle, Manager, WebviewWindow};

pub const LABEL: &str = "overlay";

pub fn get(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(LABEL)
}

#[cfg(target_os = "windows")]
fn cursor_monitor(window: &WebviewWindow) -> Option<tauri::Monitor> {
    let cursor = window.cursor_position().ok()?;
    window.monitor_from_point(cursor.x, cursor.y).ok()?
}

#[cfg(target_os = "windows")]
fn center_on(window: &WebviewWindow, monitor: &tauri::Monitor) {
    let area = monitor.work_area();
    let Ok(size) = window.outer_size() else {
        return;
    };

    let x = area.position.x + (area.size.width as i32 - size.width as i32) / 2;
    let y = area.position.y + (area.size.height as i32 - size.height as i32) / 2;

    let _ = window.set_position(tauri::PhysicalPosition::new(
        x.max(area.position.x),
        y.max(area.position.y),
    ));
}

#[cfg(not(target_os = "windows"))]
fn reveal(window: &WebviewWindow) -> tauri::Result<()> {
    let _ = window.center();
    window.show()
}

#[cfg(target_os = "windows")]
fn reveal(window: &WebviewWindow) -> tauri::Result<()> {
    let Some(monitor) = cursor_monitor(window) else {
        let _ = window.center();
        return window.show();
    };

    let area = monitor.work_area();
    let _ = window.set_position(tauri::PhysicalPosition::new(
        area.position.x,
        area.position.y,
    ));
    center_on(window, &monitor);
    window.show()?;
    center_on(window, &monitor);

    Ok(())
}

pub fn show(app: &AppHandle) -> tauri::Result<()> {
    let Some(window) = get(app) else {
        return Ok(());
    };

    reveal(&window)?;
    window.set_focus()?;
    Ok(())
}

pub fn hide(app: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = get(app) {
        window.hide()?;
    }
    Ok(())
}

pub fn contains_point(window: &WebviewWindow, point: Option<(f64, f64)>) -> bool {
    let Some((x, y)) = point else {
        return false;
    };

    let (Ok(position), Ok(size)) = (window.outer_position(), window.outer_size()) else {
        return false;
    };

    #[cfg(target_os = "macos")]
    let (x, y) = {
        let Ok(scale) = window.scale_factor() else {
            return false;
        };
        (x * scale, y * scale)
    };

    let left = position.x as f64;
    let top = position.y as f64;
    let right = left + size.width as f64;
    let bottom = top + size.height as f64;

    x >= left && x <= right && y >= top && y <= bottom
}
