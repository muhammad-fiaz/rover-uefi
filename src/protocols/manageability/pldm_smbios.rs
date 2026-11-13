//! # PLDM SMBIOS Transfer Protocol
//!
//! Provides DMTF DSP0246 Platform Level Data Model (PLDM) SMBIOS Transfer protocol
//! interfaces for exchanging SMBIOS structures and tables between host firmware and BMC.

#![allow(
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::ref_as_ptr,
    clippy::borrow_as_ptr,
    clippy::ptr_as_ptr
)]

use crate::base::{EfiGuid, EfiStatus};

/// PLDM SMBIOS Transfer Protocol GUID.
///
/// `{fa431c3c-816b-4b32-a3e0-ad9b7f64272e}`
pub const PLDM_SMBIOS_TRANSFER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xfa43_1c3c,
    0x816b,
    0x4b32,
    [0xa3, 0xe0, 0xad, 0x9b, 0x7f, 0x64, 0x27, 0x2e],
);

/// Major version of the PLDM SMBIOS Transfer Protocol interface.
pub const PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION_MAJOR: u8 = 1;

/// Minor version of the PLDM SMBIOS Transfer Protocol interface.
pub const PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION_MINOR: u8 = 0;

/// Combined 16-bit version of the PLDM SMBIOS Transfer Protocol.
pub const PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION: u16 =
    ((PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION_MAJOR as u16) << 8)
        | (PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION_MINOR as u16);

/// Sentinel value indicating unassigned terminus ID (uses platform default).
pub const PLDM_TERMINUS_ID_UNASSIGNED: u8 = 0xFF;

/// DMTF DSP0246 PLDM SMBIOS Transfer Command Codes.
pub mod command_code {
    /// Get SMBIOS structure table metadata.
    pub const GET_SMBIOS_STRUCTURE_TABLE_METADATA: u8 = 0x01;
    /// Set SMBIOS structure table metadata.
    pub const SET_SMBIOS_STRUCTURE_TABLE_METADATA: u8 = 0x02;
    /// Get entire SMBIOS structure table.
    pub const GET_SMBIOS_STRUCTURE_TABLE: u8 = 0x03;
    /// Set entire SMBIOS structure table.
    pub const SET_SMBIOS_STRUCTURE_TABLE: u8 = 0x04;
    /// Get SMBIOS structure by type.
    pub const GET_SMBIOS_STRUCTURE_BY_TYPE: u8 = 0x05;
    /// Get SMBIOS structure by handle.
    pub const GET_SMBIOS_STRUCTURE_BY_HANDLE: u8 = 0x06;
}

/// DMTF DSP0246 PLDM SMBIOS Command Specific Completion Codes.
pub mod completion_code {
    /// Invalid data transfer handle passed in request.
    pub const INVALID_DATA_TRANSFER_HANDLE: u8 = 0x80;
    /// Invalid transfer operation flag specified.
    pub const INVALID_TRANSFER_OPERATION_FLAG: u8 = 0x81;
    /// Invalid transfer flag specified.
    pub const INVALID_TRANSFER_FLAG: u8 = 0x82;
    /// SMBIOS structure table metadata is not present.
    pub const NO_SMBIOS_STRUCTURE_TABLE_METADATA: u8 = 0x83;
    /// Data integrity checksum failed validation.
    pub const INVALID_DATA_INTEGRITY_CHECK: u8 = 0x84;
    /// SMBIOS structure table currently unavailable.
    pub const SMBIOS_STRUCTURE_TABLE_UNAVAILABLE: u8 = 0x85;
}

/// PLDM Transfer Flags.
pub mod transfer_flag {
    /// Start of multi-part transfer.
    pub const START: u8 = 0x01;
    /// Middle chunk of multi-part transfer.
    pub const MIDDLE: u8 = 0x02;
    /// End of multi-part transfer.
    pub const END: u8 = 0x04;
    /// Transfer payload fits entirely in a single packet.
    pub const START_AND_END: u8 = 0x05;
}

/// PLDM Transfer Operation Flags.
pub mod transfer_op {
    /// Request next chunk of data.
    pub const GET_NEXT_PART: u8 = 0x00;
    /// Request the first chunk of data.
    pub const GET_FIRST_PART: u8 = 0x01;
}

/// DMTF DSP0246 SMBIOS Structure Table Metadata.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PldmSmbiosStructureTableMetadata {
    /// Major version of SMBIOS specification supported.
    pub smbios_major_version: u8,
    /// Minor version of SMBIOS specification supported.
    pub smbios_minor_version: u8,
    /// Maximum size in bytes of any individual SMBIOS structure.
    pub maximum_structure_size: u16,
    /// Total length in bytes of the entire SMBIOS structure table.
    pub smbios_structure_table_length: u16,
    /// Total number of structures present in the SMBIOS table.
    pub number_of_smbios_structures: u16,
    /// 32-bit CRC or checksum of the SMBIOS structure table.
    pub smbios_structure_table_integrity_checksum: u32,
}

/// Raw function pointer for setting source and destination terminus IDs.
pub type PldmSetSmbiosTransferTerminusId =
    unsafe extern "efiapi" fn(source_id: u8, destination_id: u8) -> EfiStatus;

/// Raw function pointer for retrieving SMBIOS table metadata.
pub type PldmGetSmbiosStructureTableMetadata = unsafe extern "efiapi" fn(
    this: *mut PldmSmbiosTransferProtocol,
    buffer: *mut PldmSmbiosStructureTableMetadata,
) -> EfiStatus;

/// Raw function pointer for setting SMBIOS table metadata.
pub type PldmSetSmbiosStructureTableMetadata = unsafe extern "efiapi" fn(
    this: *mut PldmSmbiosTransferProtocol,
    buffer: *const PldmSmbiosStructureTableMetadata,
) -> EfiStatus;

/// Raw function pointer for retrieving the entire SMBIOS structure table.
pub type PldmGetSmbiosStructureTable = unsafe extern "efiapi" fn(
    this: *mut PldmSmbiosTransferProtocol,
    buffer: *mut *mut u8,
    buffer_size: *mut u32,
) -> EfiStatus;

/// Raw function pointer for setting the SMBIOS structure table.
pub type PldmSetSmbiosStructureTable =
    unsafe extern "efiapi" fn(this: *mut PldmSmbiosTransferProtocol) -> EfiStatus;

/// Raw function pointer for retrieving structures by type.
pub type PldmGetSmbiosStructureByType = unsafe extern "efiapi" fn(
    this: *mut PldmSmbiosTransferProtocol,
    type_id: u8,
    structure_instance_id: u16,
    buffer: *mut *mut u8,
    buffer_size: *mut u32,
) -> EfiStatus;

/// Raw function pointer for retrieving structures by handle.
pub type PldmGetSmbiosStructureByHandle = unsafe extern "efiapi" fn(
    this: *mut PldmSmbiosTransferProtocol,
    handle: u16,
    buffer: *mut *mut u8,
    buffer_size: *mut u32,
) -> EfiStatus;

/// Version 1.0 function table for the PLDM SMBIOS Transfer Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PldmSmbiosTransferProtocolV1_0 {
    /// Sets source and destination terminus IDs.
    pub set_pldm_smbios_transfer_terminus_id: PldmSetSmbiosTransferTerminusId,
    /// Gets SMBIOS structure table metadata.
    pub get_smbios_structure_table_metadata: PldmGetSmbiosStructureTableMetadata,
    /// Sets SMBIOS structure table metadata.
    pub set_smbios_structure_table_metadata: PldmSetSmbiosStructureTableMetadata,
    /// Gets the full SMBIOS structure table.
    pub get_smbios_structure_table: PldmGetSmbiosStructureTable,
    /// Sets the full SMBIOS structure table.
    pub set_smbios_structure_table: PldmSetSmbiosStructureTable,
    /// Gets an SMBIOS structure by type and instance ID.
    pub get_smbios_structure_by_type: PldmGetSmbiosStructureByType,
    /// Gets an SMBIOS structure by handle.
    pub get_smbios_structure_by_handle: PldmGetSmbiosStructureByHandle,
}

/// Union of function table pointers across protocol revisions.
#[repr(C)]
#[derive(Clone, Copy)]
pub union PldmSmbiosTransferProtocolFunctions {
    /// Version 1.0 interface functions.
    pub version_1_0: *mut PldmSmbiosTransferProtocolV1_0,
}

/// EFI PLDM SMBIOS Transfer Protocol interface structure.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PldmSmbiosTransferProtocol {
    /// Interface protocol version.
    pub protocol_version: u16,
    /// Protocol function dispatch pointers.
    pub functions: PldmSmbiosTransferProtocolFunctions,
}

/// High-level safe wrapper around [`PldmSmbiosTransferProtocol`].
pub struct PldmSmbiosTransfer<'a> {
    proto: &'a mut PldmSmbiosTransferProtocol,
}

impl<'a> PldmSmbiosTransfer<'a> {
    /// Wraps a mutable reference to [`PldmSmbiosTransferProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut PldmSmbiosTransferProtocol) -> Self {
        Self { proto }
    }

    /// Retrieves a reference to the v1.0 function table.
    fn v1(&self) -> Result<&'static PldmSmbiosTransferProtocolV1_0, EfiStatus> {
        let major = (self.proto.protocol_version >> 8) as u8;
        if major < 1 {
            return Err(EfiStatus::UNSUPPORTED);
        }
        let ptr = unsafe { self.proto.functions.version_1_0 };
        if ptr.is_null() {
            Err(EfiStatus::NOT_READY)
        } else {
            Ok(unsafe { &*ptr })
        }
    }

    /// Configures source and destination PLDM terminus IDs.
    ///
    /// Pass [`PLDM_TERMINUS_ID_UNASSIGNED`] to use platform default IDs.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if terminus IDs are rejected or unsupported.
    pub fn set_terminus_ids(&mut self, source_id: u8, dest_id: u8) -> Result<(), EfiStatus> {
        let v1 = self.v1()?;
        let status = unsafe { (v1.set_pldm_smbios_transfer_terminus_id)(source_id, dest_id) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Retrieves SMBIOS structure table metadata from the BMC terminus.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on transport failure or unsupported operation.
    pub fn get_metadata(&mut self) -> Result<PldmSmbiosStructureTableMetadata, EfiStatus> {
        let v1 = self.v1()?;
        let mut meta = PldmSmbiosStructureTableMetadata::default();
        let this_ptr = self.proto as *mut PldmSmbiosTransferProtocol;
        let status = unsafe { (v1.get_smbios_structure_table_metadata)(this_ptr, &mut meta) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(meta)
        }
    }

    /// Transmits SMBIOS structure table metadata to the BMC terminus.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on transport failure or unsupported operation.
    pub fn set_metadata(
        &mut self,
        meta: &PldmSmbiosStructureTableMetadata,
    ) -> Result<(), EfiStatus> {
        let v1 = self.v1()?;
        let this_ptr = self.proto as *mut PldmSmbiosTransferProtocol;
        let status = unsafe { (v1.set_smbios_structure_table_metadata)(this_ptr, meta) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Queries the full SMBIOS table from the BMC terminus.
    ///
    /// Returns a raw slice pointer and length allocated by the protocol driver.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if the table is unavailable or unsupported.
    pub fn get_table(&mut self) -> Result<(&'static [u8], *mut u8), EfiStatus> {
        let v1 = self.v1()?;
        let this_ptr = self.proto as *mut PldmSmbiosTransferProtocol;
        let mut buffer: *mut u8 = core::ptr::null_mut();
        let mut size: u32 = 0;
        let status = unsafe { (v1.get_smbios_structure_table)(this_ptr, &mut buffer, &mut size) };
        if status.is_error() {
            return Err(status);
        }
        if buffer.is_null() || size == 0 {
            return Err(EfiStatus::NOT_FOUND);
        }
        let slice = unsafe { core::slice::from_raw_parts(buffer, size as usize) };
        Ok((slice, buffer))
    }

    /// Triggers pushing the host SMBIOS structure table to the BMC.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] on failure.
    pub fn set_table(&mut self) -> Result<(), EfiStatus> {
        let v1 = self.v1()?;
        let this_ptr = self.proto as *mut PldmSmbiosTransferProtocol;
        let status = unsafe { (v1.set_smbios_structure_table)(this_ptr) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Queries an SMBIOS structure by type and instance ID.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if structure not found or on transport failure.
    pub fn get_structure_by_type(
        &mut self,
        type_id: u8,
        instance_id: u16,
    ) -> Result<(&'static [u8], *mut u8), EfiStatus> {
        let v1 = self.v1()?;
        let this_ptr = self.proto as *mut PldmSmbiosTransferProtocol;
        let mut buffer: *mut u8 = core::ptr::null_mut();
        let mut size: u32 = 0;
        let status = unsafe {
            (v1.get_smbios_structure_by_type)(
                this_ptr,
                type_id,
                instance_id,
                &mut buffer,
                &mut size,
            )
        };
        if status.is_error() {
            return Err(status);
        }
        if buffer.is_null() || size == 0 {
            return Err(EfiStatus::NOT_FOUND);
        }
        let slice = unsafe { core::slice::from_raw_parts(buffer, size as usize) };
        Ok((slice, buffer))
    }

    /// Queries an SMBIOS structure by handle.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if structure not found or on transport failure.
    pub fn get_structure_by_handle(
        &mut self,
        handle: u16,
    ) -> Result<(&'static [u8], *mut u8), EfiStatus> {
        let v1 = self.v1()?;
        let this_ptr = self.proto as *mut PldmSmbiosTransferProtocol;
        let mut buffer: *mut u8 = core::ptr::null_mut();
        let mut size: u32 = 0;
        let status = unsafe {
            (v1.get_smbios_structure_by_handle)(this_ptr, handle, &mut buffer, &mut size)
        };
        if status.is_error() {
            return Err(status);
        }
        if buffer.is_null() || size == 0 {
            return Err(EfiStatus::NOT_FOUND);
        }
        let slice = unsafe { core::slice::from_raw_parts(buffer, size as usize) };
        Ok((slice, buffer))
    }
}
