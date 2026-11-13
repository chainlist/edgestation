//! Native browser views that widgets put over their cell (`api.webview`), for sites that do not
//! load in an iframe. Each view is a child web view of the Edge window. A view in the Default
//! profile shares the browser data of the app. A view in a profile that the user made keeps its
//! data in the folder of that profile, so its sign-ins are separate. Sign-ins stay after a
//! restart in both cases. Remote pages get no capability, so their sites cannot call the
//! commands of the app.

#[cfg_attr(windows, path = "windows/mod.rs")]
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
mod native;

pub(crate) mod capture;
pub(crate) mod control;
pub(crate) mod open;
pub(crate) mod profiles;
mod shape;
mod user_agent;

use std::sync::mpsc;

use tauri::{AppHandle, Manager, Runtime, Url, Webview};

/// The start of each view label. The label is `widget-webview-<n>` in the Default profile, and
/// `widget-webview-<n>_<profile id>` in other profiles.
const PREFIX: &str = "widget-webview-";

/// Gets a view that this module opened. It never returns a web view of the app itself.
pub(crate) fn view<R: Runtime>(app: &AppHandle<R>, label: &str) -> Option<Webview<R>> {
    label.starts_with(PREFIX).then(|| app.get_webview(label)).flatten()
}

/// Gives the focus to view `view`, which takes the typed text from now on.
pub(crate) fn focus<R: Runtime>(view: &Webview<R>) -> tauri::Result<()> {
    view.set_focus()?;
    native::take_keyboard(view)
}

/// Parses `url` as a web page that a view can load. Only `http` and `https` are alowed.
fn web_page(url: &str) -> Result<Url, String> {
    let parsed = Url::parse(url).map_err(|e| format!("{url}: {e}"))?;
    match parsed.scheme() {
        "http" | "https" => Ok(parsed),
        _ => Err(format!("not a web page: {url}")),
    }
}

/// Waits for `rx`, but does not block the async runtime.
async fn receive<T: Send + 'static>(rx: mpsc::Receiver<T>) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(move || rx.recv())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|_| "the browser stopped answering".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn views_only_load_web_pages() {
        assert_eq!(web_page("https://example.com").unwrap().as_str(), "https://example.com/");
        assert!(web_page("http://example.com/a?b#c").is_ok());
        for url in ["file:///C:/a.txt", "javascript:alert(1)", "about:blank", "example.com"] {
            assert!(web_page(url).is_err(), "{url}");
        }
    }
}
