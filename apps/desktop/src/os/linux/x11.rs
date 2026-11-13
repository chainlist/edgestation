//! X11 calls for the Linux side: the active window, the windows of this app, and the monitors.
//! The app runs on X11, also under Wayland through XWayland (see `run`).

use x11rb::connection::Connection;
use x11rb::protocol::randr::ConnectionExt as _;
use x11rb::protocol::xproto::{Atom, AtomEnum, ConnectionExt as _, Window};
use x11rb::rust_connection::RustConnection;

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        _NET_ACTIVE_WINDOW,
        _NET_WM_PID,
        _NET_WM_STATE,
        _NET_WM_STATE_FULLSCREEN,
        _NET_WM_STATE_HIDDEN,
        _NET_WM_STATE_MAXIMIZED_HORZ,
        _NET_WM_STATE_MAXIMIZED_VERT,
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_DESKTOP,
        _NET_WM_WINDOW_TYPE_DOCK,
    }
}

/// A monitor, in pixels of the screen.
#[derive(Clone, Debug, PartialEq)]
pub struct Monitor {
    /// The name of its connector, for example `DP-1` or `HDMI-A-1`.
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// A connection to the X server, with its root window and the atoms that the app uses.
pub struct X11 {
    pub conn: RustConnection,
    pub root: Window,
    pub atoms: Atoms,
}

/// Turns any X11 error into text.
pub fn error(error: impl std::fmt::Display) -> String {
    format!("X11: {error}")
}

impl X11 {
    pub fn connect() -> Result<Self, String> {
        let (conn, screen) = x11rb::connect(None).map_err(error)?;
        let root = conn.setup().roots[screen].root;
        let atoms = Atoms::new(&conn).map_err(error)?.reply().map_err(error)?;
        Ok(Self { conn, root, atoms })
    }

    /// The window that has the focus, as the window manager reports it.
    pub fn active(&self) -> Option<Window> {
        let active = self.atoms._NET_ACTIVE_WINDOW;
        self.cardinals(self.root, active, AtomEnum::WINDOW.into())
            .first()
            .copied()
            .filter(|&window| window != x11rb::NONE)
    }

    /// True if `window` belongs to this app.
    pub fn is_own(&self, window: Window) -> bool {
        let pid = self.cardinals(window, self.atoms._NET_WM_PID, AtomEnum::CARDINAL.into());
        pid.first() == Some(&std::process::id())
    }

    /// The atoms in list property `property` of `window`, for example its `_NET_WM_STATE`.
    pub fn atoms_of(&self, window: Window, property: Atom) -> Vec<Atom> {
        self.cardinals(window, property, AtomEnum::ATOM.into())
    }

    /// The 32-bit values of `property` of `window`, or none if it has no such property.
    fn cardinals(&self, window: Window, property: Atom, kind: Atom) -> Vec<u32> {
        let reply = self.conn.get_property(false, window, property, kind, 0, 1024);
        let reply = reply.ok().and_then(|cookie| cookie.reply().ok());
        reply.and_then(|reply| Some(reply.value32()?.collect())).unwrap_or_default()
    }

    /// The monitors, from RandR. None if the server has no RandR 1.5.
    pub fn monitors(&self) -> Vec<Monitor> {
        let Ok(Ok(reply)) = self.conn.randr_get_monitors(self.root, true).map(|c| c.reply()) else {
            return Vec::new();
        };
        reply
            .monitors
            .iter()
            .map(|monitor| {
                let name = self.conn.get_atom_name(monitor.name).ok().and_then(|c| c.reply().ok());
                Monitor {
                    name: name
                        .map(|name| String::from_utf8_lossy(&name.name).into_owned())
                        .unwrap_or_default(),
                    x: monitor.x.into(),
                    y: monitor.y.into(),
                    width: monitor.width.into(),
                    height: monitor.height.into(),
                }
            })
            .collect()
    }
}
