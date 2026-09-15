use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VIRTUAL_KEY, VK_CONTROL, VK_RETURN, VK_SHIFT, VK_V,
};

fn key(code: VIRTUAL_KEY, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: code,
                wScan: 0,
                dwFlags: if up {
                    KEYEVENTF_KEYUP
                } else {
                    KEYBD_EVENT_FLAGS(0)
                },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn chord(modifier: VIRTUAL_KEY, code: VIRTUAL_KEY) {
    let events = [
        key(modifier, false),
        key(code, false),
        key(code, true),
        key(modifier, true),
    ];

    unsafe { SendInput(&events, std::mem::size_of::<INPUT>() as i32) };
}

pub fn send_paste() {
    chord(VK_CONTROL, VK_V);
}

pub fn send_newline() {
    chord(VK_SHIFT, VK_RETURN);
}

pub fn accessibility_trusted() -> bool {
    true
}

pub fn request_accessibility() -> bool {
    true
}

pub fn open_accessibility_settings() {}
