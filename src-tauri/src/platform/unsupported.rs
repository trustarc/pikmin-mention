#![allow(dead_code)]

use super::types::{Clipboard, Frontmost};

pub fn frontmost_app() -> Option<Frontmost> {
    None
}

pub fn active_url(_bundle_id: &str) -> Option<String> {
    None
}

pub fn browser_url_via_a11y(_pid: i32) -> Option<String> {
    None
}

pub fn activate_app(_bundle_id: &str) {}

pub fn accessibility_trusted() -> bool {
    true
}

pub fn request_accessibility() -> bool {
    true
}

pub fn open_accessibility_settings() {}

pub fn send_paste() {}

pub fn send_newline() {}

pub fn read_clipboard() -> Option<Clipboard> {
    None
}

pub fn watch_outside_clicks<F: Fn() + 'static>(_on_click: F) {}

pub fn screen_click_point() -> Option<(f64, f64)> {
    None
}
