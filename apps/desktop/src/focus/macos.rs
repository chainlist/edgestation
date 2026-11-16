//! The macOS side, a first version. The Edge window can take the focus untill `give_back`, but
//! the focus then stays with this app. The real side gives it back to the app that was in front.

use tauri::{Runtime, Window};

/// Lets the Edge window take the focus. It stores no other app, so it returns false.
pub fn remember<R: Runtime>(own: &Window<R>) -> tauri::Result<bool> {
    own.set_focusable(true)?;
    Ok(false)
}

pub fn give_back<R: Runtime>(own: Option<Window<R>>) {
    if let Some(own) = own {
        let _ = own.set_focusable(false);
    }
}
