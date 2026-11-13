use tauri::ipc::Channel;
use tauri::{Runtime, Webview, Window};

use crate::widgets::webview::open::PageLoad;

/// As on Windows: each load once when it starts and once when it ends, with or without success.
/// WebKit reports a redirect as part of the same load. A load that fails also ends.
pub fn report_loads<R: Runtime>(
    webview: &Webview<R>,
    on_load: Channel<PageLoad>,
) -> tauri::Result<()> {
    use std::cell::Cell;
    use webkit2gtk::{LoadEvent, WebViewExt};

    webview.with_webview(move |platform| {
        // WebKit gives loads no id, so they are counted.
        let id = Cell::new(0);
        platform.inner().connect_load_changed(move |view, event| match event {
            LoadEvent::Started => {
                id.set(id.get() + 1);
                if view.uri().is_none_or(|uri| uri != "about:blank") {
                    let _ = on_load.send(PageLoad { id: id.get(), done: false });
                }
            }
            LoadEvent::Finished => {
                let _ = on_load.send(PageLoad { id: id.get(), done: true });
            }
            _ => {}
        });
    })
}

/// As on Windows: the address of the page each time it changes, also inside a page that loads
/// nothing new.
pub fn report_sources<R: Runtime>(
    webview: &Webview<R>,
    on_navigate: Channel<String>,
) -> tauri::Result<()> {
    use webkit2gtk::WebViewExt;

    webview.with_webview(move |platform| {
        platform.inner().connect_uri_notify(move |view| {
            if let Some(uri) = view.uri() {
                let _ = on_navigate.send(uri.into());
            }
        });
    })
}

/// As on Windows: WebKit makes the window full screen with the page, and takes it out with the
/// page. This puts the window back in full screen if it was full screen before.
pub fn keep_fullscreen<R: Runtime>(
    webview: &Webview<R>,
    window: Window<R>,
    on_fullscreen: Channel<bool>,
) -> tauri::Result<()> {
    use std::cell::Cell;
    use std::rc::Rc;
    use webkit2gtk::WebViewExt;

    webview.with_webview(move |platform| {
        let view = platform.inner();
        let was_fullscreen = Rc::new(Cell::new(false));
        let (entered, on_enter, own) =
            (Rc::clone(&was_fullscreen), on_fullscreen.clone(), window.clone());
        view.connect_enter_fullscreen(move |_| {
            let _ = on_enter.send(true);
            entered.set(own.is_fullscreen().unwrap_or(false));
            false
        });
        view.connect_leave_fullscreen(move |_| {
            let _ = on_fullscreen.send(false);
            if was_fullscreen.get() {
                // After WebKit takes the window out of full screen.
                let window = window.clone();
                tauri::async_runtime::spawn(async move { window.set_fullscreen(true) });
            }
            false
        });
    })
}
