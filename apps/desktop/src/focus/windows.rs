//! The Windows side. Windows tells which window is in front, and lets this app give the focus
//! away while the app has it.

use std::ffi::c_void;
use std::sync::atomic::{AtomicIsize, Ordering};

use tauri::{Runtime, Window};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};

/// The window that had the focus before the last `focus_borrow`.
static LENDER: AtomicIsize = AtomicIsize::new(0);

/// Stores the window in front, unless it is `own`. Returns true if it stored it.
pub fn remember<R: Runtime>(own: &Window<R>) -> tauri::Result<bool> {
    let front = unsafe { GetForegroundWindow() };
    let other = front != own.hwnd()?;
    if other {
        LENDER.store(front.0 as isize, Ordering::Relaxed);
    }
    Ok(other)
}

/// Windows lets this app give the focus away only while the app has it. The app keeps the
/// focus until this call.
pub fn give_back<R: Runtime>(_own: Option<Window<R>>) {
    let window = HWND(LENDER.swap(0, Ordering::Relaxed) as *mut c_void);
    if !window.is_invalid() {
        let _ = unsafe { SetForegroundWindow(window) };
    }
}
