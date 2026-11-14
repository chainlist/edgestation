//! What plays on the machine, from the media sessions that apps publish to Windows, or the MPRIS
//! players on Linux. The volume flyout of Windows and the media controls of Linux desktops show
//! the same sessions. The host reads them when a widget asks. While a widget listens, the system
//! tells the app when they change, and the Edge window gets `media:changed`. Nothing runs at other
//! times.

#[cfg_attr(windows, path = "windows/mod.rs")]
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
mod native;

mod art;
mod progress;

use std::sync::{Mutex, PoisonError};
use std::time::Instant;

use serde::{Deserialize, Serialize};
use tauri::webview::PageLoadPayload;
use tauri::{AppHandle, Runtime, Webview};

use crate::blocking;
use crate::changes::{self, Throttled};
use progress::Progress;

static PROGRESS: Mutex<Progress> = Mutex::new(Progress::new());
static CHANGED: Throttled = Throttled::new("media:changed");

/// Tells the Edge window that a session opened, closed or changed.
fn changed(app: &AppHandle) {
    CHANGED.send(app);
}

#[derive(Serialize)]
pub struct MediaInfo {
    /// Names the session in `media_art` and `media_control`. On Windows, the app id, with `#2`,
    /// `#3`, ... for the next sessions of the same app. On Linux, the bus name of the player.
    id: String,
    /// True for the session that the media keys control.
    current: bool,
    /// The app that owns the session, as Windows names it, for example `Spotify.exe` or `MSEdge`.
    /// On Linux, the desktop entry of the player, for example `spotify` or `firefox`.
    app: String,
    title: String,
    artist: String,
    album: String,
    playing: bool,
    /// The playback speed, 1 at normal speed. While the track plays, the position moves on by
    /// this many seconds per second, and no event tells about it.
    rate: f64,
    /// The position in the track in seconds, at the time of the call. `None` if the app reports
    /// no timeline. When the app clears the timeline, the host carries the last position foward
    /// (see `progress`).
    position: Option<f64>,
    duration: Option<f64>,
    /// Changes when the album art changes. `None` if there is no art. `media_art` returns the art.
    art: Option<u64>,
    can: Controls,
}

/// The actions that the app accepts now.
#[derive(Serialize)]
struct Controls {
    play: bool,
    pause: bool,
    next: bool,
    previous: bool,
    /// True if the app accepts `media_seek`.
    seek: bool,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Play,
    Pause,
    Next,
    Previous,
}

/// A session as the platform reads it, before the host adds the carried position and the art.
struct Session {
    id: String,
    current: bool,
    app: String,
    title: String,
    artist: String,
    album: String,
    playing: bool,
    /// 1 at normal speed.
    rate: f64,
    /// The position and the length in seconds, as the app reports them.
    timeline: Option<(f64, f64)>,
    can: Controls,
}

impl Session {
    /// The info of the session. `art` reads the album art as a data URL.
    fn info(self, art: impl FnOnce() -> Option<String>) -> MediaInfo {
        let track = format!("{}\n{}\n{}", self.title, self.artist, self.album);
        let (position, duration) = PROGRESS
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .update(Instant::now(), &self.id, &track, self.timeline, self.playing, self.rate)
            .unzip();
        let art = art::id(&self.id, track, self.current, art);
        MediaInfo {
            id: self.id,
            current: self.current,
            app: self.app,
            title: self.title,
            artist: self.artist,
            album: self.album,
            playing: self.playing,
            rate: self.rate,
            position,
            duration,
            art,
            can: self.can,
        }
    }
}

/// The session wich Windows thinks is current, or `None` if nothing plays. On Linux, this is the
/// first player that plays, else the player that playerctld follows, else a paused player.
#[tauri::command]
pub async fn media_info() -> Result<Option<MediaInfo>, String> {
    blocking::run(read).await
}

/// All sessions, in the order of the platform. On Linux, the players that play or are paused.
#[tauri::command]
pub async fn media_sessions() -> Result<Vec<MediaInfo>, String> {
    blocking::run(read_all).await
}

/// The album art of session `id` from the latest read, as a data URL. Without `id`, the art of
/// the session that the latest read found current.
#[tauri::command]
pub async fn media_art(id: Option<String>) -> Result<Option<String>, String> {
    Ok(art::get(id.as_deref()))
}

/// Asks the app of session `id`, or of the current session, to play, pause or skip. Returns
/// `false` if the app refuses or the session is gone.
#[tauri::command]
pub async fn media_control(action: Action, id: Option<String>) -> Result<bool, String> {
    blocking::run(move || control(action, id.as_deref())).await
}

/// Asks the app of session `id`, or of the current session, to go to `position` seconds in the
/// track. Returns `false` if the app refuses or the session is gone.
#[tauri::command]
pub async fn media_seek(position: f64, id: Option<String>) -> Result<bool, String> {
    if position < 0.0 {
        return Err(format!("Position {position} is negative"));
    }
    blocking::run(move || seek(position, id.as_deref())).await
}

/// While `on` is true, sends `media:changed` to the Edge window when a session opens, closes or
/// becomes current, or when its track, playback state or timeline changes.
#[tauri::command]
pub async fn media_watch(app: AppHandle, on: bool) -> Result<(), String> {
    blocking::run(move || {
        native::watch(&app, on)?;
        let mut progress = PROGRESS.lock().unwrap_or_else(PoisonError::into_inner);
        progress.watching(on.then(Instant::now));
        Ok(())
    })
    .await
}

/// Stops the watch when the Edge page starts to load. The widgets that listened are gone then.
pub fn stop_on_reload<R: Runtime>(page: &Webview<R>, payload: &PageLoadPayload<'_>) {
    if changes::edge_reloads(page, payload) {
        tauri::async_runtime::spawn_blocking(|| {
            native::unwatch();
            PROGRESS.lock().unwrap_or_else(PoisonError::into_inner).watching(None);
        });
    }
}

use native::{control, read, read_all, seek};
