use serde_json::Value;
use tauri_plugin_http::reqwest::Client;

use crate::http::get;

const REGISTRY: &str =
    "https://raw.githubusercontent.com/chainlist/edgestation-widgets/main/widgets.json";
const STATS: &str =
    "https://raw.githubusercontent.com/chainlist/edgestation-widgets/stats/stats.json";
const GITHUB: &str = "https://github.com";
const RAW: &str = "https://raw.githubusercontent.com";

/// The largest JSON file to read, for the registry, the stats and the manifests.
const MAX_JSON: usize = 1 << 20;

pub(super) async fn get_json(client: &Client, url: &str) -> Result<Value, String> {
    let body = get(client, url, MAX_JSON).await?;
    serde_json::from_slice(&body).map_err(|e| format!("{url} is not valid JSON: {e}"))
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
