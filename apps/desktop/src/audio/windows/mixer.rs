use std::ptr::null;
use std::sync::PoisonError;

use windows::Win32::Media::Audio::{
    DEVICE_STATE_ACTIVE, ISimpleAudioVolume, eCapture, eConsole, eRender,
};
use windows::core::Interface;

use super::apps::apps;
use super::default_output;
use super::devices::{default, device, enumerator, id, inputs, name, volume};
use super::meters::METERS;
use super::sessions::app_sessions;
use super::with_com;
use crate::audio::{Mixer, Output, Target};

pub fn mixer() -> std::result::Result<Mixer, String> {
    with_com(|| unsafe {
        // If a widget listens, move the callbacks to the devices and sesions of this read.
        super::watch::rewire();
        let devices = enumerator()?;
        let output = default(&devices, eRender, eConsole);
        let apps = match &output {
            Some(device) => apps(device)?,
            None => Vec::new(),
        };
        let input = match inputs(&devices)?.as_slice() {
            [] => None,
            [first, others @ ..] => {
                let mut input = device(first)?;
                // The input shows as muted only if all its devices are muted.
                for other in others {
                    input.muted &= volume(other)?.GetMute()?.as_bool();
                }
                Some(input)
            }
        };
        Ok(Mixer { output: output.as_ref().map(|output| device(output)).transpose()?, input, apps })
    })
}

pub fn set_volume(target: Target, level: f32) -> std::result::Result<bool, String> {
    with_com(|| unsafe {
        let devices = enumerator()?;
        match target {
            Target::Output | Target::Input => {
                let flow = if matches!(target, Target::Output) { eRender } else { eCapture };
                let Some(device) = default(&devices, flow, eConsole) else { return Ok(false) };
                volume(&device)?.SetMasterVolumeLevelScalar(level, null())?;
                Ok(true)
            }
            Target::App(id) => {
                let found = app_sessions(&devices, &id)?;
                for session in &found {
                    session.cast::<ISimpleAudioVolume>()?.SetMasterVolume(level, null())?;
                }
                Ok(!found.is_empty())
            }
        }
    })
}

pub fn set_muted(target: Target, muted: bool) -> std::result::Result<bool, String> {
    with_com(|| unsafe {
        let devices = enumerator()?;
        match target {
            Target::Output => {
                let Some(device) = default(&devices, eRender, eConsole) else { return Ok(false) };
                volume(&device)?.SetMute(muted, null())?;
                Ok(true)
            }
            Target::Input => {
                let inputs = inputs(&devices)?;
                for device in &inputs {
                    volume(device)?.SetMute(muted, null())?;
                }
                Ok(!inputs.is_empty())
            }
            Target::App(id) => {
                let found = app_sessions(&devices, &id)?;
                for session in &found {
                    session.cast::<ISimpleAudioVolume>()?.SetMute(muted, null())?;
                }
                Ok(!found.is_empty())
            }
        }
    })
}

pub fn outputs() -> std::result::Result<Vec<Output>, String> {
    with_com(|| unsafe {
        let all = enumerator()?.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
        let mut outputs = Vec::new();
        for i in 0..all.GetCount()? {
            let device = all.Item(i)?;
            outputs.push(Output { id: id(&device)?, name: name(&device)? });
        }
        outputs.sort_by_cached_key(|output| output.name.to_lowercase());
        Ok(outputs)
    })
}

pub fn set_output(target: &str) -> std::result::Result<bool, String> {
    // The kept meters are for the old output, so clear them.
    *METERS.lock().unwrap_or_else(PoisonError::into_inner) = None;
    with_com(|| unsafe {
        let all = enumerator()?.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
        for i in 0..all.GetCount()? {
            if id(&all.Item(i)?)? == target {
                default_output::set_default(target)?;
                return Ok(true);
            }
        }
        Ok(false)
    })
}

#[cfg(test)]
mod tests {
    use super::super::meters::levels;
    use super::*;
    use std::time::Instant;

    /// Reads the real sound of the machine. Run it with `--ignored --nocapture` to see the values.
    #[test]
    #[ignore]
    fn reads_the_mixer() {
        let mixer = mixer().unwrap();
        println!("{}", serde_json::to_string_pretty(&mixer).unwrap());
        println!("{}", serde_json::to_string_pretty(&levels().unwrap()).unwrap());
        println!("{}", serde_json::to_string_pretty(&outputs().unwrap()).unwrap());
        let start = Instant::now();
        for _ in 0..100 {
            levels().unwrap();
        }
        println!("levels(): {:.2} ms each", start.elapsed().as_secs_f64() * 10.0);
    }
}
