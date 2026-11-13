use serde::Serialize;
use std::path::{Component, Path, PathBuf};
use tauri::{AppHandle, Manager, Runtime};

/// Where a widget folder comes from. `list_widgets` gives it to Settings as `source`.
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    /// The user put it in the widgets folder.
    Local,
    /// Installed from the marketplace, which also keeps it up to date.
    Marketplace,
    /// Bundled with the app.
    Builtin,
}

/// A folder that contains one sub-folder for each widget.
pub struct Root {
    pub dir: PathBuf,
    pub source: Source,
}

/// Users put their own widgets in this folder, `<app data>/widgets`.
pub fn user_dir<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<PathBuf> {
    Ok(app.path().app_data_dir()?.join("widgets"))
}

/// The folder for marketplace widgets, `<app data>/marketplace`. Only the marketplace writes
/// their, so an update never changes the user's own folders.
pub fn marketplace_dir<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<PathBuf> {
    Ok(app.path().app_data_dir()?.join("marketplace"))
}

/// Debug builds read the repo folder directly, so a change to an example needs only a reload.
#[cfg(debug_assertions)]
fn bundled_dir<R: Runtime>(_app: &AppHandle<R>) -> Option<PathBuf> {
    Some(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../widgets"))
}

#[cfg(not(debug_assertions))]
fn bundled_dir<R: Runtime>(app: &AppHandle<R>) -> Option<PathBuf> {
    app.path().resource_dir().ok().map(|dir| dir.join("widgets"))
}

/// Returns the roots in search order. A user widget hides a marketplace widget with the same id,
/// and both hide an bundled one.
pub fn roots<R: Runtime>(app: &AppHandle<R>) -> Vec<Root> {
    let user = user_dir(app).ok().map(|dir| Root { dir, source: Source::Local });
    let marketplace =
        marketplace_dir(app).ok().map(|dir| Root { dir, source: Source::Marketplace });
    let bundled = bundled_dir(app).map(|dir| Root { dir, source: Source::Builtin });
    user.into_iter().chain(marketplace).chain(bundled).collect()
}

/// Maps `<widget id>/<file>` to a file on disk. The id selects the first root that has that
/// folder. Only plain path segments are accepted, not `..` or drive prefixes, so a request can
/// never leave the widget folder.
pub fn resolve(roots: &[Root], rel: &str) -> Option<PathBuf> {
    let rel = Path::new(rel);
    if !rel.components().all(|c| matches!(c, Component::Normal(_))) {
        return None;
    }
    let mut parts = rel.components();
    let id = parts.next()?;
    let dir = roots.iter().map(|r| r.dir.join(id)).find(|d| d.is_dir())?;
    let file = dir.join(parts.as_path());
    file.is_file().then_some(file)
}
