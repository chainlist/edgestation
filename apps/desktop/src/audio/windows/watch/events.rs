use std::sync::atomic::Ordering;

use tauri::AppHandle;
use windows::Win32::Foundation::PROPERTYKEY;
use windows::Win32::Media::Audio::Endpoints::{
    IAudioEndpointVolumeCallback, IAudioEndpointVolumeCallback_Impl,
};
use windows::Win32::Media::Audio::{
    AUDIO_VOLUME_NOTIFICATION_DATA, AudioSessionDisconnectReason, AudioSessionState, DEVICE_STATE,
    EDataFlow, ERole, IAudioSessionControl, IAudioSessionEvents, IAudioSessionEvents_Impl,
    IAudioSessionNotification, IAudioSessionNotification_Impl, IMMNotificationClient,
    IMMNotificationClient_Impl,
};
use windows::core::{BOOL, GUID, PCWSTR, Ref, Result, implement};

use super::wiring::REWIRE;

/// Tells the Edge window that the mixer changed. Set `rewire` if the devices or sessions to
/// listen to also changed.
fn changed(app: &AppHandle, rewire: bool) {
    if rewire {
        REWIRE.store(true, Ordering::Release);
    }
    crate::audio::changed(app);
}

/// One object for all kinds of callbacks.
#[implement(IMMNotificationClient, IAudioEndpointVolumeCallback, IAudioSessionNotification, IAudioSessionEvents)]
pub(super) struct Events {
    pub(super) app: AppHandle,
}

impl IMMNotificationClient_Impl for Events_Impl {
    fn OnDeviceStateChanged(&self, _: &PCWSTR, _: DEVICE_STATE) -> Result<()> {
        changed(&self.app, true);
        Ok(())
    }

    fn OnDeviceAdded(&self, _: &PCWSTR) -> Result<()> {
        changed(&self.app, true);
        Ok(())
    }

    fn OnDeviceRemoved(&self, _: &PCWSTR) -> Result<()> {
        changed(&self.app, true);
        Ok(())
    }

    fn OnDefaultDeviceChanged(&self, _: EDataFlow, _: ERole, _: &PCWSTR) -> Result<()> {
        changed(&self.app, true);
        Ok(())
    }

    /// Device propeties change often, and the mixer shows none of them except the name.
    fn OnPropertyValueChanged(&self, _: &PCWSTR, _: &PROPERTYKEY) -> Result<()> {
        Ok(())
    }
}

impl IAudioEndpointVolumeCallback_Impl for Events_Impl {
    fn OnNotify(&self, _: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> Result<()> {
        changed(&self.app, false);
        Ok(())
    }
}

impl IAudioSessionNotification_Impl for Events_Impl {
    fn OnSessionCreated(&self, _: Ref<IAudioSessionControl>) -> Result<()> {
        changed(&self.app, true);
        Ok(())
    }
}

impl IAudioSessionEvents_Impl for Events_Impl {
    fn OnDisplayNameChanged(&self, _: &PCWSTR, _: *const GUID) -> Result<()> {
        changed(&self.app, false);
        Ok(())
    }

    fn OnIconPathChanged(&self, _: &PCWSTR, _: *const GUID) -> Result<()> {
        Ok(())
    }

    fn OnSimpleVolumeChanged(&self, _: f32, _: BOOL, _: *const GUID) -> Result<()> {
        changed(&self.app, false);
        Ok(())
    }

    fn OnChannelVolumeChanged(&self, _: u32, _: *const f32, _: u32, _: *const GUID) -> Result<()> {
        Ok(())
    }

    fn OnGroupingParamChanged(&self, _: *const GUID, _: *const GUID) -> Result<()> {
        changed(&self.app, false);
        Ok(())
    }

    /// The session started or stopped. The meters read a session only while a stream is open.
    fn OnStateChanged(&self, _: AudioSessionState) -> Result<()> {
        changed(&self.app, false);
        Ok(())
    }

    fn OnSessionDisconnected(&self, _: AudioSessionDisconnectReason) -> Result<()> {
        changed(&self.app, true);
        Ok(())
    }
}
