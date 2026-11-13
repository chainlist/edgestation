//! The executables of running processes, and the names that people know them by. The top
//! programs (`system`) and the apps of the audio mixer (`audio`) show these names.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

/// The path of the executable of process `pid`. On Windows, this is a kernel path, for example
/// `\Device\HarddiskVolume3\...`, which `program_name` takes.
pub use native::image_path;
/// The name that people know the executable at `path` by. On Windows, this is the description in
/// the executable, as in Task Manager. On Linux, it is the name of its app in the app menu.
pub use native::program_name;
