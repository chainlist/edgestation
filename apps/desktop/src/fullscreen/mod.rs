//! Tells the Edge window when a window of another app is full screen, so it can pause its
//! widgets. The system calls this code when the focused window changes. It also calls it when
//! the app in front moves, resizes, hides or closes a window. Nothing polls, and nothing listens
//! when the Edge window does not watch. The windows of this app never count. A touch on the Edge
//! keeps the state of the app that was in front.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

use tauri::{AppHandle, Emitter};

use crate::changes::EDGE;

/// The event to the Edge window. It is `true` while another app is full screen, then `false`.
const EVENT: &str = "fullscreen:changed";

/// Starts or stops the watch. A start sends the current state at once, also when the watch
/// already runs.
#[tauri::command]
pub fn fullscreen_watch(app: AppHandle, on: bool) -> Result<(), String> {
    native::set(app, on)
}

/// Sends the state to the Edge window. Each side calls this only when the state changed, or when
/// a watch starts.
fn send(app: &AppHandle, fullscreen: bool) {
    if let Err(error) = app.emit_to(EDGE, EVENT, fullscreen) {
        eprintln!("could not send the full-screen state: {error}");
    }
}
