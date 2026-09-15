use objc2_app_kit::{NSApplicationActivationOptions, NSRunningApplication, NSWorkspace};
use objc2_foundation::NSString;

use crate::platform::types::Frontmost;

pub fn frontmost_app() -> Option<Frontmost> {
    let app = NSWorkspace::sharedWorkspace().frontmostApplication()?;

    Some(Frontmost {
        name: app
            .localizedName()
            .map(|value| value.to_string())
            .unwrap_or_default(),
        bundle_id: app
            .bundleIdentifier()
            .map(|value| value.to_string())
            .unwrap_or_default(),
        pid: app.processIdentifier(),
    })
}

pub fn activate_app(bundle_id: &str) {
    if bundle_id.is_empty() {
        return;
    }

    let identifier = NSString::from_str(bundle_id);
    let matches = NSRunningApplication::runningApplicationsWithBundleIdentifier(&identifier);

    if let Some(app) = matches.iter().next() {
        app.activateWithOptions(NSApplicationActivationOptions::ActivateAllWindows);
    }
}
