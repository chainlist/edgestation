use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;
use tauri::ipc::Channel;
use tauri::webview::NewWindowResponse;
use tauri::{
    LogicalPosition, Manager, Rect, Runtime, Url, Webview, WebviewBuilder, WebviewUrl, Window,
};

use super::native::{self, keep_fullscreen, present_as, report_loads, report_sources, shape};
use super::profiles::profile_dir;
use super::shape::Bounds;
use super::user_agent::UserAgent;
use super::{PREFIX, web_page};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// The start or the end of a page load in a view. `id` is the same for both.
#[derive(Clone, Serialize)]
pub struct PageLoad {
    pub id: u64,
    pub done: bool,
}

/// Opens web page `url` in a new view of the size of `bounds` in the calling window, and returns
/// its label. The view starts out of the window and goes to `bounds` when the widget places it.
/// Out there it draws its first page, so it comes in painted and not blank. A hidden WebView2
/// draws nothing, and it is white for a moment when it shows. If `user_agent` is set, it
/// replaces the user agent of the browser and its client hints for each page. `profile` is the
/// id of the profile for the view, or `None` for the Default profile. The address of each page
/// in the view goes to `on_navigate`, each page load goes to `on_load`, and each full screen
/// change goes to `on_fullscreen`. The command is async because a sync command that creates a
/// web view deadlocks on Windows.
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn webview_open<R: Runtime>(
    window: Window<R>,
    url: String,
    bounds: Bounds,
    user_agent: Option<UserAgent>,
    profile: Option<String>,
    on_navigate: Channel<String>,
    on_load: Channel<PageLoad>,
    on_fullscreen: Channel<bool>,
) -> Result<String, String> {
    let url = web_page(&url)?;
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let label = match &profile {
        Some(profile) => format!("{PREFIX}{id}_{profile}"),
        None => format!("{PREFIX}{id}"),
    };
    let rect = Rect::from(&bounds);
    // The view starts blank and loads `url` after the setup. Thus the first request also has the
    // user agent, and the first page is reported.
    let blank = Url::parse("about:blank").map_err(|e| e.to_string())?;
    // Without a handler, Tauri refuses evry new window, so new-tab links and sign-in popups do
    // not open. With this handler, WebView2 opens its own popup window. That window uses the
    // profile of the view and stays linked to the page. Sign-in popups need this link.
    let mut builder = WebviewBuilder::new(&label, WebviewUrl::External(blank))
        .focused(false)
        // On Windows, the Tauri drop handler replaces the handler of WebView2. Then sites cannot
        // use HTML5 drag and drop, and files dropped on a page get lost.
        .disable_drag_drop_handler()
        .on_new_window(|_, _| NewWindowResponse::Allow);
    if let Some(user_agent) = &user_agent {
        builder = builder.user_agent(&user_agent.user_agent);
    }
    if let Some(profile) = &profile {
        builder = builder.data_directory(profile_dir(window.app_handle(), profile)?);
    }
    // Far to the left of the window, so that no part of it shows.
    let outside = LogicalPosition::new(-100_000.0, 0.0);
    let webview = window
        .add_child(builder, outside, rect.size)
        .map_err(|e| e.to_string())?;
    // On Linux, Tauri packs the view below the page. There it moves into a window of its own,
    // over the page.
    native::added(&webview, Rect { position: outside.into(), size: rect.size })
        .map_err(|e| e.to_string())?;
    // The widget never gets the label of a view that failed to set up, so close the view here.
    let set = set_up(
        &webview,
        window,
        &bounds,
        user_agent,
        on_navigate,
        on_load,
        on_fullscreen,
        url,
    );
    if let Err(e) = set.await {
        let _ = webview.close();
        return Err(e);
    }
    Ok(label)
}

/// Completes the setup of `webview` for `bounds`. It sends the pages of the view to
/// `on_navigate`, its loads to `on_load` and its full screen changes to `on_fullscreen`. It
/// applies `user_agent` if it is set, then loads `url`.
#[allow(clippy::too_many_arguments)]
async fn set_up<R: Runtime>(
    webview: &Webview<R>,
    window: Window<R>,
    bounds: &Bounds,
    user_agent: Option<UserAgent>,
    on_navigate: Channel<String>,
    on_load: Channel<PageLoad>,
    on_fullscreen: Channel<bool>,
    url: Url,
) -> Result<(), String> {
    shape(webview, bounds).map_err(|e| e.to_string())?;
    keep_fullscreen(webview, window, on_fullscreen).map_err(|e| e.to_string())?;
    report_sources(webview, on_navigate).map_err(|e| e.to_string())?;
    report_loads(webview, on_load).map_err(|e| e.to_string())?;
    if let Some(user_agent) = user_agent {
        present_as(webview, &user_agent).await?;
    }
    webview.navigate(url).map_err(|e| e.to_string())
}
