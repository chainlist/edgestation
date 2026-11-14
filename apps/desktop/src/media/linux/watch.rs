//! The changes that the Edge listens to, while a widget asks for them. The players tell the
//! session bus when their track, playback state or position changes, and the bus tells when a
//! player starts or quits. The Edge window then gets `media:changed`, and widgets read the
//! players again.

use std::sync::{Mutex, PoisonError};

use futures_util::stream::{Stream, StreamExt, select};
use tauri::AppHandle;
use tauri::async_runtime::JoinHandle;
use zbus::message::Type;
use zbus::{MatchRule, Message, MessageStream};

use super::{PATH, PLAYER, ROOT};
use crate::media::changed;
use crate::os::linux::dbus::{self, Bus};

/// The task that listens, while one runs.
static WATCH: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);

/// Starts a new watch, or stops it. A start replaces the watch that runs.
pub fn watch(app: &AppHandle, on: bool) -> Result<(), String> {
    let mut watch = WATCH.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(old) = watch.take() {
        old.abort();
    }
    if !on {
        return Ok(());
    }
    let bus = dbus::with(Bus::Session, |bus| Ok(bus.inner().clone()));
    let bus = bus.map_err(|error| error.to_string())?;
    // Subscribe before the command returns, so the widget gets the error.
    let changes = tauri::async_runtime::block_on(subscribe(bus));
    let mut changes = Box::pin(changes.map_err(|error| error.to_string())?);
    let app = app.clone();
    *watch = Some(tauri::async_runtime::spawn(async move {
        while changes.next().await.is_some() {
            changed(&app);
        }
    }));
    Ok(())
}

pub fn unwatch() {
    if let Some(old) = WATCH.lock().unwrap_or_else(PoisonError::into_inner).take() {
        old.abort();
    }
}

/// The signals of the players. Each player sends `PropertiesChanged` for its `Player` interface,
/// and `Seeked` when the position jumps. MPRIS sends no change of `Position` while a track plays,
/// so a seek has its own signal. The bus sends `NameOwnerChanged` when a player name comes or
/// goes. The names of the players are in the namespace of the root interface, such as
/// `org.mpris.MediaPlayer2.spotify`.
///
/// It takes the connection, not a reference. In Rust 2024 the stream type then holds no borrow,
/// so the task can keep it.
async fn subscribe(
    bus: zbus::Connection,
) -> zbus::Result<impl Stream<Item = zbus::Result<Message>> + Send> {
    let properties = MatchRule::builder()
        .msg_type(Type::Signal)
        .interface("org.freedesktop.DBus.Properties")?
        .member("PropertiesChanged")?
        .path(PATH)?
        .arg(0, PLAYER)?
        .build();
    let seeks = MatchRule::builder()
        .msg_type(Type::Signal)
        .interface(PLAYER)?
        .member("Seeked")?
        .path(PATH)?
        .build();
    let players = MatchRule::builder()
        .msg_type(Type::Signal)
        .sender("org.freedesktop.DBus")?
        .interface("org.freedesktop.DBus")?
        .member("NameOwnerChanged")?
        .arg0ns(ROOT)?
        .build();
    let properties = MessageStream::for_match_rule(properties, &bus, None).await?;
    let seeks = MessageStream::for_match_rule(seeks, &bus, None).await?;
    let players = MessageStream::for_match_rule(players, &bus, None).await?;
    Ok(select(select(properties, seeks), players))
}
