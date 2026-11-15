use serde_json::Value;
use tauri_plugin_http::reqwest::Client;

use super::fetch::{get_json, registry_url, release_url};

/// A widget of the registry.
#[derive(Clone)]
pub(super) struct Entry {
    pub(super) id: String,
    pub(super) repo: String,
}

/// A widget that the registry bans. The app removes it and does not offer it again.
pub(super) struct Ban {
    pub(super) id: String,
    pub(super) reason: String,
}

/// The valid entries of the registry, sorted into listed and banned widgets.
pub(super) struct Registry {
    pub(super) listed: Vec<Entry>,
    pub(super) banned: Vec<Ban>,
}

/// True if the marketplace can create a folder with this name. Only lowercase letters, digits
/// and dashes are allowed.
pub(super) fn valid_id(id: &str) -> bool {
    !id.starts_with('-')
        && !id.is_empty()
        && id.bytes().all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'-'))
}

/// True for a GitHub repo in the form `owner/name`.
fn valid_repo(repo: &str) -> bool {
    let part = |p: &str| {
        !p.is_empty()
            && p != "."
            && p != ".."
            && p.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
    };
    matches!(repo.split_once('/'), Some((owner, name)) if part(owner) && part(name))
}

/// Gets the widgets of the registry.
pub(super) async fn registry(client: &Client) -> Result<Registry, String> {
    let list = get_json(client, &registry_url()).await.map_err(marketplace_unavailable)?;
    let entries = list
        .as_array()
        .ok_or_else(|| marketplace_unavailable("its list is not an array".into()))?;
    Ok(sort(entries))
}

/// Sorts the entries into listed and banned widgets, and skips each entry that is not valid. An
/// entry with `banned` needs only a valid id. Thus a ban can drop the `repo`, and older app
/// versions, wich do not know `banned`, skip the entry too.
fn sort(entries: &[Value]) -> Registry {
    let mut registry = Registry { listed: Vec::new(), banned: Vec::new() };
    for entry in entries {
        let Some(id) = entry["id"].as_str().filter(|id| valid_id(id)) else { continue };
        let ban = &entry["banned"];
        if !ban.is_null() {
            let reason = ban["reason"].as_str().unwrap_or_default();
            registry.banned.push(Ban { id: id.into(), reason: reason.into() });
        } else if let Some(repo) = entry["repo"].as_str().filter(|repo| valid_repo(repo)) {
            registry.listed.push(Entry { id: id.into(), repo: repo.into() });
        }
    }
    registry
}

/// Gets the `manifest.json` of the latest release of `repo`.
pub(super) async fn release_manifest(client: &Client, repo: &str) -> Result<Value, String> {
    match get_json(client, &release_url(repo, "manifest.json")).await {
        Ok(manifest) if manifest.is_object() => Ok(manifest),
        Ok(_) => Err(unavailable(repo, "its manifest.json must contain an object".into())),
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
    use serde_json::json;

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
    fn a_ban_needs_only_a_valid_id() {
        let registry = sort(&[
            json!({ "id": "weather", "repo": "jane/weather" }),
            json!({ "id": "rogue", "repo": "joe/rogue", "banned": { "reason": "Steals tokens" } }),
            json!({ "id": "gone", "banned": { "reason": "Mines coins", "version": "1.2.0" } }),
            json!({ "id": "Bad", "banned": { "reason": "x" } }),
            json!({ "id": "norepo" }),
        ]);
        let listed: Vec<_> = registry.listed.iter().map(|e| e.id.as_str()).collect();
        assert_eq!(listed, ["weather"]);
        let banned: Vec<_> =
            registry.banned.iter().map(|b| (b.id.as_str(), b.reason.as_str())).collect();
        assert_eq!(banned, [("rogue", "Steals tokens"), ("gone", "Mines coins")]);
    }

    #[test]
    fn messages_hide_the_cause() {
        let url = "https://github.com/jane/w/releases/latest/download/widget.zip";
        let message = unavailable("jane/w", format!("{url} answered 404 Not Found"));
        assert_eq!(message, "This widget is currently unavailable for download.");
        let message = marketplace_unavailable(format!("{url} answered 404 Not Found"));
        assert!(!message.contains(url));
    }
}
