use std::ffi::c_void;
use std::sync::atomic::{AtomicIsize, Ordering};

use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, MAX_PATH};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindowThreadProcessId, IsWindow, IsWindowVisible,
    SetForegroundWindow,
};

use crate::platform::types::Frontmost;

/// The window that was in front when the context was captured. Going back to
/// it directly beats guessing from the process, which may own several windows.
static LAST_FOREGROUND: AtomicIsize = AtomicIsize::new(0);

fn process_path(pid: u32) -> Option<String> {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }.ok()?;

    let mut buffer = [0u16; MAX_PATH as usize];
    let mut length = buffer.len() as u32;

    let result = unsafe {
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            windows::core::PWSTR(buffer.as_mut_ptr()),
            &mut length,
        )
    };

    let _ = unsafe { CloseHandle(handle) };
    result.ok()?;

    Some(String::from_utf16_lossy(&buffer[..length as usize]))
}

fn display_name(path: &str) -> String {
    path.rsplit(['\\', '/'])
        .next()
        .unwrap_or(path)
        .trim_end_matches(".exe")
        .to_string()
}

fn owner_pid(window: HWND) -> u32 {
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(window, Some(&mut pid)) };
    pid
}

pub fn frontmost_app() -> Option<Frontmost> {
    let window = unsafe { GetForegroundWindow() };
    if window.is_invalid() {
        return None;
    }

    let pid = owner_pid(window);
    if pid == 0 {
        return None;
    }

    let path = process_path(pid)?;
    LAST_FOREGROUND.store(window.0 as isize, Ordering::Relaxed);

    Some(Frontmost {
        name: display_name(&path),
        bundle_id: path,
        pid: pid as i32,
    })
}

struct Search {
    pid: u32,
    found: HWND,
}

unsafe extern "system" fn find_window(window: HWND, param: LPARAM) -> windows::core::BOOL {
    let search = unsafe { &mut *(param.0 as *mut Search) };

    if owner_pid(window) == search.pid && unsafe { IsWindowVisible(window) }.as_bool() {
        search.found = window;
        return false.into();
    }

    true.into()
}

fn remembered_window(pid: u32) -> Option<HWND> {
    let window = HWND(LAST_FOREGROUND.load(Ordering::Relaxed) as *mut c_void);

    if window.is_invalid() || !unsafe { IsWindow(Some(window)) }.as_bool() {
        return None;
    }

    (owner_pid(window) == pid).then_some(window)
}

fn first_visible_window(pid: u32) -> Option<HWND> {
    let mut search = Search {
        pid,
        found: HWND::default(),
    };

    let _ = unsafe {
        EnumWindows(
            Some(find_window),
            LPARAM(&mut search as *mut Search as isize),
        )
    };

    (!search.found.is_invalid()).then_some(search.found)
}

pub fn activate_app(_bundle_id: &str, pid: i32) {
    if pid <= 0 {
        return;
    }

    let pid = pid as u32;
    let Some(window) = remembered_window(pid).or_else(|| first_visible_window(pid)) else {
        return;
    };

    let _ = unsafe { SetForegroundWindow(window) };
}
