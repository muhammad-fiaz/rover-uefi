//! # PXE mode
//!
//! Mode information helpers.

/// Returns whether PXE is started.
#[must_use]
pub fn is_started(started: u8) -> bool {
    started != 0
}
