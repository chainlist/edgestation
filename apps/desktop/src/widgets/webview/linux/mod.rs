//! The Linux side: WebKitGTK. Tauri packs a child view below the page, so each view moves into an
//! X window of its own (see `layer`).

mod capture;
mod events;
mod layer;
mod profiles;
mod shape;

use tauri::{Rect, Runtime, Webview};

pub use capture::capture;
pub use events::{keep_fullscreen, report_loads, report_sources};
pub use layer::{set_bounds, take_keyboard};
pub use profiles::release;
pub use shape::shape;

/// Moves a new view out of the box of its window, into a window of its own over the page, at
/// `rect`.
pub fn added<R: Runtime>(webview: &Webview<R>, rect: Rect) -> tauri::Result<()> {
    set_bounds(webview, rect)
}

/// Only the user agent of the builder applies. WebKit sends no client hints.
pub async fn present_as<R: Runtime>(
    _webview: &Webview<R>,
    _user_agent: &crate::widgets::webview::user_agent::UserAgent,
) -> Result<(), String> {
    Ok(())
}
