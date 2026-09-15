use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use url::Url;

use crate::platform;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveContext {
    pub app: String,
    pub bundle_id: String,
    pub pid: i32,
    pub browser: Option<String>,
    pub hostname: Option<String>,
}

#[derive(Default)]
pub struct ContextState(pub Mutex<ActiveContext>);

fn browser_id(identity: &str) -> Option<&'static str> {
    let lower = identity.to_lowercase();

    match lower.as_str() {
        "com.google.chrome" => return Some("chrome"),
        "com.microsoft.edgemac" => return Some("edge"),
        "com.apple.safari" => return Some("safari"),
        "org.mozilla.firefox" | "org.mozilla.firefoxdeveloperedition" => return Some("firefox"),
        _ => {}
    }

    let executable = lower.rsplit(['\\', '/']).next().unwrap_or(&lower);

    match executable {
        "chrome.exe" | "chrome" => Some("chrome"),
        "msedge.exe" | "msedge" => Some("edge"),
        "firefox.exe" | "firefox" => Some("firefox"),
        _ => None,
    }
}

fn hostname_of(url: &str) -> Option<String> {
    let parsed = Url::parse(url)
        .or_else(|_| Url::parse(&format!("https://{url}")))
        .ok()?;

    parsed
        .host_str()
        .map(|host| host.trim_start_matches("www.").to_lowercase())
}

pub fn capture() -> ActiveContext {
    let Some(front) = platform::frontmost_app() else {
        return ActiveContext::default();
    };

    let browser = browser_id(&front.bundle_id);
    let url = match browser {
        Some("firefox") => platform::browser_url_via_a11y(front.pid),
        Some(_) => platform::active_url(&front.bundle_id)
            .or_else(|| platform::browser_url_via_a11y(front.pid)),
        None => None,
    };

    ActiveContext {
        app: front.name,
        bundle_id: front.bundle_id,
        pid: front.pid,
        browser: browser.map(str::to_string),
        hostname: url.as_deref().and_then(hostname_of),
    }
}

pub fn refresh(app: &AppHandle) {
    let captured = capture();

    if let Ok(mut current) = app.state::<ContextState>().0.lock() {
        *current = captured.clone();
    }

    let _ = app.emit("context", captured);
}
