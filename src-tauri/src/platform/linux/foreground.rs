use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    AtomEnum, ClientMessageEvent, ConnectionExt, EventMask, Window, CLIENT_MESSAGE_EVENT,
};

use super::conn::{atom, open, Display};
use crate::platform::types::Frontmost;

fn active_window(display: &Display) -> Option<Window> {
    let property = atom(display, "_NET_ACTIVE_WINDOW")?;

    display
        .conn
        .get_property(false, display.root, property, AtomEnum::WINDOW, 0, 1)
        .ok()?
        .reply()
        .ok()?
        .value32()?
        .next()
        .filter(|window| *window != 0)
}

fn window_pid(display: &Display, window: Window) -> Option<u32> {
    let property = atom(display, "_NET_WM_PID")?;

    display
        .conn
        .get_property(false, window, property, AtomEnum::CARDINAL, 0, 1)
        .ok()?
        .reply()
        .ok()?
        .value32()?
        .next()
}

fn process_name(pid: u32) -> Option<String> {
    std::fs::read_to_string(format!("/proc/{pid}/comm"))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn process_exe(pid: u32) -> Option<String> {
    std::fs::read_link(format!("/proc/{pid}/exe"))
        .ok()
        .map(|path| path.to_string_lossy().to_string())
}

pub fn frontmost_app() -> Option<Frontmost> {
    let display = open()?;
    let window = active_window(&display)?;
    let pid = window_pid(&display, window)?;
    let name = process_name(pid)?;

    Some(Frontmost {
        bundle_id: process_exe(pid).unwrap_or_else(|| name.clone()),
        name,
        pid: pid as i32,
    })
}

pub fn activate_app(_bundle_id: &str, pid: i32) {
    if pid <= 0 {
        return;
    }

    let Some(display) = open() else {
        return;
    };
    let Some(property) = atom(&display, "_NET_ACTIVE_WINDOW") else {
        return;
    };

    let Some(window) = find_window(&display, pid as u32) else {
        return;
    };

    let event = ClientMessageEvent {
        response_type: CLIENT_MESSAGE_EVENT,
        format: 32,
        sequence: 0,
        window,
        type_: property,
        data: [2, x11rb::CURRENT_TIME, 0, 0, 0].into(),
    };

    let _ = display.conn.send_event(
        false,
        display.root,
        EventMask::SUBSTRUCTURE_NOTIFY | EventMask::SUBSTRUCTURE_REDIRECT,
        event,
    );
    let _ = display.conn.flush();
}

fn find_window(display: &Display, pid: u32) -> Option<Window> {
    let list = atom(display, "_NET_CLIENT_LIST")?;

    let windows = display
        .conn
        .get_property(false, display.root, list, AtomEnum::WINDOW, 0, 1024)
        .ok()?
        .reply()
        .ok()?
        .value32()?
        .collect::<Vec<_>>();

    windows
        .into_iter()
        .find(|window| window_pid(display, *window) == Some(pid))
}
