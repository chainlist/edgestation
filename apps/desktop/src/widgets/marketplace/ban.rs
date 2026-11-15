//! The registry can ban a widget whose author went rogue (`banned` in `widgets.json`). The app
//! then removes the installed copy, its placements on the pages and the data that it saved.

use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Runtime};
use tauri_plugin_store::StoreExt;

use super::changed;
use super::install::remove;
use super::registry::{Ban, registry};
use super::switch::is_on;
use crate::http::client;
use crate::widgets::installed::read_manifest;
use crate::widgets::roots::{marketplace_dir, user_dir};

/// The event to every window, with the banned widgets that a marketplace read removed.
const BANNED: &str = "widgets:banned";

/// A banned widget that the app removed. The Edge tells the user about it.
#[derive(Clone, Serialize)]
pub struct Removed {
    id: String,
    name: String,
    reason: String,
}

/// Removes the banned widgets. The Edge calls this when it opens, and shows what it removed.
/// While the marketplace is off, its widgets do not run, so nothing needs the registry.
#[tauri::command]
pub async fn marketplace_remove_banned<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<Removed>, String> {
    if !is_on(&app) {
        return Ok(Vec::new());
    }
    let registry = registry(&client()?).await?;
    Ok(remove_banned(&app, &registry.banned).await)
}

/// Removes the banned widgets and tells the Edge, for a marketplace read in Settings.
pub(super) async fn remove_and_announce<R: Runtime>(app: &AppHandle<R>, banned: &[Ban]) {
    let removed = remove_banned(app, banned).await;
    if !removed.is_empty()
        && let Err(error) = app.emit(BANNED, &removed)
    {
        eprintln!("failed to announce banned widgets: {error}");
    }
}

/// Removes each banned widget that is installed. A failure goes to the log, and the next
/// marketplace read tries again.
async fn remove_banned<R: Runtime>(app: &AppHandle<R>, banned: &[Ban]) -> Vec<Removed> {
    let mut removed = Vec::new();
    for ban in banned {
        match remove_one(app, ban).await {
            Ok(Some(widget)) => removed.push(widget),
            Ok(None) => {}
            Err(error) => eprintln!("can't remove the banned widget \"{}\": {error}", ban.id),
        }
    }
    let ids: Vec<String> = removed.iter().map(|widget| widget.id.clone()).collect();
    if !ids.is_empty() {
        changed(app, &ids);
    }
    removed
}

/// Removes banned widget `ban.id` if it is installed. Gives the widget to show to the user, or
/// `None` if nothing ran.
async fn remove_one<R: Runtime>(app: &AppHandle<R>, ban: &Ban) -> Result<Option<Removed>, String> {
    let dir = marketplace_dir(app).map_err(|e| e.to_string())?.join(&ban.id);
    if !dir.is_dir() {
        return Ok(None);
    }
    let name = read_manifest(&dir)
        .ok()
        .and_then(|manifest| manifest["name"].as_str().map(str::to_owned))
        .unwrap_or_else(|| ban.id.clone());
    remove(app, &ban.id).await?;
    // A widget of the user with the same id hid this copy, so it never ran. The placements and
    // the data belong too the user's widget.
    if user_dir(app).map_err(|e| e.to_string())?.join(&ban.id).exists() {
        return Ok(None);
    }
    forget(app, &ban.id)?;
    Ok(Some(Removed { id: ban.id.clone(), name, reason: ban.reason.clone() }))
}

/// Removes the placements of widget `id` from the pages, with their settings, and the data that
/// the widget saved. The stores tell the windows, so the Edge drops the placements at once.
fn forget<R: Runtime>(app: &AppHandle<R>, id: &str) -> Result<(), String> {
    let mut prefixes = vec![format!("widget/{id}/")];
    let settings = app.store("settings.json").map_err(|e| e.to_string())?;
    if let Some(mut pages) = settings.get("pages") {
        let instances = drop_placements(&mut pages, id);
        if !instances.is_empty() {
            settings.set("pages", pages);
            prefixes.extend(instances.iter().map(|instance| format!("instance/{instance}/")));
        }
    }
    let data = app.store("widget-data.json").map_err(|e| e.to_string())?;
    for key in data.keys() {
        if prefixes.iter().any(|prefix| key.starts_with(prefix)) {
            data.delete(key);
        }
    }
    Ok(())
}

/// Removes the placements of widget `id` from `pages`, and gives their instance ids.
fn drop_placements(pages: &mut Value, id: &str) -> Vec<String> {
    let mut instances = Vec::new();
    for page in pages.as_array_mut().into_iter().flatten() {
        let Some(items) = page.get_mut("items").and_then(Value::as_array_mut) else { continue };
        items.retain(|item| {
            let theirs = item["widget"] == id;
            if theirs {
                instances.extend(item["id"].as_str().map(str::to_owned));
            }
            !theirs
        });
    }
    instances
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn drops_only_the_placements_of_the_widget() {
        let mut pages = json!([
            { "id": "p1", "items": [
                { "id": "a", "widget": "rogue" },
                { "id": "b", "widget": "clock", "settings": { "zone": "UTC" } }
            ] },
            { "id": "p2", "items": [{ "id": "c", "widget": "rogue", "settings": { "token": "x" } }] },
            { "id": "p3" }
        ]);
        assert_eq!(drop_placements(&mut pages, "rogue"), ["a", "c"]);
        assert_eq!(
            pages,
            json!([
                { "id": "p1", "items": [
                    { "id": "b", "widget": "clock", "settings": { "zone": "UTC" } }
                ] },
                { "id": "p2", "items": [] },
                { "id": "p3" }
            ])
        );
    }

    #[test]
    fn leaves_pages_without_the_widget_alone() {
        let mut pages = json!([{ "id": "p1", "items": [{ "id": "a", "widget": "clock" }] }]);
        let before = pages.clone();
        assert!(drop_placements(&mut pages, "rogue").is_empty());
        assert_eq!(pages, before);
    }
}
