//! # SCSI command blocks
//!
//! The SCSI command blocks used by the USB mass storage class.
//!
//! The commands follow the SCSI command set that the mass storage
//! class references, which is the primary command set with the
//! sequential access device subset that the bulk-only transport uses.

/// The test unit ready operation code.
pub const OP_TEST_UNIT_READY: u8 = 0x00;

/// The request sense operation code.
pub const OP_REQUEST_SENSE: u8 = 0x03;

/// The inquiry operation code.
pub const OP_INQUIRY: u8 = 0x12;

/// The mode sense operation code, six byte variant.
pub const OP_MODE_SENSE_6: u8 = 0x1A;

/// The start stop unit operation code.
pub const OP_START_STOP_UNIT: u8 = 0x1B;

/// The prevent allow medium removal operation code.
pub const OP_PREVENT_ALLOW_MEDIUM_REMOVAL: u8 = 0x1E;

/// The read capacity operation code, ten byte variant.
pub const OP_READ_CAPACITY_10: u8 = 0x25;

/// The read operation code, ten byte variant.
pub const OP_READ_10: u8 = 0x28;

/// The write operation code, ten byte variant.
pub const OP_WRITE_10: u8 = 0x2A;

/// The synchronize cache operation code, ten byte variant.
pub const OP_SYNCHRONIZE_CACHE_10: u8 = 0x35;

/// The mode sense operation code, ten byte variant.
pub const OP_MODE_SENSE_10: u8 = 0x5A;

/// The SCSI command the bulk-only transport sends to a logical unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScsiCommand {
    /// Tests whether the logical unit is ready.
    TestUnitReady,
    /// Requests the sense data of the last error.
    RequestSense {
        /// The number of bytes to return.
        allocation_length: u16,
    },
    /// Queries the identity of the device.
    Inquiry {
        /// Whether a vital product data page is requested.
        vital_product_data: bool,
        /// The page code to request.
        page_code: u8,
        /// The number of bytes to return.
        allocation_length: u8,
    },
    /// Reads the capacity of the medium.
    ReadCapacity10,
    /// Reads logical blocks.
    Read10 {
        /// The first logical block to read.
        logical_block_address: u32,
        /// The number of logical blocks to read.
        transfer_length: u16,
    },
    /// Writes logical blocks.
    Write10 {
        /// The first logical block to write.
        logical_block_address: u32,
        /// The number of logical blocks to write.
        transfer_length: u16,
    },
    /// Reads the mode parameters of the logical unit.
    ModeSense6 {
        /// The page code to request.
        page_code: u8,
        /// The number of bytes to return.
        allocation_length: u8,
    },
    /// Writes back the cache of the medium.
    SynchronizeCache10 {
        /// The first logical block to write back.
        logical_block_address: u32,
        /// The number of logical blocks to write back, zero for all.
        number_of_blocks: u32,
    },
}

impl ScsiCommand {
    /// Returns the operation code of the command.
    #[must_use]
    pub fn opcode(&self) -> u8 {
        match self {
            Self::TestUnitReady => OP_TEST_UNIT_READY,
            Self::RequestSense { .. } => OP_REQUEST_SENSE,
            Self::Inquiry { .. } => OP_INQUIRY,
            Self::ReadCapacity10 => OP_READ_CAPACITY_10,
            Self::Read10 { .. } => OP_READ_10,
            Self::Write10 { .. } => OP_WRITE_10,
            Self::ModeSense6 { .. } => OP_MODE_SENSE_6,
            Self::SynchronizeCache10 { .. } => OP_SYNCHRONIZE_CACHE_10,
        }
    }

    /// Returns the length of the command block in bytes.
    #[must_use]
    pub fn length(&self) -> usize {
        match self {
            Self::ReadCapacity10
            | Self::Read10 { .. }
            | Self::Write10 { .. }
            | Self::SynchronizeCache10 { .. } => 10,
            _ => 6,
        }
    }

    /// Encodes the command block into `out`.
    ///
    /// Returns `false` when `out` is too small.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        let length = self.length();
        if out.len() < length {
            return false;
        }
        let Some(chunk) = out.get_mut(0..length) else {
            return false;
        };
        chunk.fill(0);
        let Some(opcode) = chunk.get_mut(0) else {
            return false;
        };
        *opcode = self.opcode();
        match self {
            Self::TestUnitReady | Self::ReadCapacity10 => {}
            Self::RequestSense { allocation_length } => {
                write_le16(chunk, 3, *allocation_length);
            }
            Self::Inquiry {
                vital_product_data,
                page_code,
                allocation_length,
            } => {
                if let Some(flags) = chunk.get_mut(1) {
                    *flags = (page_code & 0x3F) << 1 | u8::from(*vital_product_data);
                }
                if let Some(length) = chunk.get_mut(3) {
                    *length = *allocation_length;
                }
            }
            Self::Read10 {
                logical_block_address,
                transfer_length,
            }
            | Self::Write10 {
                logical_block_address,
                transfer_length,
            } => {
                write_le32(chunk, 2, *logical_block_address);
                write_le16(chunk, 7, *transfer_length);
            }
            Self::ModeSense6 {
                page_code,
                allocation_length,
            } => {
                if let Some(flags) = chunk.get_mut(1) {
                    *flags = page_code & 0x3F;
                }
                let value = u16::from(*allocation_length) & 0x0FFF;
                if let Some(low) = chunk.get_mut(2) {
                    *low = u8::try_from(value & 0x0F).unwrap_or_default();
                }
                if let Some(high) = chunk.get_mut(3) {
                    *high = u8::try_from((value >> 4) & 0xFF).unwrap_or_default();
                }
            }
            Self::SynchronizeCache10 {
                logical_block_address,
                number_of_blocks,
            } => {
                write_le32(chunk, 2, *logical_block_address);
                write_le24(chunk, 6, *number_of_blocks);
            }
        }
        true
    }
}

/// Writes a little-endian 16-bit value at `offset`.
fn write_le16(buffer: &mut [u8], offset: usize, value: u16) {
    let bytes = value.to_le_bytes();
    if let Some(slot) = buffer.get_mut(offset..offset + 2) {
        slot.copy_from_slice(&bytes);
    }
}

/// Writes a little-endian 32-bit value at `offset`.
fn write_le32(buffer: &mut [u8], offset: usize, value: u32) {
    let bytes = value.to_le_bytes();
    if let Some(slot) = buffer.get_mut(offset..offset + 4) {
        slot.copy_from_slice(&bytes);
    }
}

/// Writes a little-endian 24-bit value at `offset`.
fn write_le24(buffer: &mut [u8], offset: usize, value: u32) {
    let bytes = value.to_le_bytes();
    if let Some(slot) = buffer.get_mut(offset..offset + 3) {
        slot.copy_from_slice(bytes.get(0..3).unwrap_or(&[0; 3]));
    }
}

/// The response to a read capacity command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReadCapacity {
    /// The number of the last logical block of the medium.
    pub last_logical_block: u32,
    /// The size of one logical block in bytes.
    pub block_length: u32,
}

impl ReadCapacity {
    /// Parses the eight byte response of a read capacity command.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let last_logical_block = u32::from_le_bytes([
            *bytes.first()?,
            *bytes.get(1)?,
            *bytes.get(2)?,
            *bytes.get(3)?,
        ]);
        let block_length = u32::from_le_bytes([
            *bytes.get(4)?,
            *bytes.get(5)?,
            *bytes.get(6)?,
            *bytes.get(7)?,
        ]);
        Some(Self {
            last_logical_block,
            block_length,
        })
    }
}
