//! # MBR
//!
//! Master Boot Record parsing.

pub mod ebr;
pub mod entry;
pub mod parser;
pub mod sector;
pub mod validation;

pub use ebr::{absolute_lba, is_extended, validate_ebr_chain};
pub use entry::{MbrEntry, is_empty, parse_mbr_entry};
pub use parser::mbr_scheme;
pub use sector::{MbrSector, is_protective_mbr, parse_mbr_sector};
pub use validation::{checked_end_lba, entries_overlap};
