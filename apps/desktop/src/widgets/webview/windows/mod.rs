//! The Windows side: WebView2, through the controller that Tauri gives each view. Each view is in
//! a window of its own, from wry.

mod capture;
mod events;
mod profiles;
mod shape;
mod user_agent;

use tauri::{Rect, Runtime, Webview};

pub use capture::capture;
pub use events::{keep_fullscreen, report_loads, report_sources};
pub use profiles::release;
pub use shape::shape;
pub use user_agent::present_as;

/// A new view is where Tauri put it.
pub fn added<R: Runtime>(_webview: &Webview<R>, _rect: Rect) -> tauri::Result<()> {
    Ok(())
}

pub fn set_bounds<R: Runtime>(webview: &Webview<R>, rect: Rect) -> tauri::Result<()> {
    webview.set_bounds(rect)
}

/// The focus of a view is enough for it to take the typed text.
pub fn take_keyboard<R: Runtime>(_webview: &Webview<R>) -> tauri::Result<()> {
    Ok(())
}
