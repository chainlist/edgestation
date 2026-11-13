use std::collections::HashMap;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::Media::Audio::{eCapture, eConsole, eRender};
use windows::Win32::System::Com::CLSCTX_ALL;
use windows::core::{Interface, Result};

use super::devices::{default, enumerator};
use super::sessions::sessions;
use super::with_com;
use crate::audio::Levels;
use crate::os::windows::com::{Apartment, Com, in_mta, keep_mta};

/// How long later reads use the meters of one read. A list of the sessions costs several times
/// more than a read of their peaks, and meters read many time a second. An app that starts in
/// this time gets a meter at the next list.
const METERS_FOR: Duration = Duration::from_secs(1);
pub(super) static METERS: Mutex<Option<Meters>> = Mutex::new(None);

pub(super) struct Meters {
    at: Instant,
    output: IAudioMeterInformation,
    input: Option<IAudioMeterInformation>,
    apps: Vec<(String, IAudioMeterInformation)>,
}

// Only threads of the multithreaded apartment keep this value, and `keep_mta` keeps that
// apartment alive between requests. Its objects work from any of its threads.
unsafe impl Send for Meters {}

/// Reads the levels for the sampler thread. COM stays set up on that thread while the meter
/// lives, so each read does not set it up again.
pub struct Meter {
    _com: Com,
}

impl Meter {
    pub fn open() -> Self {
        keep_mta();
        Self { _com: Com::set_up(Apartment::Multithreaded) }
    }

    pub fn read(&mut self) -> std::result::Result<Levels, String> {
        levels()
    }
}

pub fn levels() -> std::result::Result<Levels, String> {
    keep_mta();
    with_com(|| unsafe {
        let mut cache = METERS.lock().unwrap_or_else(PoisonError::into_inner);
        // A thread in another apartment cannot use the kept meters, so it reads its own meters.
        let keep = in_mta();
        let fresh = !keep || cache.as_ref().is_none_or(|meters| meters.at.elapsed() > METERS_FOR);
        if fresh {
            *cache = meters()?;
        }
        let mut levels = cache.as_ref().map(|meters| peaks(meters)).transpose();
        // A kept meter fails when its device is gone, for example unplugged or disabled. Then
        // list the meters again.
        if levels.is_err() && !fresh {
            *cache = meters()?;
            levels = cache.as_ref().map(|meters| peaks(meters)).transpose();
        }
        if !keep {
            *cache = None;
        }
        Ok(levels?.unwrap_or_default())
    })
}

/// The meters of the default output, its apps and the default microphone. `None` if there is no
/// default output.
unsafe fn meters() -> Result<Option<Meters>> {
    unsafe {
        let devices = enumerator()?;
        let Some(output) = default(&devices, eRender, eConsole) else { return Ok(None) };
        let mut apps = Vec::new();
        for (id, session) in sessions(&output)? {
            apps.push((id, session.cast()?));
        }
        let input = match default(&devices, eCapture, eConsole) {
            Some(input) => Some(input.Activate(CLSCTX_ALL, None)?),
            None => None,
        };
        let output = output.Activate(CLSCTX_ALL, None)?;
        Ok(Some(Meters { at: Instant::now(), output, input, apps }))
    }
}

unsafe fn peaks(meters: &Meters) -> Result<Levels> {
    unsafe {
        let mut apps = HashMap::new();
        for (id, meter) in &meters.apps {
            let level: &mut f32 = apps.entry(id.clone()).or_default();
            *level = level.max(meter.GetPeakValue()?);
        }
        let input = match &meters.input {
            Some(meter) => meter.GetPeakValue()?,
            None => 0.0,
        };
        Ok(Levels { output: meters.output.GetPeakValue()?, input, apps })
    }
}
