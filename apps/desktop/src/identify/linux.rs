//! The Linux side.

use tauri::{Runtime, WebviewWindow};

/// A window that was never shown has no GDK window yet. Tao needs one to let clicks through, and
/// the app aborts without it, so `shown` does it.
pub fn created<R: Runtime>(_window: &WebviewWindow<R>) -> tauri::Result<()> {
    Ok(())
}

/// Lets clicks through the box. Tao runs the window requests in order, so the box is shown when
/// this one runs. Some window managers place a window only when it maps, so the box moves again
/// then, with `place`.
pub fn shown<R: Runtime>(
    window: &WebviewWindow<R>,
    place: impl Fn() + Send + 'static,
) -> tauri::Result<()> {
    window.set_ignore_cursor_events(true)?;
    crate::app_windows::place_again_on_map(window, place)
}
