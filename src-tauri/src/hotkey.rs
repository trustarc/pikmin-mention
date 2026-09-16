use std::str::FromStr;

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::{context, overlay};

#[cfg(target_os = "macos")]
pub const DEFAULT: &str = "Super+Shift+Comma";

#[cfg(not(target_os = "macos"))]
pub const DEFAULT: &str = "Control+Shift+Comma";

pub const LEGACY_DEFAULTS: &[&str] = &[
    "Control+Alt+KeyP",
    "Super+Alt+KeyP",
    "Alt+Shift+Period",
    "Super+Shift+Period",
    "Control+Shift+Period",
];

pub fn parse(value: &str) -> Result<Shortcut, String> {
    Shortcut::from_str(value).map_err(|error| error.to_string())
}

fn on_pressed(app: &AppHandle) {
    let visible = overlay::get(app)
        .and_then(|window| window.is_visible().ok())
        .unwrap_or(false);

    if visible {
        let _ = overlay::hide(app);
        return;
    }

    context::refresh(app);
    let _ = overlay::show(app);
}

pub fn register(app: &AppHandle, value: &str) -> Result<(), String> {
    let shortcut = parse(value)?;
    let manager = app.global_shortcut();

    manager
        .unregister_all()
        .map_err(|error| error.to_string())?;
    manager
        .on_shortcut(shortcut, |app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                on_pressed(app);
            }
        })
        .map_err(|error| error.to_string())
}
