//! # TCG capabilities
//!
//! Capability data and bitmap definitions.

/// The event log format for the 1.2 log.
pub const EVENT_LOG_FORMAT_TCG_1_2: u32 = 0x0000_0001;

/// The event log format for the 2.0 log.
pub const EVENT_LOG_FORMAT_TCG_2: u32 = 0x0000_0002;

/// The hash algorithm bit for SHA-1.
pub const HASH_ALG_SHA1: u32 = 0x0000_0001;

/// The hash algorithm bit for SHA-256.
pub const HASH_ALG_SHA256: u32 = 0x0000_0002;

/// The hash algorithm bit for SHA-384.
pub const HASH_ALG_SHA384: u32 = 0x0000_0004;

/// The hash algorithm bit for SHA-512.
pub const HASH_ALG_SHA512: u32 = 0x0000_0008;

/// The hash algorithm bit for SM3-256.
pub const HASH_ALG_SM3_256: u32 = 0x0000_0010;

/// The extend-only flag.
pub const EXTEND_ONLY: u64 = 0x0000_0000_0000_0001;

/// The PE/COFF image flag.
pub const PE_COFF_IMAGE_FLAG: u64 = 0x0000_0000_0000_0010;

/// The maximum PCR index.
pub const MAX_PCR_INDEX: u32 = 23;

/// The TCG2 boot service capability structure.
///
/// Field order matches the specification. The size field is
/// set by the caller to indicate the allocated space and is
/// updated by firmware.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiTcg2BootServiceCapability {
    /// The allocated size of the structure.
    pub size: u8,
    /// The structure major version.
    pub structure_version_major: u8,
    /// The structure minor version.
    pub structure_version_minor: u8,
    /// The protocol major version.
    pub protocol_version_major: u8,
    /// The protocol minor version.
    pub protocol_version_minor: u8,
    /// The supported hash algorithm bitmap.
    pub hash_algorithm_bitmap: u32,
    /// The supported event log bitmap.
    pub supported_event_logs: u32,
    /// Whether a trusted platform module is present.
    pub tpm_present_flag: u8,
    /// The maximum command size in bytes.
    pub max_command_size: u16,
    /// The maximum response size in bytes.
    pub max_response_size: u16,
    /// The manufacturer identifier.
    pub manufacturer_id: u32,
    /// The maximum number of PCR banks.
    pub number_of_pcr_banks: u32,
    /// The active PCR bank bitmap.
    pub active_pcr_banks: u32,
}

/// The final events table version.
pub const FINAL_EVENTS_TABLE_VERSION: u64 = 1;

/// The final events table structure header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiTcg2FinalEventsTable {
    /// The structure version.
    pub version: u64,
    /// The number of events recorded after the event log call.
    pub number_of_events: u64,
}
