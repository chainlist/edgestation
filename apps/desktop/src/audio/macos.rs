//! The macOS side, a stub. The mixer calls fail and the meters stay silent. The real side uses
//! Core Audio, which has no public way to change the volume of one app.

use tauri::AppHandle;

use super::{Levels, Mixer, Output, Target};
use crate::os::macos::MISSING;

pub fn mixer() -> Result<Mixer, String> {
    Err(MISSING.into())
}

pub fn set_volume(_target: Target, _level: f32) -> Result<bool, String> {
    Err(MISSING.into())
}

pub fn set_muted(_target: Target, _muted: bool) -> Result<bool, String> {
    Err(MISSING.into())
}

pub fn outputs() -> Result<Vec<Output>, String> {
    Err(MISSING.into())
}

pub fn set_output(_id: &str) -> Result<bool, String> {
    Err(MISSING.into())
}

/// Silence, so the sampler has no error to print 25 times a second.
pub fn levels() -> Result<Levels, String> {
    Ok(Levels::default())
}

pub struct Meter;

impl Meter {
    pub fn open() -> Self {
        Self
    }

    pub fn read(&mut self) -> Result<Levels, String> {
        levels()
    }
}

/// No change events come, so a watch has nothing to start.
pub fn watch(_app: &AppHandle, _on: bool) -> Result<(), String> {
    Ok(())
}

pub fn unwatch() {}
