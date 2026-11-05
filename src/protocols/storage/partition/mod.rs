//! # Partition tables
//!
//! GPT and MBR parsing.
//!
//! Parsing uses checked arithmetic and never
//! dereferences unaligned disk bytes. Block
//! I/O helpers read through the existing
//! wrapper with caller buffers.

pub mod crc32;
pub mod discovery;
pub mod gpt;
pub mod guid;
pub mod info;
pub mod mbr;
pub mod reader;
pub mod types;

pub use crc32::crc32;
pub use discovery::detect_scheme;
pub use guid::{
    APPLE_HFS_HFS_PLUS, EFI_SYSTEM_PARTITION, LINUX_FILE_SYSTEM_DATA, LINUX_HOME,
    LINUX_ROOT_AARCH64, LINUX_ROOT_X86_64, LINUX_SWAP, MICROSOFT_BASIC_DATA, MICROSOFT_RECOVERY,
    MICROSOFT_RESERVED,
};
pub use info::{
    EFI_PARTITION_INFO_PROTOCOL_REVISION, EfiPartitionInfoData, EfiPartitionInfoProtocol,
    GptPartitionRecord, MbrPartitionRecord, PARTITION_INFO_PROTOCOL_GUID, PARTITION_TYPE_GPT,
    PARTITION_TYPE_MBR, PARTITION_TYPE_OTHER,
};
pub use types::{Partition, PartitionKind, PartitionScheme, PartitionTableError};
