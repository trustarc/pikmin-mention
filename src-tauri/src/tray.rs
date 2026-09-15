use tauri::image::Image;
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter};

use tauri_plugin_autostart::ManagerExt;

use crate::overlay;

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    let settings = MenuItem::with_id(app, "settings", "Open Settings", true, None::<&str>)?;
    let updates = MenuItem::with_id(app, "updates", "Check for Updates…", true, None::<&str>)?;
    let autostart = CheckMenuItem::with_id(
        app,
        "autostart",
        "Launch at Login",
        true,
        app.autolaunch().is_enabled().unwrap_or(false),
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[
            &settings,
            &autostart,
            &updates,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;

    let toggle = autostart.clone();

    let mut builder = TrayIconBuilder::with_id("tray")
        .tooltip("Pikmin Mention")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "settings" => {
                let _ = overlay::show(app);
            }
            "autostart" => {
                let launcher = app.autolaunch();
                let enabled = launcher.is_enabled().unwrap_or(false);

                let result = if enabled {
                    launcher.disable()
                } else {
                    launcher.enable()
                };

                if let Err(error) = result {
                    eprintln!("launch at login could not be changed: {error}");
                }

                let _ = toggle.set_checked(launcher.is_enabled().unwrap_or(false));
            }
            "updates" => {
                let _ = overlay::show(app);
                let _ = app.emit("check-update", ());
            }
            "quit" => app.exit(0),
            _ => {}
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}

pub fn show_update(app: &AppHandle, version: &str) {
    let Some(tray) = app.tray_by_id("tray") else {
        return;
    };

    if let Ok(icon) = Image::from_bytes(include_bytes!("../icons/32x32-update.png")) {
        let _ = tray.set_icon(Some(icon));
    }

    let _ = tray.set_tooltip(Some(
        format!("Pikmin Mention {version} is available").as_str(),
    ));
}
