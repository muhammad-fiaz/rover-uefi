//! # Discovery
//!
//! Scheme detection from parsed headers.
//!
//! The helper distinguishes GPT, protective
//! MBR, and ordinary MBR without performing
//! firmware I/O.

use crate::protocols::storage::partition::gpt::header::GptHeader;
use crate::protocols::storage::partition::mbr::sector::MbrSector;
use crate::protocols::storage::partition::types::PartitionScheme;

/// Returns the scheme for a sector and an
/// optional GPT header.
#[must_use]
pub fn detect_scheme(sector: &MbrSector, header: Option<&GptHeader>) -> PartitionScheme {
    if header.is_some() {
        return PartitionScheme::Gpt;
    }
    if crate::protocols::storage::partition::mbr::sector::is_protective_mbr(sector) {
        return PartitionScheme::ProtectiveMbr;
    }
    PartitionScheme::Mbr
}
