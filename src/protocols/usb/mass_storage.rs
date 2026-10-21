//! # USB Mass Storage class
//!
//! The Bulk-Only Transport (`BOT`) command block, status block and the
//! small subset of the SCSI command set that a firmware boot path needs.
//!
//! The structures are byte-level encodings rather than `#[repr(C)]`
//! types because the transport wraps them in a 31-byte command block and
//! a 13-byte status block with fixed little-endian fields. Encoding and
//! decoding are explicit so that host alignment never changes the wire
//! layout.
//!
//! The module implements the data structures and command builders. It
//! does not perform transfers; a controller driver supplies the bulk
//! endpoint transfers and drives [`BulkOnlyState`].

/// The size of a command block wrapper in bytes.
pub const COMMAND_BLOCK_SIZE: usize = 31;

/// The size of a command status wrapper in bytes.
pub const COMMAND_STATUS_SIZE: usize = 13;

/// The command block wrapper signature, `USBC`.
pub const CBW_SIGNATURE: u32 = u32::from_le_bytes(*b"USBC");

/// The command status wrapper signature, `USBS`.
pub const CSW_SIGNATURE: u32 = u32::from_le_bytes(*b"USBS");

/// The maximum logical block address accepted by a 32-bit read.
pub const MAX_LBA_32: u32 = u32::MAX;

/// The maximum transfer length of a single SCSI command.
pub const MAX_TRANSFER_LENGTH: u32 = 0xFFFF;

/// The direction of the data stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DataDirection {
    /// The device sends data to the host.
    In = 0x80,
    /// The host sends data to the device.
    Out = 0x00,
    /// The command has no data stage.
    None = 0xFF,
}

impl DataDirection {
    /// Returns the direction bit used by the command block wrapper.
    #[must_use]
    pub const fn to_cbw_flag(self) -> u8 {
        match self {
            Self::In => 0x80,
            Self::Out => 0x00,
            Self::None => 0x00,
        }
    }
}

/// A Bulk-Only Transport command block wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandBlock {
    /// The data transfer direction.
    pub direction: DataDirection,
    /// The logical unit number.
    pub lun: u8,
    /// The expected data transfer length in bytes.
    pub transfer_length: u32,
    /// The command descriptor block.
    pub cdb: [u8; 16],
    /// The number of valid bytes in `cdb`.
    pub cdb_length: u8,
}

impl CommandBlock {
    /// Creates a command block for a command descriptor block.
    ///
    /// # Errors
    ///
    /// Returns `None` when `cdb_length` is outside `1..=16` or the
    /// direction and transfer length are inconsistent (a non-zero
    /// transfer length with [`DataDirection::None`]).
    #[must_use]
    pub fn new(
        direction: DataDirection,
        lun: u8,
        transfer_length: u32,
        cdb: &[u8],
    ) -> Option<Self> {
        if cdb.is_empty() || cdb.len() > 16 {
            return None;
        }
        if direction == DataDirection::None && transfer_length != 0 {
            return None;
        }
        let mut block = [0_u8; 16];
        block[..cdb.len()].copy_from_slice(cdb);
        Some(Self {
            direction,
            lun,
            transfer_length,
            cdb: block,
            cdb_length: u8::try_from(cdb.len()).ok()?,
        })
    }

    /// Encodes the wrapper into its 31-byte little-endian image.
    #[must_use]
    pub fn encode(&self, tag: u32) -> [u8; COMMAND_BLOCK_SIZE] {
        let mut bytes = [0_u8; COMMAND_BLOCK_SIZE];
        bytes[0..4].copy_from_slice(&CBW_SIGNATURE.to_le_bytes());
        bytes[4..8].copy_from_slice(&tag.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.transfer_length.to_le_bytes());
        bytes[12] = self.direction.to_cbw_flag();
        bytes[13] = self.lun & 0x0F;
        bytes[14] = self.cdb_length;
        let length = usize::from(self.cdb_length).min(16);
        bytes[15..15 + length].copy_from_slice(&self.cdb[..length]);
        bytes
    }
}

/// The status of a Bulk-Only Transport command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CommandStatus {
    /// The command completed successfully.
    Passed = 0,
    /// The command failed.
    Failed = 1,
    /// The command phase failed.
    PhaseError = 2,
}

impl CommandStatus {
    /// Decodes the status byte.
    ///
    /// Unknown values decode to [`CommandStatus::Failed`].
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Passed,
            1 => Self::Failed,
            2 => Self::PhaseError,
            _ => Self::Failed,
        }
    }
}

/// A Bulk-Only Transport command status wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandStatusWrapper {
    /// The tag echoed from the command.
    pub tag: u32,
    /// The number of bytes not transferred.
    pub residue: u32,
    /// The command status.
    pub status: CommandStatus,
}

impl CommandStatusWrapper {
    /// Decodes a status wrapper.
    ///
    /// Returns `None` when the signature is not [`CSW_SIGNATURE`].
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < COMMAND_STATUS_SIZE {
            return None;
        }
        let signature = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if signature != CSW_SIGNATURE {
            return None;
        }
        Some(Self {
            tag: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            residue: u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            status: CommandStatus::from_u8(bytes[12]),
        })
    }

    /// Encodes the wrapper into its 13-byte little-endian image.
    #[must_use]
    pub fn encode(&self) -> [u8; COMMAND_STATUS_SIZE] {
        let mut bytes = [0_u8; COMMAND_STATUS_SIZE];
        bytes[0..4].copy_from_slice(&CSW_SIGNATURE.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.tag.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.residue.to_le_bytes());
        bytes[12] = self.status as u8;
        bytes
    }
}

/// The states of the bulk-only transport state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BulkOnlyState {
    /// Waiting for a command to be issued.
    Idle,
    /// The command block wrapper has been sent.
    CommandSent,
    /// The data stage is in progress.
    DataTransfer,
    /// The status stage is pending.
    StatusPending,
    /// The command completed successfully.
    Complete,
    /// The transport reported a failure.
    Failed,
}

impl BulkOnlyState {
    /// Advances the state after the command wrapper was accepted.
    #[must_use]
    pub const fn on_command_sent(self) -> Self {
        match self {
            Self::Idle => Self::CommandSent,
            other => other,
        }
    }

    /// Advances the state after the data stage was submitted.
    #[must_use]
    pub const fn on_data_submitted(self, has_data: bool) -> Self {
        match self {
            Self::CommandSent => {
                if has_data {
                    Self::DataTransfer
                } else {
                    Self::StatusPending
                }
            }
            other => other,
        }
    }

    /// Advances the state after the data stage completed.
    #[must_use]
    pub const fn on_data_complete(self) -> Self {
        match self {
            Self::DataTransfer => Self::StatusPending,
            other => other,
        }
    }

    /// Advances the state after the status wrapper was received.
    #[must_use]
    pub const fn on_status_received(self, status: CommandStatus) -> Self {
        match self {
            Self::StatusPending => match status {
                CommandStatus::Passed => Self::Complete,
                CommandStatus::Failed | CommandStatus::PhaseError => Self::Failed,
            },
            other => other,
        }
    }

    /// Returns whether the command reached a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Complete | Self::Failed)
    }
}

/// Builds a `TEST UNIT READY` command.
#[must_use]
pub const fn test_unit_ready() -> [u8; 6] {
    [0x00, 0, 0, 0, 0, 0]
}

/// Builds a `REQUEST SENSE` command.
#[must_use]
pub const fn request_sense() -> [u8; 6] {
    [0x03, 0, 0, 0, 18, 0]
}

/// Builds an `INQUIRY` command for the standard inquiry data.
#[must_use]
pub const fn inquiry() -> [u8; 6] {
    [0x12, 0, 0, 0, 36, 0]
}

/// Builds a `READ CAPACITY(10)` command.
#[must_use]
pub const fn read_capacity_10() -> [u8; 10] {
    [0x25, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}

/// Builds a `READ(10)` command.
///
/// Returns `None` when the transfer exceeds [`MAX_TRANSFER_LENGTH`]
/// blocks.
#[must_use]
pub fn read_10(lba: u32, blocks: u16) -> Option<[u8; 10]> {
    if blocks == 0 {
        return None;
    }
    let mut cdb = [0_u8; 10];
    cdb[0] = 0x28;
    cdb[2..6].copy_from_slice(&lba.to_be_bytes());
    cdb[7..9].copy_from_slice(&blocks.to_be_bytes());
    Some(cdb)
}

/// Builds a `WRITE(10)` command.
///
/// Returns `None` when `blocks` is zero.
#[must_use]
pub fn write_10(lba: u32, blocks: u16) -> Option<[u8; 10]> {
    if blocks == 0 {
        return None;
    }
    let mut cdb = [0_u8; 10];
    cdb[0] = 0x2A;
    cdb[2..6].copy_from_slice(&lba.to_be_bytes());
    cdb[7..9].copy_from_slice(&blocks.to_be_bytes());
    Some(cdb)
}

/// Builds a `START STOP UNIT` command that spins the medium up.
#[must_use]
pub const fn start_unit() -> [u8; 6] {
    [0x1B, 0, 0, 0, 0x01, 0]
}

/// A parsed `READ CAPACITY(10)` parameter data block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReadCapacity {
    /// The last addressable logical block.
    pub last_lba: u32,
    /// The size of a logical block in bytes.
    pub block_length: u32,
}

impl ReadCapacity {
    /// Parses the eight-byte big-endian parameter data.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        Some(Self {
            last_lba: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            block_length: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        })
    }

    /// Returns the media capacity in bytes.
    ///
    /// Returns `None` on overflow.
    #[must_use]
    pub fn capacity_bytes(&self) -> Option<u64> {
        let blocks = u64::from(self.last_lba).checked_add(1)?;
        blocks.checked_mul(u64::from(self.block_length))
    }
}

/// A parsed `INQUIRY` standard data block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inquiry {
    /// The peripheral device type.
    pub peripheral_type: u8,
    /// The removable-medium flag.
    pub removable: bool,
    /// The `SCSI` version.
    pub version: u8,
    /// The vendor identification, trimmed of trailing spaces.
    pub vendor: [u8; 8],
    /// The product identification, trimmed of trailing spaces.
    pub product: [u8; 16],
    /// The product revision, trimmed of trailing spaces.
    pub revision: [u8; 4],
}

impl Inquiry {
    /// Parses the standard inquiry data.
    ///
    /// Returns `None` when the input is shorter than the 36-byte
    /// standard response.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 36 {
            return None;
        }
        let mut vendor = [0_u8; 8];
        vendor.copy_from_slice(&bytes[8..16]);
        let mut product = [0_u8; 16];
        product.copy_from_slice(&bytes[16..32]);
        let mut revision = [0_u8; 4];
        revision.copy_from_slice(&bytes[32..36]);
        Some(Self {
            peripheral_type: bytes[0] & 0x1F,
            removable: bytes[1] & 0x80 != 0,
            version: bytes[2],
            vendor: trim_trailing_spaces(vendor),
            product: trim_trailing_spaces(product),
            revision: trim_trailing_spaces(revision),
        })
    }
}

/// Replaces trailing spaces in a fixed-width field with zero bytes.
fn trim_trailing_spaces<const N: usize>(mut field: [u8; N]) -> [u8; N] {
    let mut length = field.len();
    while length > 0 && field[length - 1] == b' ' {
        field[length - 1] = 0;
        length -= 1;
    }
    field
}

/// The size of the standard inquiry data.
pub const INQUIRY_STANDARD_LENGTH: usize = 36;

#[cfg(test)]
mod tests {
    use super::{
        BulkOnlyState, CBW_SIGNATURE, COMMAND_BLOCK_SIZE, COMMAND_STATUS_SIZE, CSW_SIGNATURE,
        CommandBlock, CommandStatus, CommandStatusWrapper, DataDirection, Inquiry, ReadCapacity,
        inquiry, read_10, read_capacity_10,
    };

    #[test]
    fn command_block_round_trip() {
        let block = CommandBlock::new(DataDirection::In, 0, 512, &read_10(4, 1).unwrap())
            .expect("command block");
        let encoded = block.encode(0x1234_5678);
        assert_eq!(encoded.len(), COMMAND_BLOCK_SIZE);
        assert_eq!(
            u32::from_le_bytes([encoded[0], encoded[1], encoded[2], encoded[3]]),
            CBW_SIGNATURE
        );
        assert_eq!(
            u32::from_le_bytes([encoded[4], encoded[5], encoded[6], encoded[7]]),
            0x1234_5678
        );
        assert_eq!(encoded[12], 0x80);
        assert_eq!(encoded[14], 10);
        assert_eq!(encoded[15], 0x28);
    }

    #[test]
    fn rejects_inconsistent_command() {
        assert!(CommandBlock::new(DataDirection::None, 0, 1, &[0]).is_none());
        assert!(CommandBlock::new(DataDirection::In, 0, 0, &[]).is_none());
    }

    #[test]
    fn status_wrapper_round_trip() {
        let wrapper = CommandStatusWrapper {
            tag: 7,
            residue: 0,
            status: CommandStatus::Passed,
        };
        let encoded = wrapper.encode();
        assert_eq!(encoded.len(), COMMAND_STATUS_SIZE);
        assert_eq!(
            u32::from_le_bytes([encoded[0], encoded[1], encoded[2], encoded[3]]),
            CSW_SIGNATURE
        );
        assert_eq!(CommandStatusWrapper::decode(&encoded), Some(wrapper));
    }

    #[test]
    fn rejects_bad_status_signature() {
        let mut bytes = [0_u8; COMMAND_STATUS_SIZE];
        bytes[0] = b'X';
        assert!(CommandStatusWrapper::decode(&bytes).is_none());
    }

    #[test]
    fn state_machine_reaches_terminal_state() {
        let state = BulkOnlyState::Idle
            .on_command_sent()
            .on_data_submitted(true)
            .on_data_complete()
            .on_status_received(CommandStatus::Passed);
        assert_eq!(state, BulkOnlyState::Complete);
        assert!(state.is_terminal());
    }

    #[test]
    fn read_capacity_parsing() {
        let mut bytes = [0_u8; 8];
        bytes[0..4].copy_from_slice(&1023_u32.to_be_bytes());
        bytes[4..8].copy_from_slice(&512_u32.to_be_bytes());
        let capacity = ReadCapacity::parse(&bytes).expect("capacity");
        assert_eq!(capacity.last_lba, 1023);
        assert_eq!(capacity.capacity_bytes(), Some(1024 * 512));
        assert!(ReadCapacity::parse(&bytes[..7]).is_none());
    }

    #[test]
    fn inquiry_parsing() {
        let mut bytes = [0_u8; 36];
        bytes[0] = 0x80;
        bytes[1] = 0x00;
        bytes[2] = 0x06;
        bytes[8..16].copy_from_slice(b"ROVER   ");
        bytes[16..32].copy_from_slice(b"VIRTUAL DISK    ");
        bytes[32..36].copy_from_slice(b"1.0 ");
        let inquiry_data = Inquiry::parse(&bytes).expect("inquiry");
        assert_eq!(inquiry_data.peripheral_type, 0);
        assert!(!inquiry_data.removable);
        assert_eq!(&inquiry_data.vendor[..5], b"ROVER");
        assert_eq!(inquiry_data.vendor[5], 0);
        assert_eq!(inquiry_data.revision[3], 0);
        assert_eq!(inquiry(), [0x12, 0, 0, 0, 36, 0]);
        assert_eq!(read_capacity_10()[0], 0x25);
    }
}
