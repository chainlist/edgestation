//! The macOS side, a stub. The calls fail. The real side needs the private MediaRemote framework,
//! which macOS 15.4 and later block for other apps.

use tauri::AppHandle;

use super::{Action, MediaInfo};
use crate::os::macos::MISSING;

pub fn read() -> Result<Option<MediaInfo>, String> {
    Err(MISSING.into())
}

pub fn read_all() -> Result<Vec<MediaInfo>, String> {
    Err(MISSING.into())
}

pub fn control(_action: Action, _id: Option<&str>) -> Result<bool, String> {
    Err(MISSING.into())
}

pub fn seek(_position: f64, _id: Option<&str>) -> Result<bool, String> {
    Err(MISSING.into())
}

pub fn watch(_app: &AppHandle, _on: bool) -> Result<(), String> {
    Ok(())
}

pub fn unwatch() {}
