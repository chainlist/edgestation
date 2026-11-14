use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};
use tauri::{AppHandle, Runtime};
use tauri_plugin_opener::OpenerExt;

use super::roots::{roots, user_dir};
use crate::blocking;

/// Lists every installed widget. Each entry is its `manifest.json` with `id` (the folder name)
/// and `source` added. A broken manifest stays in the list with an `error`, so widget authors
/// can see the cause.
#[tauri::command]
pub async fn widgets_list<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Value>, String> {
    // The scan reads many files, so it runs on a blocking thread.
    blocking::run(move || Ok(read_widgets(&app))).await
}

fn read_widgets<R: Runtime>(app: &AppHandle<R>) -> Vec<Value> {
    let mut seen = HashSet::new();
    let mut widgets = Vec::new();
    for root in roots(app) {
        let Ok(entries) = fs::read_dir(&root.dir) else { continue };
        for dir in entries.flatten().map(|e| e.path()).filter(|p| p.is_dir()) {
            let id = dir.file_name().unwrap_or_default().to_string_lossy().into_owned();
            if !seen.insert(id.clone()) {
                continue;
            }
            let mut manifest = read_manifest(&dir).unwrap_or_else(|error| {
                json!({ "name": id, "error": format!("Could not read manifest.json: {error}") })
            });
            manifest["id"] = json!(id);
            manifest["source"] = json!(root.source);
            widgets.push(manifest);
        }
    }
    widgets
}

pub(super) fn read_manifest(dir: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(dir.join("manifest.json")).map_err(|e| e.to_string())?;
    let manifest: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if manifest.is_object() {
        Ok(manifest)
    } else {
        Err("manifest.json must contain an object".into())
    }
}

#[tauri::command]
pub fn widgets_open_dir<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let dir = user_dir(&app).map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    app.opener().open_path(dir.to_string_lossy(), None::<&str>).map_err(|e| e.to_string())
}
