//! # Block I/O reader
//!
//! Reads for partition discovery.
//!
//! All reads go through the existing Block I/O
//! wrapper with caller buffers.

use crate::base::EfiStatus;
use crate::protocols::storage::block_io::BlockIo;
use crate::protocols::storage::partition::gpt::header::{GptHeader, parse_gpt_header};
use crate::protocols::storage::partition::mbr::sector::{MbrSector, parse_mbr_sector};
use crate::protocols::storage::partition::types::PartitionTableError;

/// Reads an MBR sector.
///
/// # Errors
///
/// Returns `INVALID_PARAMETER` when the
/// buffer is too small, otherwise the firmware
/// status.
pub fn read_mbr_sector(
    io: &BlockIo<'_>,
    media_id: u32,
    buffer: &mut [u8],
) -> Result<MbrSector, EfiStatus> {
    let block_size = io.block_size().ok_or(EfiStatus::NO_MEDIA)?;
    let block_len = usize::try_from(block_size).unwrap_or(usize::MAX);
    if buffer.len() < block_len {
        return Err(EfiStatus::INVALID_PARAMETER);
    }
    let bytes = if buffer.len() > block_len {
        &mut buffer[..block_len]
    } else {
        buffer
    };
    io.read_blocks(media_id, 0, bytes)?;
    parse_mbr_sector(bytes).map_err(|_| EfiStatus::DEVICE_ERROR)
}

/// Reads a GPT header at an LBA.
///
/// # Errors
///
/// Returns the firmware status on read
/// failure and `DEVICE_ERROR` when parsing
/// fails.
pub fn read_gpt_header(
    io: &BlockIo<'_>,
    media_id: u32,
    lba: u64,
    buffer: &mut [u8],
) -> Result<GptHeader, EfiStatus> {
    let block_size = io.block_size().ok_or(EfiStatus::NO_MEDIA)?;
    let block_len = usize::try_from(block_size).unwrap_or(usize::MAX);
    if buffer.len() < block_len {
        return Err(EfiStatus::INVALID_PARAMETER);
    }
    let bytes = if buffer.len() > block_len {
        &mut buffer[..block_len]
    } else {
        buffer
    };
    io.read_blocks(media_id, lba, bytes)?;
    parse_gpt_header(bytes).map_err(|error| match error {
        PartitionTableError::InvalidSignature => EfiStatus::NOT_FOUND,
        _ => EfiStatus::DEVICE_ERROR,
    })
}
