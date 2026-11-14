#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

use std::sync::atomic::{AtomicBool, Ordering};

use tauri::window::Color;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_store::StoreExt;

use crate::identify;

const SETTINGS: &str = "settings";

/// Returns the page background for the saved theme. This is `--background` of layout.css.
/// A window shows its own color until its page is drawn, and that color is white by default.
/// After that, the page keeps it in step with the theme (`setWindowBackground`).
pub fn page_background<R: Runtime>(app: &AppHandle<R>) -> Color {
    let light = app
        .store("settings.json")
        .ok()
        .and_then(|store| store.get("theme"))
        .is_some_and(|theme| theme["mode"] == "light");
    if light {
        Color(255, 255, 255, 255)
    } else {
        Color(10, 10, 10, 255)
    }
}

/// Opens the settings, as `open_settings` does, and prints the error if that fails. The tray and
/// a second launch use this, because they have no caller to tell.
pub fn show_settings<R: Runtime>(app: &AppHandle<R>) {
    if let Err(error) = open_settings(app) {
        eprintln!("failed to open settings: {error}");
    }
}

/// Gives the focus to the settings window, and creates the window if necessary. When the user
/// closes it, Tauri destroys it, so it uses no memory. Only the Edge window stays for the whole
/// session.
pub fn open_settings<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(SETTINGS) {
        window.unminimize()?;
        return window.set_focus();
    }
    // The page draws its own title bar (TitleBar.svelte), the same on Windows and Linux, so the
    // window has no frame. It can still resize from its edges, and on Windows it keeps its shadow
    // and round corners.
    let window = WebviewWindowBuilder::new(app, SETTINGS, WebviewUrl::App("settings".into()))
        .title("EdgeStation Settings")
        .inner_size(1100.0, 720.0)
        .min_inner_size(760.0, 520.0)
        .decorations(false)
        .background_color(page_background(app))
        .visible(false)
        .build()?;
    // The page does not clean up when the window closes. The identify boxes that the Display
    // page made close here with the window.
    let handle = app.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            let handle = handle.clone();
            // `identify::release` must not block the main thread.
            tauri::async_runtime::spawn_blocking(move || identify::release(&handle));
        }
    });
    // By default, Windows opens a new window next to the other window of the app, wich is on the
    // Edge. Move it to the main screen first, then center it there. Some Linux window managers
    // place a window only when it maps, so there it moves again then.
    if let Some(monitor) = window.primary_monitor()? {
        window.set_position(*monitor.position())?;
        window.center()?;
        let (moved, at) = (window.clone(), *monitor.position());
        place_again_on_map(&window, move || {
            let _ = moved.set_position(at);
            let _ = moved.center();
        })?;
    }
    window.show()?;
    window.set_focus()
}

/// True when the Hide or show shortcut hid the Edge. The app keeps this value here and does not
/// read it from Windows. The page shows the window when it loads, but Tauri queues `show` only
/// when the call returns. Right after, Windows can still report the window as hidden. A page
/// reload finds the value as it was.
static HIDDEN: AtomicBool = AtomicBool::new(false);

/// Shows or hides the calling window (the Edge) if `visible` is set. Returns true if the window
/// shows, as the shortcut left it.
#[tauri::command]
pub fn edge_visible<R: Runtime>(
    window: tauri::Window<R>,
    visible: Option<bool>,
) -> tauri::Result<bool> {
    if let Some(visible) = visible {
        set_visible(&window, visible)?;
        HIDDEN.store(!visible, Ordering::Relaxed);
    }
    Ok(!HIDDEN.load(Ordering::Relaxed))
}

/// Shows or hides `window` and leaves the focus where it is. On Windows, Tauri `show` activates
/// the window and takes the focus from the app in front, so the Windows side calls Windows
/// directly.
pub use native::set_visible;

/// Runs `place` again when `window` maps, for window managers that place a window themselves
/// then. Call it after placing a window that does not show yet.
pub use native::place_again_on_map;

/// Puts the calling window (the Edge) full screen on the monitor at `x`, `y`, in pixels. On
/// Linux, the window also shows, because window managers ignore the position and the full screen
/// of a hidden window.
#[tauri::command]
pub fn edge_fullscreen<R: Runtime>(window: tauri::Window<R>, x: i32, y: i32) -> tauri::Result<()> {
    native::fullscreen_on(&window, x, y)
}
