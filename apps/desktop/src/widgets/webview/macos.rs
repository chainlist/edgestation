//! The macOS side, a stub: the WKWebView that Tauri adds, with no change. A view keeps square
//! corners and covers the buttons of its widget. It reports no loads, addresses or full screen.
//! WKWebView has no data folder, so all profiles share the data of the Default profile.

use std::sync::mpsc;

use tauri::ipc::Channel;
use tauri::{Rect, Runtime, Webview, Window};

use super::open::PageLoad;
use super::shape::Bounds;
use super::user_agent::UserAgent;
use crate::os::macos::MISSING;

/// A new view is where Tauri put it, above the page.
pub fn added<R: Runtime>(_webview: &Webview<R>, _rect: Rect) -> tauri::Result<()> {
    Ok(())
}

pub fn set_bounds<R: Runtime>(webview: &Webview<R>, rect: Rect) -> tauri::Result<()> {
    webview.set_bounds(rect)
}

pub fn take_keyboard<R: Runtime>(_webview: &Webview<R>) -> tauri::Result<()> {
    Ok(())
}

pub fn shape<R: Runtime>(_webview: &Webview<R>, _bounds: &Bounds) -> tauri::Result<()> {
    Ok(())
}

pub fn report_loads<R: Runtime>(
    _webview: &Webview<R>,
    _on_load: Channel<PageLoad>,
) -> tauri::Result<()> {
    Ok(())
}

pub fn report_sources<R: Runtime>(
    _webview: &Webview<R>,
    _on_navigate: Channel<String>,
) -> tauri::Result<()> {
    Ok(())
}

pub fn keep_fullscreen<R: Runtime>(
    _webview: &Webview<R>,
    _window: Window<R>,
    _on_fullscreen: Channel<bool>,
) -> tauri::Result<()> {
    Ok(())
}

/// Only the user agent of the builder applies. WebKit sends no client hints.
pub async fn present_as<R: Runtime>(
    _webview: &Webview<R>,
    _user_agent: &UserAgent,
) -> Result<(), String> {
    Ok(())
}

pub async fn capture<R: Runtime>(_webview: &Webview<R>) -> Result<Vec<u8>, String> {
    Err(MISSING.into())
}

/// The profile has no data of its own, so the receiver is ready at once.
pub async fn release<R: Runtime>(
    _view: &Webview<R>,
    _profile: String,
) -> Result<mpsc::Receiver<()>, String> {
    let (ready_tx, ready) = mpsc::channel();
    let _ = ready_tx.send(());
    Ok(ready)
}
