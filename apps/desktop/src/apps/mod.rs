//! The apps of the app menu for launcher widgets. On Windows, they come from the All apps list
//! of the Start menu, which is the Apps folder of the shell. On Linux, they are the desktop
//! entries that the app menu shows. The host lists them, starts them and draws their icons only
//! when a widget asks.

/// `pub(crate)` for the Linux side of `process`, which names programs after their entry in the
/// app menu.
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
pub(crate) mod native;

use percent_encoding::percent_decode_str;
use serde::Serialize;
use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Runtime, UriSchemeContext, UriSchemeResponder, Window};

use crate::blocking;

/// The icon sizes that the protocol draws, in pixels.
const ICON_SIZES: std::ops::RangeInclusive<u32> = 16..=256;

/// Draws an icon as PNG: the icon of an app id or of an executable path, and its size.
type Draw = fn(&str, u32) -> Option<Vec<u8>>;

#[derive(Serialize)]
pub struct App {
    /// On Windows, its name in the Apps folder. This is the AppUserModelID of the app. A program
    /// without one has a path form a known folder. On Linux, the id of its desktop entry, for
    /// example `firefox.desktop`.
    id: String,
    /// The name that the Start menu or the app menu shows.
    name: String,
}

#[tauri::command]
pub async fn apps_list() -> Result<Vec<App>, String> {
    blocking::run(list).await
}

/// Starts app `id` as the Start menu does, and asks for its window on a screen other than the
/// screen of the caller (the Edge). Only Windows takes this hint. Fails if the app is gone.
///
/// Takes a `Window`, not a `WebviewWindow`. When a widget opens a child web view
/// (`api.webview`), the Edge window is no longer a `WebviewWindow`, and Tauri refuses the call.
#[tauri::command]
pub async fn apps_launch<R: Runtime>(window: Window<R>, id: String) -> Result<(), String> {
    let away = away_from(&window);
    blocking::run(move || launch(&id, away)).await
}

/// Serves app icons as PNG at `appicon://localhost/<size>/<id>`, and at
/// `http://appicon.localhost/...` on Windows. Widgets can show them with a plain `<img>`, and the
/// web view loads them only when it needs them. The icons of executables are at
/// `appicon://localhost/file/<size>/<path>`, for `api.audio`.
pub fn icon_protocol<R: Runtime>(
    _: UriSchemeContext<'_, R>,
    request: Request<Vec<u8>>,
    responder: UriSchemeResponder,
) {
    let path = request.uri().path().to_owned();
    // To draw an icon, the shell reads the files of the app. Do it off the main thread.
    tauri::async_runtime::spawn_blocking(move || responder.respond(icon_response(&path)));
}

fn icon_response(path: &str) -> Response<Vec<u8>> {
    let path = path.trim_start_matches('/');
    let (draw, path): (Draw, _) = match path.strip_prefix("file/") {
        Some(path) => (executable_icon, path),
        None => (icon, path),
    };
    let png = path.split_once('/').and_then(|(size, id)| {
        let size = size.parse::<u32>().ok()?.clamp(*ICON_SIZES.start(), *ICON_SIZES.end());
        draw(&percent_decode_str(id).decode_utf8().ok()?, size)
    });
    let response = Response::builder().header(header::CACHE_CONTROL, "max-age=3600");
    match png {
        Some(png) => response.header(header::CONTENT_TYPE, "image/png").body(png),
        None => response.status(StatusCode::NOT_FOUND).body(Vec::new()),
    }
    .unwrap_or_default()
}

use native::{away_from, executable_icon, icon, launch, list};

#[cfg(test)]
mod tests {
    use tauri::http::StatusCode;

    use super::icon_response;

    #[test]
    fn answers_not_found_for_bad_paths() {
        for path in ["/big/x", "/48", "/"] {
            assert_eq!(icon_response(path).status(), StatusCode::NOT_FOUND, "{path}");
        }
    }
}
