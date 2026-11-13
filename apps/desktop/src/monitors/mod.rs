//! The names of the monitors, as the Display settings show and save them. On Linux, Tauri names
//! a monitor after its model, so two monitors of one model have the same name. There, the name
//! also has the connector of the monitor, for example `XENEON EDGE (DP-2)`.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

use serde::Serialize;
use tauri::{AppHandle, Monitor, Runtime};

/// The name of the monitor at `x`, `y`. Monitors do not overlap, so the top-left corner tells
/// them apart.
#[derive(Serialize)]
pub struct MonitorName {
    x: i32,
    y: i32,
    name: String,
}

/// Each monitor that has a name, with its name.
pub fn named<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Vec<(String, Monitor)>> {
    Ok(native::name(app.available_monitors()?))
}

/// The names of the monitors, for the settings. `availableMonitors` gives the names of Tauri.
#[tauri::command]
pub async fn monitor_names<R: Runtime>(app: AppHandle<R>) -> Result<Vec<MonitorName>, String> {
    let named = named(&app).map_err(|error| error.to_string())?;
    let name = |(name, monitor): (String, Monitor)| {
        let at = monitor.position();
        MonitorName { x: at.x, y: at.y, name }
    };
    Ok(named.into_iter().map(name).collect())
}
