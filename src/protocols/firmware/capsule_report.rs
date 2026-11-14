//! # Capsule Result Report and Variables
//!
//! Provides data structures and parsers for recording and inspecting capsule
//! processing results through UEFI persistent variables.

use crate::base::{EfiGuid, EfiStatus, EfiTime};

/// GUID for capsule result variables (`EFI_CAPSULE_REPORT_GUID`).
pub const EFI_CAPSULE_REPORT_GUID: EfiGuid = EfiGuid::new(
    0x39b6_8c46,
    0xf7fb,
    0x441b,
    [0xb6, 0xec, 0x16, 0xb0, 0xf6, 0x98, 0x21, 0xf3],
);

/// Current version for FMP capsule result variable structure.
pub const CAPSULE_RESULT_VARIABLE_FMP_VERSION: u16 = 0x0001;

/// Current version for JSON capsule result variable structure.
pub const CAPSULE_RESULT_VARIABLE_JSON_VERSION: u32 = 0x0001;

/// Specification reserved range start for JSON capsule schema identifier.
pub const REDFISH_DEFINED_JSON_SCHEMA: u32 = 0xF000_0000;

/// Header structure of a capsule result variable (`EFI_CAPSULE_RESULT_VARIABLE_HEADER`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiCapsuleResultVariableHeader {
    /// Size in bytes of the variable including header and payload.
    pub variable_total_size: u32,
    /// Reserved field for 64-bit alignment.
    pub reserved: u32,
    /// GUID from the processed capsule header.
    pub capsule_guid: EfiGuid,
    /// Timestamp when capsule processing completed.
    pub capsule_processed: EfiTime,
    /// Result status code of capsule processing.
    pub capsule_status: EfiStatus,
}

impl EfiCapsuleResultVariableHeader {
    /// Creates a new result variable header.
    #[must_use]
    pub const fn new(
        variable_total_size: u32,
        capsule_guid: EfiGuid,
        capsule_processed: EfiTime,
        capsule_status: EfiStatus,
    ) -> Self {
        Self {
            variable_total_size,
            reserved: 0,
            capsule_guid,
            capsule_processed,
            capsule_status,
        }
    }
}

/// Firmware Management Protocol (FMP) capsule result variable payload (`EFI_CAPSULE_RESULT_VARIABLE_FMP`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiCapsuleResultVariableFmp {
    /// Version of structure, currently `0x0001`.
    pub version: u16,
    /// Zero-based index of payload within FMP capsule.
    pub payload_index: u8,
    /// Update image index from FMP capsule image header.
    pub update_image_index: u8,
    /// Update image type ID GUID.
    pub update_image_type_id: EfiGuid,
}

impl Default for EfiCapsuleResultVariableFmp {
    fn default() -> Self {
        Self {
            version: CAPSULE_RESULT_VARIABLE_FMP_VERSION,
            payload_index: 0,
            update_image_index: 0,
            update_image_type_id: EfiGuid::ZERO,
        }
    }
}

/// JSON capsule result variable payload (`EFI_CAPSULE_RESULT_VARIABLE_JSON`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiCapsuleResultVariableJson {
    /// Version of structure, currently `0x0001`.
    pub version: u32,
    /// Identifier describing the schema of JSON payload.
    pub capsule_id: u32,
    /// Length in bytes of the replied JSON payload.
    pub resp_length: u32,
}

impl Default for EfiCapsuleResultVariableJson {
    fn default() -> Self {
        Self {
            version: CAPSULE_RESULT_VARIABLE_JSON_VERSION,
            capsule_id: REDFISH_DEFINED_JSON_SCHEMA,
            resp_length: 0,
        }
    }
}

/// Parsed capsule result report view referencing an in-memory variable buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapsuleResultReport<'a> {
    /// Parsed capsule result variable header.
    pub header: EfiCapsuleResultVariableHeader,
    /// Trailing payload bytes following the header.
    pub payload: &'a [u8],
}

impl<'a> CapsuleResultReport<'a> {
    /// Parses a raw byte slice into a `CapsuleResultReport`.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if slice is smaller than the header.
    /// Returns `EfiStatus::INVALID_PARAMETER` if total size does not match buffer length.
    pub fn parse(buffer: &'a [u8]) -> Result<Self, EfiStatus> {
        let header_size = core::mem::size_of::<EfiCapsuleResultVariableHeader>();
        if buffer.len() < header_size {
            return Err(EfiStatus::BUFFER_TOO_SMALL);
        }

        let header_bytes = &buffer[..header_size];
        let mut header = EfiCapsuleResultVariableHeader {
            variable_total_size: 0,
            reserved: 0,
            capsule_guid: EfiGuid::ZERO,
            capsule_processed: EfiTime::default(),
            capsule_status: EfiStatus::SUCCESS,
        };

        unsafe {
            core::ptr::copy_nonoverlapping(
                header_bytes.as_ptr(),
                (&raw mut header).cast::<u8>(),
                header_size,
            );
        }

        if header.variable_total_size as usize > buffer.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }

        let payload_size = (header.variable_total_size as usize).saturating_sub(header_size);
        let payload = &buffer[header_size..header_size + payload_size];

        Ok(Self { header, payload })
    }

    /// Attempts to parse the payload as an FMP capsule result report.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if payload is too short for FMP header.
    /// Returns `EfiStatus::UNSUPPORTED` if FMP version is unrecognized.
    pub fn parse_fmp(&self) -> Result<EfiCapsuleResultVariableFmp, EfiStatus> {
        let fmp_header_size = core::mem::size_of::<EfiCapsuleResultVariableFmp>();
        if self.payload.len() < fmp_header_size {
            return Err(EfiStatus::BUFFER_TOO_SMALL);
        }

        let mut fmp = EfiCapsuleResultVariableFmp::default();
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.payload.as_ptr(),
                (&raw mut fmp).cast::<u8>(),
                fmp_header_size,
            );
        }

        if fmp.version != CAPSULE_RESULT_VARIABLE_FMP_VERSION {
            return Err(EfiStatus::UNSUPPORTED);
        }

        Ok(fmp)
    }

    /// Attempts to parse the payload as a JSON capsule result report.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if payload is too short for JSON header.
    /// Returns `EfiStatus::UNSUPPORTED` if JSON version is unrecognized.
    pub fn parse_json(&self) -> Result<(EfiCapsuleResultVariableJson, &'a [u8]), EfiStatus> {
        let json_header_size = core::mem::size_of::<EfiCapsuleResultVariableJson>();
        if self.payload.len() < json_header_size {
            return Err(EfiStatus::BUFFER_TOO_SMALL);
        }

        let mut json = EfiCapsuleResultVariableJson::default();
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.payload.as_ptr(),
                (&raw mut json).cast::<u8>(),
                json_header_size,
            );
        }

        if json.version != CAPSULE_RESULT_VARIABLE_JSON_VERSION {
            return Err(EfiStatus::UNSUPPORTED);
        }

        let resp_bytes = &self.payload[json_header_size..];
        if resp_bytes.len() < json.resp_length as usize {
            return Err(EfiStatus::BUFFER_TOO_SMALL);
        }

        Ok((json, &resp_bytes[..json.resp_length as usize]))
    }
}
