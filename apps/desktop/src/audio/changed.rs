//! The `audio:changed` event to the Edge window. The change callbacks of Windows and the
//! PulseAudio subscription on Linux send it from here.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter};

use super::EDGE;

const CHANGED: &str = "audio:changed";

/// The minimum time between two change events. A burst, for example a slider drag in the volume
/// flyout, sends one event at once and one event when it stops.
const THROTTLE: Duration = Duration::from_millis(100);

/// The time of the last change event, and wether one more event waits for the throttle.
static LAST: Mutex<Option<Instant>> = Mutex::new(None);
static PENDING: AtomicBool = AtomicBool::new(false);

/// Tells the Edge window that the mixer changed, at most once per `THROTTLE`.
pub fn changed(app: &AppHandle) {
    let mut last = LAST.lock().unwrap_or_else(PoisonError::into_inner);
    let now = Instant::now();
    match *last {
        Some(at) if now - at < THROTTLE => {
            if !PENDING.swap(true, Ordering::AcqRel) {
                let app = app.clone();
                let wait = THROTTLE - (now - at);
                tauri::async_runtime::spawn_blocking(move || {
                    thread::sleep(wait);
                    PENDING.store(false, Ordering::Release);
                    *LAST.lock().unwrap_or_else(PoisonError::into_inner) = Some(Instant::now());
                    send(&app);
                });
            }
        }
        _ => {
            *last = Some(now);
            drop(last);
            send(app);
        }
    }
}

fn send(app: &AppHandle) {
    if let Err(error) = app.emit_to(EDGE, CHANGED, ()) {
        eprintln!("failed to send an audio change: {error}");
    }
}
