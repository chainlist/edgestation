//! The marketplace is off until the user turns it on in Settings and accepts the warning. While it
//! is off, its widgets do not run and the app does not contact the registry.

use std::fs;

use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use super::changed;
use crate::widgets::roots::marketplace_dir;

/// The key in `settings.json`. Settings reads it as `marketplaceOn`.
const KEY: &str = "marketplace";

/// Tells if the user turned the marketplace on.
pub fn is_on<R: Runtime>(app: &AppHandle<R>) -> bool {
    app.store("settings.json")
        .ok()
        .and_then(|store| store.get(KEY))
        .and_then(|on| on.as_bool())
        .unwrap_or(false)
}

/// Stops a command that contacts the registry while the marketplace is off.
pub(super) fn require_on<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    if is_on(app) { Ok(()) } else { Err("The marketplace is off.".into()) }
}

/// Turns the marketplace on or off. The installed marketplace widgets start or stop at once, and
/// keep their placements and data.
#[tauri::command]
pub fn marketplace_switch<R: Runtime>(app: AppHandle<R>, on: bool) -> Result<(), String> {
    let settings = app.store("settings.json").map_err(|e| e.to_string())?;
    settings.set(KEY, on);
    // Settings rescans, and the Edge remounts the tiles of theese widgets.
    let ids: Vec<String> = fs::read_dir(marketplace_dir(&app).map_err(|e| e.to_string())?)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    changed(&app, &ids);
    Ok(())
}
