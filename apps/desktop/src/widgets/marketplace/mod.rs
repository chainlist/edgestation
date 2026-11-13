//! The widget marketplace is a list of GitHub repos in `widgets.json` of the registry repo.
//! Each repo releases its widget as two assets, `manifest.json` and `widget.zip`. The release
//! manifest tells if there is an update. The app downloads the zip only to install it.

mod fetch;
pub(crate) mod install;
pub(crate) mod listing;
mod registry;
mod unpack;
pub(crate) mod update;
mod version;

use tauri::{AppHandle, Emitter, Runtime};

/// The event to every window, with the ids of the installed, updated or removed widgets.
const CHANGED: &str = "widgets:changed";

fn changed<R: Runtime>(app: &AppHandle<R>, ids: &[String]) {
    if let Err(error) = app.emit(CHANGED, ids) {
        eprintln!("failed to announce widget changes: {error}");
    }
}

/// Makes a new, empty folder for the test `name`.
#[cfg(test)]
fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("edgestation-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
