//! The Windows side: the global media transport controls, the sessions that apps publish for their
//! media keys.

use std::sync::{Mutex, PoisonError};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as Session,
    GlobalSystemMediaTransportControlsSessionManager as Manager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as Properties,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as Status,
    GlobalSystemMediaTransportControlsSessionTimelineProperties as Timeline,
};
use windows::Storage::Streams::DataReader;

use super::art;
use super::progress::Progress;
use super::{Action, Controls, MediaInfo};

/// Windows times are in ticks of 100 ns.
const TICKS_PER_SECOND: f64 = 10_000_000.0;
/// The ticks from 1601-01-01, the start of Windows dates, to the Unix epoch.
const UNIX_EPOCH_TICKS: i64 = 116_444_736_000_000_000;

static PROGRESS: Mutex<Progress> = Mutex::new(Progress::new());

pub fn read() -> Result<Option<MediaInfo>, String> {
    read_current().map_err(|error| error.to_string())
}

pub fn control(action: Action) -> Result<bool, String> {
    let request = || {
        let Some(session) = current()? else { return Ok(false) };
        match action {
            Action::Play => session.TryPlayAsync()?,
            Action::Pause => session.TryPauseAsync()?,
            Action::Next => session.TrySkipNextAsync()?,
            Action::Previous => session.TrySkipPreviousAsync()?,
        }
        .join()
    };
    request().map_err(|error: windows::core::Error| error.to_string())
}

fn current() -> windows::core::Result<Option<Session>> {
    let manager = Manager::RequestAsync()?.join()?;
    Ok(manager.GetCurrentSession().ok())
}

fn read_current() -> windows::core::Result<Option<MediaInfo>> {
    let Some(session) = current()? else { return Ok(None) };
    let properties = session.TryGetMediaPropertiesAsync()?.join()?;
    let playback = session.GetPlaybackInfo()?;
    let controls = playback.Controls()?;
    let playing = playback.PlaybackStatus()? == Status::Playing;
    // Apps set this only when they play faster or slower than normal.
    let rate = playback.PlaybackRate().and_then(|rate| rate.Value()).unwrap_or(1.0);
    let reported = timeline(&session.GetTimelineProperties()?, playing, rate)?;

    let app = session.SourceAppUserModelId()?.to_string();
    let title = properties.Title()?.to_string();
    let artist = properties.Artist()?.to_string();
    let album = properties.AlbumTitle()?.to_string();
    let track = format!("{app}\n{title}\n{artist}\n{album}");
    let (position, duration) = PROGRESS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .update(Instant::now(), &track, reported, playing, rate)
        .unzip();
    let art = art::id(track, || thumbnail(&properties));
    Ok(Some(MediaInfo {
        app,
        title,
        artist,
        album,
        playing,
        position,
        duration,
        art,
        can: Controls {
            play: controls.IsPlayEnabled()?,
            pause: controls.IsPauseEnabled()?,
            next: controls.IsNextEnabled()?,
            previous: controls.IsPreviousEnabled()?,
        },
    }))
}

/// The position and the length in seconds, as the app reports them. `None` if the timeline is
/// empty. Apps report the position at their last update, and Spotify updates only on play, pause
/// and seek. Thus while the track plays, the position moves on by the time since than.
fn timeline(
    timeline: &Timeline,
    playing: bool,
    rate: f64,
) -> windows::core::Result<Option<(f64, f64)>> {
    let start = timeline.StartTime()?.Duration;
    let length = timeline.EndTime()?.Duration - start;
    if length <= 0 {
        return Ok(None);
    }
    let mut position = (timeline.Position()?.Duration - start) as f64;
    let updated = timeline.LastUpdatedTime()?.UniversalTime;
    if playing && updated > 0 {
        position += (now_ticks() - updated).max(0) as f64 * rate;
    }
    let length = length as f64;
    Ok(Some((position.clamp(0.0, length) / TICKS_PER_SECOND, length / TICKS_PER_SECOND)))
}

fn now_ticks() -> i64 {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    UNIX_EPOCH_TICKS + (since_epoch.as_nanos() / 100) as i64
}

/// The art as a data URL. `None` if the app gives no art or the host cannot read it.
fn thumbnail(properties: &Properties) -> Option<String> {
    let stream = properties.Thumbnail().ok()?.OpenReadAsync().ok()?.join().ok()?;
    let size = u32::try_from(stream.Size().ok()?).ok().filter(|&size| size > 0)?;
    let reader = DataReader::CreateDataReader(&stream).ok()?;
    reader.LoadAsync(size).ok()?.join().ok()?;
    let mut bytes = vec![0; size as usize];
    reader.ReadBytes(&mut bytes).ok()?;
    // Image decoders look at the bytes, so the art of an app that names no type still shows.
    let mime = stream.ContentType().map(|mime| mime.to_string()).unwrap_or_default();
    let mime = if mime.is_empty() { "image/png".into() } else { mime };
    Some(format!("data:{mime};base64,{}", STANDARD.encode(bytes)))
}
