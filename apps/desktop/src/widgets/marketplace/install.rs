use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde_json::Value;
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Runtime};
use tauri_plugin_http::reqwest::Client;

use super::changed;
use super::fetch::release_url;
use super::registry::{Entry, registry, release_manifest, unavailable, valid_id};
use super::unpack::{check_widget, unpack};
use super::version::compatibility;
use crate::blocking;
use crate::http::{client, get};
use crate::widgets::roots::{marketplace_dir, user_dir};

/// The largest `widget.zip` to download, and the largest size it can unpack to.
const MAX_ZIP: usize = 50 << 20;
pub(super) const MAX_UNPACKED: u64 = 200 << 20;

/// Allows one install or removal at a time, so a launch update and a click never change a
/// folder at the same time.
static BUSY: Mutex<()> = Mutex::const_new(());

/// Installs the latest release of marketplace widget `id`, or replaces the installed copy with it.
#[tauri::command]
pub async fn marketplace_install<R: Runtime>(app: AppHandle<R>, id: String) -> Result<(), String> {
    let client = client()?;
    let entry = registry(&client)
        .await?
        .into_iter()
        .find(|entry| entry.id == id)
        .ok_or_else(|| format!("\"{id}\" is not in the marketplace"))?;
    let manifest = release_manifest(&client, &entry.repo).await?;
    install(&app, &client, &entry, &manifest).await?;
    changed(&app, &[id]);
    Ok(())
}

/// Removes marketplace widget `id`. Its saved data stays, so a new install recieves that data
/// again.
#[tauri::command]
pub async fn marketplace_uninstall<R: Runtime>(
    app: AppHandle<R>,
    id: String,
) -> Result<(), String> {
    if !valid_id(&id) {
        return Err(format!("\"{id}\" is not a widget id"));
    }
    let _busy = BUSY.lock().await;
    let root = marketplace_dir(&app).map_err(|e| e.to_string())?;
    let target = id.clone();
    blocking::run(move || {
        let trash = work_dir(&root).join("trash").join(&target);
        // Move the folder out first. Then a file that Windows holds open cannot leave half a
        // widget in place.
        let _ = fs::remove_dir_all(&trash);
        fs::create_dir_all(work_dir(&root).join("trash")).map_err(|e| e.to_string())?;
        fs::rename(root.join(&target), &trash)
            .map_err(|e| format!("Couldn't remove \"{target}\": {e}"))?;
        let _ = fs::remove_dir_all(&trash);
        Ok(())
    })
    .await?;
    changed(&app, &[id]);
    Ok(())
}

/// Downloads the `widget.zip` of `entry`, from the release that has `manifest`. It then replaces
/// the installed copy, if there is one.
pub(super) async fn install<R: Runtime>(
    app: &AppHandle<R>,
    client: &Client,
    entry: &Entry,
    manifest: &Value,
) -> Result<(), String> {
    let _busy = BUSY.lock().await;
    let id = &entry.id;
    if user_dir(app).map_err(|e| e.to_string())?.join(id).exists() {
        return Err(format!(
            "One of your own widgets uses the id \"{id}\". Rename or remove its folder first."
        ));
    }
    compatibility(
        &app.package_info().version.to_string(),
        manifest["minAppVersion"].as_str(),
    )?;

    let zip = get(client, &release_url(&entry.repo, "widget.zip"), MAX_ZIP)
        .await
        .map_err(|e| unavailable(&entry.repo, e))?;
    let root = marketplace_dir(app).map_err(|e| e.to_string())?;
    let id = id.clone();
    let repo = entry.repo.clone();
    blocking::run(move || {
        let staged = work_dir(&root).join("staging").join(&id);
        let _ = fs::remove_dir_all(&staged);
        let result = unpack(&zip, &staged, MAX_UNPACKED)
            .and_then(|()| check_widget(&staged))
            .map_err(|e| unavailable(&repo, e))
            .and_then(|()| swap(&root, &id, &staged).map_err(|e| format!("Couldn't install: {e}")));
        let _ = fs::remove_dir_all(&staged);
        result
    })
    .await
}

/// Puts `staged` at `root/<id>`. It first moves the installed copy out of the way, and moves it
/// back if the move fails. Thus the widget is never half replaced.
fn swap(root: &Path, id: &str, staged: &Path) -> io::Result<()> {
    let dest = root.join(id);
    let trash = work_dir(root).join("trash").join(id);
    let _ = fs::remove_dir_all(&trash);
    fs::create_dir_all(root)?;
    let replacing = dest.exists();
    if replacing {
        fs::create_dir_all(work_dir(root).join("trash"))?;
        fs::rename(&dest, &trash)?;
    }
    if let Err(error) = fs::rename(staged, &dest) {
        if replacing {
            let _ = fs::rename(&trash, &dest);
        }
        return Err(error);
    }
    let _ = fs::remove_dir_all(&trash);
    Ok(())
}

/// The folder where installs unpack and where old copies wait for deletion. It is next too the
/// marketplace folder, so its contents never show as widgets.
fn work_dir(root: &Path) -> PathBuf {
    root.with_file_name("marketplace-work")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::marketplace::temp_dir;

    #[test]
    fn swap_replaces_the_installed_copy() {
        let dir = temp_dir("swap");
        let root = dir.join("marketplace");
        fs::create_dir_all(root.join("w")).unwrap();
        fs::write(root.join("w/old.txt"), "old").unwrap();
        let staged = dir.join("staged");
        fs::create_dir_all(&staged).unwrap();
        fs::write(staged.join("new.txt"), "new").unwrap();

        swap(&root, "w", &staged).unwrap();
        assert!(root.join("w/new.txt").is_file());
        assert!(!root.join("w/old.txt").exists());
        assert!(!staged.exists());
        assert!(!work_dir(&root).join("trash/w").exists());
        fs::remove_dir_all(dir).unwrap();
    }
}
