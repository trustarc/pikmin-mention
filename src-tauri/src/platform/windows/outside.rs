use std::sync::OnceLock;

use windows::Win32::Foundation::{LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetCursorPos, SetWindowsHookExW, HHOOK, WH_MOUSE_LL, WM_LBUTTONDOWN,
    WM_MBUTTONDOWN, WM_RBUTTONDOWN,
};

/// `GetCursorPos` reports physical pixels in the virtual screen, the same
/// space as the window geometry, so no scaling applies when comparing them.
pub const CURSOR_IS_PHYSICAL: bool = true;

type Callback = Box<dyn Fn() + Send + Sync>;

static HANDLER: OnceLock<Callback> = OnceLock::new();

unsafe extern "system" fn hook(code: i32, event: WPARAM, data: LPARAM) -> LRESULT {
    if code >= 0 {
        let pressed = matches!(
            event.0 as u32,
            WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN
        );

        if pressed {
            if let Some(handler) = HANDLER.get() {
                handler();
            }
        }
    }

    unsafe { CallNextHookEx(None, code, event, data) }
}

pub fn screen_click_point() -> Option<(f64, f64)> {
    let mut point = POINT::default();
    unsafe { GetCursorPos(&mut point) }.ok()?;
    Some((point.x as f64, point.y as f64))
}

pub fn watch_outside_clicks<F>(on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    if HANDLER.set(Box::new(on_click)).is_err() {
        return;
    }

    let _: Option<HHOOK> = unsafe { SetWindowsHookExW(WH_MOUSE_LL, Some(hook), None, 0) }.ok();
}
