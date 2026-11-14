//! # Firmware File System (FFS) section parsing
//!
//! Defined by the UEFI Platform Initialization (PI) Specification Volume 3:
//! Shared Architectural Elements.
//!
//! FFS files are composed of typed sections including executable code (PE32, TE),
//! dependency expressions, compression encapsulations, GUID-defined encapsulations,
//! and human-readable metadata.

use crate::base::EfiGuid;

// Standard FFS Section types
/// Wildcard matching all section types.
pub const EFI_SECTION_ALL: u8 = 0x00;
/// Encapsulation section containing compressed data.
pub const EFI_SECTION_COMPRESSION: u8 = 0x01;
/// Encapsulation section defined by an arbitrary GUID.
pub const EFI_SECTION_GUID_DEFINED: u8 = 0x02;
/// Disposable section that can be discarded after processing.
pub const EFI_SECTION_DISPOSABLE: u8 = 0x03;
/// PE32 / PE32+ executable image section.
pub const EFI_SECTION_PE32: u8 = 0x10;
/// Position-independent code section.
pub const EFI_SECTION_PIC: u8 = 0x11;
/// Terse Executable (TE) image section.
pub const EFI_SECTION_TE: u8 = 0x12;
/// DXE dependency expression section.
pub const EFI_SECTION_DXE_DEPEX: u8 = 0x13;
/// Build version and number section.
pub const EFI_SECTION_VERSION: u8 = 0x14;
/// Unicode user interface name section.
pub const EFI_SECTION_USER_INTERFACE: u8 = 0x15;
/// Compatibility 16-bit real mode code section.
pub const EFI_SECTION_COMPATIBILITY16: u8 = 0x16;
/// Firmware volume image encapsulated in a section.
pub const EFI_SECTION_FIRMWARE_VOLUME_IMAGE: u8 = 0x17;
/// Freeform payload with sub-type identifier GUID.
pub const EFI_SECTION_FREEFORM_SUBTYPE_GUID: u8 = 0x18;
/// Raw opaque binary data section.
pub const EFI_SECTION_RAW: u8 = 0x19;
/// PEI dependency expression section.
pub const EFI_SECTION_PEI_DEPEX: u8 = 0x1B;
/// SMM dependency expression section.
pub const EFI_SECTION_SMM_DEPEX: u8 = 0x1C;

/// Section compression types.
pub const EFI_NOT_COMPRESSED: u8 = 0x00;
/// Standard UEFI decompression algorithm (LZH).
pub const EFI_STANDARD_COMPRESSION: u8 = 0x01;
/// Custom decompression algorithm (e.g., Tiano or custom vendor compression).
pub const EFI_CUSTOMIZED_COMPRESSION: u8 = 0x02;

/// Common section header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FfsSectionHeader {
    /// Section payload and header total length in bytes.
    pub total_size: usize,
    /// Header length in bytes (4 bytes for standard, 8 bytes for extended).
    pub header_size: usize,
    /// Section type.
    pub section_type: u8,
}

/// A parsed section borrowing its payload from the input slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FfsSection<'a> {
    /// Section header metadata.
    pub header: FfsSectionHeader,
    /// Slice containing the section payload (excluding header).
    pub payload: &'a [u8],
}

/// Parses an FFS section header from the beginning of `input`.
///
/// Returns `None` if the input is too short or malformed.
#[must_use]
pub fn parse_section_header(input: &[u8]) -> Option<FfsSectionHeader> {
    if input.len() < 4 {
        return None;
    }
    let size_24 = u32::from(input[0]) | (u32::from(input[1]) << 8) | (u32::from(input[2]) << 16);
    let section_type = input[3];

    if size_24 == 0x00FF_FFFF {
        // Extended section header
        if input.len() < 8 {
            return None;
        }
        let ext_size = u32::from_le_bytes([input[4], input[5], input[6], input[7]]);
        let total_size = usize::try_from(ext_size).ok()?;
        if total_size < 8 {
            return None;
        }
        Some(FfsSectionHeader {
            total_size,
            header_size: 8,
            section_type,
        })
    } else {
        let total_size = usize::try_from(size_24).ok()?;
        if total_size < 4 {
            return None;
        }
        Some(FfsSectionHeader {
            total_size,
            header_size: 4,
            section_type,
        })
    }
}

/// Parses an entire section including validating bounds.
#[must_use]
pub fn parse_section(input: &[u8]) -> Option<FfsSection<'_>> {
    let header = parse_section_header(input)?;
    if input.len() < header.total_size {
        return None;
    }
    let payload = input.get(header.header_size..header.total_size)?;
    Some(FfsSection { header, payload })
}

/// Iterator over consecutive sections in a file payload.
#[derive(Clone, Debug)]
pub struct FfsSectionIter<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> FfsSectionIter<'a> {
    /// Creates a new iterator over section data.
    #[must_use]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }
}

impl<'a> Iterator for FfsSectionIter<'a> {
    type Item = FfsSection<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.data.len() {
            return None;
        }
        let remaining = self.data.get(self.offset..)?;
        let section = parse_section(remaining)?;
        // Sections in FFS are aligned to 4-byte boundaries
        let next_offset = self.offset.checked_add(section.header.total_size)?;
        self.offset = (next_offset.checked_add(3)? / 4) * 4;
        Some(section)
    }
}

/// Parsed compression section metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompressionSectionHeader {
    /// Uncompressed size of the enclosed data.
    pub uncompressed_length: u32,
    /// Compression type (`EFI_NOT_COMPRESSED`, `EFI_STANDARD_COMPRESSION`, or `EFI_CUSTOMIZED_COMPRESSION`).
    pub compression_type: u8,
}

/// Parses the compression header from a compression section's payload.
#[must_use]
pub fn parse_compression_header(payload: &[u8]) -> Option<(CompressionSectionHeader, &[u8])> {
    if payload.len() < 5 {
        return None;
    }
    let uncompressed_length = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let compression_type = payload[4];
    Some((
        CompressionSectionHeader {
            uncompressed_length,
            compression_type,
        },
        &payload[5..],
    ))
}

/// Parsed GUID-defined section metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuidDefinedSectionHeader {
    /// Section definition GUID.
    pub section_definition_guid: EfiGuid,
    /// Offset from beginning of section header to data.
    pub data_offset: u16,
    /// Attributes bitmask.
    pub attributes: u16,
}

/// Parses the header of a GUID-defined section.
#[must_use]
pub fn parse_guid_defined_header(payload: &[u8]) -> Option<(GuidDefinedSectionHeader, &[u8])> {
    if payload.len() < 20 {
        return None;
    }
    let guid = EfiGuid::from_bytes(payload.get(..16)?.try_into().ok()?);
    let data_offset = u16::from_le_bytes([payload[16], payload[17]]);
    let attributes = u16::from_le_bytes([payload[18], payload[19]]);
    let inner_offset = usize::from(data_offset).checked_sub(4)?; // Minus common section header length
    let data = payload.get(inner_offset..)?;
    Some((
        GuidDefinedSectionHeader {
            section_definition_guid: guid,
            data_offset,
            attributes,
        },
        data,
    ))
}

/// Parsed GUID-defined section 2 (extended) metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuidDefinedSection2Header {
    /// Section definition GUID.
    pub section_definition_guid: EfiGuid,
    /// Offset from beginning of extended section header to data.
    pub data_offset: u16,
    /// Attributes bitmask.
    pub attributes: u16,
}

/// Parses the header of an extended GUID-defined section 2.
#[must_use]
pub fn parse_guid_defined_section2_header(
    payload: &[u8],
) -> Option<(GuidDefinedSection2Header, &[u8])> {
    if payload.len() < 20 {
        return None;
    }
    let guid = EfiGuid::from_bytes(payload.get(..16)?.try_into().ok()?);
    let data_offset = u16::from_le_bytes([payload[16], payload[17]]);
    let attributes = u16::from_le_bytes([payload[18], payload[19]]);
    let inner_offset = usize::from(data_offset).checked_sub(8)?; // Minus extended 8-byte section header length
    let data = payload.get(inner_offset..)?;
    Some((
        GuidDefinedSection2Header {
            section_definition_guid: guid,
            data_offset,
            attributes,
        },
        data,
    ))
}
