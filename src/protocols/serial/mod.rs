//! # Serial
//!
//! Serial I/O protocol access.

pub mod guid;
pub mod protocol;
pub mod types;

pub use guid::{PC_ANSI_GUID, SERIAL_IO_GUID, SERIAL_TERMINAL_DEVICE_TYPE_GUID};
pub use protocol::{EfiSerialIoMode, EfiSerialIoProtocol, SerialIo};
pub use types::{ParityType, StopBitsType};
