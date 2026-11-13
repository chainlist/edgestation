//! Puts a view at its bounds in the window. Tauri packs a child web view in the box of its
//! window, below the page, and `set_bounds` does nothing there. So at its first placement, a view
//! moves into a GTK window of its own, whose X window is a child of the window of the page, as
//! wry does for X11 child web views. The page and the views then do not overlap inside one GTK
//! window, and the page never moves. That X window moves to the bounds of the view. It stays
//! above the page and inside it, and hides with it. A view moves before it loads its first page.

use std::sync::OnceLock;

use gdkx11::{X11Display, X11Window};
use gtk::gdk;
use gtk::prelude::*;
use tauri::{Rect, Runtime, Webview};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt as _, CreateWindowAux, InputFocus, WindowClass};
use x11rb::wrapper::ConnectionExt as _;

use crate::os::linux::x11::X11;

/// The key of the window that holds a view.
const HOLDER: &str = "edgestation-holder";
/// X11 keeps window positions in 16 bits, so a view outside the window stays in this range.
const FAR: f64 = 30_000.0;

/// The connection that makes the X windows of the views. It stays open, because the X
/// server destroys the windows of a connection that closes.
fn x11() -> Option<&'static X11> {
    static X11: OnceLock<Option<X11>> = OnceLock::new();
    X11.get_or_init(|| X11::connect().ok()).as_ref()
}

/// Cuts the X window that holds `view` to `rectangles`, for what it draws and for the input
/// that it gets: x, y, width and height in pixels. `None` removes the cut.
pub fn cut(view: &gtk::Widget, rectangles: Option<&[(i32, i32, i32, i32)]>) {
    use x11rb::protocol::shape::{ConnectionExt as _, SK, SO};
    use x11rb::protocol::xproto::ClipOrdering;
    use x11rb::protocol::xproto::Rectangle;

    let (Some(xid), Some(x11)) = (holder_xid(view), x11()) else { return };
    for kind in [SK::BOUNDING, SK::INPUT] {
        let _ = match rectangles {
            Some(rectangles) => {
                let rectangles: Vec<_> = rectangles
                    .iter()
                    .map(|&(x, y, width, height)| Rectangle {
                        x: x as i16,
                        y: y as i16,
                        width: width as u16,
                        height: height as u16,
                    })
                    .collect();
                let unsorted = ClipOrdering::UNSORTED;
                x11.conn.shape_rectangles(SO::SET, kind, unsorted, xid, 0, 0, &rectangles)
            }
            None => x11.conn.shape_mask(SO::SET, kind, xid, 0, 0, x11rb::NONE),
        };
    }
    let _ = x11.conn.flush();
}

/// The X window that holds `view`.
fn holder_xid(view: &gtk::Widget) -> Option<u32> {
    let window = holder_of(view)?.window()?.downcast::<X11Window>().ok()?;
    Some(window.xid() as u32)
}

/// The GTK window that holds `view`, if the view moved into one.
fn holder_of(view: &gtk::Widget) -> Option<gtk::Window> {
    // SAFETY: only this module reads and writes this key, always as a `gtk::Window`.
    unsafe { view.data::<gtk::Window>(HOLDER).map(|holder| holder.as_ref().clone()) }
}

pub fn set_bounds<R: Runtime>(webview: &Webview<R>, rect: Rect) -> tauri::Result<()> {
    let scale = webview.window().scale_factor()?;
    let position = rect.position.to_logical::<f64>(scale);
    let size = rect.size.to_logical::<f64>(scale);
    let at = |value: f64| value.round().clamp(-FAR, FAR) as i32;
    let (x, y) = (at(position.x), at(position.y));
    let (width, height) = (size.width.round() as i32, size.height.round() as i32);
    webview.with_webview(move |platform| {
        let view: gtk::Widget = platform.inner().upcast();
        let Some(holder) = holder_of(&view).or_else(|| adopt(&view)) else { return };
        holder.move_(x, y);
        if let Some(window) = holder.window() {
            window.resize(width, height);
        }
        holder.size_allocate(&gtk::Allocation::new(0, 0, width, height));
    })
}

/// Gives the keyboard to `webview`, which takes the typed text from now on. A view is in an
/// X window of its own, so the focus of the view is not enough. The X window of its holder
/// gets the input focus, inside the window of the page.
pub fn take_keyboard<R: Runtime>(webview: &Webview<R>) -> tauri::Result<()> {
    webview.with_webview(|platform| {
        let view: gtk::Widget = platform.inner().upcast();
        let (Some(xid), Some(x11)) = (holder_xid(&view), x11()) else { return };
        let _ = x11.conn.set_input_focus(InputFocus::PARENT, xid, x11rb::CURRENT_TIME);
        let _ = x11.conn.flush();
    })
}

/// Moves `view` from the box of its window into a holder: a GTK window on an X window
/// inside the window of the page. Returns the holder.
fn adopt(view: &gtk::Widget) -> Option<gtk::Window> {
    let page = view.toplevel()?.window()?;
    let display = page.display().downcast::<X11Display>().ok()?;
    let page = page.downcast::<X11Window>().ok()?;
    let x11 = x11()?;
    let xid = x11.conn.generate_id().ok()?;
    let no_attributes = CreateWindowAux::new();
    let class = WindowClass::INPUT_OUTPUT;
    let parent = page.xid() as u32;
    x11.conn.create_window(0, xid, parent, 0, 0, 1, 1, 0, class, 0, &no_attributes).ok()?;
    // GDK reads the window from the server, so it must exist there first.
    x11.conn.sync().ok()?;
    let held: gdk::Window = X11Window::foreign_new_for_display(&display, xid.into()).upcast();

    let holder = gtk::Window::new(gtk::WindowType::Toplevel);
    holder.connect_realize(move |holder| holder.set_window(held.clone()));
    holder.set_has_window(true);
    holder.realize();
    let shown = view.is_visible();
    if let Some(parent) = view.parent().and_then(|p| p.downcast::<gtk::Container>().ok()) {
        parent.remove(view);
    }
    holder.add(view);
    view.set_visible(shown);
    holder.set_visible(shown);
    // SAFETY: as in `holder_of`, the key always holds a `gtk::Window`.
    unsafe { view.set_data(HOLDER, holder.clone()) };

    // The holder shows and hides with the view, and goes with it.
    let on_show = holder.clone();
    view.connect_show(move |_| on_show.show());
    let on_hide = holder.clone();
    view.connect_hide(move |_| on_hide.hide());
    let on_destroy = holder.downgrade();
    view.connect_destroy(move |_| {
        if let Some(holder) = on_destroy.upgrade() {
            // SAFETY: the holder belongs to this module, and nothing uses it after.
            unsafe { holder.destroy() };
        }
        let _ = x11.conn.destroy_window(xid);
        let _ = x11.conn.flush();
    });
    Some(holder)
}
