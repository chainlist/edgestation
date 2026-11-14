use futures_util::{StreamExt, stream};
use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Runtime};
use tauri_plugin_opener::OpenerExt;

use super::fetch::{get_json, raw_url, stats_url};
use super::registry::{registry, release_manifest};
use crate::http::client;

/// The number of release manifests to fetch at the same time for the list. This is fast, but it
/// does not overlaod GitHub when the registry gets long.
const PARALLEL_FETCHES: usize = 8;

/// A registry widget, as Settings lists it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Listed {
    id: String,
    repo: String,
    /// The downloads of its `widget.zip` for all releases, if the stats have them.
    downloads: Option<u64>,
    /// The date of its latest release, if the stats have it.
    released: Option<String>,
    /// The `manifest.json` of its latest release. If the app cannot read it, `error` gives the
    /// reason.
    manifest: Option<Value>,
    error: Option<String>,
    /// The URLs of its icon and README on the default branch of the repo.
    icon: Option<String>,
    readme: String,
}

/// Lists every widget of the marketplace, with the manifest of its latest release and its
/// download count.
#[tauri::command]
pub async fn marketplace_list() -> Result<Vec<Listed>, String> {
    let client = client()?;
    let registry = registry(&client).await?;
    // If there are no stats yet, or a GitHub error occured, only the counts are missing.
    let stats = get_json(&client, &stats_url()).await.unwrap_or_default();

    // One small request for each widget, a few at a time, in registry order.
    let client = &client;
    let fetched: Vec<_> = stream::iter(registry)
        .map(|entry| async move {
            let manifest = release_manifest(client, &entry.repo).await;
            (entry, manifest)
        })
        .buffered(PARALLEL_FETCHES)
        .collect()
        .await;

    let mut listed = Vec::new();
    for (entry, manifest) in fetched {
        let stat = &stats["widgets"][&entry.id];
        let icon = manifest
            .as_ref()
            .ok()
            .and_then(|m| m["icon"].as_str())
            .map(|path| raw_url(&entry.repo, path));
        listed.push(Listed {
            downloads: stat["downloads"].as_u64(),
            released: stat["released"].as_str().map(str::to_owned),
            icon,
            readme: raw_url(&entry.repo, "README.md"),
            error: manifest.as_ref().err().cloned(),
            manifest: manifest.ok(),
            id: entry.id,
            repo: entry.repo,
        });
    }
    Ok(listed)
}

/// Opens an `https` link in the default browser, for example the repo or README of a widget.
#[tauri::command]
pub fn marketplace_open_link<R: Runtime>(app: AppHandle<R>, url: String) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err(format!("Not an https link: {url}"));
    }
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| e.to_string())
}
