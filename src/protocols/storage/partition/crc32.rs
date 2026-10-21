//! # CRC32
//!
//! IEEE CRC-32 for `GUID` partition table validation.
//!
//! The implementation is the shared firmware CRC-32 defined in
//! [`crate::base::crc`]. This module re-exports it so partition
//! parsing code can address the algorithm by the storage-specific
//! path without duplicating the algorithm.

pub use crate::base::crc::crc32;
