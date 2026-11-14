use std::path::Path;

use percent_encoding::percent_decode_str;
use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Runtime, UriSchemeContext, UriSchemeResponder};

use super::roots::{Root, resolve, roots};

/// Serves widget files at `widget://localhost/<id>/<file>`, and at `http://widget.localhost/...`
/// on Windows. Real slashes in the URL let relative `import`s in widget modules work.
pub fn handle<R: Runtime>(
    ctx: UriSchemeContext<'_, R>,
    request: Request<Vec<u8>>,
    responder: UriSchemeResponder,
) {
    let roots = roots(ctx.app_handle());
    // Read the file off the main thread, so a large aset does not stop the UI.
    tauri::async_runtime::spawn_blocking(move || {
        responder.respond(respond(&roots, request.uri().path()));
    });
}

fn respond(roots: &[Root], path: &str) -> Response<Vec<u8>> {
    let rel = percent_decode_str(path.trim_start_matches('/')).decode_utf8_lossy();
    let found = resolve(roots, &rel)
        .and_then(|file| std::fs::read(&file).ok().map(|body| (mime(&file), body)));

    // The app origin is not the widget origin, and module scripts need CORS.
    let response = Response::builder()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::CACHE_CONTROL, "no-cache");
    match found {
        Some((mime, body)) => response.header(header::CONTENT_TYPE, mime).body(body),
        None => response.status(StatusCode::NOT_FOUND).body(Vec::new()),
    }
    .unwrap_or_default()
}

fn mime(file: &Path) -> &'static str {
    match file.extension().and_then(|e| e.to_str()).unwrap_or_default() {
        "js" | "mjs" => "text/javascript",
        "css" => "text/css",
        "json" => "application/json",
        "html" => "text/html",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "woff2" => "font/woff2",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}
