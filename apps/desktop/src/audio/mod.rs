//! The sound of the machine, as the Windows volume mixer shows it: the default output and
//! microphone, the apps that play through that output, and how loud each one is now. On Linux,
//! the same values come from PulseAudio, which PipeWire also serves.
//! The host reads and changes these values only when a widget asks. While a widget listens,
//! the system tells the app about changes and a thread sends the levels (see `sampler`). Nothing
//! runs at other times.

/// The side of each platform: Core Audio on Windows, PulseAudio on Linux.
#[cfg_attr(windows, path = "windows/mod.rs")]
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
mod native;

mod changed;
mod sampler;

use std::collections::HashMap;

use serde::Serialize;
use tauri::webview::{PageLoadEvent, PageLoadPayload};
use tauri::{AppHandle, Runtime, Webview};

use crate::blocking;

/// The window that gets the audio events.
const EDGE: &str = "edge";

#[derive(Serialize)]
pub struct Mixer {
    /// The default output, which the volume flyout shows.
    output: Option<Device>,
    /// The default microphone.
    input: Option<Device>,
    /// The apps that have a sound session on the default output. They are in name order, with
    /// System sounds last.
    apps: Vec<App>,
}

#[derive(Serialize)]
pub struct Device {
    /// The Windows id of the device, or the name of the PulseAudio sink or source.
    /// `audio_set_output` takes this id.
    id: String,
    /// The name that Sound settings shows, for example `Speakers (Realtek(R) Audio)`.
    name: String,
    /// From 0 to 1, as the slider in the volume flyout shows it.
    volume: f32,
    muted: bool,
}

/// The sound of one app: all the sessions that the the Windows volume mixer shows as one. On
/// Linux, all the streams of one executable on the default output.
#[derive(Serialize)]
pub struct App {
    /// Identifies the sessions of the app while it runs. The id is not the same after a restart.
    id: String,
    /// The name that the app gives its sessions. If it gives none, the host uses the description
    /// of the executable. If that is empty too, the host uses the file name.
    name: String,
    /// The path of the executable, for its icon. It is `None` for System sounds and for the rare
    /// executable without a drive letter. On Linux, it is `None` when the app runs as another
    /// user.
    path: Option<String>,
    /// The file name of the executable, for example `Discord.exe`. It identifies the app accross
    /// restarts and updates. The path cannot, because an update can move the folder. It is
    /// `None` for System sounds.
    program: Option<String>,
    /// From 0 to 1, as a fraction of the output volume.
    volume: f32,
    /// `true` if all the sessions of the app are muted.
    muted: bool,
    /// `true` if the app plays sound now. An open session with no sound is not active.
    active: bool,
}

/// The sound levels now. Each value is the peak sample of the last few miliseconds, from 0 to 1.
#[derive(Clone, Default, Serialize)]
pub struct Levels {
    output: f32,
    /// The default microphone. The value is 0 when no app records from it.
    input: f32,
    /// The keys are app ids.
    apps: HashMap<String, f32>,
}

impl Levels {
    fn loud(&self, threshold: f32) -> bool {
        self.output >= threshold
            || self.input >= threshold
            || self.apps.values().any(|&level| level >= threshold)
    }
}

#[derive(Serialize)]
pub struct Output {
    id: String,
    name: String,
}

/// The target of a volume or mute change. The `target` argument of a command names it.
pub enum Target {
    Output,
    Input,
    App(String),
}

impl From<String> for Target {
    fn from(target: String) -> Self {
        match target.as_str() {
            "output" => Self::Output,
            "input" => Self::Input,
            _ => Self::App(target),
        }
    }
}

#[tauri::command]
pub async fn audio_mixer() -> Result<Mixer, String> {
    blocking::run(mixer).await
}

#[tauri::command]
pub async fn audio_levels() -> Result<Levels, String> {
    blocking::run(levels).await
}

/// Sets the volume of `target` to a value from 0 to 1. `target` is `output`, `input` or an app id.
/// Returns `false` if the target is gone, for example if the app closed or the device was
/// unplugged.
#[tauri::command]
pub async fn audio_set_volume(target: String, volume: f32) -> Result<bool, String> {
    if !(0.0..=1.0).contains(&volume) {
        return Err(format!("Volume {volume} is not between 0 and 1"));
    }
    blocking::run(move || set_volume(target.into(), volume)).await
}

/// Mutes or unmutes `target`, as `audio_set_volume` does. For `input`, this also changes the
/// default microphone for calls if it is a diferent device (Windows only).
#[tauri::command]
pub async fn audio_set_muted(target: String, muted: bool) -> Result<bool, String> {
    blocking::run(move || set_muted(target.into(), muted)).await
}

/// Returns all output devices that are plugged in and enabled, in name order.
#[tauri::command]
pub async fn audio_outputs() -> Result<Vec<Output>, String> {
    blocking::run(outputs).await
}

/// Makes output `id` the default output. Returns `false` if the device is no longer plugged in.
#[tauri::command]
pub async fn audio_set_output(id: String) -> Result<bool, String> {
    blocking::run(move || set_output(&id)).await
}

/// While `on` is true, sends `audio:changed` to the Edge window when the mixer changes. A change
/// is a new volume or mute, a session that opens, closes, starts or stops, or a new default
/// device.
#[tauri::command]
pub async fn audio_watch(app: AppHandle, on: bool) -> Result<(), String> {
    blocking::run(move || watch(&app, on)).await
}

/// While `on` is true, sends `audio:levels` to the Edge window 25 times a second when there is
/// sound.
#[tauri::command]
pub fn audio_meters(app: AppHandle, on: bool) {
    meters(&app, on);
}

/// Stops the change callbacks and the meters when the Edge page starts to load. The widget that
/// listened are gone then.
pub fn stop_on_reload<R: Runtime>(page: &Webview<R>, payload: &PageLoadPayload<'_>) {
    if payload.event() == PageLoadEvent::Started && page.label() == "edge" {
        tauri::async_runtime::spawn_blocking(stop);
    }
}

use native::{levels, mixer, outputs, set_muted, set_output, set_volume, watch};
use sampler::meters;

/// Stops the change callbacks and the level thread.
fn stop() {
    sampler::stop();
    native::unwatch();
}
