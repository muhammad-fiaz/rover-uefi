//! # MBR parser
//!
//! High-level MBR parsing.
//!
//! The parser distinguishes protective MBR
//! from ordinary MBR.

use crate::protocols::storage::partition::mbr::sector::{MbrSector, is_protective_mbr};
use crate::protocols::storage::partition::types::PartitionScheme;

/// Returns the scheme for a parsed sector.
#[must_use]
pub fn mbr_scheme(sector: &MbrSector) -> PartitionScheme {
    if is_protective_mbr(sector) {
        PartitionScheme::ProtectiveMbr
    } else {
        PartitionScheme::Mbr
    }
}
