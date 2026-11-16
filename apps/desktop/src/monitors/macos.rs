//! The macOS side, with the names that Tauri gives. It is not known yet if two monitors of one
//! model get different names.

use tauri::Monitor;

pub fn name(monitors: Vec<Monitor>) -> Vec<(String, Monitor)> {
    monitors.into_iter().filter_map(|monitor| Some((monitor.name()?.clone(), monitor))).collect()
}
