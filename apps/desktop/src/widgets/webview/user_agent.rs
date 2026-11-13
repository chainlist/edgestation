use serde::{Deserialize, Serialize};

/// What a view tells sites about itself, in the form that the DevTools method
/// `Emulation.setUserAgentOverride` takes. It has the user agent and its client hints.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAgent {
    pub(super) user_agent: String,
    /// The client hints (`Sec-CH-UA-*` headers, `navigator.userAgentData`). The app sends them
    /// on with no change.
    user_agent_metadata: serde_json::Value,
}
