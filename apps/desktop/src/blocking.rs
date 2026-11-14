/// Runs `f` on a blocking thread and returns what it returns. The shell, Core Audio and the
/// media session can make a call wait, so the commands use this to keep the async runtime free.
pub async fn run<T: Send + 'static>(
    f: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(f).await.map_err(|error| error.to_string())?
}
