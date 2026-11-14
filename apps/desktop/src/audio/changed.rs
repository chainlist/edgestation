//! The `audio:changed` event to the Edge window. The change callbacks of Windows and the
//! PulseAudio subscription on Linux send it from here.

use tauri::AppHandle;

use crate::changes::Throttled;

static CHANGED: Throttled = Throttled::new("audio:changed");

/// Tells the Edge window that the mixer changed.
pub fn changed(app: &AppHandle) {
    CHANGED.send(app);
}
