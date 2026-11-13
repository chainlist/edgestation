//! The Linux side: the MPRIS players on the session bus, the media players, browsers and other apps that the
//! media keys and the media controls of Linux desktops control.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tauri::Url;
use tauri_plugin_http::reqwest::Client;
use zbus::blocking::Connection;
use zbus::zvariant::{OwnedValue, Value};

use super::progress::Progress;
use super::{Action, Controls, MediaInfo, art};
use crate::os::linux::dbus::{self, Bus};

const PREFIX: &str = "org.mpris.MediaPlayer2.";
/// The playerctl daemon. It follows the player that the user used last.
const PLAYERCTLD: &str = "org.mpris.MediaPlayer2.playerctld";
const PATH: &str = "/org/mpris/MediaPlayer2";
const ROOT: &str = "org.mpris.MediaPlayer2";
const PLAYER: &str = "org.mpris.MediaPlayer2.Player";
/// MPRIS times are in microseconds.
const MICROSECONDS: f64 = 1_000_000.0;
/// The largest art that the host loads, in bytes.
const MAX_ART: usize = 10 << 20;

static PROGRESS: Mutex<Progress> = Mutex::new(Progress::new());

type Properties = HashMap<String, OwnedValue>;

/// What a player publishes on its `Player` interface.
#[derive(Debug, Default, PartialEq)]
struct Player {
    /// `Playing`, `Paused` or `Stopped`.
    status: String,
    rate: f64,
    /// In microseconds.
    position: Option<f64>,
    length: Option<f64>,
    title: String,
    artist: String,
    album: String,
    art_url: Option<String>,
    can_play: bool,
    can_pause: bool,
    can_next: bool,
    can_previous: bool,
}

pub fn read() -> Result<Option<MediaInfo>, String> {
    let found = dbus::with(Bus::Session, |bus| {
        let Some((name, properties)) = current(bus)? else {
            return Ok(None);
        };
        let root = dbus::properties(bus, &name, PATH, ROOT).unwrap_or_default();
        Ok(Some((app_name(&name, &root), parse(&properties))))
    });
    let Some((app, player)) = found.map_err(|error| error.to_string())? else {
        return Ok(None);
    };
    let playing = player.status == "Playing";
    let track = format!("{app}\n{}\n{}\n{}", player.title, player.artist, player.album);
    let (position, duration) = PROGRESS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .update(Instant::now(), &track, timeline(&player), playing, player.rate)
        .unzip();
    let art = art::id(track, || player.art_url.as_deref().and_then(load_art));
    Ok(Some(MediaInfo {
        app,
        title: player.title,
        artist: player.artist,
        album: player.album,
        playing,
        position,
        duration,
        art,
        can: Controls {
            play: player.can_play,
            pause: player.can_pause,
            next: player.can_next,
            previous: player.can_previous,
        },
    }))
}

pub fn control(action: Action) -> Result<bool, String> {
    let request = |bus: &Connection| {
        let Some((name, properties)) = current(bus)? else {
            return Ok(false);
        };
        let player = parse(&properties);
        let (allowed, method) = match action {
            Action::Play => (player.can_play, "Play"),
            Action::Pause => (player.can_pause, "Pause"),
            Action::Next => (player.can_next, "Next"),
            Action::Previous => (player.can_previous, "Previous"),
        };
        if allowed {
            bus.call_method(Some(name.as_str()), PATH, Some(PLAYER), method, &())?;
        }
        Ok(allowed)
    };
    dbus::with(Bus::Session, request).map_err(|error| error.to_string())
}

/// The current player and its properties: the first player that plays, else the player that
/// playerctld follows, else the first paused player. A stopped player does not count.
fn current(bus: &Connection) -> zbus::Result<Option<(String, Properties)>> {
    let names: Vec<String> = dbus::call(
        bus,
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        "org.freedesktop.DBus",
        "ListNames",
        &(),
    )?;
    let mut players: Vec<&String> =
        names.iter().filter(|name| name.starts_with(PREFIX) && *name != PLAYERCTLD).collect();
    players.sort();
    let mut paused = None;
    for name in players {
        // A player that does not answer does not count.
        let Ok(properties) = dbus::properties(bus, name, PATH, PLAYER) else {
            continue;
        };
        match text(get(&properties, "PlaybackStatus")).as_deref() {
            Some("Playing") => return Ok(Some((name.clone(), properties))),
            Some("Paused") if paused.is_none() => paused = Some((name.clone(), properties)),
            _ => {}
        }
    }
    if names.iter().any(|name| name == PLAYERCTLD)
        && let Ok(properties) = dbus::properties(bus, PLAYERCTLD, PATH, PLAYER)
        && text(get(&properties, "PlaybackStatus")).as_deref() == Some("Paused")
    {
        return Ok(Some((PLAYERCTLD.to_owned(), properties)));
    }
    Ok(paused)
}

/// The name of the app of player `bus_name`: its desktop entry, for example `spotify`. A player
/// without one gets its bus name, without the instance number that browsers add.
fn app_name(bus_name: &str, root: &Properties) -> String {
    let entry = text(get(root, "DesktopEntry")).filter(|entry| !entry.is_empty());
    entry.unwrap_or_else(|| {
        let name = bus_name.strip_prefix(PREFIX).unwrap_or(bus_name);
        name.split(".instance").next().unwrap_or(name).to_owned()
    })
}

fn parse(properties: &Properties) -> Player {
    let property = |name: &str| get(properties, name);
    let metadata: HashMap<String, &Value> = match property("Metadata").map(inner) {
        Some(Value::Dict(dict)) => {
            dict.iter().filter_map(|(key, value)| Some((text(Some(key))?, value))).collect()
        }
        _ => HashMap::new(),
    };
    let meta = |key: &str| metadata.get(key).copied();
    // A player that cannot be controlled has no actions.
    let control = !matches!(property("CanControl").map(inner), Some(Value::Bool(false)));
    let can = |name: &str| control && matches!(property(name).map(inner), Some(Value::Bool(true)));
    Player {
        status: text(property("PlaybackStatus")).unwrap_or_default(),
        rate: number(property("Rate")).filter(|&rate| rate > 0.0).unwrap_or(1.0),
        position: number(property("Position")),
        length: number(meta("mpris:length")),
        title: text(meta("xesam:title")).unwrap_or_default(),
        artist: texts(meta("xesam:artist")).join(", "),
        album: text(meta("xesam:album")).unwrap_or_default(),
        art_url: text(meta("mpris:artUrl")).filter(|url| !url.is_empty()),
        can_play: can("CanPlay"),
        can_pause: can("CanPause"),
        can_next: can("CanGoNext"),
        can_previous: can("CanGoPrevious"),
    }
}

/// The position and the length in seconds, as the player reports them. `None` without a length.
/// The player gives the position at the time of the call, so it needs no correction.
fn timeline(player: &Player) -> Option<(f64, f64)> {
    let length = player.length.filter(|&length| length > 0.0)?;
    let position = player.position?.clamp(0.0, length);
    Some((position / MICROSECONDS, length / MICROSECONDS))
}

fn get<'a>(properties: &'a Properties, name: &str) -> Option<&'a Value<'static>> {
    properties.get(name).map(|value| &**value)
}

/// The value in the variants around `value`. The values of a dictionary of variants come in one.
fn inner<'a, 'v>(value: &'a Value<'v>) -> &'a Value<'v> {
    match value {
        Value::Value(value) => inner(value),
        value => value,
    }
}

fn text(value: Option<&Value>) -> Option<String> {
    match inner(value?) {
        Value::Str(text) => Some(text.to_string()),
        Value::ObjectPath(path) => Some(path.to_string()),
        _ => None,
    }
}

/// A list of texts. Some players give one text where MPRIS asks for a list.
fn texts(value: Option<&Value>) -> Vec<String> {
    match value.map(inner) {
        Some(Value::Array(items)) => items.iter().filter_map(|item| text(Some(item))).collect(),
        value => text(value).into_iter().collect(),
    }
}

/// A number of any type. MPRIS asks for 64-bit integers, but some players give other types.
fn number(value: Option<&Value>) -> Option<f64> {
    Some(match inner(value?) {
        Value::U8(n) => f64::from(*n),
        Value::I16(n) => f64::from(*n),
        Value::U16(n) => f64::from(*n),
        Value::I32(n) => f64::from(*n),
        Value::U32(n) => f64::from(*n),
        Value::I64(n) => *n as f64,
        Value::U64(n) => *n as f64,
        Value::F64(n) => *n,
        _ => return None,
    })
}

/// The art at `url` as a data URL. Players give a file, a web address or a data URL. The host
/// keeps the last art, so it does not load it again while the art settles.
fn load_art(url: &str) -> Option<String> {
    static LOADED: Mutex<Option<(String, String)>> = Mutex::new(None);
    let mut loaded = LOADED.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some((from, data)) = &*loaded
        && from == url
    {
        return Some(data.clone());
    }
    let data = match url.strip_prefix("data:") {
        Some(_) => url.to_owned(),
        None => {
            let bytes = fetch(url)?;
            format!("data:{};base64,{}", image_type(&bytes), STANDARD.encode(bytes))
        }
    };
    *loaded = Some((url.to_owned(), data.clone()));
    Some(data)
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let url = Url::parse(url).ok()?;
    match url.scheme() {
        "file" => {
            let path = url.to_file_path().ok()?;
            let small = std::fs::metadata(&path).ok()?.len() <= MAX_ART as u64;
            small.then(|| std::fs::read(path).ok()).flatten()
        }
        "http" | "https" => download(url),
        _ => None,
    }
}

/// Downloads the art at `url`. Spotify and browsers give the art on the web.
fn download(url: Url) -> Option<Vec<u8>> {
    static CLIENT: OnceLock<Option<Client>> = OnceLock::new();
    let client =
        CLIENT.get_or_init(|| Client::builder().timeout(Duration::from_secs(10)).build().ok());
    let client = client.as_ref()?;
    tauri::async_runtime::block_on(async {
        let mut response = client.get(url).send().await.ok()?.error_for_status().ok()?;
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.ok()? {
            bytes.extend_from_slice(&chunk);
            if bytes.len() > MAX_ART {
                return None;
            }
        }
        Some(bytes)
    })
}

/// The type of an image, from its first bytes. Image decoders look at the bytes too, so an image
/// of an other type still shows.
fn image_type(bytes: &[u8]) -> &'static str {
    match bytes {
        [0xFF, 0xD8, 0xFF, ..] => "image/jpeg",
        [b'G', b'I', b'F', b'8', ..] => "image/gif",
        [b'R', b'I', b'F', b'F', _, _, _, _, b'W', b'E', b'B', b'P', ..] => "image/webp",
        _ if bytes.starts_with(b"<svg") || bytes.starts_with(b"<?xml") => "image/svg+xml",
        _ => "image/png",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(value: Value) -> OwnedValue {
        value.try_to_owned().unwrap()
    }

    fn spotify() -> Properties {
        let metadata = HashMap::from([
            ("mpris:trackid", Value::from("/com/spotify/track/1")),
            ("mpris:length", Value::from(215_000_000_u64)),
            ("mpris:artUrl", Value::from("https://i.scdn.co/image/ab67")),
            ("xesam:title", Value::from("Song")),
            ("xesam:artist", Value::from(vec!["One", "Two"])),
            ("xesam:album", Value::from("Album")),
        ]);
        HashMap::from([
            ("PlaybackStatus".to_owned(), owned(Value::from("Playing"))),
            ("Rate".to_owned(), owned(Value::from(1.0))),
            ("Position".to_owned(), owned(Value::from(30_500_000_i64))),
            ("Metadata".to_owned(), owned(Value::from(metadata))),
            ("CanPlay".to_owned(), owned(Value::from(true))),
            ("CanPause".to_owned(), owned(Value::from(true))),
            ("CanGoNext".to_owned(), owned(Value::from(true))),
            ("CanGoPrevious".to_owned(), owned(Value::from(false))),
            ("CanControl".to_owned(), owned(Value::from(true))),
        ])
    }

    #[test]
    fn reads_the_player_and_its_metadata() {
        let player = parse(&spotify());
        assert_eq!(
            player,
            Player {
                status: "Playing".into(),
                rate: 1.0,
                position: Some(30_500_000.0),
                length: Some(215_000_000.0),
                title: "Song".into(),
                artist: "One, Two".into(),
                album: "Album".into(),
                art_url: Some("https://i.scdn.co/image/ab67".into()),
                can_play: true,
                can_pause: true,
                can_next: true,
                can_previous: false,
            }
        );
        assert_eq!(timeline(&player), Some((30.5, 215.0)));
    }

    #[test]
    fn a_player_that_cannot_be_controlled_has_no_actions() {
        let mut properties = spotify();
        properties.insert("CanControl".into(), owned(Value::from(false)));
        let player = parse(&properties);
        assert!(!player.can_play && !player.can_pause && !player.can_next);
    }

    #[test]
    fn takes_one_artist_and_other_number_types() {
        let metadata = HashMap::from([
            ("xesam:artist", Value::from("Solo")),
            ("mpris:length", Value::from(5_000_000_i32)),
        ]);
        let properties = HashMap::from([("Metadata".to_owned(), owned(Value::from(metadata)))]);
        let player = parse(&properties);
        assert_eq!(player.artist, "Solo");
        assert_eq!(player.length, Some(5_000_000.0));
        // Without a position, there is no timeline. `Progress` carries the last one.
        assert_eq!(timeline(&player), None);
        assert_eq!(player.rate, 1.0);
    }

    #[test]
    fn names_the_app_by_its_desktop_entry() {
        let root = HashMap::from([("DesktopEntry".to_owned(), owned(Value::from("firefox")))]);
        assert_eq!(app_name("org.mpris.MediaPlayer2.firefox.instance_1_42", &root), "firefox");
        let none = HashMap::new();
        assert_eq!(app_name("org.mpris.MediaPlayer2.chromium.instance5104", &none), "chromium");
        assert_eq!(app_name("org.mpris.MediaPlayer2.mpv", &none), "mpv");
    }

    #[test]
    fn tells_image_types_apart() {
        assert_eq!(image_type(b"\x89PNG\r\n"), "image/png");
        assert_eq!(image_type(b"\xFF\xD8\xFF\xE0"), "image/jpeg");
        assert_eq!(image_type(b"RIFF1234WEBPVP8 "), "image/webp");
        assert_eq!(image_type(b"<svg xmlns"), "image/svg+xml");
        assert_eq!(image_type(b""), "image/png");
    }

    /// Reads the real player. Run it with `--ignored --nocapture` while something plays.
    #[test]
    #[ignore]
    fn reads_the_current_player() {
        let info = read().unwrap().expect("no player");
        println!("{}", serde_json::to_string_pretty(&info).unwrap());
        println!("art: {:?}", art::current().map(|url| url[..40.min(url.len())].to_owned()));
    }
}
