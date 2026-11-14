//! The changes that the Edge listens to, while a widget asks for them. Windows calls back when
//! the default devices, the sessions of the output or the device list change. The Edge window
//! then gets `audio:changed`, and widgets read the mixer again.

mod events;
mod wiring;

use std::sync::PoisonError;

use super::with_com;
use wiring::WATCH;

pub use wiring::{rewire, watch};

/// Stops the change callbacks.
pub fn unwatch() {
    let _ = with_com(|| unsafe {
        if let Some(old) = WATCH.lock().unwrap_or_else(PoisonError::into_inner).take() {
            old.stop();
        }
        Ok(())
    });
}
