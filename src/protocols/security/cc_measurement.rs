//! # Confidential Computing (CC) Measurement Protocol
//!
//! Defined by the UEFI Specification Section 38.3: Confidential Computing Measurement Protocol.
//!
//! Provides hardware-rooted attestation and measurement interfaces for confidential guest
//! environments (AMD SEV-SNP, Intel TDX, ARM CCA).

use crate::abi::types::Bool;
use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};
use core::ffi::c_void;

/// The Confidential Computing Measurement Protocol GUID (`EFI_CC_MEASUREMENT_PROTOCOL_GUID`).
///
/// `{96751a3d-72f4-41a6-a794-ed5d0e67ae6b}`
pub const CC_MEASUREMENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9675_1A3D,
    0x72F4,
    0x41A6,
    [0xA7, 0x94, 0xED, 0x5D, 0x0E, 0x67, 0xAE, 0x6B],
);

/// CC Final Events Table GUID (`EFI_CC_FINAL_EVENTS_TABLE_GUID`).
///
/// `{dd4a4648-2de7-4665-964d-21d9ef5fb446}`
pub const CC_FINAL_EVENTS_TABLE_GUID: EfiGuid = EfiGuid::new(
    0xDD4A_4648,
    0x2DE7,
    0x4665,
    [0x96, 0x4D, 0x21, 0xD9, 0xEF, 0x5F, 0xB4, 0x46],
);

// Intel TDX measurement register indices
/// Intel TDX MRTD index (build-time measurement).
pub const TDX_MR_INDEX_MRTD: u32 = 0;
/// Intel TDX RTMR0 index (runtime measurement register 0).
pub const TDX_MR_INDEX_RTMR0: u32 = 1;
/// Intel TDX RTMR1 index (runtime measurement register 1).
pub const TDX_MR_INDEX_RTMR1: u32 = 2;
/// Intel TDX RTMR2 index (runtime measurement register 2).
pub const TDX_MR_INDEX_RTMR2: u32 = 3;
/// Intel TDX RTMR3 index (runtime measurement register 3).
pub const TDX_MR_INDEX_RTMR3: u32 = 4;

// Arm CCA Realm measurement register indices
/// Arm CCA Realm Initial Measurement Register (RIM).
pub const ARMCCA_MR_INDEX_0_RIM: u32 = 0;
/// Arm CCA Realm Extensible Measurement Register 0 (REM0).
pub const ARMCCA_MR_INDEX_1_REM0: u32 = 1;
/// Arm CCA Realm Extensible Measurement Register 1 (REM1).
pub const ARMCCA_MR_INDEX_2_REM1: u32 = 2;
/// Arm CCA Realm Extensible Measurement Register 2 (REM2).
pub const ARMCCA_MR_INDEX_3_REM2: u32 = 3;

/// Crypto-agile TCG2 event log format.
pub const CC_EVENT_LOG_FORMAT_TCG_2: u32 = 0x0000_0002;

/// SHA-256 boot hash algorithm bitmask.
pub const CC_BOOT_HASH_ALG_SHA256: u32 = 0x0000_0002;
/// SHA-384 boot hash algorithm bitmask.
pub const CC_BOOT_HASH_ALG_SHA384: u32 = 0x0000_0004;
/// SHA-512 boot hash algorithm bitmask.
pub const CC_BOOT_HASH_ALG_SHA512: u32 = 0x0000_0008;

/// Flag indicating that an event shall only be extended into the MR, not logged.
pub const CC_FLAG_EXTEND_ONLY: u64 = 0x0000_0000_0000_0001;

/// Flag indicating that the event payload represents a PE/COFF image.
pub const CC_FLAG_PE_COFF_IMAGE: u64 = 0x0000_0000_0000_0010;

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
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct EfiCcMeasurementBootServiceCapability {
    /// Size of this structure in bytes.
    pub size: u8,
    /// Supported specification version major.
    pub structure_version_major: u8,
    /// Supported specification version minor.
    pub structure_version_minor: u8,
    /// Supported hash algorithms bitmask.
    pub hash_algorithm_bitmap: u32,
    /// Supported event log formats bitmask.
    pub supported_event_logs: u32,
    /// CC hardware guest type.
    pub cc_type: u8,
    /// Subtype of CC environment.
    pub cc_subtype: u8,
}

/// Header for a CC measurement event log entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, packed)]
pub struct EfiCcMeasurementEventHeader {
    /// Size of this header in bytes.
    pub header_size: u32,
    /// Header version (typically 1).
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

/// Safe client wrapper around `EfiCcMeasurementProtocol`.
#[derive(Debug)]
pub struct CcMeasurement<'a> {
    protocol: &'a EfiCcMeasurementProtocol,
}

impl<'a> CcMeasurement<'a> {
    /// Wrap an active `EfiCcMeasurementProtocol` instance.
    #[must_use]
    pub const fn new(protocol: &'a EfiCcMeasurementProtocol) -> Self {
        Self { protocol }
    }

    /// Retrieve the hardware guest capabilities and supported hash algorithms.
    ///
    /// # Errors
    /// Returns `EfiStatus` if query fails.
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_capability(&self) -> Result<EfiCcMeasurementBootServiceCapability, EfiStatus> {
        let mut cap = EfiCcMeasurementBootServiceCapability {
            size: core::mem::size_of::<EfiCcMeasurementBootServiceCapability>() as u8,
            ..Default::default()
        };

        let status = unsafe { (self.protocol.get_capability)(self.protocol, &raw mut cap) };
        if status == EfiStatus::SUCCESS {
            Ok(cap)
        } else {
            Err(status)
        }
    }

    /// Retrieve the memory address of the confidential computing event log.
    ///
    /// # Errors
    /// Returns `EfiStatus` if the event log cannot be retrieved or format is unsupported.
    pub fn get_event_log(
        &self,
        format: u32,
    ) -> Result<(EfiPhysicalAddress, EfiPhysicalAddress, bool), EfiStatus> {
        let mut location = EfiPhysicalAddress::NULL;
        let mut last_entry = EfiPhysicalAddress::NULL;
        let mut truncated: Bool = 0;

        let status = unsafe {
            (self.protocol.get_event_log)(
                self.protocol,
                format,
                &raw mut location,
                &raw mut last_entry,
                &raw mut truncated,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok((location, last_entry, truncated != 0))
        } else {
            Err(status)
        }
    }

    /// Maps a standard TPM PCR index to a Confidential Computing measurement register index.
    ///
    /// # Errors
    /// Returns `EfiStatus` if mapping is not supported or PCR index is out of bounds.
    pub fn map_pcr_to_mr_index(&self, pcr_index: u32) -> Result<u32, EfiStatus> {
        let mut mr_index = 0u32;
        let status = unsafe {
            (self.protocol.map_pcr_to_mr_index)(self.protocol, pcr_index, &raw mut mr_index)
        };
        if status == EfiStatus::SUCCESS {
            Ok(mr_index)
        } else {
            Err(status)
        }
    }

    /// Hash a buffer in memory and extend the corresponding measurement register.
    ///
    /// # Errors
    /// Returns `EfiStatus` if hash extension fails.
    pub fn hash_log_extend_event(
        &self,
        flags: u64,
        data_to_hash: EfiPhysicalAddress,
        data_to_hash_len: u64,
        event: &EfiCcMeasurementEvent,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.protocol.hash_log_extend_event)(
                self.protocol,
                flags,
                data_to_hash,
                data_to_hash_len,
                event,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
