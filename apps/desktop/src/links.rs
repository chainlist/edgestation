//! Web links that open in the default browser, outside the app.

use tauri::{AppHandle, Runtime, Url};
use tauri_plugin_opener::OpenerExt;

/// Returns `url` as the browser gets it, or an error if it is not an `http` or `https` link.
/// Other schemes can start any app, so they never reach the opener.
fn web_link(url: &str) -> Result<Url, String> {
    match Url::parse(url) {
        Ok(link) if matches!(link.scheme(), "http" | "https") => Ok(link),
        _ => Err(format!("Not a web link: {url}")),
    }
}

/// Opens an `http` or `https` link in the default browser. Settings opens the repo of a widget
/// with it, and widgets call it through `api.openUrl`.
#[tauri::command]
pub fn open_link<R: Runtime>(app: AppHandle<R>, url: String) -> Result<(), String> {
    let link = web_link(&url)?;
    crate::focus::let_next_come_forward();
    app.opener().open_url(link.as_str(), None::<&str>).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::web_link;

    #[test]
    fn keeps_web_links() {
        assert_eq!(
            web_link("https://example.com/a b").unwrap().as_str(),
            "https://example.com/a%20b"
        );
        assert!(web_link("HTTP://example.com").is_ok());
    }

    #[test]
    fn refuses_other_schemes() {
        for url in [
            "file:///C:/Windows/System32/calc.exe",
            "ms-settings:",
            "javascript:alert(1)",
            "example.com",
        ] {
            assert!(web_link(url).is_err(), "{url}");
        }
    }
}
