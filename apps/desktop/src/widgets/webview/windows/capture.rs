use std::sync::mpsc;

use tauri::{Runtime, Webview};

use crate::widgets::webview::receive;

pub async fn capture<R: Runtime>(webview: &Webview<R>) -> Result<Vec<u8>, String> {
    use webview2_com::CapturePreviewCompletedHandler;
    use webview2_com::Microsoft::Web::WebView2::Win32::COREWEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT_JPEG;
    use windows::Win32::System::Com::{IStream, STREAM_SEEK_SET};
    use windows::Win32::UI::Shell::SHCreateMemStream;
    use windows::core::Error;

    /// Reads all the bytes written to `stream`.
    fn read_all(stream: &IStream) -> windows::core::Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut chunk = vec![0u8; 64 * 1024];
        // SAFETY: `chunk` is valid for `chunk.len()` bytes.
        unsafe {
            stream.Seek(0, STREAM_SEEK_SET, None)?;
            loop {
                let mut read = 0;
                stream.Read(chunk.as_mut_ptr().cast(), chunk.len() as u32, Some(&mut read)).ok()?;
                if read == 0 {
                    return Ok(bytes);
                }
                bytes.extend_from_slice(&chunk[..read as usize]);
            }
        }
    }

    let (done_tx, done) = mpsc::channel();
    webview
        .with_webview(move |platform| {
            let start = || -> windows::core::Result<()> {
                // SAFETY: WebView2 calls on the thread that runs its events, while the view is
                // alive.
                unsafe {
                    let core = platform.controller().CoreWebView2()?;
                    let stream = SHCreateMemStream(None).ok_or_else(Error::empty)?;
                    let written = stream.clone();
                    let done_tx = done_tx.clone();
                    let handler = CapturePreviewCompletedHandler::create(Box::new(move |result| {
                        let picture = result.and_then(|()| read_all(&written));
                        let _ = done_tx.send(picture.map_err(|e| e.to_string()));
                        Ok(())
                    }));
                    core.CapturePreview(
                        COREWEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT_JPEG,
                        &stream,
                        &handler,
                    )
                }
            };
            if let Err(e) = start() {
                let _ = done_tx.send(Err(e.to_string()));
            }
        })
        .map_err(|e| e.to_string())?;
    receive(done).await?
}
