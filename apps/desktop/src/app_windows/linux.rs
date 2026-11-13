//! The Linux side: GTK windows on X11.

use gtk::prelude::*;
use tauri::{Runtime, Window};

pub fn set_visible<R: Runtime>(window: &Window<R>, visible: bool) -> tauri::Result<()> {
    if visible { window.show() } else { window.hide() }
}

/// Runs `place` when `window` maps, or at once if it is mapped. Some window managers (Weston)
/// ignore the place of a window that is not mapped yet, and choose one themselves.
pub fn place_again_on_map<R: Runtime>(
    window: &tauri::WebviewWindow<R>,
    place: impl Fn() + Send + 'static,
) -> tauri::Result<()> {
    use std::cell::Cell;
    use std::rc::Rc;

    let own = window.clone();
    window.run_on_main_thread(move || {
        let Ok(gtk_window) = own.gtk_window() else { return };
        if gtk_window.is_mapped() {
            return place();
        }
        let handler = Rc::new(Cell::new(None));
        let once = Rc::clone(&handler);
        let id = gtk_window.connect_map_event(move |gtk_window, _| {
            if let Some(id) = once.take() {
                gtk_window.disconnect(id);
                place();
            }
            gtk::glib::Propagation::Proceed
        });
        handler.set(Some(id));
    })
}

/// Once the window shows, and is not full screen, it moves to the monitor, and GTK asks for
/// full screen on that monitor. Some window managers take only the second, others only the
/// first, and some move only a window that shows and is not full screen. This runs on the
/// main thread, as a sync command, so it can call GTK.
pub fn fullscreen_on<R: Runtime>(window: &Window<R>, x: i32, y: i32) -> tauri::Result<()> {
    use std::cell::Cell;
    use std::rc::Rc;

    use gtk::gdk::WindowState;
    use gtk::glib::Propagation;

    let gtk_window = window.gtk_window()?;
    let display = WidgetExt::display(&gtk_window);
    let at = |index| {
        let monitor = display.monitor(index)?;
        let (area, scale) = (monitor.geometry(), monitor.scale_factor());
        (area.x() * scale == x && area.y() * scale == y).then_some((index, area))
    };
    let Some((index, area)) = (0..display.n_monitors()).find_map(at) else {
        window.show()?;
        return window.set_fullscreen(true);
    };
    let place = move |gtk_window: &gtk::ApplicationWindow| {
        gtk_window.move_(area.x(), area.y());
        if let Some(screen) = GtkWindowExt::screen(gtk_window) {
            gtk_window.fullscreen_on_monitor(&screen, index);
        }
    };
    let handler = Rc::new(Cell::new(None));
    let once = Rc::clone(&handler);
    if !gtk_window.is_mapped() {
        // Window managers place a window when it maps, and can ignore a place asked before.
        let id = gtk_window.connect_map_event(move |gtk_window, _| {
            if let Some(id) = once.take() {
                gtk_window.disconnect(id);
                place(gtk_window);
            }
            Propagation::Proceed
        });
        handler.set(Some(id));
        return window.show();
    }
    let fullscreen = |state: WindowState| state.contains(WindowState::FULLSCREEN);
    if !gtk_window.window().is_some_and(|window| fullscreen(window.state())) {
        place(&gtk_window);
        return Ok(());
    }
    // The window leaves full screen first, then moves.
    let id = gtk_window.connect_window_state_event(move |gtk_window, event| {
        if !fullscreen(event.new_window_state())
            && let Some(id) = once.take()
        {
            gtk_window.disconnect(id);
            place(gtk_window);
        }
        Propagation::Proceed
    });
    handler.set(Some(id));
    gtk_window.unfullscreen();
    Ok(())
}
