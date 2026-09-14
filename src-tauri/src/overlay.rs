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
