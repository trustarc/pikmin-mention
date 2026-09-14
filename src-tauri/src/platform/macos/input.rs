use std::ffi::c_void;
use std::process::Command;
use std::ptr;
use std::thread;
use std::time::Duration;

use objc2_core_foundation::CFRetained;
use objc2_core_graphics::{
    CGEvent, CGEventFlags, CGEventSource, CGEventSourceStateID, CGEventTapLocation,
};

const KEY_V: u16 = 9;
const KEY_RETURN: u16 = 36;
const TYPE_DELAY: Duration = Duration::from_millis(28);
const SETTINGS_URL: &str =
    "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility";

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
    fn AXIsProcessTrustedWithOptions(options: *const c_void) -> bool;
    static kAXTrustedCheckOptionPrompt: *const c_void;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFBooleanTrue: *const c_void;
    static kCFTypeDictionaryKeyCallBacks: c_void;
    static kCFTypeDictionaryValueCallBacks: c_void;

    fn CFDictionaryCreate(
        allocator: *const c_void,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: isize,
        key_callbacks: *const c_void,
        value_callbacks: *const c_void,
    ) -> *const c_void;
    fn CFRelease(cf: *const c_void);
}

pub fn accessibility_trusted() -> bool {
    unsafe { AXIsProcessTrusted() }
}

pub fn request_accessibility() -> bool {
    unsafe {
        let keys = [kAXTrustedCheckOptionPrompt];
        let values = [kCFBooleanTrue];

        let options = CFDictionaryCreate(
            ptr::null(),
            keys.as_ptr(),
            values.as_ptr(),
            1,
            &kCFTypeDictionaryKeyCallBacks as *const c_void,
            &kCFTypeDictionaryValueCallBacks as *const c_void,
        );

        let trusted = AXIsProcessTrustedWithOptions(options);

        if !options.is_null() {
            CFRelease(options);
        }

        trusted
    }
}

pub fn open_accessibility_settings() {
    let _ = Command::new("open").arg(SETTINGS_URL).spawn();
}

fn source() -> Option<CFRetained<CGEventSource>> {
    CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
}

fn tap_key(source: Option<&CGEventSource>, key: u16, flags: CGEventFlags) {
    for down in [true, false] {
        let Some(event) = CGEvent::new_keyboard_event(source, key, down) else {
            return;
        };
        CGEvent::set_flags(Some(&event), flags);
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(&event));
    }
}

fn tap_char(source: Option<&CGEventSource>, unit: u16) {
    for down in [true, false] {
        let Some(event) = CGEvent::new_keyboard_event(source, 0, down) else {
            return;
        };
        unsafe { CGEvent::keyboard_set_unicode_string(Some(&event), 1, &unit) };
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(&event));
    }
}

pub fn send_paste() {
    let source = source();
    tap_key(source.as_deref(), KEY_V, CGEventFlags::MaskCommand);
}

pub fn send_return() {
    let source = source();
    tap_key(source.as_deref(), KEY_RETURN, CGEventFlags::empty());
}

pub fn send_newline() {
    let source = source();
    tap_key(source.as_deref(), KEY_RETURN, CGEventFlags::MaskShift);
}

pub fn type_text(text: &str) {
    let source = source();
    for unit in text.encode_utf16() {
        tap_char(source.as_deref(), unit);
        thread::sleep(TYPE_DELAY);
    }
}
