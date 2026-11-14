//! The changes that the Edge listens to, while a widget asks for them. Windows tells the app
//! when a session opens, closes or becomes current, and when the track, the playback state or
//! the timeline of a session changes. The Edge window then gets `media:changed`, and widgets read
//! the sessions again.

use std::sync::{Mutex, PoisonError};

use tauri::AppHandle;
use windows::Foundation::TypedEventHandler;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as WinSession,
    GlobalSystemMediaTransportControlsSessionManager as Manager,
};
use windows::core::{Result, RuntimeType};

use super::manager;
use crate::media::changed;
use crate::os::windows::com::keep_mta;

static WATCH: Mutex<Option<Watch>> = Mutex::new(None);

struct Watch {
    manager: Manager,
    /// The tokens of `SessionsChanged` and `CurrentSessionChanged`.
    tokens: [i64; 2],
    /// Each open session with the tokens of its three change events, `None` for one that failed.
    sessions: Vec<(WinSession, [Option<i64>; 3])>,
}

/// Starts a new watch, or stops it. A start replaces the watch that runs.
pub fn watch(app: &AppHandle, on: bool) -> std::result::Result<(), String> {
    keep_mta();
    let mut watch = WATCH.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(old) = watch.take() {
        old.stop();
    }
    if on {
        *watch = Some(Watch::start(app).map_err(|error| error.to_string())?);
    }
    Ok(())
}

pub fn unwatch() {
    if let Some(old) = WATCH.lock().unwrap_or_else(PoisonError::into_inner).take() {
        old.stop();
    }
}

/// A handler that tells the Edge window about the change. Windows sends some changes several
/// times, for example a new track, and the throttle makes them one or two events.
fn notify<S: RuntimeType + 'static, A: RuntimeType + 'static>(
    app: &AppHandle,
) -> TypedEventHandler<S, A> {
    let app = app.clone();
    TypedEventHandler::new(move |_, _| {
        changed(&app);
        Ok(())
    })
}

/// Listens to the sessions that are open now, then tells about the change. It runs on a thread
/// of its own, so the handler returns to Windows before the host reads the new list.
fn rewire(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        if let Some(watch) = WATCH.lock().unwrap_or_else(PoisonError::into_inner).as_mut() {
            watch.unwire();
            if let Err(error) = watch.wire(&app) {
                eprintln!("failed to listen to media sessions: {error}");
            }
        }
        changed(&app);
    });
}

impl Watch {
    fn start(app: &AppHandle) -> Result<Self> {
        let manager = manager()?;
        let rewired = {
            let app = app.clone();
            TypedEventHandler::new(move |_, _| {
                rewire(&app);
                Ok(())
            })
        };
        let sessions = manager.SessionsChanged(&rewired)?;
        let current = match manager.CurrentSessionChanged(&notify(app)) {
            Ok(token) => token,
            Err(error) => {
                let _ = manager.RemoveSessionsChanged(sessions);
                return Err(error);
            }
        };
        let mut watch = Self { manager, tokens: [sessions, current], sessions: Vec::new() };
        if let Err(error) = watch.wire(app) {
            watch.stop();
            return Err(error);
        }
        Ok(watch)
    }

    /// Listens to the changes of each open session. A session can close during the calls, and
    /// then Windows sends `SessionsChanged`.
    fn wire(&mut self, app: &AppHandle) -> Result<()> {
        for session in self.manager.GetSessions()? {
            let tokens = [
                session.MediaPropertiesChanged(&notify(app)).ok(),
                session.PlaybackInfoChanged(&notify(app)).ok(),
                session.TimelinePropertiesChanged(&notify(app)).ok(),
            ];
            self.sessions.push((session, tokens));
        }
        Ok(())
    }

    fn unwire(&mut self) {
        for (session, [properties, playback, timeline]) in self.sessions.drain(..) {
            if let Some(token) = properties {
                let _ = session.RemoveMediaPropertiesChanged(token);
            }
            if let Some(token) = playback {
                let _ = session.RemovePlaybackInfoChanged(token);
            }
            if let Some(token) = timeline {
                let _ = session.RemoveTimelinePropertiesChanged(token);
            }
        }
    }

    fn stop(mut self) {
        self.unwire();
        let [sessions, current] = self.tokens;
        let _ = self.manager.RemoveSessionsChanged(sessions);
        let _ = self.manager.RemoveCurrentSessionChanged(current);
    }
}
