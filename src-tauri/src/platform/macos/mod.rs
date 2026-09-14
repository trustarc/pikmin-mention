mod axurl;
mod browser;
mod foreground;
mod input;
mod outside;
mod pasteboard;

pub use browser::active_url;
pub use foreground::{activate_app, frontmost_app};
pub use outside::watch_outside_clicks;
pub use pasteboard::read as read_clipboard;
pub use input::{
    accessibility_trusted, open_accessibility_settings, request_accessibility, send_newline,
    send_paste, send_return, type_text,
};

pub fn firefox_url(pid: i32) -> Option<String> {
    axurl::focused_url(pid)
}
