use objc2_app_kit::{NSPasteboard, NSPasteboardTypeHTML, NSPasteboardTypeString};

use crate::platform::types::Clipboard;

pub fn read() -> Option<Clipboard> {
    let pasteboard = NSPasteboard::generalPasteboard();

    let text = unsafe { pasteboard.stringForType(NSPasteboardTypeString) }
        .map(|value| value.to_string())
        .unwrap_or_default();

    let html = unsafe { pasteboard.stringForType(NSPasteboardTypeHTML) }
        .map(|value| value.to_string())
        .filter(|value| !value.trim().is_empty());

    if text.trim().is_empty() && html.is_none() {
        return None;
    }

    Some(Clipboard {
        text: text.trim().to_string(),
        html,
    })
}
