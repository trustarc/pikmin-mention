use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt, Keycode};
use x11rb::protocol::xtest::ConnectionExt as TestConnectionExt;

use super::conn::{open, Display};

const KEY_PRESS: u8 = 2;
const KEY_RELEASE: u8 = 3;

const KEYSYM_CONTROL: u32 = 0xffe3;
const KEYSYM_SHIFT: u32 = 0xffe1;
const KEYSYM_RETURN: u32 = 0xff0d;
const KEYSYM_V: u32 = 0x0076;

fn keycode(display: &Display, keysym: u32) -> Option<Keycode> {
    let setup = display.conn.setup();
    let first = setup.min_keycode;
    let count = setup.max_keycode - setup.min_keycode + 1;

    let mapping = display
        .conn
        .get_keyboard_mapping(first, count)
        .ok()?
        .reply()
        .ok()?;

    let per_code = mapping.keysyms_per_keycode as usize;

    mapping
        .keysyms
        .chunks(per_code)
        .position(|chunk| chunk.contains(&keysym))
        .map(|index| first + index as u8)
}

fn tap(display: &Display, modifier: Keycode, key: Keycode) {
    for (kind, code) in [
        (KEY_PRESS, modifier),
        (KEY_PRESS, key),
        (KEY_RELEASE, key),
        (KEY_RELEASE, modifier),
    ] {
        let _ = display
            .conn
            .xtest_fake_input(kind, code, 0, display.root, 0, 0, 0);
    }

    let _ = display.conn.flush();
}

fn chord(modifier_sym: u32, key_sym: u32) {
    let Some(display) = open() else {
        return;
    };
    let (Some(modifier), Some(key)) = (keycode(&display, modifier_sym), keycode(&display, key_sym))
    else {
        return;
    };

    tap(&display, modifier, key);
}

pub fn send_paste() {
    chord(KEYSYM_CONTROL, KEYSYM_V);
}

pub fn send_newline() {
    chord(KEYSYM_SHIFT, KEYSYM_RETURN);
}

pub fn accessibility_trusted() -> bool {
    true
}

pub fn request_accessibility() -> bool {
    true
}

pub fn open_accessibility_settings() {}
