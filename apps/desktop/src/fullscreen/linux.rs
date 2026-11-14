//! The Linux side, on X11. A thread listens to the window manager: the root window reports the
//! active window, and the active window reports its state, moves, resizes and hides. The thread
//! has its own connection, and it ends when the watch stops.

use std::sync::{Arc, Mutex, PoisonError};
use std::thread;

use tauri::AppHandle;
use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::protocol::xproto::{
    AtomEnum, ChangeWindowAttributesAux, ClientMessageEvent, ConnectionExt as _, CreateWindowAux,
    EventMask, MapState, Window, WindowClass,
};

use crate::os::linux::x11::{Monitor, X11, error};

/// The connection of the thread, and its own window. `stop` sends an event to that window,
/// which wakes the thread so it can end.
static RUNNING: Mutex<Option<(Arc<X11>, Window)>> = Mutex::new(None);

pub fn set(app: AppHandle, enabled: bool) -> Result<(), String> {
    stop();
    if enabled { start(app) } else { Ok(()) }
}

fn start(app: AppHandle) -> Result<(), String> {
    let x11 = Arc::new(X11::connect()?);
    let conn = &x11.conn;
    let wake = conn.generate_id().map_err(error)?;
    let only_input = CreateWindowAux::new();
    conn.create_window(0, wake, x11.root, -1, -1, 1, 1, 0, WindowClass::INPUT_ONLY, 0, &only_input)
        .map_err(error)?;
    let root_events = ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE);
    conn.change_window_attributes(x11.root, &root_events).map_err(error)?;
    conn.flush().map_err(error)?;
    *RUNNING.lock().unwrap_or_else(PoisonError::into_inner) = Some((Arc::clone(&x11), wake));
    thread::Builder::new()
        .name("fullscreen watch".into())
        .spawn(move || listen(&x11, wake, &app))
        .map(drop)
        .map_err(|error| error.to_string())
}

fn stop() {
    if let Some((x11, wake)) = RUNNING.lock().unwrap_or_else(PoisonError::into_inner).take() {
        let event = ClientMessageEvent::new(32, wake, AtomEnum::NONE, [0u32; 5]);
        let _ = x11.conn.send_event(false, wake, EventMask::NO_EVENT, event);
        let _ = x11.conn.flush();
    }
}

/// The thread. It ends when `wake` gets an event or the connection fails.
fn listen(x11: &X11, wake: Window, app: &AppHandle) {
    let mut watch = Watch { x11, app, target: None, sent: None };
    watch.focus(x11.active());
    while let Ok(event) = x11.conn.wait_for_event() {
        match event {
            Event::ClientMessage(event) if event.window == wake => break,
            Event::PropertyNotify(event)
                if event.window == x11.root && event.atom == x11.atoms._NET_ACTIVE_WINDOW =>
            {
                watch.focus(x11.active());
            }
            Event::PropertyNotify(event) if Some(event.window) == watch.target => {
                if event.atom == x11.atoms._NET_WM_STATE {
                    watch.check();
                }
            }
            Event::ConfigureNotify(event) if Some(event.window) == watch.target => watch.check(),
            Event::MapNotify(event) if Some(event.window) == watch.target => watch.check(),
            Event::UnmapNotify(event) if Some(event.window) == watch.target => watch.check(),
            Event::DestroyNotify(event) if Some(event.window) == watch.target => {
                watch.target = None;
                watch.check();
            }
            _ => {}
        }
    }
    let _ = x11.conn.destroy_window(wake);
    let _ = x11.conn.flush();
}

struct Watch<'a> {
    x11: &'a X11,
    app: &'a AppHandle,
    /// The last window of another app that had the focus. It stays when a window of this
    /// app gets the focus.
    target: Option<Window>,
    /// The last state sent to the Edge window.
    sent: Option<bool>,
}

impl Watch<'_> {
    /// `window` got the focus. It becomes the target, unless it is a window of this app.
    fn focus(&mut self, window: Option<Window>) {
        match window {
            Some(window) if self.x11.is_own(window) => {}
            Some(window) => {
                if self.target != Some(window) {
                    self.follow(window);
                }
            }
            // The desktop has the focus.
            None => self.target = None,
        }
        self.check();
    }

    /// Makes `window` the target, and listens to its changes.
    fn follow(&mut self, window: Window) {
        let none = ChangeWindowAttributesAux::new().event_mask(EventMask::NO_EVENT);
        if let Some(old) = self.target.replace(window) {
            let _ = self.x11.conn.change_window_attributes(old, &none);
        }
        let mask = EventMask::STRUCTURE_NOTIFY | EventMask::PROPERTY_CHANGE;
        let events = ChangeWindowAttributesAux::new().event_mask(mask);
        let _ = self.x11.conn.change_window_attributes(window, &events);
        let _ = self.x11.conn.flush();
    }

    /// Tells the Edge window wether the target covers its monitor. It sends only a changed
    /// state.
    fn check(&mut self) {
        let fullscreen = self.target.is_some_and(|window| self.is_fullscreen(window));
        if self.sent == Some(fullscreen) {
            return;
        }
        self.sent = Some(fullscreen);
        super::send(self.app, fullscreen);
    }

    fn is_fullscreen(&self, window: Window) -> bool {
        let (x11, atoms) = (self.x11, &self.x11.atoms);
        // X reuses window ids. The target can be a window that closed, whose id is now a
        // window of this app.
        if x11.is_own(window) {
            return false;
        }
        let shown = x11.conn.get_window_attributes(window).ok().and_then(|c| c.reply().ok());
        if shown.is_none_or(|attributes| attributes.map_state != MapState::VIEWABLE) {
            return false;
        }
        let state = x11.atoms_of(window, atoms._NET_WM_STATE);
        let kinds = x11.atoms_of(window, atoms._NET_WM_WINDOW_TYPE);
        let has = |list: &[u32], atom| list.contains(&atom);
        let window_state = State {
            fullscreen: has(&state, atoms._NET_WM_STATE_FULLSCREEN),
            maximized: has(&state, atoms._NET_WM_STATE_MAXIMIZED_HORZ)
                && has(&state, atoms._NET_WM_STATE_MAXIMIZED_VERT),
            hidden: has(&state, atoms._NET_WM_STATE_HIDDEN),
            desktop: has(&kinds, atoms._NET_WM_WINDOW_TYPE_DESKTOP)
                || has(&kinds, atoms._NET_WM_WINDOW_TYPE_DOCK),
        };
        let Some(bounds) = self.bounds(window) else { return false };
        covers_monitor(bounds, &window_state, &x11.monitors())
    }

    /// The place of `window` on the screen: x, y, width and height.
    fn bounds(&self, window: Window) -> Option<(i32, i32, i32, i32)> {
        let conn = &self.x11.conn;
        let size = conn.get_geometry(window).ok()?.reply().ok()?;
        let origin = conn.translate_coordinates(window, self.x11.root, 0, 0).ok()?.reply().ok()?;
        let (x, y) = (origin.dst_x.into(), origin.dst_y.into());
        Some((x, y, size.width.into(), size.height.into()))
    }
}

/// What the window manager says about a window.
struct State {
    fullscreen: bool,
    maximized: bool,
    /// Minimized.
    hidden: bool,
    /// The desktop or a panel.
    desktop: bool,
}

/// True if the window at `bounds` covers the full monitor that holds its center. The desktop
/// and the panels never count. A maximized window that covers a monitor, for example a window
/// that draws its own title bar on a monitor without panel, does not count either.
fn covers_monitor(bounds: (i32, i32, i32, i32), state: &State, monitors: &[Monitor]) -> bool {
    if state.hidden || state.desktop || (state.maximized && !state.fullscreen) {
        return false;
    }
    let (x, y, width, height) = bounds;
    let (center_x, center_y) = (x + width / 2, y + height / 2);
    monitors
        .iter()
        .find(|m| {
            (m.x..m.x + m.width).contains(&center_x) && (m.y..m.y + m.height).contains(&center_y)
        })
        .is_some_and(|m| {
            x <= m.x && y <= m.y && x + width >= m.x + m.width && y + height >= m.y + m.height
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn monitors() -> Vec<Monitor> {
        let monitor =
            |name: &str, x, width, height| Monitor { name: name.into(), x, y: 0, width, height };
        vec![monitor("DP-1", 0, 2560, 1440), monitor("DP-2", 2560, 2560, 720)]
    }

    const NORMAL: State =
        State { fullscreen: false, maximized: false, hidden: false, desktop: false };

    #[test]
    fn a_window_that_covers_its_monitor_is_full_screen() {
        assert!(covers_monitor((0, 0, 2560, 1440), &NORMAL, &monitors()));
        assert!(covers_monitor((2560, 0, 2560, 720), &NORMAL, &monitors()));
        let fullscreen = State { fullscreen: true, ..NORMAL };
        assert!(covers_monitor((0, 0, 2560, 1440), &fullscreen, &monitors()));
    }

    #[test]
    fn a_smaller_window_is_not() {
        assert!(!covers_monitor((0, 40, 2560, 1400), &NORMAL, &monitors()));
        // The state says full screen, but the window manager did not resize it yet.
        let fullscreen = State { fullscreen: true, ..NORMAL };
        assert!(!covers_monitor((100, 100, 800, 600), &fullscreen, &monitors()));
    }

    #[test]
    fn desktops_panels_minimized_and_maximized_windows_are_not() {
        let bounds = (0, 0, 2560, 1440);
        for state in [
            State { maximized: true, ..NORMAL },
            State { hidden: true, fullscreen: true, ..NORMAL },
            State { desktop: true, ..NORMAL },
        ] {
            assert!(!covers_monitor(bounds, &state, &monitors()));
        }
        assert!(!covers_monitor(bounds, &NORMAL, &[]));
    }
}
