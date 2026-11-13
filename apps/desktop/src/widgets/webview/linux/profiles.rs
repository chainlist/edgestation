use std::sync::mpsc;

use tauri::{Runtime, Webview};

use crate::widgets::webview::receive;

/// WebKit reports no exit of its processes. It clears all the data of the profile of `view`, on
/// disk and in memory, so the profile is signed out also before the app restarts. Returns a
/// receiver that is ready at once.
pub async fn release<R: Runtime>(
    view: &Webview<R>,
    _profile: String,
) -> Result<mpsc::Receiver<()>, String> {
    use webkit2gtk::{WebViewExt, WebsiteDataManagerExtManual, WebsiteDataTypes};

    let (cleared_tx, cleared) = mpsc::channel();
    view.with_webview(move |platform| match platform.inner().website_data_manager() {
        Some(manager) => {
            let cancel = None::<&gtk::gio::Cancellable>;
            let everything = gtk::glib::TimeSpan(0);
            manager.clear(WebsiteDataTypes::ALL, everything, cancel, move |result| {
                let _ = cleared_tx.send(result.map_err(|e| e.to_string()));
            });
        }
        None => drop(cleared_tx.send(Ok(()))),
    })
    .map_err(|e| e.to_string())?;
    receive(cleared).await??;
    let (ready_tx, ready) = mpsc::channel();
    let _ = ready_tx.send(());
    Ok(ready)
}
