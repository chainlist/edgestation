//! The Linux side.

/// The path of the executable of a process. It ends with ` (deleted)` when the file was
/// replaced, for example by an update, so this takes it off.
pub fn image_path(pid: usize) -> Option<String> {
    let path = std::fs::read_link(format!("/proc/{pid}/exe")).ok()?;
    let path = path.to_str()?;
    Some(path.strip_suffix(" (deleted)").unwrap_or(path).to_owned())
}

/// The name of the app that runs the executable at `path`, from its entry in the app menu.
pub fn program_name(path: &str) -> Option<String> {
    crate::apps::native::program_name(path)
}
