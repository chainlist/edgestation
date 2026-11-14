//! The executables of running processes, and the names that people know them by. The top
//! programs (`system`) and the apps of the audio mixer (`audio`) show these names.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

use std::collections::HashMap;
use std::sync::{Mutex, PoisonError};

/// The path of the executable of process `pid`. On Windows, this is a kernel path, for example
/// `\Device\HarddiskVolume3\...`, which `program_name` takes.
pub use native::image_path;
/// The name that people know the executable at `path` by. On Windows, this is the description in
/// the executable, as in Task Manager. On Linux, it is the name of its app in the app menu.
pub use native::program_name;

/// The names that `display_name` found, by path. The host reads each file only once.
static NAMES: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

/// The name to show for the executable at `path`: its `program_name`, else its file name without
/// `.exe`. The top programs and the audio mixer show the same name for one program.
pub fn display_name(path: &str) -> String {
    let mut names = NAMES.lock().unwrap_or_else(PoisonError::into_inner);
    let names = names.get_or_insert_with(HashMap::new);
    names
        .entry(path.to_owned())
        .or_insert_with(|| {
            program_name(path).unwrap_or_else(|| {
                let file = file_name(path);
                file.strip_suffix(".exe").unwrap_or(file).to_owned()
            })
        })
        .clone()
}

/// The last part of `path`, a Windows kernel path or a Linux path.
pub fn file_name(path: &str) -> &str {
    path.rsplit(['\\', '/']).next().unwrap_or(path)
}
