//! The Edge window cannot take the focus, so a touch on it leaves the keyboard with the app in
//! front. The parts that take typed text borrow the focus, then give it back. The "Go to page"
//! box gives it back when the user picks a page. A text field or a web view gives it back when
//! the user touches something else on the Edge.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

use tauri::{AppHandle, Manager, Runtime, Webview};

use crate::changes::EDGE;

/// Gives the focus to the window of the calling page, then to web view `view` or to the page.
/// Keeps the window that had the focus before, so `focus_return` can give it back. If this
/// window already has the focus, the first lender stays.
/// Returns true when the focus came from an other window. Only the system can tell this. A click
/// makes `document.hasFocus()` true, but the keyboard stays with the window in front.
#[tauri::command]
pub fn focus_borrow<R: Runtime>(webview: Webview<R>, view: Option<String>) -> tauri::Result<bool> {
    let window = webview.window();
    let took = native::remember(&window)?;
    window.set_focus()?;
    match view.and_then(|label| crate::widgets::webview::view(webview.app_handle(), &label)) {
        Some(view) => crate::widgets::webview::focus(&view)?,
        None => webview.set_focus()?,
    }
    Ok(took)
}

/// Gives the focus back to the window that had it before `focus_borrow`. The Edge window then
/// takes no focus again.
#[tauri::command]
pub fn focus_return<R: Runtime>(app: AppHandle<R>) {
    native::give_back(app.get_window(EDGE));
}
