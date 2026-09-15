mod axurl;
mod browser;
mod foreground;
mod input;
mod outside;
mod pasteboard;

pub use browser::active_url;
pub use foreground::{activate_app, frontmost_app};
pub use input::{
    accessibility_trusted, open_accessibility_settings, request_accessibility, send_newline,
    send_paste,
};
pub use outside::{screen_click_point, watch_outside_clicks};
pub use pasteboard::read as read_clipboard;

pub fn browser_url_via_a11y(pid: i32) -> Option<String> {
    axurl::focused_url(pid)
}
