//! # Platform hand-off blocks
//!
//! Bounded parsing for the hand-off block list described by the
//! Platform Initialization specification. Each block begins with an
//! eight-byte generic header carrying the type, total length and
//! reserved field, followed by a type-specific payload.
//!
//! Iteration validates header lengths, rejects truncation and
//! misaligned lengths, and stops at the end-of-list marker. Typed
//! decoders borrow from the caller's buffer and never allocate.

use crate::base::EfiGuid;
use crate::base::bytes::{read_u32_le, read_u64_le};

/// The generic header size in bytes.
pub const HOB_HEADER_SIZE: usize = 8;

/// The hand-off information table type.
pub const HOB_TYPE_HANDOFF: u16 = 0x0001;
/// The memory allocation type.
pub const HOB_TYPE_MEMORY_ALLOCATION: u16 = 0x0002;
/// The resource descriptor type.
pub const HOB_TYPE_RESOURCE_DESCRIPTOR: u16 = 0x0003;
/// The firmware volume type.
pub const HOB_TYPE_FIRMWARE_VOLUME: u16 = 0x0004;
/// The central processing unit type.
pub const HOB_TYPE_CPU: u16 = 0x0005;
/// The memory pool type.
pub const HOB_TYPE_MEMORY_POOL: u16 = 0x0006;
/// The firmware volume 2 type.
pub const HOB_TYPE_FIRMWARE_VOLUME2: u16 = 0x0007;
/// The load PEIM type.
pub const HOB_TYPE_LOAD_PEIM: u16 = 0x0008;
/// The GUID extension type.
pub const HOB_TYPE_GUID_EXTENSION: u16 = 0x0009;
/// The firmware volume file type.
pub const HOB_TYPE_FV_FILE: u16 = 0x000B;
/// The memory allocation module type.
pub const HOB_TYPE_MEMORY_ALLOCATION_MODULE: u16 = 0x000C;
/// The end-of-list marker.
pub const HOB_TYPE_END_OF_HOB_LIST: u16 = 0xFFFE;

/// A hand-off block decoding failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HobError {
    /// The remaining input was shorter than a generic header.
    TruncatedHeader,
    /// A declared length was below the header size.
    InvalidLength,
    /// A declared length exceeded the remaining input.
    OutOfBounds,
    /// A declared length was not a multiple of eight.
    MisalignedLength,
    /// The list ended without an end-of-list marker.
    MissingEnd,
}

impl core::fmt::Display for HobError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Self::TruncatedHeader => "truncated HOB header",
            Self::InvalidLength => "invalid HOB length",
            Self::OutOfBounds => "HOB length exceeds buffer",
            Self::MisalignedLength => "misaligned HOB length",
            Self::MissingEnd => "HOB list has no end marker",
        };
        formatter.write_str(text)
    }
}

/// A validated hand-off block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hob<'a> {
    /// The block type.
    pub hob_type: u16,
    /// The total block length including the header.
    pub length: u16,
    /// The payload after the generic header.
    pub payload: &'a [u8],
}

/// Iterates a hand-off block list.
#[derive(Clone, Debug)]
pub struct HobIter<'a> {
    bytes: &'a [u8],
    offset: usize,
    finished: bool,
}

impl<'a> Iterator for HobIter<'a> {
    type Item = Result<Hob<'a>, HobError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let remaining = self.bytes.get(self.offset..)?;
        if remaining.len() < HOB_HEADER_SIZE {
            self.finished = true;
            return Some(Err(HobError::TruncatedHeader));
        }
        let hob_type = u16::from_le_bytes([remaining[0], remaining[1]]);
        let length = u16::from_le_bytes([remaining[2], remaining[3]]);
        let total = usize::from(length);
        if total < HOB_HEADER_SIZE {
            self.finished = true;
            return Some(Err(HobError::InvalidLength));
        }
        if total % HOB_HEADER_SIZE != 0 {
            self.finished = true;
            return Some(Err(HobError::MisalignedLength));
        }
        let Some(payload) = remaining.get(HOB_HEADER_SIZE..total) else {
            self.finished = true;
            return Some(Err(HobError::OutOfBounds));
        };
        if hob_type == HOB_TYPE_END_OF_HOB_LIST {
            self.finished = true;
        } else {
            self.offset += total;
        }
        Some(Ok(Hob {
            hob_type,
            length,
            payload,
        }))
    }
}

/// Creates a hand-off block iterator.
#[must_use]
pub const fn hob_iter(bytes: &[u8]) -> HobIter<'_> {
    HobIter {
        bytes,
        offset: 0,
        finished: false,
    }
}

/// The hand-off information table payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandoffInfoTable {
    /// The structure version.
    pub version: u32,
    /// The boot mode.
    pub boot_mode: u32,
    /// The top of the firmware memory region.
    pub memory_top: u64,
    /// The bottom of the firmware memory region.
    pub memory_bottom: u64,
    /// The top of the free memory region.
    pub free_memory_top: u64,
    /// The bottom of the free memory region.
    pub free_memory_bottom: u64,
    /// The address of the end-of-list marker.
    pub end_of_hob_list: u64,
}

/// A memory allocation payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryAllocation {
    /// The base address of the region.
    pub base_address: u64,
    /// The region length in bytes.
    pub length: u64,
    /// The firmware memory type.
    pub memory_type: u32,
}

/// A resource descriptor payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceDescriptor {
    /// The owner of the resource.
    pub owner: EfiGuid,
    /// The resource type.
    pub resource_type: ResourceType,
    /// The resource attributes.
    pub attributes: u32,
    /// The physical start address.
    pub physical_start: u64,
    /// The resource length in bytes.
    pub resource_length: u64,
}

/// A resource descriptor type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceType {
    /// System memory.
    SystemMemory,
    /// System I/O.
    SystemIo,
    /// Firmware device.
    FirmwareDevice,
    /// Memory-mapped I/O.
    MemoryMappedIo,
    /// Memory-mapped I/O port space.
    MemoryMappedIoPortSpace,
    /// PCI I/O.
    PciIo,
    /// 32-bit PCI memory.
    PciMemory32,
    /// 64-bit PCI memory.
    PciMemory64,
    /// A type outside the standard range.
    Other(u32),
}

impl From<u32> for ResourceType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::SystemMemory,
            1 => Self::SystemIo,
            2 => Self::FirmwareDevice,
            3 => Self::MemoryMappedIo,
            4 => Self::MemoryMappedIoPortSpace,
            5 => Self::PciIo,
            6 => Self::PciMemory32,
            7 => Self::PciMemory64,
            other => Self::Other(other),
        }
    }
}

/// A GUID extension payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuidHob<'a> {
    /// The extension name.
    pub name: EfiGuid,
    /// The extension data.
    pub data: &'a [u8],
}

/// A firmware volume payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirmwareVolumeHob {
    /// The volume base address.
    pub base_address: u64,
    /// The volume length in bytes.
    pub length: u64,
}

/// A firmware volume 2 payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirmwareVolume2Hob {
    /// The volume base address.
    pub base_address: u64,
    /// The volume length in bytes.
    pub length: u64,
    /// The volume name.
    pub volume_name: EfiGuid,
    /// The file name.
    pub file_name: EfiGuid,
}

/// Reads a GUID at an offset.
fn guid_at(bytes: &[u8], offset: usize) -> Option<EfiGuid> {
    let slice = bytes.get(offset..offset.checked_add(16)?)?;
    let mut raw = [0_u8; 16];
    raw.copy_from_slice(slice);
    Some(EfiGuid::from_bytes(raw))
}

/// Decodes a hand-off information table payload.
#[must_use]
pub fn parse_handoff_info_table(hob: &Hob<'_>) -> Option<HandoffInfoTable> {
    if hob.hob_type != HOB_TYPE_HANDOFF {
        return None;
    }
    Some(HandoffInfoTable {
        version: read_u32_le(hob.payload, 0)?,
        boot_mode: read_u32_le(hob.payload, 4)?,
        memory_top: read_u64_le(hob.payload, 8)?,
        memory_bottom: read_u64_le(hob.payload, 16)?,
        free_memory_top: read_u64_le(hob.payload, 24)?,
        free_memory_bottom: read_u64_le(hob.payload, 32)?,
        end_of_hob_list: read_u64_le(hob.payload, 40)?,
    })
}

/// Decodes a memory allocation payload.
#[must_use]
pub fn parse_memory_allocation(hob: &Hob<'_>) -> Option<MemoryAllocation> {
    if hob.hob_type != HOB_TYPE_MEMORY_ALLOCATION {
        return None;
    }
    Some(MemoryAllocation {
        base_address: read_u64_le(hob.payload, 0)?,
        length: read_u64_le(hob.payload, 8)?,
        memory_type: read_u32_le(hob.payload, 16)?,
    })
}

/// Decodes a resource descriptor payload.
#[must_use]
pub fn parse_resource_descriptor(hob: &Hob<'_>) -> Option<ResourceDescriptor> {
    if hob.hob_type != HOB_TYPE_RESOURCE_DESCRIPTOR {
        return None;
    }
    Some(ResourceDescriptor {
        owner: guid_at(hob.payload, 0)?,
        resource_type: ResourceType::from(read_u32_le(hob.payload, 16)?),
        attributes: read_u32_le(hob.payload, 20)?,
        physical_start: read_u64_le(hob.payload, 24)?,
        resource_length: read_u64_le(hob.payload, 32)?,
    })
}

/// Decodes a GUID extension payload.
#[must_use]
pub fn parse_guid_hob<'a>(hob: &Hob<'a>) -> Option<GuidHob<'a>> {
    if hob.hob_type != HOB_TYPE_GUID_EXTENSION {
        return None;
    }
    Some(GuidHob {
        name: guid_at(hob.payload, 0)?,
        data: hob.payload.get(16..)?,
    })
}

/// Decodes a firmware volume payload.
#[must_use]
pub fn parse_firmware_volume(hob: &Hob<'_>) -> Option<FirmwareVolumeHob> {
    if hob.hob_type != HOB_TYPE_FIRMWARE_VOLUME {
        return None;
    }
    Some(FirmwareVolumeHob {
        base_address: read_u64_le(hob.payload, 0)?,
        length: read_u64_le(hob.payload, 8)?,
    })
}

/// Decodes a firmware volume 2 payload.
#[must_use]
pub fn parse_firmware_volume2(hob: &Hob<'_>) -> Option<FirmwareVolume2Hob> {
    if hob.hob_type != HOB_TYPE_FIRMWARE_VOLUME2 {
        return None;
    }
    Some(FirmwareVolume2Hob {
        base_address: read_u64_le(hob.payload, 0)?,
        length: read_u64_le(hob.payload, 8)?,
        volume_name: guid_at(hob.payload, 16)?,
        file_name: guid_at(hob.payload, 32)?,
    })
}

/// Returns whether `bytes` contains a complete, well-formed block list.
#[must_use]
pub fn is_valid_hob_list(bytes: &[u8]) -> bool {
    let mut ended = false;
    for item in hob_iter(bytes) {
        match item {
            Ok(hob) => {
                if hob.hob_type == HOB_TYPE_END_OF_HOB_LIST {
                    ended = true;
                    break;
                }
            }
            Err(_) => return false,
        }
    }
    ended
}

/// Returns the number of blocks before the end marker, or `None` for a
/// malformed list.
#[must_use]
pub fn hob_count(bytes: &[u8]) -> Option<usize> {
    let mut count = 0_usize;
    for item in hob_iter(bytes) {
        let hob = item.ok()?;
        if hob.hob_type == HOB_TYPE_END_OF_HOB_LIST {
            return Some(count);
        }
        count = count.checked_add(1)?;
    }
    None
}
