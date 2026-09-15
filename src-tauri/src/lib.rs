mod context;
mod hotkey;
mod overlay;
mod platform;
mod settings;
mod tray;

use tauri::AppHandle;

use context::{ActiveContext, ContextState};
use settings::Settings;

fn hide_then(app: &AppHandle, bundle_id: String, after: Option<u64>) {
    let handle = app.clone();

    let _ = app.run_on_main_thread(move || {
        let hidden = overlay::hide(&handle);
        let visible = overlay::get(&handle).and_then(|w| w.is_visible().ok());
        eprintln!("[overlay] hide -> {hidden:?}, visible = {visible:?}, prev = {bundle_id:?}");

        platform::activate_app(&bundle_id);

        let Some(delay) = after else {
            return;
        };

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(delay));
            platform::send_paste();
            std::thread::sleep(std::time::Duration::from_millis(80));
            platform::send_newline();
        });
    });
}

fn previous_app(state: &tauri::State<'_, ContextState>) -> String {
    state
        .0
        .lock()
        .map(|current| current.bundle_id.clone())
        .unwrap_or_default()
}

#[tauri::command]
fn dismiss(app: AppHandle, state: tauri::State<'_, ContextState>) {
    hide_then(&app, previous_app(&state), None);
}

#[tauri::command]
fn insert_snippet(app: AppHandle, state: tauri::State<'_, ContextState>) -> Result<(), String> {
    let previous = previous_app(&state);

    if !platform::accessibility_trusted() {
        hide_then(&app, previous, None);
        return Err("accessibility".into());
    }

    hide_then(&app, previous, Some(180));
    Ok(())
}

#[tauri::command]
fn accessibility_status() -> bool {
    platform::accessibility_trusted()
}

#[tauri::command]
fn request_accessibility() -> bool {
    let trusted = platform::request_accessibility();
    if !trusted {
        platform::open_accessibility_settings();
    }
    trusted
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardMention {
    text: String,
    html: Option<String>,
}

#[tauri::command]
fn read_clipboard_mention() -> Option<ClipboardMention> {
    platform::read_clipboard().map(|clipboard| ClipboardMention {
        text: clipboard.text,
        html: clipboard.html,
    })
}

#[tauri::command]
fn default_hotkey() -> &'static str {
    hotkey::DEFAULT
}

#[tauri::command]
fn reset_hotkey(app: AppHandle) -> Result<Settings, String> {
    hotkey::register(&app, hotkey::DEFAULT)?;

    let mut settings = settings::load(&app);
    settings.hotkey = hotkey::DEFAULT.to_string();
    settings::save(&app, &settings)?;

    Ok(settings)
}

#[tauri::command]
fn set_last_used(app: AppHandle, pack_id: String, id: String) -> Result<Settings, String> {
    let mut settings = settings::load(&app);
    settings.last_used.insert(pack_id, id);
    settings::save(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn set_pinned(app: AppHandle, pinned: Vec<String>) -> Result<Settings, String> {
    let mut settings = settings::load(&app);
    settings.pinned = pinned;
    settings::save(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn toggle_pin(app: AppHandle, id: String) -> Result<Settings, String> {
    let mut settings = settings::load(&app);

    if let Some(index) = settings.pinned.iter().position(|value| value == &id) {
        settings.pinned.remove(index);
    } else {
        settings.pinned.push(id);
    }

    settings::save(&app, &settings)?;
    Ok(settings)
}

fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_nanos())
        .unwrap_or_default();
    format!("{nanos:x}")
}

#[tauri::command]
fn add_custom(
    app: AppHandle,
    pack_id: String,
    label: String,
    insert: String,
    mention: bool,
    mention_text: Option<String>,
    mention_html: Option<String>,
) -> Result<Settings, String> {
    let mut settings = settings::load(&app);
    let id = format!("custom-{}", uuid());

    settings
        .custom
        .entry(pack_id)
        .or_default()
        .push(settings::CustomShortcut {
            id,
            label,
            insert,
            mention,
            mention_text,
            mention_html,
        });

    settings::save(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn remove_custom(app: AppHandle, pack_id: String, id: String) -> Result<Settings, String> {
    let mut settings = settings::load(&app);

    if let Some(entries) = settings.custom.get_mut(&pack_id) {
        entries.retain(|entry| entry.id != id);
    }
    settings.pinned.retain(|value| value != &id);

    settings::save(&app, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn get_active_context(state: tauri::State<'_, ContextState>) -> ActiveContext {
    state
        .0
        .lock()
        .map(|value| value.clone())
        .unwrap_or_default()
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Settings {
    settings::load(&app)
}

#[tauri::command]
fn set_hotkey(app: AppHandle, hotkey: String) -> Result<Settings, String> {
    let previous = settings::load(&app);

    if let Err(error) = hotkey::register(&app, &hotkey) {
        let _ = hotkey::register(&app, &previous.hotkey);
        return Err(error);
    }

    let next = Settings { hotkey, ..previous };
    settings::save(&app, &next)?;
    Ok(next)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ContextState::default())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            accessibility_status,
            add_custom,
            default_hotkey,
            dismiss,
            get_active_context,
            get_settings,
            insert_snippet,
            read_clipboard_mention,
            remove_custom,
            reset_hotkey,
            request_accessibility,
            set_hotkey,
            set_last_used,
            set_pinned,
            toggle_pin
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            tray::init(app.handle())?;

            let handle = app.handle().clone();
            platform::watch_outside_clicks(move || {
                let Some(window) = overlay::get(&handle) else {
                    return;
                };
                if !window.is_visible().unwrap_or(false) {
                    return;
                }
                if overlay::contains_point(&window, platform::screen_click_point()) {
                    return;
                }

                let _ = overlay::hide(&handle);
            });

            let mut stored = settings::load(app.handle());
            if hotkey::LEGACY_DEFAULTS.contains(&stored.hotkey.as_str()) {
                stored.hotkey = hotkey::DEFAULT.to_string();
                let _ = settings::save(app.handle(), &stored);
            }
            let configured = stored.hotkey;
            if let Err(error) = hotkey::register(app.handle(), &configured) {
                eprintln!("hotkey '{configured}' failed: {error}");
                if configured != hotkey::DEFAULT {
                    let _ = hotkey::register(app.handle(), hotkey::DEFAULT);
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to start Pikmin Mention");
}
