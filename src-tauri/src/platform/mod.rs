#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
mod cfhtml;
mod types;
mod unsupported;

#[allow(unused_imports)]
pub use types::{Clipboard, Frontmost};

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub use unsupported::*;
