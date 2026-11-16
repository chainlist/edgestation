//! The macOS side, a first version with the window calls of Tauri only.

use tauri::{PhysicalPosition, Runtime, WebviewWindow, Window};

/// Tauri `show` makes the app active, so it takes the focus from the app in front. The real side
/// shows the window without that.
pub fn set_visible<R: Runtime>(window: &Window<R>, visible: bool) -> tauri::Result<()> {
    if visible { window.show() } else { window.hide() }
}

/// Full screen applies to the monitor that holds the window, so the window moves there first.
/// macOS puts a full-screen window in a Space of its own. The real side covers the monitor and
/// its menu bar without a new Space.
pub fn fullscreen_on<R: Runtime>(window: &Window<R>, x: i32, y: i32) -> tauri::Result<()> {
    window.set_fullscreen(false)?;
    window.set_position(PhysicalPosition::new(x, y))?;
    window.set_fullscreen(true)
}

/// macOS keeps the place that the app asks for a window that does not show yet.
pub fn place_again_on_map<R: Runtime>(
    _window: &WebviewWindow<R>,
    _place: impl Fn() + Send + 'static,
) -> tauri::Result<()> {
    Ok(())
}
