mod atspi;
mod conn;
mod foreground;
mod input;
mod outside;
mod pasteboard;

pub use atspi::browser_url_via_a11y;
pub use foreground::{activate_app, frontmost_app};
pub use input::{
    accessibility_trusted, open_accessibility_settings, request_accessibility, send_newline,
    send_paste,
};
pub use outside::{screen_click_point, watch_outside_clicks, CURSOR_IS_PHYSICAL};
pub use pasteboard::read as read_clipboard;

pub use crate::platform::unsupported::active_url;
