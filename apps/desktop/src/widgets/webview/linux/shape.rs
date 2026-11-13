use tauri::{Runtime, Webview};

use crate::widgets::webview::shape::{Bounds, Hole};

/// Cuts `webview` as on Windows. The cut applies to the X window that holds the view (see
/// `layer`). The X
/// The X server clips what the view draws and the input that it gets, so the page under the holes
/// gets the touches.
pub fn shape<R: Runtime>(webview: &Webview<R>, bounds: &Bounds) -> tauri::Result<()> {
    use gtk::cairo::{RectangleInt, Region};
    use gtk::prelude::*;

    /// The region from `x1, y1` to `x2, y2`, with corners of radius `r`.
    fn region(x1: i32, y1: i32, x2: i32, y2: i32, r: i32) -> Region {
        let region = Region::create();
        for (x, y, width, height) in rounded(x1, y1, x2, y2, r) {
            let _ = region.union_rectangle(&RectangleInt::new(x, y, width, height));
        }
        region
    }

    let scale = webview.window().scale_factor()?;
    let px = |value: f64| (value * scale).round() as i32;
    let (width, height, radius) = (px(bounds.width), px(bounds.height), px(bounds.radius));
    let hole = |h: &Hole| (px(h.x), px(h.y), px(h.x + h.width), px(h.y + h.height), px(h.radius));
    let holes: Vec<_> = bounds.holes.iter().map(hole).collect();
    let rectangles = (radius > 0 || !holes.is_empty()).then(|| {
        let view = region(0, 0, width, height, radius);
        for &(x1, y1, x2, y2, r) in &holes {
            let _ = view.subtract(&region(x1, y1, x2, y2, r));
        }
        let rectangle = |i| {
            let r: RectangleInt = view.rectangle(i);
            (r.x(), r.y(), r.width(), r.height())
        };
        (0..view.num_rectangles()).map(rectangle).collect::<Vec<_>>()
    });
    webview.with_webview(move |platform| {
        super::layer::cut(platform.inner().upcast_ref(), rectangles.as_deref());
    })
}

/// The box from `x1, y1` to `x2, y2` with corners of radius `r`, as rectangles: x, y, width and
/// height. A region of GDK is made of rectangles, so each round corner is a stair of rows of one
/// pixel. A radius of half the side makes a circle.
fn rounded(x1: i32, y1: i32, x2: i32, y2: i32, r: i32) -> Vec<(i32, i32, i32, i32)> {
    let (width, height) = (x2 - x1, y2 - y1);
    let r = r.min(width / 2).min(height / 2).max(0);
    let mut rows = Vec::new();
    for row in 0..r {
        // The distance from the center of the corner to the middle of the row.
        let dy = f64::from(r - row) - 0.5;
        let inset = (f64::from(r) - (f64::from(r * r) - dy * dy).sqrt()).round() as i32;
        rows.push((x1 + inset, y1 + row, width - 2 * inset, 1));
        rows.push((x1 + inset, y2 - 1 - row, width - 2 * inset, 1));
    }
    if height > 2 * r {
        rows.push((x1, y1 + r, width, height - 2 * r));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::rounded;

    fn area(rects: &[(i32, i32, i32, i32)]) -> i32 {
        rects.iter().map(|&(_, _, width, height)| width * height).sum()
    }

    #[test]
    fn a_box_without_radius_is_one_rectangle() {
        assert_eq!(rounded(10, 20, 110, 70, 0), [(10, 20, 100, 50)]);
    }

    #[test]
    fn round_corners_cut_about_a_quarter_circle_each() {
        let rects = rounded(0, 0, 200, 100, 20);
        // Each corner cuts r² - πr²/4 pixels, about 86 for 20.
        let cut = 200 * 100 - area(&rects);
        assert!((4 * 80..=4 * 92).contains(&cut), "{cut}");
        // The first row is the narrowest, and every row stays in the box.
        assert!(rects.iter().all(|&(x, y, w, h)| x >= 0 && y >= 0 && x + w <= 200 && y + h <= 100));
        assert!(rects[0].0 > 10);
    }

    #[test]
    fn a_radius_of_half_the_side_makes_a_circle() {
        let rects = rounded(0, 0, 40, 40, 20);
        let disc = area(&rects);
        // π · 20² is about 1257.
        assert!((1200..=1320).contains(&disc), "{disc}");
        // A larger radius is limited to half the side.
        assert_eq!(rounded(0, 0, 40, 40, 99), rects);
    }
}
