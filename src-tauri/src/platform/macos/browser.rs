use std::process::Command;

fn script_for(bundle_id: &str) -> Option<&'static str> {
    match bundle_id {
        "com.google.Chrome" => {
            Some(r#"tell application "Google Chrome" to get URL of active tab of front window"#)
        }
        "com.microsoft.edgemac" => {
            Some(r#"tell application "Microsoft Edge" to get URL of active tab of front window"#)
        }
        "com.apple.Safari" => Some(r#"tell application "Safari" to get URL of front document"#),
        _ => None,
    }
}

pub fn active_url(bundle_id: &str) -> Option<String> {
    let script = script_for(bundle_id)?;
    let output = Command::new("osascript")
        .args(["-e", script])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let url = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if url.is_empty() {
        None
    } else {
        Some(url)
    }
}
