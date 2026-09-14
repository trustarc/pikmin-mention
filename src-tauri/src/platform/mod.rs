#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::{
    accessibility_trusted, activate_app, active_url, firefox_url, frontmost_app,
    open_accessibility_settings, request_accessibility, send_newline, send_paste, send_return,
    read_clipboard, type_text, watch_outside_clicks,
};

#[cfg(not(target_os = "macos"))]
pub struct Frontmost {
    pub name: String,
    pub bundle_id: String,
    pub pid: i32,
}

#[cfg(not(target_os = "macos"))]
pub fn frontmost_app() -> Option<Frontmost> {
    None
}

#[cfg(not(target_os = "macos"))]
pub fn firefox_url(_pid: i32) -> Option<String> {
    None
}

#[cfg(not(target_os = "macos"))]
pub fn active_url(_bundle_id: &str) -> Option<String> {
    None
}

#[cfg(not(target_os = "macos"))]
pub fn activate_app(_bundle_id: &str) {}

#[cfg(not(target_os = "macos"))]
pub fn accessibility_trusted() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn request_accessibility() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn open_accessibility_settings() {}

#[cfg(not(target_os = "macos"))]
pub fn send_paste() {}

#[cfg(not(target_os = "macos"))]
pub fn send_newline() {}

#[cfg(not(target_os = "macos"))]
pub fn watch_outside_clicks<F: Fn() + 'static>(_on_click: F) {}

#[cfg(not(target_os = "macos"))]
pub struct Clipboard {
    pub text: String,
    pub html: Option<String>,
}

#[cfg(not(target_os = "macos"))]
pub fn read_clipboard() -> Option<Clipboard> {
    None
}

#[cfg(not(target_os = "macos"))]
pub fn send_return() {}

#[cfg(not(target_os = "macos"))]
pub fn type_text(_text: &str) {}
