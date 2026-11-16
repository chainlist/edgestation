//! The macOS side, a stub. It lists no apps and draws no icons. The real side reads the app
//! bundles of the Applications folders.

use tauri::{Runtime, Window};

use super::App;
use crate::os::macos::MISSING;

pub fn list() -> Result<Vec<App>, String> {
    Err(MISSING.into())
}

/// macOS does not let an app pick the screen for the windows of an other app.
pub fn away_from<R: Runtime>(_window: &Window<R>) -> Option<isize> {
    None
}

pub fn launch(_id: &str, _screen: Option<isize>) -> Result<(), String> {
    Err(MISSING.into())
}

pub fn icon(_id: &str, _size: u32) -> Option<Vec<u8>> {
    None
}

pub fn executable_icon(_path: &str, _size: u32) -> Option<Vec<u8>> {
    None
}
