//! The macOS side.

/// macOS gives the name of the Wi-Fi network only to apps with Location access. The real side
/// asks for it.
pub fn wifi_name() -> Option<String> {
    None
}

/// Only the `en` interfaces are devices: Ethernet, Wi-Fi and USB. The others, for example `lo0`
/// and the `utun` interfaces of VPNs, are virtual.
pub fn counts(name: &str) -> bool {
    name.starts_with("en")
}
