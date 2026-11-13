//! The system code that several features share, one folder per platform. The code of a single
//! feature stays with that feature (see Platforms in ARCHITECTURE.md).

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(windows)]
pub mod windows;

/// Sets up the platform before Tauri starts. GTK reads its settings when it starts.
#[cfg(target_os = "linux")]
pub use linux::prepare;
#[cfg(windows)]
pub use windows::prepare;
