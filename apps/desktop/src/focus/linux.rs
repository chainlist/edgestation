//! The Linux side, on X11. The window manager tells which window has the focus, and gives it
//! back when the app asks as a pager does.

use std::sync::atomic::{AtomicU32, Ordering};

use tauri::{Runtime, Window};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ClientMessageEvent, ConnectionExt as _, EventMask};

use crate::os::linux::x11::kept;

/// The window that had the focus before the last `focus_borrow`, or 0.
static LENDER: AtomicU32 = AtomicU32::new(0);

/// Stores the window that has the focus, unless it is a window of this app. Returns true if
/// it stored it. The Edge window takes no focus, so this lets it take the focus until
/// `give_back`.
pub fn remember<R: Runtime>(own: &Window<R>) -> tauri::Result<bool> {
    own.set_focusable(true)?;
    let Some(x11) = kept() else { return Ok(false) };
    let other = x11.active().filter(|&window| !x11.is_own(window));
    if let Some(window) = other {
        LENDER.store(window, Ordering::Relaxed);
    }
    Ok(other.is_some())
}

/// Asks the window manager to give the focus to the lender. Source 2 says that the request
/// comes from a pager, as from a click on the taskbar, so the window manager does not refuse
/// it.
pub fn give_back<R: Runtime>(own: Option<Window<R>>) {
    let window = LENDER.swap(0, Ordering::Relaxed);
    if window != 0
        && let Some(x11) = kept()
    {
        let data = [2, x11rb::CURRENT_TIME, 0, 0, 0];
        let event = ClientMessageEvent::new(32, window, x11.atoms._NET_ACTIVE_WINDOW, data);
        let mask = EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY;
        let _ = x11.conn.send_event(false, x11.root, mask, event);
        let _ = x11.conn.flush();
    }
    if let Some(own) = own {
        let _ = own.set_focusable(false);
    }
}
