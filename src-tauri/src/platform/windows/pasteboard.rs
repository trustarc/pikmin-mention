use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::DataExchange::{
    CloseClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
    RegisterClipboardFormatW,
};
use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};
use windows::Win32::System::Ole::CF_UNICODETEXT;

use crate::platform::types::Clipboard;

fn read_format(format: u32) -> Option<Vec<u8>> {
    if unsafe { IsClipboardFormatAvailable(format) }.is_err() {
        return None;
    }

    let handle: HANDLE = unsafe { GetClipboardData(format) }.ok()?;
    let pointer = unsafe { GlobalLock(std::mem::transmute(handle)) };
    if pointer.is_null() {
        return None;
    }

    let mut bytes = Vec::new();
    let mut offset = 0isize;
    loop {
        let byte = unsafe { *(pointer as *const u8).offset(offset) };
        if byte == 0 {
            break;
        }
        bytes.push(byte);
        offset += 1;
    }

    let _ = unsafe { GlobalUnlock(std::mem::transmute(handle)) };
    Some(bytes)
}

fn read_unicode(format: u32) -> Option<String> {
    if unsafe { IsClipboardFormatAvailable(format) }.is_err() {
        return None;
    }

    let handle: HANDLE = unsafe { GetClipboardData(format) }.ok()?;
    let pointer = unsafe { GlobalLock(std::mem::transmute(handle)) } as *const u16;
    if pointer.is_null() {
        return None;
    }

    let mut units = Vec::new();
    let mut offset = 0isize;
    loop {
        let unit = unsafe { *pointer.offset(offset) };
        if unit == 0 {
            break;
        }
        units.push(unit);
        offset += 1;
    }

    let _ = unsafe { GlobalUnlock(std::mem::transmute(handle)) };
    Some(String::from_utf16_lossy(&units))
}

fn html_fragment(raw: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(raw);
    let start = text.find("<html")?;
    Some(text[start..].to_string())
}

pub fn read() -> Option<Clipboard> {
    unsafe { OpenClipboard(None) }.ok()?;

    let html_format = unsafe { RegisterClipboardFormatW(windows::core::w!("HTML Format")) };

    let text = read_unicode(CF_UNICODETEXT.0 as u32).unwrap_or_default();
    let html = read_format(html_format).as_deref().and_then(html_fragment);

    let _ = unsafe { CloseClipboard() };

    if text.trim().is_empty() && html.is_none() {
        return None;
    }

    Some(Clipboard {
        text: text.trim().to_string(),
        html,
    })
}
