use std::time::{Duration, Instant};

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, CreateWindowAux, EventMask, WindowClass};
use x11rb::protocol::Event;
use x11rb::COPY_DEPTH_FROM_PARENT;

use super::conn::{atom, open, Display};
use crate::platform::types::Clipboard;

fn receiver(display: &Display) -> Option<u32> {
    let window = display.conn.generate_id().ok()?;

    display
        .conn
        .create_window(
            COPY_DEPTH_FROM_PARENT,
            window,
            display.root,
            0,
            0,
            1,
            1,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new().event_mask(EventMask::PROPERTY_CHANGE),
        )
        .ok()?;

    Some(window)
}

fn read_target(display: &Display, window: u32, target: &str) -> Option<String> {
    let clipboard = atom(display, "CLIPBOARD")?;
    let target_atom = atom(display, target)?;
    let destination = atom(display, "PIKMIN_MENTION_SELECTION")?;

    display
        .conn
        .convert_selection(
            window,
            clipboard,
            target_atom,
            destination,
            x11rb::CURRENT_TIME,
        )
        .ok()?;
    display.conn.flush().ok()?;

    let deadline = Instant::now() + Duration::from_millis(500);

    while Instant::now() < deadline {
        match display.conn.poll_for_event().ok()? {
            Some(Event::SelectionNotify(event)) => {
                if event.property == 0 {
                    return None;
                }

                let reply = display
                    .conn
                    .get_property(true, window, destination, AtomEnum::ANY, 0, u32::MAX)
                    .ok()?
                    .reply()
                    .ok()?;

                return String::from_utf8(reply.value).ok();
            }
            Some(_) => continue,
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    }

    None
}

pub fn read() -> Option<Clipboard> {
    let display = open()?;
    let window = receiver(&display)?;

    let text = read_target(&display, window, "UTF8_STRING").unwrap_or_default();
    let html = read_target(&display, window, "text/html").filter(|value| !value.trim().is_empty());

    let _ = display.conn.destroy_window(window);

    if text.trim().is_empty() && html.is_none() {
        return None;
    }

    Some(Clipboard {
        text: text.trim().to_string(),
        html,
    })
}
