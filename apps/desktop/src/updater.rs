use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

/// Looks once for a newer release and installs it. The app quits when the installer starts, and
/// the installer opens the app again when it is done. Debug builds skip this step, so `tauri dev`
/// never installs a release over the installed app.
pub fn install_at_launch(app: &AppHandle) {
    if cfg!(debug_assertions) {
        return;
    }
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(error) = install(&app).await {
            eprintln!("failed to update: {error}");
        }
    });
}

async fn install(app: &AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        update.download_and_install(|_, _| {}, || {}).await?;
    }
    Ok(())
}
