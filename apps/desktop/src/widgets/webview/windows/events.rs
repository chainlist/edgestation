use tauri::ipc::Channel;
use tauri::{Runtime, Webview, Window};

use crate::widgets::webview::open::PageLoad;

/// Sends each page load of `webview` to `on_load`, once when it starts and once when it ends,
/// with or without success. A redirect starts the same load again. Links inside a page that load
/// nothing new do not count. The start of the blank first page is not sent, so its end has no
/// match.
pub fn report_loads<R: Runtime>(
    webview: &Webview<R>,
    on_load: Channel<PageLoad>,
) -> tauri::Result<()> {
    use webview2_com::{
        NavigationCompletedEventHandler, NavigationStartingEventHandler, take_pwstr,
    };
    use windows::core::{Error, PWSTR};

    webview.with_webview(move |platform| {
        let on_start = on_load.clone();
        let start = NavigationStartingEventHandler::create(Box::new(move |_, args| {
            let args = args.ok_or_else(Error::empty)?;
            let (mut id, mut uri) = (0, PWSTR::null());
            // SAFETY: WebView2 calls on the thread that runs its events. `Uri` allocates the
            // string for the caller, and `take_pwstr` frees it.
            let uri = unsafe {
                args.NavigationId(&mut id)?;
                args.Uri(&mut uri)?;
                take_pwstr(uri)
            };
            if uri != "about:blank" {
                let _ = on_start.send(PageLoad { id, done: false });
            }
            Ok(())
        }));
        let end = NavigationCompletedEventHandler::create(Box::new(move |_, args| {
            let mut id = 0;
            // SAFETY: a WebView2 call on the thread that runs its events.
            unsafe { args.ok_or_else(Error::empty)?.NavigationId(&mut id)? };
            let _ = on_load.send(PageLoad { id, done: true });
            Ok(())
        }));
        let mut token = 0;
        // SAFETY: the controller is alive while its webview is, and this runs on its thread.
        unsafe {
            if let Ok(core) = platform.controller().CoreWebView2() {
                let _ = core.add_NavigationStarting(&start, &mut token);
                let _ = core.add_NavigationCompleted(&end, &mut token);
            }
        }
    })
}

/// Sends the address of the page in `webview` to `on_navigate` each time it changes. This occurs
/// when a page loads, after its redirects. It also occurs inside a page that loads nothing new,
/// for example the links of single-page sites and `#` anchors.
pub fn report_sources<R: Runtime>(
    webview: &Webview<R>,
    on_navigate: Channel<String>,
) -> tauri::Result<()> {
    use webview2_com::{SourceChangedEventHandler, take_pwstr};
    use windows::core::{Error, PWSTR};

    webview.with_webview(move |platform| {
        let handler = SourceChangedEventHandler::create(Box::new(move |sender, _| {
            let mut source = PWSTR::null();
            // SAFETY: a WebView2 call on the thread that runs its events. It allocates the
            // string for the caller, and `take_pwstr` frees it.
            let source = unsafe {
                sender.ok_or_else(Error::empty)?.Source(&mut source)?;
                take_pwstr(source)
            };
            let _ = on_navigate.send(source);
            Ok(())
        }));
        let mut token = 0;
        // SAFETY: the controller is alive while its webview is, and this runs on its thread.
        unsafe {
            if let Ok(core) = platform.controller().CoreWebView2() {
                let _ = core.add_SourceChanged(&handler, &mut token);
            }
        }
    })
}

/// Tauri applies the full screen of a page to its window. When a video in the view leaves full
/// screen, the whole Edge window leaves it too. This function puts the window back in full
/// screen if it was full screen before the video entered it. It also sends each change to
/// `on_fullscreen`, true when the page enters full screen, so the widget can hide its buttons.
pub fn keep_fullscreen<R: Runtime>(
    webview: &Webview<R>,
    window: Window<R>,
    on_fullscreen: Channel<bool>,
) -> tauri::Result<()> {
    use webview2_com::ContainsFullScreenElementChangedEventHandler;
    use windows::core::{BOOL, Error};

    webview.with_webview(move |platform| {
        let mut was_fullscreen = false;
        let handler =
            ContainsFullScreenElementChangedEventHandler::create(Box::new(move |sender, _| {
                let mut entered = BOOL::default();
                // SAFETY: a WebView2 call on the thread that runs its events.
                unsafe {
                    sender.ok_or_else(Error::empty)?.ContainsFullScreenElement(&mut entered)?
                };
                let _ = on_fullscreen.send(entered.as_bool());
                if entered.as_bool() {
                    // Tauri queued its own change but did not aply it yet. This is the state
                    // before.
                    was_fullscreen = window.is_fullscreen().unwrap_or(false);
                } else if was_fullscreen {
                    // Call it from another thread, so it goes in the queue after the exit of
                    // Tauri. On this thread it runs at once, before that exit.
                    let window = window.clone();
                    tauri::async_runtime::spawn(async move { window.set_fullscreen(true) });
                }
                Ok(())
            }));
        let mut token = 0;
        // SAFETY: the controller is alive while its webview is, and this runs on its thread.
        unsafe {
            if let Ok(core) = platform.controller().CoreWebView2() {
                let _ = core.add_ContainsFullScreenElementChanged(&handler, &mut token);
            }
        }
    })
}
