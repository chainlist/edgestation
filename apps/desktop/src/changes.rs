//! Change events to the Edge window. While a widget listens, a watch sends one when the system
//! tells it that something changed. The widgets then read the new values.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use tauri::webview::{PageLoadEvent, PageLoadPayload};
use tauri::{AppHandle, Emitter, Runtime, Webview};

/// The window that gets the change events.
pub const EDGE: &str = "edge";

/// True when the Edge page starts to load. The widgets that listened are gone then, so the
/// watches stop.
pub fn edge_reloads<R: Runtime>(page: &Webview<R>, payload: &PageLoadPayload<'_>) -> bool {
    payload.event() == PageLoadEvent::Started && page.label() == EDGE
}

/// The minimum time between two events of one kind. A burst, for example a slider drag in the
/// volume flyout, sends one event at once and one event when it stops.
const THROTTLE: Duration = Duration::from_millis(100);

/// An event that goes to the Edge window at most once per `THROTTLE`.
pub struct Throttled {
    event: &'static str,
    /// The time of the last event.
    last: Mutex<Option<Instant>>,
    /// True while one more event waits for the throttle.
    pending: AtomicBool,
}

impl Throttled {
    pub const fn new(event: &'static str) -> Self {
        Self { event, last: Mutex::new(None), pending: AtomicBool::new(false) }
    }

    /// Tells the Edge window that something changed, at most once per `THROTTLE`.
    pub fn send(&'static self, app: &AppHandle) {
        let mut last = self.last.lock().unwrap_or_else(PoisonError::into_inner);
        let now = Instant::now();
        match *last {
            Some(at) if now - at < THROTTLE => {
                if !self.pending.swap(true, Ordering::AcqRel) {
                    let app = app.clone();
                    let wait = THROTTLE - (now - at);
                    tauri::async_runtime::spawn_blocking(move || {
                        thread::sleep(wait);
                        self.pending.store(false, Ordering::Release);
                        *self.last.lock().unwrap_or_else(PoisonError::into_inner) =
                            Some(Instant::now());
                        self.emit(&app);
                    });
                }
            }
            _ => {
                *last = Some(now);
                drop(last);
                self.emit(app);
            }
        }
    }

    fn emit(&self, app: &AppHandle) {
        if let Err(error) = app.emit_to(EDGE, self.event, ()) {
            eprintln!("failed to send {}: {error}", self.event);
        }
    }
}
