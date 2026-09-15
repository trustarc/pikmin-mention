use tauri::{AppHandle, Manager, WebviewWindow};

pub const LABEL: &str = "overlay";

pub fn get(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(LABEL)
}

pub fn show(app: &AppHandle) -> tauri::Result<()> {
    let Some(window) = get(app) else {
        return Ok(());
    };
    window.center()?;
    window.show()?;
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

    let left = position.x as f64 / scale;
    let top = position.y as f64 / scale;
    let right = left + size.width as f64 / scale;
    let bottom = top + size.height as f64 / scale;

    x >= left && x <= right && y >= top && y <= bottom
}
