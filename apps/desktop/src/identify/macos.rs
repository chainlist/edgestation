//! The macOS side. The boxes keep square corners for now.

use tauri::{Runtime, WebviewWindow};

/// Lets clicks through the new box.
pub fn created<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    window.set_ignore_cursor_events(true)
}

/// The box was ready when it was created.
pub fn shown<R: Runtime>(
    _window: &WebviewWindow<R>,
    _place: impl Fn() + Send + 'static,
) -> tauri::Result<()> {
    Ok(())
}
