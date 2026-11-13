//! The Windows side: the global media transport controls, the sessions that apps publish for their
//! media keys.

use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as WinSession,
    GlobalSystemMediaTransportControlsSessionManager as Manager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as Properties,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as Status,
    GlobalSystemMediaTransportControlsSessionTimelineProperties as Timeline,
};
use windows::Storage::Streams::DataReader;
use windows::core::HSTRING;

use super::{Action, Controls, MediaInfo, Session};

/// Windows times are in ticks of 100 ns.
const TICKS_PER_SECOND: f64 = 10_000_000.0;
/// The ticks from 1601-01-01, the start of Windows dates, to the Unix epoch.
const UNIX_EPOCH_TICKS: i64 = 116_444_736_000_000_000;

/// A session of the list, with its id and its app id.
struct Listed {
    id: String,
    app: String,
    session: WinSession,
}

pub fn read() -> Result<Option<MediaInfo>, String> {
    let request = || {
        let manager = manager()?;
        let mut listed = list(&manager)?;
        let Some(index) = current(&manager, &listed)? else { return Ok(None) };
        info(listed.swap_remove(index), true).map(Some)
    };
    request().map_err(|error: windows::core::Error| error.to_string())
}

pub fn read_all() -> Result<Vec<MediaInfo>, String> {
    let request = || {
        let manager = manager()?;
        let listed = list(&manager)?;
        let current = current(&manager, &listed)?;
        // A session can close during the read. Then it is not in the list.
        Ok(listed
            .into_iter()
            .enumerate()
            .filter_map(|(index, each)| info(each, Some(index) == current).ok())
            .collect())
    };
    request().map_err(|error: windows::core::Error| error.to_string())
}

pub fn control(action: Action, id: Option<&str>) -> Result<bool, String> {
    let request = || {
        let manager = manager()?;
        let session = match id {
            None => manager.GetCurrentSession().ok(),
            Some(id) => {
                list(&manager)?.into_iter().find(|each| each.id == id).map(|each| each.session)
            }
        };
        let Some(session) = session else { return Ok(false) };
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

fn manager() -> windows::core::Result<Manager> {
    Manager::RequestAsync()?.join()
}

/// All sessions with their ids, in the order of Windows.
fn list(manager: &Manager) -> windows::core::Result<Vec<Listed>> {
    let sessions: Vec<WinSession> = manager.GetSessions()?.into_iter().collect();
    let apps = sessions
        .iter()
        .map(|session| Ok(session.SourceAppUserModelId()?.to_string()))
        .collect::<windows::core::Result<Vec<_>>>()?;
    let ids = ids(&apps);
    Ok(ids
        .into_iter()
        .zip(apps)
        .zip(sessions)
        .map(|((id, app), session)| Listed { id, app, session })
        .collect())
}

/// The id of each session: its app id. All WebView2 apps have the same app id
/// (`msedgewebview2.exe`), so the second session of an app gets `#2` after it, the third `#3`.
fn ids(apps: &[String]) -> Vec<String> {
    let mut ids = Vec::with_capacity(apps.len());
    for (index, app) in apps.iter().enumerate() {
        let before = apps[..index].iter().filter(|other| *other == app).count();
        ids.push(if before == 0 { app.clone() } else { format!("{app}#{}", before + 1) });
    }
    ids
}

/// The index of the current session in `listed`. Windows gives a new object for the current
/// session, wich is not equal to the one in the list. Thus the host compares the app ids, and for
/// more sessions of one app, the track and the playback status too.
fn current(manager: &Manager, listed: &[Listed]) -> windows::core::Result<Option<usize>> {
    let Ok(session) = manager.GetCurrentSession() else { return Ok(None) };
    let app = session.SourceAppUserModelId()?.to_string();
    let same_app: Vec<usize> =
        (0..listed.len()).filter(|&index| listed[index].app == app).collect();
    if same_app.len() < 2 {
        return Ok(same_app.first().copied());
    }
    let state = track_state(&session)?;
    let same_state = same_app
        .iter()
        .copied()
        .find(|&index| track_state(&listed[index].session).is_ok_and(|other| other == state));
    Ok(same_state.or(same_app.first().copied()))
}

/// The title, the artist and the playback status of a session.
fn track_state(session: &WinSession) -> windows::core::Result<(HSTRING, HSTRING, Status)> {
    let properties = session.TryGetMediaPropertiesAsync()?.join()?;
    Ok((properties.Title()?, properties.Artist()?, session.GetPlaybackInfo()?.PlaybackStatus()?))
}

fn info(listed: Listed, current: bool) -> windows::core::Result<MediaInfo> {
    let Listed { id, app, session } = listed;
    let properties = session.TryGetMediaPropertiesAsync()?.join()?;
    let playback = session.GetPlaybackInfo()?;
    let controls = playback.Controls()?;
    let playing = playback.PlaybackStatus()? == Status::Playing;
    // Apps set this only when they play faster or slower than normal.
    let rate = playback.PlaybackRate().and_then(|rate| rate.Value()).unwrap_or(1.0);
    let found = Session {
        id,
        current,
        app,
        title: properties.Title()?.to_string(),
        artist: properties.Artist()?.to_string(),
        album: properties.AlbumTitle()?.to_string(),
        playing,
        rate,
        timeline: timeline(&session.GetTimelineProperties()?, playing, rate)?,
        can: Controls {
            play: controls.IsPlayEnabled()?,
            pause: controls.IsPauseEnabled()?,
            next: controls.IsNextEnabled()?,
            previous: controls.IsPreviousEnabled()?,
        },
    };
    Ok(found.info(|| thumbnail(&properties)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_the_sessions_of_one_app() {
        let apps =
            ["msedgewebview2.exe", "Spotify.exe", "msedgewebview2.exe", "msedgewebview2.exe"];
        let apps: Vec<String> = apps.into_iter().map(String::from).collect();
        assert_eq!(
            ids(&apps),
            ["msedgewebview2.exe", "Spotify.exe", "msedgewebview2.exe#2", "msedgewebview2.exe#3"]
        );
    }

    /// Reads the real sessions. Run it with `--ignored --nocapture` while something plays.
    #[test]
    #[ignore]
    fn reads_the_sessions() {
        for info in read_all().unwrap() {
            println!("{}", serde_json::to_string(&info).unwrap());
        }
        println!("current: {}", serde_json::to_string(&read().unwrap()).unwrap());
    }
}
