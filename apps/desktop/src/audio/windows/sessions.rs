use windows::Win32::Media::Audio::{
    AudioSessionStateExpired, IAudioSessionControl2, IAudioSessionManager2, IMMDevice,
    IMMDeviceEnumerator, eConsole, eRender,
};
use windows::Win32::System::Com::CLSCTX_ALL;
use windows::core::{GUID, Interface, Result};

use super::devices::default;

/// All sessions on `device` that have not ended, each with the id of its group. The Windows
/// volume mixer shows each group as one item, usually one app.
pub(super) unsafe fn sessions(device: &IMMDevice) -> Result<Vec<(String, IAudioSessionControl2)>> {
    unsafe {
        let manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)?;
        let all = manager.GetSessionEnumerator()?;
        let mut sessions = Vec::new();
        for i in 0..all.GetCount()? {
            let session: IAudioSessionControl2 = all.GetSession(i)?.cast()?;
            if session.GetState()? == AudioSessionStateExpired {
                continue;
            }
            let group = session.GetGroupingParam()?;
            // Each session without a group gets the id of its process.
            let id = if group == GUID::zeroed() {
                format!("process {}", session.GetProcessId().unwrap_or_default())
            } else {
                format!("{group:?}")
            };
            sessions.push((id, session));
        }
        Ok(sessions)
    }
}

pub(super) unsafe fn app_sessions(
    devices: &IMMDeviceEnumerator,
    id: &str,
) -> Result<Vec<IAudioSessionControl2>> {
    unsafe {
        let Some(output) = default(devices, eRender, eConsole) else { return Ok(Vec::new()) };
        let sessions = sessions(&output)?.into_iter();
        Ok(sessions.filter(|(group, _)| group == id).map(|(_, session)| session).collect())
    }
}
