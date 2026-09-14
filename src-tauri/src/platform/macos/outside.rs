use std::ptr::NonNull;

use block2::RcBlock;
use objc2_app_kit::{NSEvent, NSEventMask};

pub fn watch_outside_clicks<F>(on_click: F)
where
    F: Fn() + 'static,
{
    let handler = RcBlock::new(move |_event: NonNull<NSEvent>| {
        on_click();
    });

    let mask = NSEventMask::LeftMouseDown
        | NSEventMask::RightMouseDown
        | NSEventMask::OtherMouseDown;

    NSEvent::addGlobalMonitorForEventsMatchingMask_handler(mask, &handler);

    std::mem::forget(handler);
}
