//! The levels that the Edge listens to, while a widget shows a meter. Neither Windows nor
//! PulseAudio sends levels as they change, so a thread reads them. It sends them while there is
//! sound. In silence, it reads less often and sends nothing.

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter};

use super::EDGE;
use super::native::Meter;

const LEVELS: &str = "audio:levels";

/// The interval to read and send the levels while there is sound. A peak shows within this time.
const FAST: Duration = Duration::from_millis(40);
/// The interval to read the levels after `LINGER` of silence, with nothing sent. The meters have
/// allready fallen by then.
const SLOW: Duration = Duration::from_millis(250);
const LINGER: Duration = Duration::from_secs(2);
/// A meter shows nothing for a peak below this value (-60 dB).
const SILENT: f32 = 0.001;

/// The number of the sampler that runs, or 0 if none runs.
static SAMPLER: AtomicU64 = AtomicU64::new(0);
static NEXT_SAMPLER: AtomicU64 = AtomicU64::new(1);

pub fn meters(app: &AppHandle, on: bool) {
    if !on {
        stop();
        return;
    }
    if SAMPLER.load(Ordering::SeqCst) != 0 {
        return;
    }
    let id = NEXT_SAMPLER.fetch_add(1, Ordering::SeqCst);
    SAMPLER.store(id, Ordering::SeqCst);
    let app = app.clone();
    let spawned =
        thread::Builder::new().name("audio levels".into()).spawn(move || sample(&app, id));
    if spawned.is_err() {
        SAMPLER.store(0, Ordering::SeqCst);
    }
}

/// Stops the sampler thread.
pub fn stop() {
    SAMPLER.store(0, Ordering::SeqCst);
}

/// The sampler thread. It reads and sends the levels until `SAMPLER` holds an different number.
fn sample(app: &AppHandle, id: u64) {
    // The meter keeps what each read needs, so a read does not set it up again.
    let mut meter = Meter::open();
    let mut sound: Option<Instant> = None;
    while SAMPLER.load(Ordering::SeqCst) == id {
        let started = Instant::now();
        match meter.read() {
            Ok(levels) => {
                if levels.loud(SILENT) {
                    sound = Some(started);
                }
                // Continue to send for `LINGER` after the sound stops, so the meters can fall.
                if sound.is_some_and(|at| at.elapsed() < LINGER)
                    && let Err(error) = app.emit_to(EDGE, LEVELS, &levels)
                {
                    eprintln!("failed to send audio levels: {error}");
                }
            }
            Err(error) => eprintln!("failed to read audio levels: {error}"),
        }
        let every = if sound.is_some_and(|at| at.elapsed() < LINGER) { FAST } else { SLOW };
        thread::sleep(every.saturating_sub(started.elapsed()));
    }
}
