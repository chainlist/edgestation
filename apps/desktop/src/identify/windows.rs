//! The Windows side.

use tauri::{Runtime, WebviewWindow};
use windows::Win32::Graphics::Dwm::{
    DWM_WINDOW_CORNER_PREFERENCE, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
    DwmSetWindowAttribute,
};

/// Lets clicks through the new box, and rounds its corners. Windows 11 rounds the corners of
/// framed windows only, and the box has no frame. Earlier versions do not know the attribute and
/// keep the corners square.
pub fn created<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    window.set_ignore_cursor_events(true)?;
    let hwnd = window.hwnd()?;
    let _ = unsafe {
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            std::ptr::from_ref(&DWMWCP_ROUND).cast(),
            size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
        )
    };
    Ok(())
}

/// The box was ready when it was created.
pub fn shown<R: Runtime>(
    _window: &WebviewWindow<R>,
    _place: impl Fn() + Send + 'static,
) -> tauri::Result<()> {
    Ok(())
}
