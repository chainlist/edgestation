use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{
    EDataFlow, ERole, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator, eCapture,
    eCommunications, eConsole,
};
use windows::Win32::System::Com::StructuredStorage::{PropVariantClear, PropVariantToStringAlloc};
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance, STGM_READ};
use windows::core::Result;

use crate::audio::Device;
use crate::os::windows::com::take;

pub(crate) unsafe fn enumerator() -> Result<IMMDeviceEnumerator> {
    unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }
}

pub(crate) unsafe fn default(
    devices: &IMMDeviceEnumerator,
    flow: EDataFlow,
    role: ERole,
) -> Option<IMMDevice> {
    unsafe { devices.GetDefaultAudioEndpoint(flow, role).ok() }
}

/// The default microphone, then the default microphone for calls if it is a different device.
pub(crate) unsafe fn inputs(devices: &IMMDeviceEnumerator) -> Result<Vec<IMMDevice>> {
    unsafe {
        let mut inputs: Vec<IMMDevice> = Vec::new();
        let mut ids = Vec::new();
        for role in [eConsole, eCommunications] {
            if let Some(device) = default(devices, eCapture, role) {
                let id = id(&device)?;
                if !ids.contains(&id) {
                    ids.push(id);
                    inputs.push(device);
                }
            }
        }
        Ok(inputs)
    }
}

pub(super) unsafe fn device(device: &IMMDevice) -> Result<Device> {
    unsafe {
        let volume = volume(device)?;
        Ok(Device {
            id: id(device)?,
            name: name(device)?,
            volume: volume.GetMasterVolumeLevelScalar()?,
            muted: volume.GetMute()?.as_bool(),
        })
    }
}

pub(crate) unsafe fn volume(device: &IMMDevice) -> Result<IAudioEndpointVolume> {
    unsafe { device.Activate(CLSCTX_ALL, None) }
}

pub(super) unsafe fn id(device: &IMMDevice) -> Result<String> {
    unsafe { Ok(take(device.GetId()?)?.unwrap_or_default()) }
}

pub(super) unsafe fn name(device: &IMMDevice) -> Result<String> {
    unsafe {
        let properties = device.OpenPropertyStore(STGM_READ)?;
        let mut value = properties.GetValue(&PKEY_Device_FriendlyName)?;
        let name = PropVariantToStringAlloc(&value).and_then(|name| take(name));
        PropVariantClear(&mut value)?;
        Ok(name?.unwrap_or_default())
    }
}

