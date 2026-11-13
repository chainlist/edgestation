use std::sync::mpsc;

use tauri::{Runtime, Webview};

use crate::widgets::webview::receive;
use crate::widgets::webview::user_agent::UserAgent;

/// Makes `webview` show itself as `user_agent` to each page that it loads. The user agent
/// setting of WebView2 (from the builder) changes only the user agent string. The client hints
/// still say desktop Windows, and Google sites beleive the hints more than the string. The
/// DevTools override changes both.
pub async fn present_as<R: Runtime>(
    webview: &Webview<R>,
    user_agent: &UserAgent,
) -> Result<(), String> {
    use webview2_com::CallDevToolsProtocolMethodCompletedHandler;
    use windows::core::HSTRING;

    let params = serde_json::to_string(user_agent).map_err(|e| e.to_string())?;
    let (done_tx, done) = mpsc::channel();
    webview
        .with_webview(move |platform| {
            let failed = done_tx.clone();
            let handler =
                CallDevToolsProtocolMethodCompletedHandler::create(Box::new(move |result, _| {
                    let _ = done_tx.send(result.map_err(|e| e.to_string()));
                    Ok(())
                }));
            // SAFETY: WebView2 calls on the thread that runs its events, while the view is alive.
            let called = unsafe {
                platform.controller().CoreWebView2().and_then(|core| {
                    core.CallDevToolsProtocolMethod(
                        &HSTRING::from("Emulation.setUserAgentOverride"),
                        &HSTRING::from(params),
                        &handler,
                    )
                })
            };
            if let Err(e) = called {
                let _ = failed.send(Err(e.to_string()));
            }
        })
        .map_err(|e| e.to_string())?;
    receive(done).await?
}
