//! The Windows side.

use tauri::{PhysicalPosition, Runtime, WebviewWindow, Window};
use windows::Win32::UI::WindowsAndMessaging::{SW_HIDE, SW_SHOWNOACTIVATE, ShowWindow};

pub fn set_visible<R: Runtime>(window: &Window<R>, visible: bool) -> tauri::Result<()> {
    let hwnd = window.hwnd()?;
    // SAFETY: a plain call on a window of this app.
    let _ = unsafe { ShowWindow(hwnd, if visible { SW_SHOWNOACTIVATE } else { SW_HIDE }) };
    Ok(())
}

/// Full screen applies to the monitor that holds the window, so the window moves there
/// first.
pub fn fullscreen_on<R: Runtime>(window: &Window<R>, x: i32, y: i32) -> tauri::Result<()> {
    window.set_fullscreen(false)?;
    window.set_position(PhysicalPosition::new(x, y))?;
    window.set_fullscreen(true)
}

/// Windows keeps the place that the app asks for a window that does not show yet, so it never
/// places a window again.
pub fn place_again_on_map<R: Runtime>(
    _window: &WebviewWindow<R>,
    _place: impl Fn() + Send + 'static,
) -> tauri::Result<()> {
    Ok(())
}
