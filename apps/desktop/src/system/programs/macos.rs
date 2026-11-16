//! The macOS side, a stub. It lists no process.

use super::Process;

/// Braces, not a unit struct, because the shared code calls `Lister::default()`.
#[derive(Default)]
pub(super) struct Lister {}

impl Lister {
    pub(super) fn processes(&mut self) -> Vec<Process> {
        Vec::new()
    }
}
