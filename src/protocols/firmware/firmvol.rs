//! # Firmware volumes and files
//!
//! Bounded parsing for firmware volumes (`FV`) and firmware files
//! (`FFS`), as well as specification protocol interfaces:
//! `EFI_FIRMWARE_VOLUME2_PROTOCOL` and `EFI_FIRMWARE_VOLUME_BLOCK2_PROTOCOL`.
//! Flash updates remain a firmware operation.

use crate::abi::types::UIntN;
use crate::base::bytes::{read_u16_le, read_u32_le};
use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};
use core::ffi::c_void;

/// The firmware volume signature (`_FVH` little endian).
pub const FV_SIGNATURE: u32 = 0x48_46_56_5F;

/// The firmware volume header length.
pub const FV_HEADER_LENGTH: usize = 72;

/// `EFI_FIRMWARE_VOLUME2_PROTOCOL_GUID`.
pub const FIRMWARE_VOLUME2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x220E_73B6,
    0x6BDB,
    0x4413,
    [0x84, 0x05, 0xB9, 0x74, 0xB1, 0x08, 0x61, 0x9A],
);

/// `EFI_FIRMWARE_VOLUME_BLOCK_PROTOCOL_GUID` / `EFI_FIRMWARE_VOLUME_BLOCK2_PROTOCOL_GUID`.
pub const FIRMWARE_VOLUME_BLOCK2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x8F64_4FA9,
    0xE850,
    0x4DB1,
    [0x9C, 0xE2, 0x0B, 0x44, 0x69, 0x8E, 0x8D, 0xA4],
);

// EFI_FV_ATTRIBUTES bit definitions
/// Volume attributes: Read disable capability.
pub const EFI_FV2_READ_DISABLE_CAP: u64 = 0x0000_0000_0000_0001;
/// Volume attributes: Read enable capability.
pub const EFI_FV2_READ_ENABLE_CAP: u64 = 0x0000_0000_0000_0002;
/// Volume attributes: Read status.
pub const EFI_FV2_READ_STATUS: u64 = 0x0000_0000_0000_0004;
/// Volume attributes: Write disable capability.
pub const EFI_FV2_WRITE_DISABLE_CAP: u64 = 0x0000_0000_0000_0008;
/// Volume attributes: Write enable capability.
pub const EFI_FV2_WRITE_ENABLE_CAP: u64 = 0x0000_0000_0000_0010;
/// Volume attributes: Write status.
pub const EFI_FV2_WRITE_STATUS: u64 = 0x0000_0000_0000_0020;
/// Volume attributes: Lock capability.
pub const EFI_FV2_LOCK_CAP: u64 = 0x0000_0000_0000_0040;
/// Volume attributes: Lock status.
pub const EFI_FV2_LOCK_STATUS: u64 = 0x0000_0000_0000_0080;
/// Volume attributes: Reliable write policy.
pub const EFI_FV2_WRITE_POLICY_RELIABLE: u64 = 0x0000_0000_0000_0100;
/// Volume attributes: Read lock capability.
pub const EFI_FV2_READ_LOCK_CAP: u64 = 0x0000_0000_0000_1000;
/// Volume attributes: Read lock status.
pub const EFI_FV2_READ_LOCK_STATUS: u64 = 0x0000_0000_0000_2000;
/// Volume attributes: Write lock capability.
pub const EFI_FV2_WRITE_LOCK_CAP: u64 = 0x0000_0000_0000_4000;
/// Volume attributes: Write lock status.
pub const EFI_FV2_WRITE_LOCK_STATUS: u64 = 0x0000_0000_0000_8000;

// EFI_FVB2_ATTRIBUTES bit definitions
/// Block attributes: Read disabled capability.
pub const EFI_FVB2_READ_DISABLED_CAP: u32 = 0x0000_0001;
/// Block attributes: Read enabled capability.
pub const EFI_FVB2_READ_ENABLED_CAP: u32 = 0x0000_0002;
/// Block attributes: Read status.
pub const EFI_FVB2_READ_STATUS: u32 = 0x0000_0004;
/// Block attributes: Write disabled capability.
pub const EFI_FVB2_WRITE_DISABLED_CAP: u32 = 0x0000_0008;
/// Block attributes: Write enabled capability.
pub const EFI_FVB2_WRITE_ENABLED_CAP: u32 = 0x0000_0010;
/// Block attributes: Write status.
pub const EFI_FVB2_WRITE_STATUS: u32 = 0x0000_0020;
/// Block attributes: Lock capability.
pub const EFI_FVB2_LOCK_CAP: u32 = 0x0000_0040;
/// Block attributes: Lock status.
pub const EFI_FVB2_LOCK_STATUS: u32 = 0x0000_0080;
/// Block attributes: Sticky write.
pub const EFI_FVB2_STICKY_WRITE: u32 = 0x0000_0200;
/// Block attributes: Memory mapped.
pub const EFI_FVB2_MEMORY_MAPPED: u32 = 0x0000_0400;
/// Block attributes: Erase polarity.
pub const EFI_FVB2_ERASE_POLARITY: u32 = 0x0000_0800;
/// Block attributes: Read lock capability.
pub const EFI_FVB2_READ_LOCK_CAP: u32 = 0x0000_1000;
/// Block attributes: Read lock status.
pub const EFI_FVB2_READ_LOCK_STATUS: u32 = 0x0000_2000;
/// Block attributes: Write lock capability.
pub const EFI_FVB2_WRITE_LOCK_CAP: u32 = 0x0000_4000;
/// Block attributes: Write lock status.
pub const EFI_FVB2_WRITE_LOCK_STATUS: u32 = 0x0000_8000;

/// Unreliable write policy.
pub const EFI_FV_UNRELIABLE_WRITE: u32 = 0x0000_0000;
/// Reliable write policy.
pub const EFI_FV_RELIABLE_WRITE: u32 = 0x0000_0001;

/// Terminator value for block erase lists.
pub const EFI_LBA_LIST_TERMINATOR: u64 = 0xFFFF_FFFF_FFFF_FFFF;

/// Structure describing file data to write via `EFI_FIRMWARE_VOLUME2_PROTOCOL.WriteFile`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFvWriteFileData {
    /// Pointer to the file name GUID.
    pub name_guid: *const EfiGuid,
    /// Indicates the type of file to write.
    pub file_type: u8,
    /// Indicates file attributes.
    pub file_attributes: u32,
    /// Pointer to the buffer containing the file data.
    pub buffer: *const c_void,
    /// Size of the buffer in bytes.
    pub buffer_size: UIntN,
}

// Firmware Volume 2 Function Pointers
/// Function pointer for `EFI_FV_GET_ATTRIBUTES`.
pub type EfiFvGetAttributes = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolume2Protocol,
    fv_attributes: *mut u64,
) -> EfiStatus;

/// Function pointer for `EFI_FV_SET_ATTRIBUTES`.
pub type EfiFvSetAttributes = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolume2Protocol,
    fv_attributes: *mut u64,
) -> EfiStatus;

/// Function pointer for `EFI_FV_READ_FILE`.
pub type EfiFvReadFile = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolume2Protocol,
    name_guid: *const EfiGuid,
    buffer: *mut *mut c_void,
    buffer_size: *mut UIntN,
    found_type: *mut u8,
    file_attributes: *mut u32,
    authentication_status: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_FV_READ_SECTION`.
pub type EfiFvReadSection = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolume2Protocol,
    name_guid: *const EfiGuid,
    section_type: u8,
    section_instance: UIntN,
    buffer: *mut *mut c_void,
    buffer_size: *mut UIntN,
    authentication_status: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_FV_WRITE_FILE`.
pub type EfiFvWriteFile = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolume2Protocol,
    number_of_files: u32,
    write_policy: u32,
    file_data: *const EfiFvWriteFileData,
) -> EfiStatus;

/// Function pointer for `EFI_FV_GET_NEXT_FILE`.
pub type EfiFvGetNextFile = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolume2Protocol,
    key: *mut c_void,
    file_type: *mut u8,
    name_guid: *mut EfiGuid,
    attributes: *mut u32,
    size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_FV_GET_INFO`.
pub type EfiFvGetInfo = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolume2Protocol,
    information_type: *const EfiGuid,
    buffer_size: *mut UIntN,
    buffer: *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_FV_SET_INFO`.
pub type EfiFvSetInfo = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolume2Protocol,
    information_type: *const EfiGuid,
    buffer_size: UIntN,
    buffer: *const c_void,
) -> EfiStatus;

/// The `EFI_FIRMWARE_VOLUME2_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFirmwareVolume2Protocol {
    /// Retrieves volume attributes.
    pub get_volume_attributes: EfiFvGetAttributes,
    /// Sets volume attributes.
    pub set_volume_attributes: EfiFvSetAttributes,
    /// Reads a file from the firmware volume.
    pub read_file: EfiFvReadFile,
    /// Reads a section from a file in the firmware volume.
    pub read_section: EfiFvReadSection,
    /// Writes files to the firmware volume.
    pub write_file: EfiFvWriteFile,
    /// Iterates the next file in the firmware volume.
    pub get_next_file: EfiFvGetNextFile,
    /// Key size in bytes for `GetNextFile`.
    pub key_size: u32,
    /// Handle of the parent firmware volume.
    pub parent_handle: EfiHandle,
    /// Retrieves information about the firmware volume.
    pub get_info: EfiFvGetInfo,
    /// Sets information on the firmware volume.
    pub set_info: EfiFvSetInfo,
}

// Firmware Volume Block 2 Function Pointers
/// Function pointer for `EFI_FVB_GET_ATTRIBUTES`.
pub type EfiFvbGetAttributes = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolumeBlock2Protocol,
    attributes: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_FVB_SET_ATTRIBUTES`.
pub type EfiFvbSetAttributes = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolumeBlock2Protocol,
    attributes: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_FVB_GET_PHYSICAL_ADDRESS`.
pub type EfiFvbGetPhysicalAddress = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolumeBlock2Protocol,
    address: *mut EfiPhysicalAddress,
) -> EfiStatus;

/// Function pointer for `EFI_FVB_GET_BLOCK_SIZE`.
pub type EfiFvbGetBlockSize = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolumeBlock2Protocol,
    lba: u64,
    block_size: *mut UIntN,
    number_of_blocks: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_FVB_READ`.
pub type EfiFvbRead = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolumeBlock2Protocol,
    lba: u64,
    offset: UIntN,
    num_bytes: *mut UIntN,
    buffer: *mut u8,
) -> EfiStatus;

/// Function pointer for `EFI_FVB_WRITE`.
pub type EfiFvbWrite = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolumeBlock2Protocol,
    lba: u64,
    offset: UIntN,
    num_bytes: *mut UIntN,
    buffer: *const u8,
) -> EfiStatus;

/// Function pointer for `EFI_FVB_ERASE_BLOCKS`.
pub type EfiFvbEraseBlocks = unsafe extern "efiapi" fn(
    this: *const EfiFirmwareVolumeBlock2Protocol,
    first_lba: u64,
    num_blocks: UIntN,
    terminator: u64,
) -> EfiStatus;

/// The `EFI_FIRMWARE_VOLUME_BLOCK2_PROTOCOL` raw ABI structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFirmwareVolumeBlock2Protocol {
    /// Retrieves block attributes.
    pub get_attributes: EfiFvbGetAttributes,
    /// Sets block attributes.
    pub set_attributes: EfiFvbSetAttributes,
    /// Retrieves base physical address for memory-mapped volumes.
    pub get_physical_address: EfiFvbGetPhysicalAddress,
    /// Retrieves size of the requested block.
    pub get_block_size: EfiFvbGetBlockSize,
    /// Reads bytes from a block into a buffer.
    pub read: EfiFvbRead,
    /// Writes bytes from a buffer into a block.
    pub write: EfiFvbWrite,
    /// Erases specified block ranges.
    pub erase_blocks: EfiFvbEraseBlocks,
    /// Parent firmware volume handle.
    pub parent_handle: EfiHandle,
}

/// A parsed firmware volume header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FvHeader {
    /// The volume length.
    pub volume_length: u64,
    /// The header length.
    pub header_length: u16,
    /// The revision.
    pub revision: u8,
    /// The checksum.
    pub checksum: u16,
}

/// A firmware file header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FfsFileHeader {
    /// The file name identifier.
    pub name: [u8; 16],
    /// The file type.
    pub file_type: u8,
    /// The attributes.
    pub attributes: u8,
    /// The total file size.
    pub size: u32,
}

/// A firmware volume view borrowing from the input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirmwareVolume<'a> {
    header: FvHeader,
    volume: &'a [u8],
}

impl<'a> FirmwareVolume<'a> {
    /// Returns the header.
    #[must_use]
    pub const fn header(&self) -> &FvHeader {
        &self.header
    }

    /// Returns the full volume bytes.
    #[must_use]
    pub const fn volume(&self) -> &'a [u8] {
        self.volume
    }

    /// Iterates files in the volume.
    #[must_use]
    pub fn files(&self) -> FfsIter<'_> {
        FfsIter {
            volume: self.volume,
            offset: FV_HEADER_LENGTH,
        }
    }
}

/// A firmware file iterator.
#[derive(Clone, Debug)]
pub struct FfsIter<'a> {
    volume: &'a [u8],
    offset: usize,
}

impl Iterator for FfsIter<'_> {
    type Item = FfsFileHeader;

    /// Returns the next file header.
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset + 24 <= self.volume.len() {
            let header = parse_ffs_header(self.volume, self.offset)?;
            if header.size < 24 {
                return None;
            }
            let advance = align_up_8(header.size as usize);
            let file = header;
            self.offset = self.offset.checked_add(advance)?;
            if file.size == 0xFF_FFFF || file.file_type == 0xFF {
                continue;
            }
            return Some(file);
        }
        None
    }
}

/// Parses a firmware volume header.
///
/// Returns `None` when the signature mismatches, the header is
/// truncated, or the declared length exceeds the buffer.
#[must_use]
pub fn parse_firmware_volume(input: &[u8]) -> Option<FirmwareVolume<'_>> {
    if input.len() < FV_HEADER_LENGTH {
        return None;
    }
    if read_u32_le(input, 64)? != FV_SIGNATURE {
        return None;
    }
    let mut len_raw = [0_u8; 8];
    len_raw.copy_from_slice(input.get(48..56)?);
    let volume_length = u64::from_le_bytes(len_raw);
    let header_length = read_u16_le(input, 68)?;
    if usize::from(header_length) < FV_HEADER_LENGTH || usize::from(header_length) > input.len() {
        return None;
    }
    if volume_length < u64::from(header_length) {
        return None;
    }
    let end = usize::try_from(volume_length).ok()?;
    let volume = input.get(..end)?;
    Some(FirmwareVolume {
        header: FvHeader {
            volume_length,
            header_length,
            revision: *input.get(71)?,
            checksum: read_u16_le(input, 70)?,
        },
        volume,
    })
}

fn parse_ffs_header(input: &[u8], offset: usize) -> Option<FfsFileHeader> {
    if offset + 24 > input.len() {
        return None;
    }
    let mut name = [0_u8; 16];
    name.copy_from_slice(input.get(offset..offset + 16)?);
    let size_bits = read_u32_le(input, offset + 20)? & 0x00FF_FFFF;
    Some(FfsFileHeader {
        name,
        file_type: *input.get(offset + 18)?,
        attributes: *input.get(offset + 19)?,
        size: size_bits,
    })
}

fn align_up_8(value: usize) -> usize {
    value.next_multiple_of(8)
}
