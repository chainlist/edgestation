use std::path::PathBuf;
use std::{fs, io};

use tauri::{AppHandle, Manager, Runtime};

use super::{PREFIX, native, receive};
use crate::blocking;

/// True if `id` can name a profile folder. Only letters, digits and dashes are allowed.
fn valid_profile(id: &str) -> bool {
    !id.is_empty() && id.len() <= 64 && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// The data folder of profile `id`, in the local data folder of the app.
pub(super) fn profile_dir<R: Runtime>(app: &AppHandle<R>, id: &str) -> Result<PathBuf, String> {
    if !valid_profile(id) {
        return Err(format!("not a profile id: {id}"));
    }
    let data = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    Ok(data.join("profiles").join(id))
}

/// True if view `label` opened in profile `id`.
fn in_profile(label: &str, id: &str) -> bool {
    label
        .strip_prefix(PREFIX)
        .and_then(|rest| rest.split_once('_'))
        .is_some_and(|(_, profile)| profile == id)
}

/// Deletes the data folder of profile `profile`, so the profile is signed out of all sites. Its
/// open views close first. On Windows, the folder goes when their browser exits, because
/// WebView2 locks it untill then. On Linux, WebKit clears the data first, because it keeps the
/// cookies of a profile in memory while the app runs.
#[tauri::command]
pub async fn webview_profile_delete<R: Runtime>(
    app: AppHandle<R>,
    profile: String,
) -> Result<(), String> {
    let dir = profile_dir(&app, &profile)?;
    let views: Vec<_> = app
        .webviews()
        .into_values()
        .filter(|view| in_profile(view.label(), &profile))
        .collect();
    if let Some(first) = views.first() {
        let released = native::release(first, profile).await?;
        for view in &views {
            view.close().map_err(|e| e.to_string())?;
        }
        receive(released).await?;
    }
    blocking::run(move || match fs::remove_dir_all(&dir) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(format!("{}: {e}", dir.display())),
        _ => Ok(()),
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_ids_are_plain_folder_names() {
        assert!(valid_profile("a1b2c3d4"));
        assert!(valid_profile("work-2"));
        for id in ["", "..", "a/b", r"a\b", "a_b", "C:", &"a".repeat(65)] {
            assert!(!valid_profile(id), "{id}");
        }
    }

    #[test]
    fn views_are_matched_to_their_profile() {
        assert!(in_profile("widget-webview-3_a1b2c3d4", "a1b2c3d4"));
        assert!(!in_profile("widget-webview-3_a1b2c3d4", "a1b2"));
        assert!(!in_profile("widget-webview-3", "a1b2c3d4"));
        assert!(!in_profile("edge", "a1b2c3d4"));
    }
}
