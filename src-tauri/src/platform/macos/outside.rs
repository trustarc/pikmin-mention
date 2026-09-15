use std::ptr::NonNull;

use block2::RcBlock;

use objc2_app_kit::{NSEvent, NSEventMask, NSScreen};
use objc2_foundation::MainThreadMarker;

pub fn screen_click_point() -> Option<(f64, f64)> {
    let marker = MainThreadMarker::new()?;
    let height = NSScreen::screens(marker)
        .iter()
        .next()
        .map(|screen| screen.frame().size.height)?;

    let point = NSEvent::mouseLocation();
    Some((point.x, height - point.y))
}

pub fn watch_outside_clicks<F>(on_click: F)
where
    F: Fn() + 'static,
{
    let handler = RcBlock::new(move |_event: NonNull<NSEvent>| {
        on_click();
    });

    let mask =
        NSEventMask::LeftMouseDown | NSEventMask::RightMouseDown | NSEventMask::OtherMouseDown;

    NSEvent::addGlobalMonitorForEventsMatchingMask_handler(mask, &handler);

    std::mem::forget(handler);
}
