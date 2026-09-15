use std::thread;
use std::time::Duration;

use x11rb::protocol::xproto::ConnectionExt;

use super::conn::open;

/// X11 reports the pointer in device pixels, the same space as the window
/// geometry, so no scaling applies when comparing them.
pub const CURSOR_IS_PHYSICAL: bool = true;

pub fn screen_click_point() -> Option<(f64, f64)> {
    let display = open()?;
    let pointer = display
        .conn
        .query_pointer(display.root)
        .ok()?
        .reply()
        .ok()?;

    Some((pointer.root_x as f64, pointer.root_y as f64))
}

pub fn watch_outside_clicks<F>(on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    thread::spawn(move || {
        let Some(display) = open() else {
            return;
        };

        let mut was_pressed = false;

        loop {
            thread::sleep(Duration::from_millis(60));

            let Ok(cookie) = display.conn.query_pointer(display.root) else {
                return;
            };
            let Ok(pointer) = cookie.reply() else {
                return;
            };

            let pressed = pointer.mask.bits() & 0x0700 != 0;

            if pressed && !was_pressed {
                on_click();
            }

            was_pressed = pressed;
        }
    });
}
