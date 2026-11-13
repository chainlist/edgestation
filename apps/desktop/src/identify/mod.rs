//! "Identify screens" shows a black box in the middle of each monitor. The box shows the name
//! and size that the display settings use for that monitor. Each box is a seperate window. It
//! stays on top, lets clicks through and never takes the focus, so the settings window keeps it.
//! Windows needs about half a second to start a new web view. Thus the app makes the boxes,
//! hidden and drawn, while the Display settings are open, and a click only shows them.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use tauri::{
    AppHandle, LogicalSize, Manager, Monitor, PhysicalSize, Runtime, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};

use crate::app_windows::set_visible;

const LABEL: &str = "identify-";
/// The size of the box, in logical pixels.
const SIZE: LogicalSize<f64> = LogicalSize::new(560.0, 220.0);
/// How long the boxes stay on screen.
const SHOWN: Duration = Duration::from_secs(3);

struct IdentifyBox {
    label: String,
    /// The text in the box. A monitor gets a new box when its name or size changes.
    name: String,
    size: PhysicalSize<u32>,
}

// Never lock this on the main thread. Code that holds the lock creates windows, and window
// creation waits for the main thread.
static BOXES: Mutex<Vec<IdentifyBox>> = Mutex::new(Vec::new());
/// The app never uses a label again. A window that was just destroyed can still hold it's label.
static NEXT: AtomicU32 = AtomicU32::new(0);
/// Counts the clicks. The boxes hide only if no later click showed them again.
static SHOWS: AtomicU32 = AtomicU32::new(0);

/// Creates the hidden boxes, so `identify_screens` can show them at once.
// This command is async because sync commands run on the main thread. A window created there
// makes a deadlock.
#[tauri::command]
pub async fn identify_prepare<R: Runtime>(app: AppHandle<R>) -> tauri::Result<()> {
    sync(&app, false)
}

/// Shows a box for a few seconds on each monitor that the display settings list.
#[tauri::command]
pub async fn identify_screens<R: Runtime>(app: AppHandle<R>) -> tauri::Result<()> {
    sync(&app, true)?;
    let shows = SHOWS.fetch_add(1, Ordering::Relaxed) + 1;
    std::thread::spawn(move || {
        std::thread::sleep(SHOWN);
        if SHOWS.load(Ordering::Relaxed) == shows {
            for found in BOXES.lock().unwrap().iter() {
                if let Some(window) = app.get_webview_window(&found.label) {
                    let _ = set_visible(&window.as_ref().window(), false);
                }
            }
        }
    });
    Ok(())
}

/// Destroys the boxes when the Display settings close.
#[tauri::command]
pub async fn identify_release<R: Runtime>(app: AppHandle<R>) {
    release(&app);
}

/// Destroys the boxes. Do not call this on the main thread (see `BOXES`).
pub fn release<R: Runtime>(app: &AppHandle<R>) {
    for found in BOXES.lock().unwrap().drain(..) {
        if let Some(window) = app.get_webview_window(&found.label) {
            let _ = window.destroy();
        }
    }
}

/// Makes one box for each named monitor and centers it there. The settings list only named
/// monitors. If `show` is true, each box shows.
fn sync<R: Runtime>(app: &AppHandle<R>, show: bool) -> tauri::Result<()> {
    let monitors = crate::monitors::named(app)?;
    let mut boxes = BOXES.lock().unwrap();
    // Remove the boxes of monitors that were unpluged or resized, and the boxes without a window.
    boxes.retain(|found| {
        let window = app.get_webview_window(&found.label);
        let current = monitors
            .iter()
            .any(|(name, m)| *name == found.name && *m.size() == found.size);
        if let (false, Some(window)) = (current, &window) {
            let _ = window.destroy();
        }
        current && window.is_some()
    });
    for (name, monitor) in &monitors {
        let index = match boxes.iter().position(|found| found.name == *name) {
            Some(index) => index,
            None => {
                boxes.push(IdentifyBox {
                    label: create(app, name, *monitor.size())?,
                    name: name.clone(),
                    size: *monitor.size(),
                });
                boxes.len() - 1
            }
        };
        let Some(window) = app.get_webview_window(&boxes[index].label) else {
            continue;
        };
        // The user can arrange the monitors again after the box was created.
        place(&window, monitor)?;
        if show {
            // A new box is transparent, so it shows only when its page is drawn.
            set_visible(&window.as_ref().window(), true)?;
            let (moved, monitor) = (window.clone(), monitor.clone());
            native::shown(&window, move || {
                let _ = place(&moved, &monitor);
            })?;
        }
    }
    Ok(())
}

/// Creates a hidden box and returns its label. Its page draws the name and size.
fn create<R: Runtime>(
    app: &AppHandle<R>,
    name: &str,
    size: PhysicalSize<u32>,
) -> tauri::Result<String> {
    let label = format!("{LABEL}{}", NEXT.fetch_add(1, Ordering::Relaxed));
    let name = utf8_percent_encode(name, NON_ALPHANUMERIC);
    let url = format!(
        "identify?name={name}&width={}&height={}",
        size.width, size.height
    );
    let window = WebviewWindowBuilder::new(app, &label, WebviewUrl::App(url.into()))
        .title("Identify screen")
        .inner_size(SIZE.width, SIZE.height)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .resizable(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .focused(false)
        .focusable(false)
        .visible(false)
        .build()?;
    // The box lets clicks through, from here or from when it shows.
    native::created(&window)?;
    Ok(label)
}

/// Moves the box to the monitor, then centers it there, as for the settings window. It sets the
/// size again there, because that monitor can have a diffrent scale.
fn place<R: Runtime>(window: &WebviewWindow<R>, monitor: &Monitor) -> tauri::Result<()> {
    window.set_position(*monitor.position())?;
    window.set_size(SIZE)?;
    window.center()
}
