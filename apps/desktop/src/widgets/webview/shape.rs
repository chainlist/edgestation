use serde::Deserialize;
use tauri::{LogicalPosition, LogicalSize, Rect};

/// A box in CSS pixels, from the top left of the client area of the window. `radius` rounds its
/// corners, and `holes` are cut out of it.
#[derive(Deserialize)]
pub struct Bounds {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
    #[serde(default)]
    pub(super) radius: f64,
    /// The places where the page under the view shows, for example the buttons of the widget
    /// and the Paused badge.
    #[serde(default)]
    pub(super) holes: Vec<Hole>,
}

/// A box in CSS pixels, from the top left of its view. `radius` rounds its corners. If `radius`
/// is half the side, the hole is a cirlce.
#[derive(Deserialize)]
pub(super) struct Hole {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
    pub(super) radius: f64,
}

impl From<&Bounds> for Rect {
    fn from(b: &Bounds) -> Self {
        Rect {
            position: LogicalPosition::new(b.x, b.y).into(),
            size: LogicalSize::new(b.width, b.height).into(),
        }
    }
}
