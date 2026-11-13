//! What plays on the machine, from the media sessions that apps publish to Windows, or the MPRIS
//! players on Linux. The volume flyout of Windows and the media controls of Linux desktops show
//! the same sessions. The host reads them when a widget asks, and nothing listens between the
//! requests.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

mod art;
mod progress;

use serde::{Deserialize, Serialize};

use crate::blocking;

#[derive(Serialize)]
pub struct MediaInfo {
    /// The app that owns the session, as Windows names it, for example `Spotify.exe` or `MSEdge`.
    /// On Linux, the desktop entry of the player, for example `spotify` or `firefox`.
    app: String,
    title: String,
    artist: String,
    album: String,
    playing: bool,
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
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Play,
    Pause,
    Next,
    Previous,
}

/// The session wich Windows thinks is current, or `None` if nothing plays. On Linux, this is the
/// first player that plays, else the player that playerctld follows, else a paused player.
#[tauri::command]
pub async fn media_info() -> Result<Option<MediaInfo>, String> {
    blocking::run(read).await
}

/// The album art from the latest `media_info`, as a data URL.
#[tauri::command]
pub async fn media_art() -> Result<Option<String>, String> {
    Ok(art::current())
}

/// Asks the app of the current session to play, pause or skip. Returns `false` if the app refuses.
#[tauri::command]
pub async fn media_control(action: Action) -> Result<bool, String> {
    blocking::run(move || control(action)).await
}

use native::{control, read};
