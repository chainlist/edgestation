pub mod dbus;
pub mod x11;

/// Child web views (`api.webview`), global shortcuts and window positions need X11. Under
/// Wayland, GTK then uses XWayland, and `GDK_BACKEND=wayland` can still pick Wayland. Apps that
/// the app starts keep the default of the session, because the environment stays the same.
pub fn prepare() {
    gtk::gdk::set_allowed_backends("x11,*");
}
