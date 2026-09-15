use windows::Win32::Foundation::{HANDLE, HGLOBAL};
use windows::Win32::System::DataExchange::{
    CloseClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
    RegisterClipboardFormatW,
};
use windows::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};
use windows::Win32::System::Ole::CF_UNICODETEXT;

use crate::platform::types::Clipboard;

const MAX_BYTES: usize = 4 * 1024 * 1024;

const START_MARKER: &str = "<!--StartFragment-->";
const END_MARKER: &str = "<!--EndFragment-->";

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

fn header_offset(text: &str, key: &str) -> Option<usize> {
    text.lines()
        .take_while(|line| !line.trim_start().starts_with('<'))
        .find_map(|line| line.strip_prefix(key))
        .and_then(|value| value.trim().parse().ok())
}

fn between_markers(text: &str) -> Option<String> {
    let start = text.find(START_MARKER)? + START_MARKER.len();
    let end = text.find(END_MARKER)?;
    (start <= end).then(|| text[start..end].to_string())
}

fn between_offsets(raw: &[u8], text: &str) -> Option<String> {
    let start = header_offset(text, "StartFragment:")?;
    let end = header_offset(text, "EndFragment:")?;
    (start < end && end <= raw.len())
        .then(|| String::from_utf8_lossy(&raw[start..end]).into_owned())
}

fn whole_document(text: &str) -> Option<String> {
    let at = text.find("<html").or_else(|| text.find("<HTML"))?;
    Some(text[at..].to_string())
}

/// CF_HTML wraps the copied markup in a header of byte offsets and a full
/// document. Keep just the fragment, the way macOS hands it over, so writing
/// it back out does not nest one document inside another.
fn html_fragment(raw: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(raw);

    let fragment = between_markers(&text)
        .or_else(|| between_offsets(raw, &text))
        .or_else(|| whole_document(&text))?;

    let trimmed = fragment
        .replace(START_MARKER, "")
        .replace(END_MARKER, "")
        .trim()
        .to_string();

    (!trimmed.is_empty()).then_some(trimmed)
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

#[cfg(test)]
mod tests {
    use super::html_fragment;

    const MENTION: &str = r#"<a href="https://trustarc.slack.com/team/U1">@Pikmin</a>"#;

    fn cf_html(before: &str, fragment: &str, after: &str) -> Vec<u8> {
        let header = |a: usize, b: usize, c: usize, d: usize| {
            format!(
                "Version:0.9\r\nStartHTML:{a:010}\r\nEndHTML:{b:010}\r\nStartFragment:{c:010}\r\nEndFragment:{d:010}\r\n"
            )
        };

        let start_html = header(0, 0, 0, 0).len();
        let start_fragment = start_html + before.len();
        let end_fragment = start_fragment + fragment.len();
        let end_html = end_fragment + after.len();

        let mut raw = header(start_html, end_html, start_fragment, end_fragment).into_bytes();
        raw.extend_from_slice(before.as_bytes());
        raw.extend_from_slice(fragment.as_bytes());
        raw.extend_from_slice(after.as_bytes());
        raw
    }

    #[test]
    fn keeps_only_what_is_between_the_markers() {
        let raw = cf_html(
            "<html>\r\n<body>\r\n<!--StartFragment-->",
            MENTION,
            "<!--EndFragment-->\r\n</body>\r\n</html>",
        );

        assert_eq!(html_fragment(&raw).as_deref(), Some(MENTION));
    }

    #[test]
    fn falls_back_to_the_header_offsets() {
        let raw = cf_html("<html><body>", MENTION, "</body></html>");

        assert_eq!(html_fragment(&raw).as_deref(), Some(MENTION));
    }

    #[test]
    fn falls_back_to_the_document_without_a_header() {
        let raw = b"<html><body>plain</body></html>";

        assert_eq!(
            html_fragment(raw).as_deref(),
            Some("<html><body>plain</body></html>")
        );
    }

    #[test]
    fn rejects_an_empty_fragment() {
        let raw = cf_html(
            "<html><body><!--StartFragment-->",
            "  ",
            "<!--EndFragment--></body></html>",
        );

        assert_eq!(html_fragment(&raw), None);
    }
}
