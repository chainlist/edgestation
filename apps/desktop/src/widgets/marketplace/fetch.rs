use std::time::Duration;

use serde_json::Value;
use tauri::{AppHandle, Runtime};
use tauri_plugin_http::reqwest::Client;

const REGISTRY: &str =
    "https://raw.githubusercontent.com/chainlist/edgestation-widgets/main/widgets.json";
const STATS: &str =
    "https://raw.githubusercontent.com/chainlist/edgestation-widgets/stats/stats.json";
const GITHUB: &str = "https://github.com";
const RAW: &str = "https://raw.githubusercontent.com";

/// The largest JSON file to read, for the registry, the stats and the manifests.
const MAX_JSON: usize = 1 << 20;

pub(super) fn client<R: Runtime>(app: &AppHandle<R>) -> Result<Client, String> {
    Client::builder()
        .user_agent(format!("EdgeStation/{}", app.package_info().version))
        .connect_timeout(Duration::from_secs(15))
        .read_timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())
}

pub(super) async fn get_json(client: &Client, url: &str) -> Result<Value, String> {
    let body = get(client, url, MAX_JSON).await?;
    serde_json::from_slice(&body).map_err(|e| format!("{url} is not valid JSON: {e}"))
}

/// Gets the body of `url`. It refuses a body of more then `max` bytes.
pub(super) async fn get(client: &Client, url: &str, max: usize) -> Result<Vec<u8>, String> {
    let mut response = client.get(url).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("{url} answered {}", response.status()));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        body.extend_from_slice(&chunk);
        if body.len() > max {
            return Err(format!("{url} is larger than {} MB", (max >> 20).max(1)));
        }
    }
    Ok(body)
}

/// Debug builds can send every marketplace URL to a local server with the same layout as GitHub
/// (`EDGESTATION_REGISTRY=http://localhost:8000`). Use this to test the marketplace before you
/// publish anything.
pub(super) fn test_host() -> Option<String> {
    if !cfg!(debug_assertions) {
        return None;
    }
    let host = std::env::var("EDGESTATION_REGISTRY").ok()?;
    Some(host.trim_end_matches('/').to_owned())
}

pub(super) fn registry_url() -> String {
    test_host().map_or_else(|| REGISTRY.into(), |host| format!("{host}/widgets.json"))
}

pub(super) fn stats_url() -> String {
    test_host().map_or_else(|| STATS.into(), |host| format!("{host}/stats.json"))
}

pub(super) fn release_url(repo: &str, asset: &str) -> String {
    let host = test_host().unwrap_or_else(|| GITHUB.into());
    format!("{host}/{repo}/releases/latest/download/{asset}")
}

pub(super) fn raw_url(repo: &str, path: &str) -> String {
    let host = test_host().unwrap_or_else(|| RAW.into());
    format!("{host}/{repo}/HEAD/{}", path.trim_start_matches("./"))
}
