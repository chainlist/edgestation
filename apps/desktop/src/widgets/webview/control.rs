use tauri::webview::{PageLoadEvent, PageLoadPayload};
use tauri::{AppHandle, Runtime, Webview};

use super::native::{set_bounds, shape};
use super::shape::Bounds;
use super::{PREFIX, view, web_page};

/// Shows view `label` at `bounds`, or hides it if `bounds` is not set.
#[tauri::command]
pub async fn webview_place<R: Runtime>(
    app: AppHandle<R>,
    label: String,
    bounds: Option<Bounds>,
) -> Result<(), String> {
    let Some(webview) = view(&app, &label) else { return Ok(()) };
    match bounds {
        Some(bounds) => set_bounds(&webview, (&bounds).into())
            .and_then(|()| shape(&webview, &bounds))
            .and_then(|()| webview.show()),
        None => webview.hide(),
    }
    .map_err(|e| e.to_string())
}

/// Loads web page `url` in view `label`. The Home button of the widget uses this to go back to
/// the first page.
#[tauri::command]
pub async fn webview_navigate<R: Runtime>(
    app: AppHandle<R>,
    label: String,
    url: String,
) -> Result<(), String> {
    let url = web_page(&url)?;
    let Some(webview) = view(&app, &label) else { return Ok(()) };
    webview.navigate(url).map_err(|e| e.to_string())
}

/// A reload of the Edge page (Reload Edge) drops its widgets, but does not unmount them.
/// Without this, the views that they opened stay on screen, and nothing places or close them.
/// This function closes them when the page starts to load, before its widgets open new views.
pub fn close_on_reload<R: Runtime>(page: &Webview<R>, payload: &PageLoadPayload<'_>) {
    if payload.event() != PageLoadEvent::Started || page.label().starts_with(PREFIX) {
        return;
    }
    for view in page.window().webviews() {
        if view.label().starts_with(PREFIX) {
            // Close it after this event returns, not during the navigation of another web view.
            tauri::async_runtime::spawn(async move { view.close() });
        }
    }
}

#[tauri::command]
pub async fn webview_close<R: Runtime>(app: AppHandle<R>, label: String) -> Result<(), String> {
    match view(&app, &label) {
        Some(webview) => webview.close().map_err(|e| e.to_string()),
        None => Ok(()),
    }
}
