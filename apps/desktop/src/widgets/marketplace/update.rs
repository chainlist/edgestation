use serde::Serialize;
use serde_json::Value;
use tauri::async_runtime;
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use super::changed;
use super::fetch::{client, test_host};
use super::install::install;
use super::registry::{registry, release_manifest};
use super::version::{compatibility, is_newer};
use crate::widgets::installed::read_manifest;
use crate::widgets::roots::{marketplace_dir, user_dir};

/// The result of `marketplace_update` for one widget that had a newer release.
#[derive(Serialize)]
pub struct Attempt {
    id: String,
    error: Option<String>,
}

/// Installs every newer, compatible release of the installed marketplace widgets. A release
/// that asks for more permissions waits for the user, because Settings shows them before that
/// update (see `adds_permissions`).
#[tauri::command]
pub async fn marketplace_update<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Attempt>, String> {
    update_all(&app).await
}

/// Updates the marketplace widgets once at launch, if Settings allows it. Debug builds skip this
/// step, like the app updater, unles `EDGESTATION_REGISTRY` points at a test registry.
pub fn update_at_launch<R: Runtime>(app: &AppHandle<R>) {
    if cfg!(debug_assertions) && test_host().is_none() {
        return;
    }
    let on = app
        .store("settings.json")
        .ok()
        .and_then(|store| store.get("widgetAutoUpdate"))
        .and_then(|value| value.as_bool())
        .unwrap_or(true);
    if !on {
        return;
    }
    let app = app.clone();
    async_runtime::spawn(async move {
        match update_all(&app).await {
            Ok(attempts) => {
                for Attempt { id, error } in attempts {
                    if let Some(error) = error {
                        eprintln!("failed to update widget {id}: {error}");
                    }
                }
            }
            Err(error) => eprintln!("failed to update widgets: {error}"),
        }
    });
}

async fn update_all<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<Attempt>, String> {
    let client = client(app)?;
    let root = marketplace_dir(app).map_err(|e| e.to_string())?;
    let user = user_dir(app).map_err(|e| e.to_string())?;
    let app_version = app.package_info().version.to_string();

    let mut attempts = Vec::new();
    // Only listed widgets get updates. A widget that leaves the registry gets no more updates.
    for entry in registry(&client).await? {
        let Ok(installed) = read_manifest(&root.join(&entry.id)) else {
            continue;
        };
        // A user widget with this id hides the marketplace widget, so skip the update.
        if user.join(&entry.id).exists() {
            continue;
        }
        let latest = match release_manifest(&client, &entry.repo).await {
            Ok(latest) => latest,
            Err(error) => {
                attempts.push(Attempt {
                    id: entry.id,
                    error: Some(error),
                });
                continue;
            }
        };
        let newer = is_newer(
            latest["version"].as_str().unwrap_or_default(),
            installed["version"].as_str().unwrap_or_default(),
        );
        if !newer || compatibility(&app_version, latest["minAppVersion"].as_str()).is_err() {
            continue;
        }
        if adds_permissions(&installed, &latest) {
            attempts.push(Attempt {
                id: entry.id,
                error: Some(
                    "Its new version asks for more access. Click its Update button to see what."
                        .into(),
                ),
            });
            continue;
        }
        let error = install(app, &client, &entry, &latest).await.err();
        attempts.push(Attempt {
            id: entry.id,
            error,
        });
    }

    let updated: Vec<String> = attempts
        .iter()
        .filter(|a| a.error.is_none())
        .map(|a| a.id.clone())
        .collect();
    if !updated.is_empty() {
        changed(app, &updated);
    }
    Ok(attempts)
}

/// Tells if the `latest` manifest asks for a permission that the `installed` one does not have.
/// The user must see a new permission before the update gets it.
fn adds_permissions(installed: &Value, latest: &Value) -> bool {
    let had = installed["permissions"].as_array();
    latest["permissions"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|permission| !had.is_some_and(|had| had.contains(permission)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_new_permission_needs_the_user() {
        let media = json!({ "permissions": ["media"] });
        assert!(adds_permissions(&media, &json!({ "permissions": ["media", "fetch"] })));
        assert!(adds_permissions(&json!({}), &media));
    }

    #[test]
    fn the_same_or_fewer_permissions_do_not() {
        let both = json!({ "permissions": ["fetch", "media"] });
        assert!(!adds_permissions(&both, &json!({ "permissions": ["media", "fetch"] })));
        assert!(!adds_permissions(&both, &json!({ "permissions": ["media"] })));
        assert!(!adds_permissions(&both, &json!({})));
    }
}
