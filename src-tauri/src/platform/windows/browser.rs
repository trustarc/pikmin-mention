use windows::core::Interface;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationValuePattern, TreeScope_Descendants,
    UIA_ControlTypePropertyId, UIA_EditControlTypeId, UIA_ValuePatternId,
};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId};

fn looks_like_url(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && !trimmed.contains(' ') && trimmed.contains('.')
}

fn address_bar(window: HWND) -> Option<String> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let automation: IUIAutomation =
            CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok()?;

        let root = automation.ElementFromHandle(window).ok()?;
        let condition = automation
            .CreatePropertyCondition(
                UIA_ControlTypePropertyId,
                &VARIANT::from(UIA_EditControlTypeId.0),
            )
            .ok()?;

        let found = automation
            .CreateTreeWalker(&condition)
            .ok()
            .and_then(|_| root.FindFirst(TreeScope_Descendants, &condition).ok())?;

        let pattern: IUIAutomationValuePattern = found
            .GetCurrentPattern(UIA_ValuePatternId)
            .ok()?
            .cast()
            .ok()?;

        let value = pattern.CurrentValue().ok()?;
        let text = value.to_string();

        looks_like_url(&text).then_some(text)
    }
}

struct Search {
    pid: u32,
    found: HWND,
}

unsafe extern "system" fn visible_window(
    window: HWND,
    param: windows::Win32::Foundation::LPARAM,
) -> windows::core::BOOL {
    let search = unsafe { &mut *(param.0 as *mut Search) };

    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(window, Some(&mut pid)) };

    if pid == search.pid {
        search.found = window;
        return false.into();
    }

    true.into()
}

pub fn browser_url_via_a11y(pid: i32) -> Option<String> {
    if pid <= 0 {
        return None;
    }

    let mut search = Search {
        pid: pid as u32,
        found: HWND::default(),
    };

    let _ = unsafe {
        EnumWindows(
            Some(visible_window),
            windows::Win32::Foundation::LPARAM(&mut search as *mut Search as isize),
        )
    };

    if search.found.is_invalid() {
        return None;
    }

    address_bar(search.found)
}

pub fn active_url(_bundle_id: &str) -> Option<String> {
    None
}
