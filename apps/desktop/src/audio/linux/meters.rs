//! Peak meters, as pavucontrol makes them: a recording stream of 25 peaks a second on the monitor
//! of the default output, one more on each app (`set_monitor_stream`), and one on the
//! microphone while an app records from it. A meter on an idle microphone would show it as in
//! use, so there is none then.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use libpulse_binding::context::Context;
use libpulse_binding::def::BufferAttr;
use libpulse_binding::sample::{Format, Spec};
use libpulse_binding::stream::{FlagSet, PeekResult, Stream};

use super::connection::{Pulse, message};
use super::mixer::{app_id, microphone};
use crate::audio::Levels;

/// The peaks per second of each meter.
const RATE: u32 = 25;
/// How long the meters of one list serve, as on Windows. An app that starts in this time gets a
/// meter at the next list.
const LIST_FOR: Duration = Duration::from_secs(1);
/// How long a one-time read waits for the first peaks.
const FIRST_PEAKS: Duration = Duration::from_millis(150);
/// How long the sampler waits before it connects again, after the sound server went away.
const RECONNECT: Duration = Duration::from_secs(2);

/// What a meter measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Key {
    Output,
    Input,
    /// The stream of an app, by its index.
    App(u32),
}

/// The source that a meter records, and the app stream on it, if any.
type Target = (u32, Option<u32>);

struct Peak {
    stream: Stream,
    target: Target,
    /// The last peak, from 0 to 1.
    level: f32,
}

/// The meters of one connection.
struct Meters {
    peaks: HashMap<Key, Peak>,
    /// The app id of each app stream.
    apps: HashMap<u32, String>,
    listed: Instant,
    pulse: Pulse,
}

/// Reads the levels for the sampler thread. It keeps its meters between reads, and lists them
/// again each `LIST_FOR`.
pub struct Meter {
    meters: Option<Meters>,
    failed: Option<Instant>,
}

impl Meter {
    pub fn open() -> Self {
        Self { meters: None, failed: None }
    }

    pub fn read(&mut self) -> Result<Levels, String> {
        if self.failed.is_some_and(|at| at.elapsed() < RECONNECT) {
            return Ok(Levels::default());
        }
        let read = self.started().and_then(Meters::read);
        if read.is_err() {
            self.meters = None;
            self.failed = Some(Instant::now());
        }
        read
    }

    fn started(&mut self) -> Result<&mut Meters, String> {
        if self.meters.is_none() {
            self.meters = Some(Meters::start()?);
        }
        Ok(self.meters.as_mut().expect("started"))
    }
}

/// The levels now, for one request. The meters give their first peaks after a short time.
pub fn levels() -> Result<Levels, String> {
    let mut meters = Meters::start()?;
    let start = Instant::now();
    while start.elapsed() < FIRST_PEAKS {
        meters.pulse.run_for(FIRST_PEAKS.saturating_sub(start.elapsed()))?;
    }
    meters.read()
}

impl Meters {
    fn start() -> Result<Self, String> {
        let mut meters = Self {
            peaks: HashMap::new(),
            apps: HashMap::new(),
            listed: Instant::now(),
            pulse: Pulse::connect()?,
        };
        meters.list()?;
        Ok(meters)
    }

    fn read(&mut self) -> Result<Levels, String> {
        if self.listed.elapsed() >= LIST_FOR {
            self.list()?;
        }
        self.pulse.drain()?;
        let mut levels = Levels::default();
        for (key, peak) in &mut self.peaks {
            let level = peak.take();
            match key {
                Key::Output => levels.output = level,
                Key::Input => levels.input = level,
                Key::App(index) => {
                    if let Some(id) = self.apps.get(index) {
                        let app = levels.apps.entry(id.clone()).or_default();
                        *app = app.max(level);
                    }
                }
            }
        }
        Ok(levels)
    }

    /// Opens a meter for each output, app and microphone to measure now, and closes the others.
    fn list(&mut self) -> Result<(), String> {
        self.listed = Instant::now();
        let mut wanted: HashMap<Key, Target> = HashMap::new();
        self.apps.clear();
        let (output, input) = self.pulse.defaults()?;
        let sinks = self.pulse.sinks()?;
        if let Some(sink) = sinks.iter().find(|sink| Some(&sink.name) == output.as_ref()) {
            wanted.insert(Key::Output, (sink.monitor, None));
            for stream in self.pulse.inputs()?.iter().filter(|stream| stream.sink == sink.index) {
                wanted.insert(Key::App(stream.index), (sink.monitor, Some(stream.index)));
                self.apps.insert(stream.index, app_id(stream));
            }
        }
        let sources = self.pulse.sources()?;
        if let Some(microphone) = microphone(&sources, input.as_deref()) {
            let own = self.pulse.context.get_index();
            let recorded =
                self.pulse.recordings()?.iter().any(|recording| {
                    recording.source == microphone.index && recording.client != own
                });
            if recorded {
                wanted.insert(Key::Input, (microphone.index, None));
            }
        }
        self.peaks.retain(|key, peak| {
            let keep = wanted.get(key) == Some(&peak.target);
            if !keep {
                peak.close();
            }
            keep
        });
        for (key, target) in wanted {
            if self.peaks.contains_key(&key) {
                continue;
            }
            match Peak::open(&mut self.pulse.context, target) {
                Ok(peak) => drop(self.peaks.insert(key, peak)),
                Err(error) => eprintln!("cannot measure {key:?}: {error}"),
            }
        }
        Ok(())
    }
}

impl Drop for Meters {
    fn drop(&mut self) {
        for peak in self.peaks.values_mut() {
            peak.close();
        }
    }
}

impl Peak {
    fn open(context: &mut Context, target: Target) -> Result<Self, String> {
        let (source, app) = target;
        let spec = Spec { format: Format::FLOAT32NE, channels: 1, rate: RATE };
        let mut stream =
            Stream::new(context, "Peak detect", &spec, None).ok_or("cannot make a meter")?;
        if let Some(app) = app {
            stream.set_monitor_stream(app).map_err(message)?;
        }
        // One peak per fragment, so each one comes at once.
        let attr = BufferAttr {
            maxlength: u32::MAX,
            tlength: u32::MAX,
            prebuf: u32::MAX,
            minreq: u32::MAX,
            fragsize: size_of::<f32>() as u32,
        };
        let flags = FlagSet::DONT_MOVE
            | FlagSet::PEAK_DETECT
            | FlagSet::ADJUST_LATENCY
            | FlagSet::DONT_INHIBIT_AUTO_SUSPEND;
        stream.connect_record(Some(&source.to_string()), Some(&attr), flags).map_err(message)?;
        Ok(Self { stream, target, level: 0.0 })
    }

    /// The highest peak since the last read. Without a new peak, the last one stays, so a read
    /// that comes just before a peak does not show silence.
    fn take(&mut self) -> f32 {
        let mut highest: Option<f32> = None;
        loop {
            match self.stream.peek() {
                Ok(PeekResult::Data(data)) => {
                    let peaks =
                        data.as_chunks::<4>().0.iter().map(|&bytes| f32::from_ne_bytes(bytes));
                    highest = peaks.chain(highest).reduce(f32::max);
                }
                Ok(PeekResult::Hole(_)) => {}
                Ok(PeekResult::Empty) | Err(_) => break,
            }
            if self.stream.discard().is_err() {
                break;
            }
        }
        if let Some(highest) = highest {
            self.level = highest.clamp(0.0, 1.0);
        }
        self.level
    }

    fn close(&mut self) {
        let _ = self.stream.disconnect();
    }
}
