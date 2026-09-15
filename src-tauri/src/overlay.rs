use tauri::{AppHandle, Manager, Monitor, PhysicalPosition, WebviewWindow};

use crate::platform;

pub const LABEL: &str = "overlay";

pub fn get(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(LABEL)
}

fn monitor_under_cursor(window: &WebviewWindow) -> Option<Monitor> {
    let cursor = window.cursor_position().ok()?;
    window.monitor_from_point(cursor.x, cursor.y).ok().flatten()
}

fn center_on(window: &WebviewWindow, monitor: &Monitor) -> tauri::Result<()> {
    let area = monitor.work_area();
    let size = window.outer_size()?;

    let x = area.position.x + (area.size.width as i32 - size.width as i32) / 2;
    let y = area.position.y + (area.size.height as i32 - size.height as i32) / 2;

    window.set_position(PhysicalPosition::new(x, y))
}

pub fn show(app: &AppHandle) -> tauri::Result<()> {
    let Some(window) = get(app) else {
        return Ok(());
    };

    // Follow the cursor rather than staying on whichever monitor the window
    // was last on, which is the primary one until it has ever moved.
    match monitor_under_cursor(&window) {
        Some(monitor) => {
            // Moving onto a monitor with a different DPI resizes the window,
            // so land on it first and centre with the size it ends up with.
            let area = monitor.work_area();
            window.set_position(PhysicalPosition::new(area.position.x, area.position.y))?;
            center_on(&window, &monitor)?;
            window.show()?;
            center_on(&window, &monitor)?;
        }
        None => {
            window.center()?;
            window.show()?;
        }
    }

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

    let (Ok(position), Ok(size), Ok(scale)) = (
        window.outer_position(),
        window.outer_size(),
        window.scale_factor(),
    ) else {
        return false;
    };

    // Window geometry is always physical. The cursor is physical too on
    // Windows and X11, but macOS reports it in logical points, so only
    // scale the window down where the two differ.
    let scale = if platform::CURSOR_IS_PHYSICAL {
        1.0
    } else {
        scale
    };

    let left = position.x as f64 / scale;
    let top = position.y as f64 / scale;
    let right = left + size.width as f64 / scale;
    let bottom = top + size.height as f64 / scale;

    x >= left && x <= right && y >= top && y <= bottom
}
