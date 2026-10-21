//! # USB mass storage
//!
//! Native support for the USB mass storage class, the SCSI command
//! blocks it uses and the Bulk-Only Transport (`BOT`) protocol that
//! carries them.

pub mod bot;
pub mod scsi;

pub use bot::{
    BulkOnlyTransport, CBW_COMMAND_OFFSET, CBW_SIGNATURE, CBW_SIZE, CSW_SIGNATURE, CSW_SIZE,
    CommandBlockWrapper, CommandStatus, CommandStatusWrapper, DataDirection, SenseData, SenseKey,
    TransportError, TransportPhase,
};
pub use scsi::{
    OP_INQUIRY, OP_MODE_SENSE_6, OP_MODE_SENSE_10, OP_READ_10, OP_READ_CAPACITY_10,
    OP_REQUEST_SENSE, OP_SYNCHRONIZE_CACHE_10, OP_TEST_UNIT_READY, OP_WRITE_10, ReadCapacity,
    ScsiCommand,
};

/// The mass storage interface class code.
pub const CLASS_MASS_STORAGE: u8 = 0x08;

/// The SCSI primary command set subclass code.
pub const SUBCLASS_SCSI: u8 = 0x06;

/// The bulk-only transport protocol code.
pub const PROTOCOL_BULK_ONLY: u8 = 0x50;
