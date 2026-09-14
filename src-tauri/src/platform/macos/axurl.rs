use std::ffi::c_void;
use std::ptr;

use objc2_core_foundation::{CFRetained, CFString};

type AXUIElementRef = *const c_void;
type CFTypeRef = *const c_void;

const MAX_DEPTH: usize = 12;
const MAX_NODES: usize = 3000;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXUIElementCreateApplication(pid: i32) -> AXUIElementRef;
    fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: *const c_void,
        value: *mut CFTypeRef,
    ) -> i32;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(cf: *const c_void);
    fn CFGetTypeID(cf: *const c_void) -> usize;
    fn CFArrayGetTypeID() -> usize;
    fn CFStringGetTypeID() -> usize;
    fn CFArrayGetCount(array: *const c_void) -> isize;
    fn CFArrayGetValueAtIndex(array: *const c_void, index: isize) -> *const c_void;
    fn CFStringGetCString(
        string: *const c_void,
        buffer: *mut u8,
        size: isize,
        encoding: u32,
    ) -> bool;
}

fn attribute(element: AXUIElementRef, name: &str) -> Option<CFTypeRef> {
    let key = CFString::from_str(name);
    let mut value: CFTypeRef = ptr::null();

    let status = unsafe {
        AXUIElementCopyAttributeValue(
            element,
            CFRetained::as_ptr(&key).as_ptr() as *const c_void,
            &mut value,
        )
    };

    if status == 0 && !value.is_null() {
        Some(value)
    } else {
        None
    }
}

fn to_string(value: CFTypeRef) -> Option<String> {
    if unsafe { CFGetTypeID(value) } != unsafe { CFStringGetTypeID() } {
        return None;
    }

    let mut buffer = [0u8; 2048];
    let ok = unsafe { CFStringGetCString(value, buffer.as_mut_ptr(), buffer.len() as isize, 0x0800_0100) };
    if !ok {
        return None;
    }

    let end = buffer.iter().position(|byte| *byte == 0).unwrap_or(0);
    String::from_utf8(buffer[..end].to_vec()).ok()
}

fn looks_like_url(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.contains(' ') {
        return false;
    }
    trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || (trimmed.contains('.') && !trimmed.contains('@'))
}

fn search(element: AXUIElementRef, depth: usize, budget: &mut usize) -> Option<String> {
    if depth > MAX_DEPTH || *budget == 0 {
        return None;
    }
    *budget -= 1;

    let role = attribute(element, "AXRole").and_then(|value| {
        let text = to_string(value);
        unsafe { CFRelease(value) };
        text
    });

    if role.as_deref() == Some("AXTextField") {
        if let Some(value) = attribute(element, "AXValue") {
            let text = to_string(value);
            unsafe { CFRelease(value) };
            if let Some(found) = text.filter(|candidate| looks_like_url(candidate)) {
                return Some(found);
            }
        }
    }

    let children = attribute(element, "AXChildren")?;
    let mut result = None;

    if unsafe { CFGetTypeID(children) } == unsafe { CFArrayGetTypeID() } {
        let count = unsafe { CFArrayGetCount(children) };
        for index in 0..count {
            let child = unsafe { CFArrayGetValueAtIndex(children, index) };
            if child.is_null() {
                continue;
            }
            if let Some(found) = search(child, depth + 1, budget) {
                result = Some(found);
                break;
            }
        }
    }

    unsafe { CFRelease(children) };
    result
}

pub fn focused_url(pid: i32) -> Option<String> {
    let app = unsafe { AXUIElementCreateApplication(pid) };
    if app.is_null() {
        return None;
    }

    let window = attribute(app, "AXFocusedWindow");
    let mut budget = MAX_NODES;

    let result = match window {
        Some(window) => {
            let found = search(window, 0, &mut budget);
            unsafe { CFRelease(window) };
            found
        }
        None => None,
    };

    unsafe { CFRelease(app) };
    result
}
