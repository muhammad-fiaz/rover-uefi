//! # GPT header
//!
//! Parsing and validation of GPT headers.
//!
//! The header is parsed with little-endian
//! reads. The CRC covers the header with the
//! CRC field set to zero.

use crate::base::EfiGuid;
use crate::protocols::storage::partition::crc32::crc32;
use crate::protocols::storage::partition::types::PartitionTableError;

/// The GPT signature bytes.
pub const GPT_SIGNATURE: [u8; 8] = *b"EFI PART";

/// Parsed GPT header data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GptHeader {
    /// The signature.
    pub signature: [u8; 8],
    /// The header size.
    pub header_size: u32,
    /// The stored CRC.
    pub crc32: u32,
    /// The current LBA.
    pub current_lba: u64,
    /// The backup LBA.
    pub backup_lba: u64,
    /// The first usable LBA.
    pub first_usable_lba: u64,
    /// The last usable LBA.
    pub last_usable_lba: u64,
    /// The disk `GUID`.
    pub disk_guid: EfiGuid,
    /// The entry array LBA.
    pub partition_entry_lba: u64,
    /// The number of entries.
    pub number_of_entries: u32,
    /// The size of each entry.
    pub size_of_entry: u32,
    /// The entry array CRC.
    pub partition_entry_array_crc32: u32,
}

fn read_u64(bytes: &[u8]) -> u64 {
    crate::base::bytes::read_u64_le(bytes, 0).unwrap_or_default()
}

fn read_u32(bytes: &[u8]) -> u32 {
    crate::base::bytes::read_u32_le(bytes, 0).unwrap_or_default()
}

/// Parses a GPT header from a buffer.
///
/// The buffer must hold at least the header
/// size. The header CRC is verified.
///
/// # Errors
///
/// Returns an error when the buffer is
/// truncated, the signature is invalid, or
/// the CRC does not match.
pub fn parse_gpt_header(buffer: &[u8]) -> Result<GptHeader, PartitionTableError> {
    if buffer.len() < 92 {
        return Err(PartitionTableError::Truncated);
    }
    let mut signature = [0_u8; 8];
    signature.copy_from_slice(&buffer[0..8]);
    if signature != GPT_SIGNATURE {
        return Err(PartitionTableError::InvalidSignature);
    }
    let header_size = read_u32(&buffer[12..16]);
    if header_size < 92 || (header_size as usize) > buffer.len() {
        return Err(PartitionTableError::InvalidHeaderSize);
    }
    let header_end = header_size as usize;
    let header_bytes = &buffer[..header_end];
    let mut zeroed: [u8; 512] = [0_u8; 512];
    let copy_len = header_end.min(512);
    zeroed[..copy_len].copy_from_slice(&header_bytes[..copy_len]);
    // CRC field is at offset 16 in the header.
    for byte in &mut zeroed[16..20] {
        *byte = 0;
    }
    let computed = crc32(&zeroed[..copy_len]);
    let stored = read_u32(&buffer[16..20]);
    if computed != stored {
        return Err(PartitionTableError::InvalidCrc);
    }
    if read_u32(&buffer[24..28]) == 0 {
        return Err(PartitionTableError::InvalidHeaderSize);
    }
    let number_of_entries = read_u32(&buffer[80..84]);
    let size_of_entry = read_u32(&buffer[84..88]);
    if number_of_entries == 0 {
        return Err(PartitionTableError::InvalidEntrySize);
    }
    if size_of_entry < 128 || !size_of_entry.is_power_of_two() {
        return Err(PartitionTableError::InvalidEntrySize);
    }
    Ok(GptHeader {
        signature,
        header_size,
        crc32: stored,
        current_lba: read_u64(&buffer[24..32]),
        backup_lba: read_u64(&buffer[32..40]),
        first_usable_lba: read_u64(&buffer[40..48]),
        last_usable_lba: read_u64(&buffer[48..56]),
        disk_guid: EfiGuid::from_bytes([
            buffer[56], buffer[57], buffer[58], buffer[59], buffer[60], buffer[61], buffer[62],
            buffer[63], buffer[64], buffer[65], buffer[66], buffer[67], buffer[68], buffer[69],
            buffer[70], buffer[71],
        ]),
        partition_entry_lba: read_u64(&buffer[72..80]),
        number_of_entries,
        size_of_entry,
        partition_entry_array_crc32: read_u32(&buffer[88..92]),
    })
}
