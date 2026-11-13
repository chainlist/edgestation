//! The Windows side. Windows names each monitor after its display device, which no other monitor
//! has.

use tauri::Monitor;

pub fn name(monitors: Vec<Monitor>) -> Vec<(String, Monitor)> {
    monitors.into_iter().filter_map(|monitor| Some((monitor.name()?.clone(), monitor))).collect()
}
