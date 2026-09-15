use windows::core::Interface;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationValuePattern, TreeScope_Descendants,
    UIA_ComboBoxControlTypeId, UIA_ControlTypePropertyId, UIA_EditControlTypeId,
    UIA_ValuePatternId,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, IsWindowVisible,
};

use super::foreground::remembered_window;
use crate::platform::address::looks_like_url;

fn address_bar(automation: &IUIAutomation, window: HWND) -> Option<String> {
    unsafe {
        let root = automation.ElementFromHandle(window).ok()?;

        let edit = automation
            .CreatePropertyCondition(
                UIA_ControlTypePropertyId,
                &VARIANT::from(UIA_EditControlTypeId.0),
            )
            .ok()?;
        let combo = automation
            .CreatePropertyCondition(
                UIA_ControlTypePropertyId,
                &VARIANT::from(UIA_ComboBoxControlTypeId.0),
            )
            .ok()?;
        let condition = automation.CreateOrCondition(&edit, &combo).ok()?;

        let found = root.FindFirst(TreeScope_Descendants, &condition).ok()?;

        let pattern: IUIAutomationValuePattern = found
            .GetCurrentPattern(UIA_ValuePatternId)
            .ok()?
            .cast()
            .ok()?;

        let text = pattern.CurrentValue().ok()?.to_string();

        looks_like_url(&text).then_some(text)
    }
}

struct Search {
    pid: u32,
    found: Vec<HWND>,
}

unsafe extern "system" fn collect_visible(window: HWND, param: LPARAM) -> windows::core::BOOL {
    let search = unsafe { &mut *(param.0 as *mut Search) };

    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(window, Some(&mut pid)) };

    if pid == search.pid && unsafe { IsWindowVisible(window) }.as_bool() {
        search.found.push(window);
    }

    true.into()
}

fn visible_windows(pid: u32) -> Vec<HWND> {
    let mut search = Search {
        pid,
        found: Vec::new(),
    };

    let _ = unsafe {
        EnumWindows(
            Some(collect_visible),
            LPARAM(&mut search as *mut Search as isize),
        )
    };

    search.found
}

pub fn browser_url_via_a11y(pid: i32) -> Option<String> {
    if pid <= 0 {
        return None;
    }
    let pid = pid as u32;

    let candidates = match remembered_window(pid) {
        Some(window) => vec![window],
        None => visible_windows(pid),
    };

    if candidates.is_empty() {
        return None;
    }

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let automation: IUIAutomation =
            CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok()?;

        candidates
            .into_iter()
            .find_map(|window| address_bar(&automation, window))
    }
}

pub fn active_url(_bundle_id: &str) -> Option<String> {
    None
}
