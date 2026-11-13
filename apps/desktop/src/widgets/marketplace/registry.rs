use serde_json::Value;
use tauri_plugin_http::reqwest::Client;

use super::fetch::{get_json, registry_url, release_url};

/// A widget of the registry.
#[derive(Clone)]
pub(super) struct Entry {
    pub(super) id: String,
    pub(super) repo: String,
}

/// True if the marketplace can create a folder with this name. Only lowercase letters, digits
/// and dashes are allowed.
pub(super) fn valid_id(id: &str) -> bool {
    !id.starts_with('-')
        && !id.is_empty()
        && id
            .bytes()
            .all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'-'))
}

/// True for a GitHub repo in the form `owner/name`.
fn valid_repo(repo: &str) -> bool {
    let part = |p: &str| {
        !p.is_empty()
            && p != "."
            && p != ".."
            && p.bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
    };
    matches!(repo.split_once('/'), Some((owner, name)) if part(owner) && part(name))
}

/// Gets the widgets of the registry. It skipps each entry that does not have a valid id and repo.
pub(super) async fn registry(client: &Client) -> Result<Vec<Entry>, String> {
    let list = get_json(client, &registry_url())
        .await
        .map_err(marketplace_unavailable)?;
    let entries = list
        .as_array()
        .ok_or_else(|| marketplace_unavailable("its list is not an array".into()))?;
    Ok(entries
        .iter()
        .filter_map(|entry| {
            let id = entry["id"].as_str()?;
            let repo = entry["repo"].as_str()?;
            (valid_id(id) && valid_repo(repo)).then(|| Entry {
                id: id.into(),
                repo: repo.into(),
            })
        })
        .collect())
}

/// Gets the `manifest.json` of the latest release of `repo`.
pub(super) async fn release_manifest(client: &Client, repo: &str) -> Result<Value, String> {
    match get_json(client, &release_url(repo, "manifest.json")).await {
        Ok(manifest) if manifest.is_object() => Ok(manifest),
        Ok(_) => Err(unavailable(
            repo,
            "its manifest.json must contain an object".into(),
        )),
        Err(cause) => Err(unavailable(repo, cause)),
    }
}

/// The message that Settings shows when it can't read the registry. The cause goes to the log
/// only.
fn marketplace_unavailable(cause: String) -> String {
    eprintln!("can't read the marketplace: {cause}");
    "The marketplace is currently unavailable. Check your internet connection and try again.".into()
}

/// The message that Settings shows when the latest release of `repo` is not usable, for example
/// when an asset is missing. The cause is for the widget author, so it goes to the log only.
pub(super) fn unavailable(repo: &str, cause: String) -> String {
    eprintln!("the latest release of {repo} is not usable: {cause}");
    "This widget is currently unavailable for download.".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_and_repos() {
        assert!(valid_id("weather-2"));
        assert!(!valid_id("Weather"));
        assert!(!valid_id("../x"));
        assert!(!valid_id("-x"));
        assert!(!valid_id(""));
        assert!(valid_repo("jane/edgestation-weather"));
        assert!(!valid_repo("jane"));
        assert!(!valid_repo("jane/a/b"));
        assert!(!valid_repo("../x"));
    }

    #[test]
    fn messages_hide_the_cause() {
        let url = "https://github.com/jane/w/releases/latest/download/widget.zip";
        let message = unavailable("jane/w", format!("{url} answered 404 Not Found"));
        assert_eq!(
            message,
            "This widget is currently unavailable for download."
        );
        let message = marketplace_unavailable(format!("{url} answered 404 Not Found"));
        assert!(!message.contains(url));
    }
}
