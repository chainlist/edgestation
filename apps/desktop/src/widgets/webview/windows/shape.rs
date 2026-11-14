use tauri::{LogicalSize, Runtime, Webview};

use crate::widgets::webview::shape::{Bounds, Hole};

/// Cuts `webview` to the shape of `bounds`. `bounds.radius` rounds its corners to match the
/// cell of its widget. The view draws above the page, so the rounded corners of the cell cannot
/// clip it. `bounds.holes` are cut out, so the page under the view shows there and gets the
/// touches. The cut uses whole pixels, without the smooth edge that CSS gives the cell border.
pub fn shape<R: Runtime>(webview: &Webview<R>, bounds: &Bounds) -> tauri::Result<()> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Gdi::{
        CombineRgn, CreateRectRgn, CreateRoundRectRgn, DeleteObject, HRGN, RGN_DIFF, SetWindowRgn,
    };

    /// Makes the region from `x1, y1` to `x2, y2`, without its right and bottom edges. `r`
    /// rounds its corners.
    fn region(x1: i32, y1: i32, x2: i32, y2: i32, r: i32) -> HRGN {
        // SAFETY: plain GDI calls. The caller owns the region.
        unsafe {
            match r {
                0 => CreateRectRgn(x1, y1, x2, y2),
                // A rounded region leaves out one more pixel on each of thes edges, so add one.
                r => CreateRoundRectRgn(x1, y1, x2 + 1, y2 + 1, 2 * r, 2 * r),
            }
        }
    }

    let scale = webview.window().scale_factor()?;
    let px = |value: f64| (value * scale).round() as i32;
    // Round the size as the view rounds its own size, so the cut corners are on its edges.
    let size = LogicalSize::new(bounds.width, bounds.height).to_physical::<i32>(scale);
    let (width, height) = (size.width, size.height);
    let radius = px(bounds.radius);
    let hole = |h: &Hole| (px(h.x), px(h.y), px(h.x + h.width), px(h.y + h.height), px(h.radius));
    let holes: Vec<_> = bounds.holes.iter().map(hole).collect();
    webview.with_webview(move |platform| {
        let mut container = HWND::default();
        // SAFETY: WebView2 calls on the thread that runs its events, while the view is alive.
        // The view is in its own window, from wry. The region of that window clips all that the
        // view draws and all input that it gets. The system owns the region after it is set.
        unsafe {
            if platform.controller().ParentWindow(&mut container).is_err() {
                return;
            }
            let shaped = (radius > 0 || !holes.is_empty()).then(|| {
                let view = region(0, 0, width, height, radius);
                for &(x1, y1, x2, y2, r) in &holes {
                    let hole = region(x1, y1, x2, y2, r);
                    CombineRgn(Some(view), Some(view), Some(hole), RGN_DIFF);
                    let _ = DeleteObject(hole.into());
                }
                view
            });
            SetWindowRgn(container, shaped, true);
        }
    })
}
