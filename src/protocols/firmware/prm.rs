//! # Platform Runtime Mechanism (PRM)
//!
//! Provides the PRM Configuration Protocol and context buffer data structures
//! defined in the UEFI Platform Runtime Mechanism Specification.

use core::ffi::c_void;

use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// The PRM Configuration Protocol `GUID`.
pub const PRM_CONFIG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4E5B_4FEA,
    0x936A,
    0x45BC,
    [0xAC, 0x6A, 0x2F, 0x8F, 0x14, 0xA6, 0xC2, 0x9E],
);

/// PRM Configuration Protocol signature (`'PMCP'`).
pub const PRM_CONFIG_PROTOCOL_SIGNATURE: u32 = 0x504D_4350;

/// PRM Configuration Protocol version.
pub const PRM_CONFIG_PROTOCOL_VERSION: u32 = 1;

/// PRM context buffer signature (`'PRMC'`).
pub const PRM_CONTEXT_BUFFER_SIGNATURE: u32 = 0x434D_5250;

/// PRM context buffer interface version.
pub const PRM_CONTEXT_BUFFER_INTERFACE_VERSION: u16 = 1;

/// PRM static data buffer signature (`'PRMS'`).
pub const PRM_DATA_BUFFER_HEADER_SIGNATURE: u32 = 0x534D_5250;

/// PRM ACPI parameter data buffer signature (`'PRMP'`).
pub const PRM_ACPI_PARAMETER_DATA_BUFFER_HEADER_SIGNATURE: u32 = 0x504D_5250;

/// Maximum length of a PRM handler name.
pub const PRM_HANDLER_NAME_MAXIMUM_LENGTH: usize = 128;

/// ACPI parameter buffer descriptor.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct AcpiParameterBufferDescriptor {
    /// Handler GUID.
    pub handler_guid: EfiGuid,
    /// ACPI parameter buffer physical/virtual address.
    pub acpi_parameter_buffer_address: u64,
}

/// PRM runtime MMIO range descriptor.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PrmRuntimeMmioRange {
    /// Physical base address.
    pub physical_base_address: EfiPhysicalAddress,
    /// Virtual base address.
    pub virtual_base_address: EfiPhysicalAddress,
    /// Range length in bytes.
    pub length: u32,
}

/// Buffer containing an array of PRM runtime MMIO ranges.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PrmRuntimeMmioRanges {
    /// Number of range elements.
    pub count: u64,
    /// First range element.
    pub range: [PrmRuntimeMmioRange; 1],
}

/// Generic header describing a PRM data buffer.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PrmDataBufferHeader {
    /// Data buffer signature ('PRMS' or 'PRMP').
    pub signature: u32,
    /// Length of the entire buffer including the header.
    pub length: u32,
}

/// Context buffer passed to a PRM handler.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PrmContextBuffer {
    /// Signature ('PRMC').
    pub signature: u32,
    /// Version of this interface.
    pub version: u16,
    /// Reserved field.
    pub reserved: u16,
    /// GUID of the PRM handler.
    pub handler_guid: EfiGuid,
    /// Static data buffer pointer.
    pub static_data_buffer: *mut u8,
    /// Pointer to runtime MMIO ranges.
    pub runtime_mmio_ranges: *mut PrmRuntimeMmioRanges,
}

/// Tracks context buffer and runtime MMIO range usage across a PRM module.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PrmModuleContextBuffers {
    /// GUID of the PRM module.
    pub module_guid: EfiGuid,
    /// Number of PRM context buffers.
    pub buffer_count: usize,
    /// Pointer to array of PRM context buffers.
    pub buffer: *mut PrmContextBuffer,
    /// Pointer to runtime MMIO ranges.
    pub runtime_mmio_ranges: *mut PrmRuntimeMmioRanges,
    /// Number of ACPI parameter buffer descriptors.
    pub acpi_parameter_buffer_descriptor_count: usize,
    /// Pointer to array of ACPI parameter buffer descriptors.
    pub acpi_parameter_buffer_descriptors: *mut AcpiParameterBufferDescriptor,
}

/// A Platform Runtime Mechanism handler function pointer.
pub type PrmHandler = unsafe extern "efiapi" fn(
    parameter_buffer: *mut c_void,
    context_buffer: *mut PrmContextBuffer,
) -> EfiStatus;

/// The PRM Configuration Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PrmConfigProtocol {
    /// Module context buffers.
    pub module_context_buffers: PrmModuleContextBuffers,
}
