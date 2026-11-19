/// The error of a feature that has no macOS side yet. Each stub returns it.
pub const MISSING: &str = "EdgeStation does not support this on macOS yet";

/// macOS needs nothing before Tauri starts.
pub fn prepare() {}
