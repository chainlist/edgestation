//! The macOS side, a stub. No app is ever full screen for it, so the widgets never pause for one.

use tauri::AppHandle;

pub fn set(app: AppHandle, enabled: bool) -> Result<(), String> {
    if enabled {
        super::send(&app, false);
    }
    Ok(())
}
