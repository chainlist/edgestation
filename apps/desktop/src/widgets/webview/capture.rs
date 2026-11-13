use tauri::ipc::Response;
use tauri::{AppHandle, Runtime};

use super::{native, view};

/// Takes a JPEG picture of view `label`. The Edge puts it in the place of the view while the
/// view is hidden. Thus the page stays visible while the Edge pages slide.
#[tauri::command]
pub async fn webview_capture<R: Runtime>(
    app: AppHandle<R>,
    label: String,
) -> Result<Response, String> {
    let webview = view(&app, &label).ok_or_else(|| format!("no view {label}"))?;
    native::capture(&webview).await.map(Response::new)
}
