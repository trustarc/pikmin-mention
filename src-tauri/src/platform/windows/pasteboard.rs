use windows::Win32::Foundation::{HANDLE, HGLOBAL};
use windows::Win32::System::DataExchange::{
    CloseClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
    RegisterClipboardFormatW,
};
use windows::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};
use windows::Win32::System::Ole::CF_UNICODETEXT;

use crate::platform::cfhtml::html_fragment;
use crate::platform::types::Clipboard;

const MAX_BYTES: usize = 4 * 1024 * 1024;

fn with_locked<T>(format: u32, read: impl FnOnce(*const u8, usize) -> Option<T>) -> Option<T> {
    if unsafe { IsClipboardFormatAvailable(format) }.is_err() {
        return None;
    }

    let handle: HANDLE = unsafe { GetClipboardData(format) }.ok()?;
    let global = HGLOBAL(handle.0);

    let size = unsafe { GlobalSize(global) };
    if size == 0 {
        return None;
    }

    let pointer = unsafe { GlobalLock(global) };
    if pointer.is_null() {
        return None;
    }

    let result = read(pointer as *const u8, size.min(MAX_BYTES));

    let _ = unsafe { GlobalUnlock(global) };
    result
}

fn read_bytes(format: u32) -> Option<Vec<u8>> {
    with_locked(format, |pointer, size| {
        let bytes = unsafe { std::slice::from_raw_parts(pointer, size) };
        let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(size);
        Some(bytes[..end].to_vec())
    })
}

fn read_unicode(format: u32) -> Option<String> {
    with_locked(format, |pointer, size| {
        let units = size / std::mem::size_of::<u16>();
        if units == 0 {
            return None;
        }

        let slice = unsafe { std::slice::from_raw_parts(pointer as *const u16, units) };
        let end = slice.iter().position(|unit| *unit == 0).unwrap_or(units);
        Some(String::from_utf16_lossy(&slice[..end]))
    })
}

pub fn read() -> Option<Clipboard> {
    unsafe { OpenClipboard(None) }.ok()?;

    let html_format = unsafe { RegisterClipboardFormatW(windows::core::w!("HTML Format")) };

    let text = read_unicode(CF_UNICODETEXT.0 as u32).unwrap_or_default();
    let html = read_bytes(html_format).as_deref().and_then(html_fragment);

    let _ = unsafe { CloseClipboard() };

    if text.trim().is_empty() && html.is_none() {
        return None;
    }

    Some(Clipboard {
        text: text.trim().to_string(),
        html,
    })
}
