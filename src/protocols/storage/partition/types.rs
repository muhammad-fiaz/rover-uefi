//! # Partition types
//!
//! Common partition model.
//!
//! The model preserves scheme-specific
//! metadata. The scheme is known without
//! parsing firmware integers.

use crate::abi::types::Char16;
use crate::base::EfiGuid;

/// The partition scheme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PartitionScheme {
    /// GUID Partition Table.
    Gpt,
    /// Master Boot Record.
    Mbr,
    /// Protective MBR for GPT.
    ProtectiveMbr,
}

/// The kind of a partition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PartitionKind {
    /// A GPT partition.
    Gpt {
        /// The partition type `GUID`.
        type_guid: EfiGuid,
        /// The unique partition `GUID`.
        unique_guid: EfiGuid,
        /// The attributes.
        attributes: u64,
        /// The UTF-16 name.
        name: [Char16; 36],
    },
    /// An MBR partition.
    Mbr {
        /// The partition type byte.
        partition_type: u8,
        /// The boot indicator.
        boot_indicator: u8,
        /// Whether the entry is extended.
        is_extended: bool,
        /// Whether the entry is logical.
        is_logical: bool,
    },
}

/// A single partition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Partition {
    /// The zero-based index.
    pub index: usize,
    /// The starting LBA.
    pub starting_lba: u64,
    /// The ending LBA, inclusive.
    pub ending_lba: u64,
    /// The number of blocks.
    pub sector_count: u64,
    /// The partition kind.
    pub kind: PartitionKind,
}

/// A partition table error.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PartitionTableError {
    /// The data is truncated.
    Truncated,
    /// The signature is invalid.
    InvalidSignature,
    /// The header size is invalid.
    InvalidHeaderSize,
    /// The CRC is invalid.
    InvalidCrc,
    /// The entry size is invalid.
    InvalidEntrySize,
    /// An LBA is out of range.
    InvalidLba,
    /// Arithmetic overflowed.
    Overflow,
    /// The boot indicator is invalid.
    InvalidBootIndicator,
    /// A cyclic reference was found.
    Loop,
    /// The traversal limit was exceeded.
    LimitExceeded,
    /// The table metadata overlaps.
    Overlap,
}
