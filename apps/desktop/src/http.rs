//! Downloads from the web: the marketplace and the album art of media players. They share one
//! client, so they also share its connections.

use std::sync::OnceLock;
use std::time::Duration;

use tauri_plugin_http::reqwest::Client;

/// The client of the app. Its user agent names the app and its version.
pub fn client() -> Result<Client, String> {
    static CLIENT: OnceLock<Result<Client, String>> = OnceLock::new();
    CLIENT
        .get_or_init(|| {
            Client::builder()
                .user_agent(concat!("EdgeStation/", env!("CARGO_PKG_VERSION")))
                .connect_timeout(Duration::from_secs(15))
                .read_timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| e.to_string())
        })
        .clone()
}

/// Gets the body of `url`. It refuses a body of more then `max` bytes.
pub async fn get(client: &Client, url: &str, max: usize) -> Result<Vec<u8>, String> {
    let mut response = client.get(url).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("{url} answered {}", response.status()));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        body.extend_from_slice(&chunk);
        if body.len() > max {
            return Err(format!("{url} is larger than {} MB", (max >> 20).max(1)));
        }
    }
    Ok(body)
}
