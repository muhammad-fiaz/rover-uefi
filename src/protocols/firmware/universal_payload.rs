//! # Universal Payload (UPL)
//!
//! Provides the Universal Payload information, generic header, and Hand-Off Block (HOB)
//! structures defined in the Universal Payload Specification (v0.75+).
//!
//! Universal Payload standardizes the bootloader-to-payload interface (such as early-stage
//! platform initialization handing off to a native UEFI payload).

use crate::base::{EfiGuid, EfiPhysicalAddress};
use core::ffi::c_void;

/// Universal Payload identifier signature (`'PLDH'`).
pub const UNIVERSAL_PAYLOAD_IDENTIFIER: u32 = 0x4844_4C50;

/// Section name for the Universal Payload Information Header.
pub const UNIVERSAL_PAYLOAD_INFO_SEC_NAME: &str = ".upld_info";

/// Prefix for Universal Payload extra data sections.
pub const UNIVERSAL_PAYLOAD_EXTRA_SEC_NAME_PREFIX: &str = ".upld.";

/// Universal Payload Base GUID: `{03d4c61d-2713-4ec5-a1cc-883be9dc18e5}`.
pub const UNIVERSAL_PAYLOAD_BASE_GUID: EfiGuid = EfiGuid::new(
    0x03d4_c61d,
    0x2713,
    0x4ec5,
    [0xa1, 0xcc, 0x88, 0x3b, 0xe9, 0xdc, 0x18, 0xe5],
);

/// Universal Payload Serial Port Info GUID: `{aa7e190d-be21-4409-8e67-a2cd0f61e170}`.
pub const UNIVERSAL_PAYLOAD_SERIAL_PORT_INFO_GUID: EfiGuid = EfiGuid::new(
    0xaa7e_190d,
    0xbe21,
    0x4409,
    [0x8e, 0x67, 0xa2, 0xcd, 0x0f, 0x61, 0xe1, 0x70],
);

/// Universal Payload PCI Root Bridge Info GUID: `{ec4ebacb-2638-416e-be80-e5fa4b511901}`.
pub const UNIVERSAL_PAYLOAD_PCI_ROOT_BRIDGE_INFO_GUID: EfiGuid = EfiGuid::new(
    0xec4e_bacb,
    0x2638,
    0x416e,
    [0xbe, 0x80, 0xe5, 0xfa, 0x4b, 0x51, 0x19, 0x01],
);

/// Universal Payload ACPI Table GUID: `{9f9a9506-5597-4515-bab6-8bcde784ba87}`.
pub const UNIVERSAL_PAYLOAD_ACPI_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x9f9a_9506,
    0x5597,
    0x4515,
    [0xba, 0xb6, 0x8b, 0xcd, 0xe7, 0x84, 0xba, 0x87],
);

/// Universal Payload SMBIOS Table GUID: `{590a0d26-06e5-4d20-8a82-59ea1b34982d}`.
pub const UNIVERSAL_PAYLOAD_SMBIOS_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x590a_0d26,
    0x06e5,
    0x4d20,
    [0x8a, 0x82, 0x59, 0xea, 0x1b, 0x34, 0x98, 0x2d],
);

/// Universal Payload SMBIOS3 Table GUID: `{92b7896c-3362-46ce-99b3-4f5e3c34eb42}`.
pub const UNIVERSAL_PAYLOAD_SMBIOS3_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x92b7_896c,
    0x3362,
    0x46ce,
    [0x99, 0xb3, 0x4f, 0x5e, 0x3c, 0x34, 0xeb, 0x42],
);

/// Universal Payload Extra Data GUID: `{15a5baf6-1c91-467d-9dfb-319d178d4bb4}`.
pub const UNIVERSAL_PAYLOAD_EXTRA_DATA_GUID: EfiGuid = EfiGuid::new(
    0x15a5_baf6,
    0x1c91,
    0x467d,
    [0x9d, 0xfb, 0x31, 0x9d, 0x17, 0x8d, 0x4b, 0xb4],
);

/// Universal Payload Device Tree GUID: `{6784b889-b13c-4c3b-ae4b-0f0a2e320ea3}`.
pub const UNIVERSAL_PAYLOAD_DEVICE_TREE_GUID: EfiGuid = EfiGuid::new(
    0x6784_b889,
    0xb13c,
    0x4c3b,
    [0xae, 0x4b, 0x0f, 0x0a, 0x2e, 0x32, 0x0e, 0xa3],
);

/// Universal Payload Serial Port Device Parent Info GUID: `{c89c359c-d316-4ff8-a98f-607d2ced1fed}`.
pub const UNIVERSAL_PAYLOAD_SERIAL_PORT_PARENT_DEVICE_INFO_GUID: EfiGuid = EfiGuid::new(
    0xc89c_359c,
    0xd316,
    0x4ff8,
    [0xa9, 0x8f, 0x60, 0x7d, 0x2c, 0xed, 0x1f, 0xed],
);

/// Universal Payload PCI Segment Info HOB GUID: `{37e0e3a9-b3fc-4e85-972b-4082fc794054}`.
pub const UNIVERSAL_PAYLOAD_PCI_SEGMENT_INFO_HOB_GUID: EfiGuid = EfiGuid::new(
    0x37e0_e3a9,
    0xb3fc,
    0x4e85,
    [0x97, 0x2b, 0x40, 0x82, 0xfc, 0x79, 0x40, 0x54],
);

/// Universal Payload Command Line GUID: `{b5aeb34f-3047-4955-b880-add36d86dc0f}`.
pub const UNIVERSAL_PAYLOAD_COMMAND_LINE_GUID: EfiGuid = EfiGuid::new(
    0xb5ae_b34f,
    0x3047,
    0x4955,
    [0xb8, 0x80, 0xad, 0xd3, 0x6d, 0x86, 0xdc, 0x0f],
);

/// Universal Payload Base structure revision.
pub const UNIVERSAL_PAYLOAD_BASE_REVISION: u8 = 1;
/// Universal Payload Serial Port Info structure revision.
pub const UNIVERSAL_PAYLOAD_SERIAL_PORT_INFO_REVISION: u8 = 1;
/// Universal Payload PCI Root Bridges structure revision.
pub const UNIVERSAL_PAYLOAD_PCI_ROOT_BRIDGES_REVISION: u8 = 1;
/// Universal Payload ACPI Table structure revision.
pub const UNIVERSAL_PAYLOAD_ACPI_TABLE_REVISION: u8 = 1;
/// Universal Payload SMBIOS Table structure revision.
pub const UNIVERSAL_PAYLOAD_SMBIOS_TABLE_REVISION: u8 = 1;

/// Non-relocatable flag for aperture address mapping.
pub const NON_RELOCATABLE: u32 = 1 << 31;
/// Prefetchable flag for aperture address mapping.
pub const PREFETCHABLE: u32 = 1 << 30;
/// Configuration space address type.
pub const SS_CONFIGURATION_SPACE: u32 = 0;
/// I/O space address type.
pub const SS_IO_SPACE: u32 = 1 << 24;
/// 32-bit memory space address type.
pub const SS_32BIT_MEMORY_SPACE: u32 = 1 << 25;
/// 64-bit memory space address type.
pub const SS_64BIT_MEMORY_SPACE: u32 = (1 << 24) | (1 << 25);

/// Universal Payload Information Header embedded in the payload binary.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct UniversalPayloadInfoHeader {
    /// Identifier ('PLDH').
    pub identifier: u32,
    /// Length of this header in bytes.
    pub header_length: u32,
    /// Specification revision.
    pub spec_revision: u16,
    /// Reserved.
    pub reserved: [u8; 2],
    /// Payload revision.
    pub revision: u32,
    /// Attribute flags.
    pub attribute: u32,
    /// Capability flags.
    pub capability: u32,
    /// Producer ID string (ASCII, space-padded).
    pub producer_id: [u8; 16],
    /// Image ID string (ASCII, space-padded).
    pub image_id: [u8; 16],
}

/// Generic header preceding all Universal Payload HOB structures.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniversalPayloadGenericHeader {
    /// Structure revision.
    pub revision: u8,
    /// Reserved.
    pub reserved: u8,
    /// Structure length including header.
    pub length: u16,
}

/// Universal Payload Base entry point information.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct UniversalPayloadBase {
    /// Generic HOB header.
    pub header: UniversalPayloadGenericHeader,
    /// Physical entry point address of the payload.
    pub entry: EfiPhysicalAddress,
}

/// Universal Payload Serial Port Information HOB.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct UniversalPayloadSerialPortInfo {
    /// Generic HOB header.
    pub header: UniversalPayloadGenericHeader,
    /// Indicates whether MMIO or I/O ports are used.
    pub use_mmio: bool,
    /// Register stride in bytes (e.g. 1 for byte, 4 for dword).
    pub register_stride: u8,
    /// Baud rate (e.g. 115200).
    pub baud_rate: u32,
    /// Physical base address of the UART registers.
    pub register_base: EfiPhysicalAddress,
}

/// Universal Payload PCI Root Bridge Aperture.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UniversalPayloadPciRootBridgeAperture {
    /// Base address of aperture.
    pub base: u64,
    /// Limit address of aperture.
    pub limit: u64,
    /// Address translation offset between host and device.
    pub translation: u64,
}

/// Universal Payload PCI Root Bridge Descriptor.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct UniversalPayloadPciRootBridge {
    /// PCI Segment number.
    pub segment: u32,
    /// Supported attributes (e.g. `EFI_PCI_ATTRIBUTE_*`).
    pub supports: u64,
    /// Initial attributes.
    pub attributes: u64,
    /// Indicates whether root bridge supports DMA above 4GB.
    pub dma_above_4g: bool,
    /// True when root bridge only supports 256-byte config space.
    pub no_extended_config_space: bool,
    /// Allocation attributes.
    pub allocation_attributes: u64,
    /// Bus aperture.
    pub bus: UniversalPayloadPciRootBridgeAperture,
    /// I/O aperture.
    pub io: UniversalPayloadPciRootBridgeAperture,
    /// MMIO aperture below 4GB.
    pub mem: UniversalPayloadPciRootBridgeAperture,
    /// MMIO aperture above 4GB.
    pub mem_above_4g: UniversalPayloadPciRootBridgeAperture,
    /// Prefetchable MMIO aperture below 4GB.
    pub pmem: UniversalPayloadPciRootBridgeAperture,
    /// Prefetchable MMIO aperture above 4GB.
    pub pmem_above_4g: UniversalPayloadPciRootBridgeAperture,
    /// ACPI Hardware ID (HID).
    pub hid: u32,
    /// ACPI Unique ID (UID).
    pub uid: u32,
}

/// Universal Payload ACPI Table HOB.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct UniversalPayloadAcpiTable {
    /// Generic HOB header.
    pub header: UniversalPayloadGenericHeader,
    /// Physical address of the ACPI Root System Description Pointer (RSDP).
    pub rsdp: EfiPhysicalAddress,
}

/// Universal Payload SMBIOS Table HOB.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct UniversalPayloadSmbiosTable {
    /// Generic HOB header.
    pub header: UniversalPayloadGenericHeader,
    /// Physical address of the SMBIOS entry point structure.
    pub smbios_entry_point: EfiPhysicalAddress,
}

/// Universal Payload entry point function prototype.
pub type UniversalPayloadEntryPoint = unsafe extern "efiapi" fn(hob_list: *const c_void);

/// Error conditions encountered during Universal Payload parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniversalPayloadError {
    /// Buffer is too small to contain the required structure.
    BufferTooSmall,
    /// Header identifier signature does not match expected value.
    InvalidSignature,
    /// Structure length is less than minimum valid header length.
    InvalidLength,
    /// Unsupported structure or specification revision.
    UnsupportedRevision,
}

/// Parses and validates a `UniversalPayloadInfoHeader` from a byte slice.
///
/// # Errors
/// Returns [`UniversalPayloadError`] if slice is smaller than header size or signature is invalid.
pub fn parse_info_header(
    slice: &[u8],
) -> Result<UniversalPayloadInfoHeader, UniversalPayloadError> {
    if slice.len() < core::mem::size_of::<UniversalPayloadInfoHeader>() {
        return Err(UniversalPayloadError::BufferTooSmall);
    }

    let mut id_raw = [0_u8; 4];
    id_raw.copy_from_slice(&slice[0..4]);
    let identifier = u32::from_le_bytes(id_raw);
    if identifier != UNIVERSAL_PAYLOAD_IDENTIFIER {
        return Err(UniversalPayloadError::InvalidSignature);
    }

    let mut len_raw = [0_u8; 4];
    len_raw.copy_from_slice(&slice[4..8]);
    let header_length = u32::from_le_bytes(len_raw);
    if (header_length as usize) < core::mem::size_of::<UniversalPayloadInfoHeader>() {
        return Err(UniversalPayloadError::InvalidLength);
    }

    let mut spec_raw = [0_u8; 2];
    spec_raw.copy_from_slice(&slice[8..10]);
    let spec_revision = u16::from_le_bytes(spec_raw);

    let reserved = [slice[10], slice[11]];

    let mut rev_raw = [0_u8; 4];
    rev_raw.copy_from_slice(&slice[12..16]);
    let revision = u32::from_le_bytes(rev_raw);

    let mut attr_raw = [0_u8; 4];
    attr_raw.copy_from_slice(&slice[16..20]);
    let attribute = u32::from_le_bytes(attr_raw);

    let mut cap_raw = [0_u8; 4];
    cap_raw.copy_from_slice(&slice[20..24]);
    let capability = u32::from_le_bytes(cap_raw);

    let mut producer_id = [0_u8; 16];
    producer_id.copy_from_slice(&slice[24..40]);

    let mut image_id = [0_u8; 16];
    image_id.copy_from_slice(&slice[40..56]);

    Ok(UniversalPayloadInfoHeader {
        identifier,
        header_length,
        spec_revision,
        reserved,
        revision,
        attribute,
        capability,
        producer_id,
        image_id,
    })
}

/// Parses and validates a `UniversalPayloadGenericHeader` from a byte slice.
///
/// # Errors
/// Returns [`UniversalPayloadError`] if slice is smaller than generic header size.
pub fn parse_generic_header(
    slice: &[u8],
) -> Result<UniversalPayloadGenericHeader, UniversalPayloadError> {
    if slice.len() < core::mem::size_of::<UniversalPayloadGenericHeader>() {
        return Err(UniversalPayloadError::BufferTooSmall);
    }

    let revision = slice[0];
    let reserved = slice[1];
    let mut len_raw = [0_u8; 2];
    len_raw.copy_from_slice(&slice[2..4]);
    let length = u16::from_le_bytes(len_raw);

    if (length as usize) < core::mem::size_of::<UniversalPayloadGenericHeader>() {
        return Err(UniversalPayloadError::InvalidLength);
    }

    Ok(UniversalPayloadGenericHeader {
        revision,
        reserved,
        length,
    })
}

/// Parses a `UniversalPayloadSerialPortInfo` HOB payload from a byte slice.
///
/// # Errors
/// Returns [`UniversalPayloadError`] if slice is smaller than structure size.
pub fn parse_serial_port_info(
    slice: &[u8],
) -> Result<UniversalPayloadSerialPortInfo, UniversalPayloadError> {
    if slice.len() < core::mem::size_of::<UniversalPayloadSerialPortInfo>() {
        return Err(UniversalPayloadError::BufferTooSmall);
    }

    let generic = parse_generic_header(slice)?;
    let use_mmio = slice[4] != 0;
    let register_stride = slice[5];

    let mut baud_raw = [0_u8; 4];
    baud_raw.copy_from_slice(&slice[6..10]);
    let baud_rate = u32::from_le_bytes(baud_raw);

    let mut base_raw = [0_u8; 8];
    base_raw.copy_from_slice(&slice[10..18]);
    let register_base = EfiPhysicalAddress::from_u64(u64::from_le_bytes(base_raw));

    Ok(UniversalPayloadSerialPortInfo {
        header: generic,
        use_mmio,
        register_stride,
        baud_rate,
        register_base,
    })
}

/// Parses a `UniversalPayloadAcpiTable` HOB payload from a byte slice.
///
/// # Errors
/// Returns [`UniversalPayloadError`] if slice is smaller than structure size.
pub fn parse_acpi_table(slice: &[u8]) -> Result<UniversalPayloadAcpiTable, UniversalPayloadError> {
    if slice.len() < core::mem::size_of::<UniversalPayloadAcpiTable>() {
        return Err(UniversalPayloadError::BufferTooSmall);
    }

    let generic = parse_generic_header(slice)?;
    let mut rsdp_raw = [0_u8; 8];
    rsdp_raw.copy_from_slice(&slice[4..12]);
    let rsdp = EfiPhysicalAddress::from_u64(u64::from_le_bytes(rsdp_raw));

    Ok(UniversalPayloadAcpiTable {
        header: generic,
        rsdp,
    })
}

/// Parses a `UniversalPayloadSmbiosTable` HOB payload from a byte slice.
///
/// # Errors
/// Returns [`UniversalPayloadError`] if slice is smaller than structure size.
pub fn parse_smbios_table(
    slice: &[u8],
) -> Result<UniversalPayloadSmbiosTable, UniversalPayloadError> {
    if slice.len() < core::mem::size_of::<UniversalPayloadSmbiosTable>() {
        return Err(UniversalPayloadError::BufferTooSmall);
    }

    let generic = parse_generic_header(slice)?;
    let mut smbios_raw = [0_u8; 8];
    smbios_raw.copy_from_slice(&slice[4..12]);
    let smbios_entry_point = EfiPhysicalAddress::from_u64(u64::from_le_bytes(smbios_raw));

    Ok(UniversalPayloadSmbiosTable {
        header: generic,
        smbios_entry_point,
    })
}
