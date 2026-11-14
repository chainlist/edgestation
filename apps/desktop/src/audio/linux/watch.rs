//! The changes that the Edge listens to, while a widget asks for them. The sound server tells a
//! thread when an output, an input, an app stream or a default device changes. The Edge window
//! then gets `audio:changed`, and widgets read the mixer again.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use libpulse_binding::context::subscribe::InterestMaskSet;
use tauri::AppHandle;

use super::connection::Pulse;
use crate::audio::changed;

/// The number of the watch that runs, or 0 if none runs.
static WATCH: AtomicU64 = AtomicU64::new(0);
static NEXT_WATCH: AtomicU64 = AtomicU64::new(1);
/// How often the thread looks if it must stop, when nothing changes.
const CHECK: Duration = Duration::from_millis(500);
/// How long the thread waits before it connects again, after the sound server went away.
const RECONNECT: Duration = Duration::from_secs(2);

/// Starts a new watch, or stops it. A start replaces the watch that runs.
pub fn watch(app: &AppHandle, on: bool) -> Result<(), String> {
    if !on {
        unwatch();
        return Ok(());
    }
    let id = NEXT_WATCH.fetch_add(1, Ordering::SeqCst);
    WATCH.store(id, Ordering::SeqCst);
    let (started_tx, started) = mpsc::channel();
    let app = app.clone();
    thread::Builder::new()
        .name("audio changes".into())
        .spawn(move || listen(&app, id, started_tx))
        .map_err(|error| error.to_string())?;
    started.recv().map_err(|error| error.to_string())?
}

pub fn unwatch() {
    WATCH.store(0, Ordering::SeqCst);
}

/// The watch thread. It sends the result of the first connection to `started`, then listens
/// until `WATCH` holds a different number. If the server goes away, for example when PipeWire
/// restarts, it connects again.
fn listen(app: &AppHandle, id: u64, started: mpsc::Sender<Result<(), String>>) {
    let mut started = Some(started);
    while WATCH.load(Ordering::SeqCst) == id {
        match subscribe(app) {
            Ok(mut pulse) => {
                match started.take() {
                    Some(started) => drop(started.send(Ok(()))),
                    // The mixer can be different after the server comes back.
                    None => changed(app),
                }
                while WATCH.load(Ordering::SeqCst) == id {
                    if let Err(error) = pulse.run_for(CHECK) {
                        eprintln!("stopped listening to audio changes: {error}");
                        break;
                    }
                }
            }
            // The widget gets the error of the first connection. Later ones try again.
            Err(error) => {
                if let Some(started) = started.take() {
                    return drop(started.send(Err(error)));
                }
            }
        }
        if WATCH.load(Ordering::SeqCst) == id {
            thread::sleep(RECONNECT);
        }
    }
}

fn subscribe(app: &AppHandle) -> Result<Pulse, String> {
    let mut pulse = Pulse::connect()?;
    let app = app.clone();
    pulse.context.set_subscribe_callback(Some(Box::new(move |_, _, _| changed(&app))));
    let mask = InterestMaskSet::SINK
        | InterestMaskSet::SOURCE
        | InterestMaskSet::SINK_INPUT
        | InterestMaskSet::SERVER;
    let subscribed = pulse.wait(|context, slot| {
        context.subscribe(mask, move |done| {
            slot.replace(Some(done));
        });
    })?;
    if subscribed { Ok(pulse) } else { Err("cannot listen to audio changes".into()) }
}
