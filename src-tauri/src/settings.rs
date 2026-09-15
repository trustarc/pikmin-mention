use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::hotkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CustomShortcut {
    pub id: String,
    pub label: String,
    pub insert: String,
    #[serde(default)]
    pub mention: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub hotkey: String,
    pub pinned: Vec<String>,
    pub custom: HashMap<String, Vec<CustomShortcut>>,
    pub last_used: HashMap<String, String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hotkey: hotkey::DEFAULT.to_string(),
            pinned: Vec::new(),
            custom: HashMap::new(),
            last_used: HashMap::new(),
        }
    }
}

fn file(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    Ok(dir.join("settings.json"))
}

pub fn load(app: &AppHandle) -> Settings {
    file(app)
        .ok()
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

pub fn save(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let raw = serde_json::to_string_pretty(settings).map_err(|error| error.to_string())?;
    fs::write(file(app)?, raw).map_err(|error| error.to_string())
}

impl Default for CustomShortcut {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            insert: String::new(),
            mention: false,
            mention_text: None,
            mention_html: None,
        }
    }
}
