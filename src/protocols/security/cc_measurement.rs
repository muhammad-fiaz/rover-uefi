//! # Confidential Computing (CC) Measurement Protocol
//!
//! Defined by the UEFI Specification Section 38.3: Confidential Computing Measurement Protocol.
//!
//! Provides hardware-rooted attestation and measurement interfaces for confidential guest
//! environments (AMD SEV-SNP, Intel TDX, ARM CCA).

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};
use core::ffi::c_void;

/// The `EFI_CC_MEASUREMENT_PROTOCOL_GUID`.
pub const EFI_CC_MEASUREMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x964E_5B29,
    0x6459,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

/// Confidential Computing hardware guest type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EfiCcType {
    /// No CC environment.
    None = 0,
    /// AMD SEV or SEV-ES or SEV-SNP.
    Sev = 1,
    /// Intel TDX.
    Tdx = 2,
    /// ARM CCA.
    ArmCca = 3,
}

/// Capability structure for CC measurement protocol.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiCcMeasurementBootServiceCapability {
    /// Size of this structure in bytes.
    pub size: u8,
    /// Supported specification version.
    pub structure_version_major: u8,
    /// Structure version minor.
    pub structure_version_minor: u8,
    /// Supported hash algorithms bitmask.
    pub hash_algorithm_bitmap: u32,
    /// Supported event log formats bitmask.
    pub supported_event_logs: u32,
    /// CC hardware guest type.
    pub cc_type: EfiCcType,
}

/// Header for a CC measurement event log entry.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiCcMeasurementEventHeader {
    /// Size of this header in bytes.
    pub header_size: u32,
    /// Header version.
    pub header_version: u16,
    /// Measurement register index.
    pub mr_index: u32,
    /// Event type code.
    pub event_type: u32,
}

/// CC measurement event structure.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiCcMeasurementEvent {
    /// Size of this event structure in bytes.
    pub size: u32,
    /// Event header.
    pub header: EfiCcMeasurementEventHeader,
    /// Event payload data buffer pointer.
    pub event: *const c_void,
}

/// Function pointer for `EFI_CC_GET_CAPABILITY`.
pub type EfiCcGetCapability = unsafe extern "efiapi" fn(
    this: *const EfiCcMeasurementProtocol,
    protocol_capability: *mut EfiCcMeasurementBootServiceCapability,
) -> EfiStatus;

/// Function pointer for `EFI_CC_GET_EVENT_LOG`.
pub type EfiCcGetEventLog = unsafe extern "efiapi" fn(
    this: *const EfiCcMeasurementProtocol,
    event_log_format: u32,
    event_log_location: *mut EfiPhysicalAddress,
    event_log_last_entry: *mut EfiPhysicalAddress,
    event_log_truncated: *mut Bool,
) -> EfiStatus;

/// Function pointer for `EFI_CC_HASH_LOG_EXTEND_EVENT`.
pub type EfiCcHashLogExtendEvent = unsafe extern "efiapi" fn(
    this: *const EfiCcMeasurementProtocol,
    flags: u64,
    data_to_hash: EfiPhysicalAddress,
    data_to_hash_len: u64,
    event: *const EfiCcMeasurementEvent,
) -> EfiStatus;

/// Function pointer for `EFI_CC_MAP_PCR_TO_MR_INDEX`.
pub type EfiCcMapPcrToMrIndex = unsafe extern "efiapi" fn(
    this: *const EfiCcMeasurementProtocol,
    pcr_index: u32,
    mr_index: *mut u32,
) -> EfiStatus;

/// The raw ABI structure for `EFI_CC_MEASUREMENT_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiCcMeasurementProtocol {
    /// Retrieves CC measurement capabilities and guest hardware type.
    pub get_capability: EfiCcGetCapability,
    /// Returns the address of the CC measurement event log buffer.
    pub get_event_log: EfiCcGetEventLog,
    /// Hashes input buffer, appends log entry, and extends measurement register.
    pub hash_log_extend_event: EfiCcHashLogExtendEvent,
    /// Maps standard TPM PCR index to CC Measurement Register (MR) index.
    pub map_pcr_to_mr_index: EfiCcMapPcrToMrIndex,
}
