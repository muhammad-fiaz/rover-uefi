//! # USB mass storage bulk-only transport
//!
//! The Bulk-Only Transport (BOT) protocol of the USB mass storage
//! class.
//!
//! A bulk-only transfer consists of a command phase that carries the
//! command block wrapper, an optional data phase and a status phase
//! that carries the command status wrapper. The wrappers carry a
//! signature that identifies the packet and a tag that pairs a status
//! wrapper with the command wrapper it belongs to.

use crate::base::bytes::{read_u8, read_u32_le};

/// The signature of a command block wrapper, `USBC`.
pub const CBW_SIGNATURE: u32 = 0x4342_5355;

/// The signature of a command status wrapper, `USBS`.
pub const CSW_SIGNATURE: u32 = 0x5342_5355;

/// The size of a command block wrapper in bytes.
pub const CBW_SIZE: usize = 31;

/// The size of a command status wrapper in bytes.
pub const CSW_SIZE: usize = 13;

/// The offset of the command block inside a command block wrapper.
pub const CBW_COMMAND_OFFSET: usize = 15;

/// The direction of a bulk-only data phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataDirection {
    /// The data flows from the host to the device.
    Out,
    /// The data flows from the device to the host.
    In,
}

impl DataDirection {
    /// Returns the direction bit of the command block wrapper flags.
    #[must_use]
    pub fn to_flags(self) -> u8 {
        match self {
            Self::Out => 0x00,
            Self::In => 0x80,
        }
    }
}

/// The status of a bulk-only command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandStatus {
    /// The command completed successfully.
    Passed,
    /// The command failed.
    Failed,
    /// The command failed and the device is out of phase.
    PhaseError,
}

impl CommandStatus {
    /// Returns the numeric status value.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Passed => 0,
            Self::Failed => 1,
            Self::PhaseError => 2,
        }
    }

    /// Creates a status from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::Passed),
            1 => Some(Self::Failed),
            2 => Some(Self::PhaseError),
            _ => None,
        }
    }
}

/// The command block wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommandBlockWrapper {
    /// The signature, always `USBC`.
    pub signature: u32,
    /// The tag that pairs the wrapper with its status wrapper.
    pub tag: u32,
    /// The number of bytes of the data phase.
    pub transfer_length: u32,
    /// The direction of the data phase.
    pub direction: DataDirection,
    /// The logical unit the command is addressed to.
    pub logical_unit: u8,
    /// The length of the command block.
    pub command_length: u8,
    /// The command block.
    pub command: [u8; 16],
}

impl CommandBlockWrapper {
    /// Creates a command block wrapper for a command block.
    ///
    /// Returns `None` when the command block is longer than 16 bytes.
    #[must_use]
    pub fn new(
        tag: u32,
        logical_unit: u8,
        command: &[u8],
        transfer_length: u32,
        direction: DataDirection,
    ) -> Option<Self> {
        if command.is_empty() || command.len() > 16 {
            return None;
        }
        let mut block = [0_u8; 16];
        let slot = block.get_mut(..command.len())?;
        slot.copy_from_slice(command);
        Some(Self {
            signature: CBW_SIGNATURE,
            tag,
            transfer_length,
            direction,
            logical_unit,
            command_length: u8::try_from(command.len()).unwrap_or_default(),
            command: block,
        })
    }

    /// Returns whether the signature of the wrapper is valid.
    #[must_use]
    pub fn has_valid_signature(&self) -> bool {
        self.signature == CBW_SIGNATURE
    }

    /// Encodes the wrapper into `out`.
    ///
    /// Returns `false` when `out` is shorter than 31 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        if out.len() < CBW_SIZE {
            return false;
        }
        let Some(chunk) = out.get_mut(0..CBW_SIZE) else {
            return false;
        };
        chunk.fill(0);
        if !write_le32(chunk, 0, self.signature)
            || !write_le32(chunk, 4, self.tag)
            || !write_le32(chunk, 8, self.transfer_length)
        {
            return false;
        }
        if let Some(flags) = chunk.get_mut(12) {
            *flags = self.direction.to_flags();
        }
        if let Some(unit) = chunk.get_mut(13) {
            *unit = self.logical_unit & 0x0F;
        }
        if let Some(length) = chunk.get_mut(14) {
            *length = self.command_length;
        }
        let command = self.command.get(..16);
        let target = chunk.get_mut(CBW_COMMAND_OFFSET..CBW_COMMAND_OFFSET + 16);
        if let (Some(command), Some(target)) = (command, target) {
            target.copy_from_slice(command);
        }
        true
    }

    /// Decodes a command block wrapper from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < CBW_SIZE {
            return None;
        }
        let signature = read_u32_le(bytes, 0)?;
        let tag = read_u32_le(bytes, 4)?;
        let transfer_length = read_u32_le(bytes, 8)?;
        let flags = read_u8(bytes, 12)?;
        let logical_unit = read_u8(bytes, 13)?;
        let command_length = read_u8(bytes, 14)?;
        let mut command = [0_u8; 16];
        let block = bytes.get(CBW_COMMAND_OFFSET..CBW_COMMAND_OFFSET + 16)?;
        command.copy_from_slice(block);
        Some(Self {
            signature,
            tag,
            transfer_length,
            direction: if flags & 0x80 != 0 {
                DataDirection::In
            } else {
                DataDirection::Out
            },
            logical_unit,
            command_length,
            command,
        })
    }
}

/// The command status wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommandStatusWrapper {
    /// The signature, always `USBS`.
    pub signature: u32,
    /// The tag of the command block wrapper this wrapper belongs to.
    pub tag: u32,
    /// The number of bytes of the data phase that were not transferred.
    pub data_residue: u32,
    /// The status of the command.
    pub status: CommandStatus,
}

impl CommandStatusWrapper {
    /// Creates a command status wrapper.
    #[must_use]
    pub const fn new(tag: u32, data_residue: u32, status: CommandStatus) -> Self {
        Self {
            signature: CSW_SIGNATURE,
            tag,
            data_residue,
            status,
        }
    }

    /// Returns whether the signature of the wrapper is valid.
    #[must_use]
    pub fn has_valid_signature(&self) -> bool {
        self.signature == CSW_SIGNATURE
    }

    /// Encodes the wrapper into `out`.
    ///
    /// Returns `false` when `out` is shorter than 13 bytes.
    #[must_use]
    pub fn encode(&self, out: &mut [u8]) -> bool {
        if out.len() < CSW_SIZE {
            return false;
        }
        let Some(chunk) = out.get_mut(0..CSW_SIZE) else {
            return false;
        };
        chunk.fill(0);
        if !write_le32(chunk, 0, self.signature)
            || !write_le32(chunk, 4, self.tag)
            || !write_le32(chunk, 8, self.data_residue)
        {
            return false;
        }
        if let Some(status) = chunk.get_mut(12) {
            *status = self.status.to_raw();
        }
        true
    }

    /// Decodes a command status wrapper from `bytes`.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < CSW_SIZE {
            return None;
        }
        let signature = read_u32_le(bytes, 0)?;
        let tag = read_u32_le(bytes, 4)?;
        let data_residue = read_u32_le(bytes, 8)?;
        let raw_status = read_u8(bytes, 12)?;
        Some(Self {
            signature,
            tag,
            data_residue,
            status: CommandStatus::from_raw(raw_status)?,
        })
    }
}

/// The sense key of a SCSI sense data buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SenseKey {
    /// No sense information.
    NoSense,
    /// The command was recovered.
    RecoveredError,
    /// The device is not ready.
    NotReady,
    /// A medium error occurred.
    MediumError,
    /// A hardware error occurred.
    HardwareError,
    /// An illegal request was made.
    IllegalRequest,
    /// The unit attention condition occurred.
    UnitAttention,
    /// The data is protected.
    DataProtect,
    /// A firmware update failed.
    FirmwareUpdate,
    /// The unit attention condition changed.
    Change,
    /// The other vendor specific sense key.
    VendorSpecific,
    /// A reserved sense key.
    Reserved,
}
impl SenseKey {
    /// Returns the numeric sense key.
    #[must_use]
    pub fn to_raw(self) -> u8 {
        match self {
            Self::NoSense => 0x00,
            Self::RecoveredError => 0x01,
            Self::NotReady => 0x02,
            Self::MediumError => 0x03,
            Self::HardwareError => 0x04,
            Self::IllegalRequest => 0x05,
            Self::UnitAttention => 0x06,
            Self::DataProtect => 0x07,
            Self::FirmwareUpdate => 0x08,
            Self::Change => 0x09,
            Self::VendorSpecific => 0x0A,
            Self::Reserved => 0x0B,
        }
    }

    /// Creates a sense key from its numeric value.
    #[must_use]
    pub fn from_raw(raw: u8) -> Self {
        match raw & 0x0F {
            0x00 => Self::NoSense,
            0x01 => Self::RecoveredError,
            0x02 => Self::NotReady,
            0x03 => Self::MediumError,
            0x04 => Self::HardwareError,
            0x05 => Self::IllegalRequest,
            0x06 => Self::UnitAttention,
            0x07 => Self::DataProtect,
            0x08 => Self::FirmwareUpdate,
            0x09 => Self::Change,
            0x0A => Self::VendorSpecific,
            _ => Self::Reserved,
        }
    }
}

/// The SCSI sense data of a failed command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SenseData {
    /// The sense key.
    pub key: SenseKey,
    /// The additional sense code.
    pub additional_sense_code: u8,
    /// The additional sense code qualifier.
    pub additional_sense_code_qualifier: u8,
}

impl SenseData {
    /// Parses the fixed format sense data buffer.
    ///
    /// Returns `None` when the buffer is too short.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let response_code = read_u8(bytes, 0)? & 0x7F;
        if response_code != 0x72 && response_code != 0x73 {
            return None;
        }
        let key = SenseKey::from_raw(read_u8(bytes, 2)?);
        let additional_sense_code = read_u8(bytes, 12)?;
        let additional_sense_code_qualifier = read_u8(bytes, 13)?;
        Some(Self {
            key,
            additional_sense_code,
            additional_sense_code_qualifier,
        })
    }
}

/// The phase of a bulk-only transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransportPhase {
    /// The command block wrapper is transferred.
    Command,
    /// The data of the command is transferred.
    Data,
    /// The command status wrapper is transferred.
    Status,
    /// The transfer is complete.
    Complete,
}

/// The errors a bulk-only transfer can fail with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransportError {
    /// The command block wrapper did not carry the expected signature.
    InvalidCommandBlockWrapper,
    /// The command status wrapper did not carry the expected signature.
    InvalidCommandStatusWrapper,
    /// The command status wrapper carried the tag of another command.
    TagMismatch,
    /// The endpoint stalled.
    Stalled,
    /// The device reported that it is out of phase.
    PhaseError,
    /// The device reported that the command failed.
    CommandFailed,
    /// The device reported a sense condition.
    Sense(SenseData),
    /// The command was issued in the wrong phase.
    UnexpectedPhase,
}

impl TransportError {
    /// Returns the sense data of the error, if it carries one.
    #[must_use]
    pub fn sense(&self) -> Option<SenseData> {
        match self {
            Self::Sense(sense) => Some(*sense),
            _ => None,
        }
    }
}

/// The state machine of a bulk-only transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BulkOnlyTransport {
    phase: TransportPhase,
    tag: u32,
    expected_length: u32,
    transferred: u32,
    direction: DataDirection,
}

impl BulkOnlyTransport {
    /// Creates a transport that waits for a command.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phase: TransportPhase::Command,
            tag: 0,
            expected_length: 0,
            transferred: 0,
            direction: DataDirection::Out,
        }
    }

    /// Returns the phase the transport is in.
    #[must_use]
    pub fn phase(&self) -> TransportPhase {
        self.phase
    }

    /// Returns the tag of the command in flight.
    #[must_use]
    pub fn tag(&self) -> u32 {
        self.tag
    }

    /// Returns the direction of the data phase in flight.
    #[must_use]
    pub fn direction(&self) -> DataDirection {
        self.direction
    }

    /// Returns the number of bytes the data phase still moves.
    #[must_use]
    pub fn remaining(&self) -> u32 {
        self.expected_length.saturating_sub(self.transferred)
    }

    /// Starts a command with the given command block wrapper.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidCommandBlockWrapper`] when the
    /// wrapper does not carry the expected signature and
    /// [`TransportError::UnexpectedPhase`] when a command is already in
    /// flight.
    pub fn begin_command(
        &mut self,
        wrapper: &CommandBlockWrapper,
    ) -> Result<TransportPhase, TransportError> {
        if !wrapper.has_valid_signature() {
            return Err(TransportError::InvalidCommandBlockWrapper);
        }
        if self.phase != TransportPhase::Command {
            return Err(TransportError::UnexpectedPhase);
        }
        self.tag = wrapper.tag;
        self.expected_length = wrapper.transfer_length;
        self.transferred = 0;
        self.direction = wrapper.direction;
        self.phase = if wrapper.transfer_length == 0 {
            TransportPhase::Status
        } else {
            TransportPhase::Data
        };
        Ok(self.phase)
    }

    /// Reports that the data phase moved `transferred` bytes.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::UnexpectedPhase`] when the transport is
    /// not in the data phase.
    pub fn complete_data(&mut self, transferred: u32) -> Result<TransportPhase, TransportError> {
        if self.phase != TransportPhase::Data {
            return Err(TransportError::UnexpectedPhase);
        }
        self.transferred = self.transferred.saturating_add(transferred);
        if self.transferred >= self.expected_length {
            self.phase = TransportPhase::Status;
        }
        Ok(self.phase)
    }

    /// Reports that the status phase received a command status wrapper.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::InvalidCommandStatusWrapper`] when the
    /// wrapper does not carry the expected signature,
    /// [`TransportError::TagMismatch`] when the wrapper belongs to
    /// another command, [`TransportError::UnexpectedPhase`] when the
    /// transport is not in the status phase and
    /// [`TransportError::PhaseError`] or [`TransportError::CommandFailed`]
    /// when the device reported a failure.
    pub fn complete_status(
        &mut self,
        wrapper: &CommandStatusWrapper,
    ) -> Result<TransportPhase, TransportError> {
        if !wrapper.has_valid_signature() {
            return Err(TransportError::InvalidCommandStatusWrapper);
        }
        if wrapper.tag != self.tag {
            return Err(TransportError::TagMismatch);
        }
        if self.phase != TransportPhase::Status {
            return Err(TransportError::UnexpectedPhase);
        }
        match wrapper.status {
            CommandStatus::Passed => {
                self.phase = TransportPhase::Complete;
                Ok(self.phase)
            }
            CommandStatus::PhaseError => Err(TransportError::PhaseError),
            CommandStatus::Failed => Err(TransportError::CommandFailed),
        }
    }

    /// Reports that the endpoint stalled.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError::Stalled`] when the transfer did not
    /// complete.
    pub fn handle_stall(&mut self) -> Result<TransportPhase, TransportError> {
        if self.phase == TransportPhase::Complete {
            return Err(TransportError::UnexpectedPhase);
        }
        self.phase = TransportPhase::Command;
        Err(TransportError::Stalled)
    }
}

impl Default for BulkOnlyTransport {
    fn default() -> Self {
        Self::new()
    }
}

/// Writes a little-endian 32-bit value at `offset`.
fn write_le32(buffer: &mut [u8], offset: usize, value: u32) -> bool {
    let Some(end) = offset.checked_add(4) else {
        return false;
    };
    let Some(chunk) = buffer.get_mut(offset..end) else {
        return false;
    };
    chunk.copy_from_slice(&value.to_le_bytes());
    true
}
