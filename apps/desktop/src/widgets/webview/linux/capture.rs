use std::sync::mpsc;

use tauri::{Runtime, Webview};

use crate::widgets::webview::receive;

/// WebKit draws what shows of the view in a cairo surface, which this encodes as JPEG.
pub async fn capture<R: Runtime>(webview: &Webview<R>) -> Result<Vec<u8>, String> {
    use gtk::cairo::{ImageSurface, Surface};
    use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
    use webkit2gtk::{SnapshotOptions, SnapshotRegion, WebViewExt};

    fn jpeg(surface: Surface) -> Result<Vec<u8>, String> {
        let image = ImageSurface::try_from(surface).map_err(|_| "the picture has no pixels")?;
        image.flush();
        let (width, height, stride) = (image.width(), image.height(), image.stride());
        let mut rgb = Vec::new();
        image
            .with_data(|data| rgb = to_rgb(data, width as usize, height as usize, stride as usize))
            .map_err(|e| e.to_string())?;
        let pixbuf =
            Pixbuf::from_mut_slice(rgb, Colorspace::Rgb, false, 8, width, height, width * 3);
        pixbuf.save_to_bufferv("jpeg", &[("quality", "85")]).map_err(|e| e.to_string())
    }

    let (done_tx, done) = mpsc::channel();
    webview
        .with_webview(move |platform| {
            let options = SnapshotOptions::NONE;
            let cancel = None::<&gtk::gio::Cancellable>;
            platform.inner().snapshot(SnapshotRegion::Visible, options, cancel, move |result| {
                let _ = done_tx.send(result.map_err(|e| e.to_string()).and_then(jpeg));
            });
        })
        .map_err(|e| e.to_string())?;
    receive(done).await?
}

/// Changes the pixels of cairo, 32-bit ARGB in the byte order of the machine, into rows of RGB.
/// The page is opaque, so its alpha does not count.
fn to_rgb(data: &[u8], width: usize, height: usize, stride: usize) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(width * height * 3);
    for row in data.chunks(stride).take(height) {
        for &pixel in row[..width * 4].as_chunks::<4>().0 {
            let [_, r, g, b] = u32::from_ne_bytes(pixel).to_be_bytes();
            rgb.extend([r, g, b]);
        }
    }
    rgb
}

#[cfg(test)]
mod tests {
    use super::to_rgb;

    #[test]
    fn turns_cairo_pixels_into_rgb_rows() {
        let pixel = |argb: u32| argb.to_ne_bytes();
        // Two pixels per row, and a stride of 12 bytes with 4 bytes of padding.
        let mut data = Vec::new();
        for row in [[0xFF10_2030, 0xFF40_5060], [0xFFAA_BBCC, 0xFF00_0000]] {
            data.extend(row.iter().flat_map(|&argb| pixel(argb)));
            data.extend([0; 4]);
        }
        let rgb = to_rgb(&data, 2, 2, 12);
        assert_eq!(rgb, [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0xAA, 0xBB, 0xCC, 0, 0, 0]);
    }
}
