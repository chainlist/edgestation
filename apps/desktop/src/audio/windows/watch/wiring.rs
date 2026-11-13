use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

use tauri::AppHandle;
use windows::Win32::Media::Audio::Endpoints::{IAudioEndpointVolume, IAudioEndpointVolumeCallback};
use windows::Win32::Media::Audio::{
    AudioSessionStateExpired, IAudioSessionControl, IAudioSessionEvents, IAudioSessionManager2,
    IAudioSessionNotification, IMMDeviceEnumerator, IMMNotificationClient, eConsole, eRender,
};
use windows::Win32::System::Com::CLSCTX_ALL;
use windows::core::{IUnknown, Interface, Result};

use super::changes::Events;
use crate::audio::native::{default, enumerator, inputs, volume, with_com};
use crate::os::windows::com::keep_mta;

pub(super) static WATCH: Mutex<Option<Watch>> = Mutex::new(None);
/// A callback sets this when the devices or sessions to listen to change. The next mixer read
/// listens to the new ones, becuase callbacks cannot call Core Audio.
pub(super) static REWIRE: AtomicBool = AtomicBool::new(false);

pub fn watch(app: &AppHandle, on: bool) -> std::result::Result<(), String> {
    keep_mta();
    with_com(|| unsafe {
        let mut watch = WATCH.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(old) = watch.take() {
            old.stop();
        }
        if on {
            *watch = Some(Watch::start(app.clone())?);
        }
        Ok(())
    })
}

/// If the devices or sessions changed, moves the callbacks to the current ones. The mixer read
/// calls this with COM set up.
pub fn rewire() {
    if !REWIRE.swap(false, Ordering::AcqRel) {
        return;
    }
    let mut watch = WATCH.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(watch) = watch.as_mut() {
        unsafe {
            watch.unwire();
            if let Err(error) = watch.wire() {
                eprintln!("failed to listen to audio changes: {error}");
            }
        }
    }
}

pub(super) struct Watch {
    devices: IMMDeviceEnumerator,
    client: IMMNotificationClient,
    volume: IAudioEndpointVolumeCallback,
    sessions: IAudioSessionNotification,
    session: IAudioSessionEvents,
    wired: Vec<Wired>,
}

// Only threads of the multithreaded apartment keep this value, and `keep_mta` keeps that
// apartment alive. Its objects work from any of its threads.
unsafe impl Send for Watch {}

enum Wired {
    Volume(IAudioEndpointVolume),
    Sessions(IAudioSessionManager2),
    Session(IAudioSessionControl),
}

impl Watch {
    unsafe fn start(app: AppHandle) -> Result<Self> {
        unsafe {
            let events: IUnknown = Events { app }.into();
            let mut watch = Self {
                devices: enumerator()?,
                client: events.cast()?,
                volume: events.cast()?,
                sessions: events.cast()?,
                session: events.cast()?,
                wired: Vec::new(),
            };
            watch.devices.RegisterEndpointNotificationCallback(&watch.client)?;
            if let Err(error) = watch.wire() {
                watch.stop();
                return Err(error);
            }
            Ok(watch)
        }
    }

    /// Registers the callbacks on the default devices and on the sessions of the output.
    unsafe fn wire(&mut self) -> Result<()> {
        unsafe {
            if let Some(output) = default(&self.devices, eRender, eConsole) {
                let endpoint = volume(&output)?;
                endpoint.RegisterControlChangeNotify(&self.volume)?;
                self.wired.push(Wired::Volume(endpoint));
                let manager: IAudioSessionManager2 = output.Activate(CLSCTX_ALL, None)?;
                manager.RegisterSessionNotification(&self.sessions)?;
                self.wired.push(Wired::Sessions(manager.clone()));
                // Windows reports new sessions only after a call that lists the sessions, as here.
                let all = manager.GetSessionEnumerator()?;
                for i in 0..all.GetCount()? {
                    let session = all.GetSession(i)?;
                    if session.GetState()? != AudioSessionStateExpired {
                        session.RegisterAudioSessionNotification(&self.session)?;
                        self.wired.push(Wired::Session(session));
                    }
                }
            }
            for input in inputs(&self.devices)? {
                let endpoint = volume(&input)?;
                endpoint.RegisterControlChangeNotify(&self.volume)?;
                self.wired.push(Wired::Volume(endpoint));
            }
            Ok(())
        }
    }

    unsafe fn unwire(&mut self) {
        unsafe {
            for wired in self.wired.drain(..) {
                let _ = match wired {
                    Wired::Volume(endpoint) => endpoint.UnregisterControlChangeNotify(&self.volume),
                    Wired::Sessions(manager) => manager.UnregisterSessionNotification(&self.sessions),
                    Wired::Session(session) => session.UnregisterAudioSessionNotification(&self.session),
                };
            }
        }
    }

    pub(super) unsafe fn stop(mut self) {
        unsafe {
            self.unwire();
            let _ = self.devices.UnregisterEndpointNotificationCallback(&self.client);
        }
    }
}
