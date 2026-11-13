use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc;

use tauri::{Runtime, Webview};
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Environment5;

use crate::widgets::webview::receive;

thread_local! {
    /// The browser environments that `release` waits for, by profile id. WebView2
    /// reports the exit only to an environment that the app still holds.
    static WAITING: RefCell<HashMap<String, ICoreWebView2Environment5>> = Default::default();
}

/// Starts to listen for the exit of the browser of `view`. The exit comes when all views of
/// profile `profile` are closed and the browser has released its folder. Returns the receiver
/// that gets the exit.
pub async fn release<R: Runtime>(
    view: &Webview<R>,
    profile: String,
) -> Result<mpsc::Receiver<()>, String> {
    use webview2_com::BrowserProcessExitedEventHandler;
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_2;
    use windows::core::Interface;

    let (exited_tx, exited) = mpsc::channel();
    let (listening_tx, listening) = mpsc::channel();
    view.with_webview(move |platform| {
        let listen = || -> windows::core::Result<()> {
            // SAFETY: WebView2 calls on the thread that runs its events, while the view is
            // alive.
            unsafe {
                let core = platform.controller().CoreWebView2()?;
                let environment = core.cast::<ICoreWebView2_2>()?.Environment()?;
                let environment = environment.cast::<ICoreWebView2Environment5>()?;
                let key = profile.clone();
                let handler = BrowserProcessExitedEventHandler::create(Box::new(move |_, _| {
                    WAITING.with_borrow_mut(|waiting| waiting.remove(&key));
                    let _ = exited_tx.send(());
                    Ok(())
                }));
                let mut token = 0;
                environment.add_BrowserProcessExited(&handler, &mut token)?;
                WAITING.with_borrow_mut(|waiting| waiting.insert(profile, environment));
            }
            Ok(())
        };
        let _ = listening_tx.send(listen().map_err(|e| e.to_string()));
    })
    .map_err(|e| e.to_string())?;
    receive(listening).await??;
    Ok(exited)
}
