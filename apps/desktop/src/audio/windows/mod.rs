//! The Windows side: the Core Audio devices and sessions that the volume mixer shows.

mod apps;
mod default_output;
mod devices;
mod meters;
mod mixer;
mod sessions;
mod watch;

pub use meters::{Meter, levels};
pub use mixer::{mixer, outputs, set_muted, set_output, set_volume};
pub use watch::{unwatch, watch};
pub(super) use devices::{default, enumerator, inputs, volume};

use crate::os::windows::com::{self, Apartment};

/// Runs `f` with COM set up on this thread for this call only, and turns a COM error into text.
/// Core Audio works in the two apartments, so a thread that is already in the other apartment
/// stays there.
pub(super) fn with_com<T>(f: impl FnOnce() -> windows::core::Result<T>) -> Result<T, String> {
    com::with(Apartment::Multithreaded, f).map_err(|error| error.to_string())
}
