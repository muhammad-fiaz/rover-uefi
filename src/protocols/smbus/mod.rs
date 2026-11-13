//! # `SMBus` (System Management Bus)
//!
//! Protocols and interfaces for `SMBus` host controllers and motherboard
//! sensor / telemetry management.

pub mod guid;
pub mod protocol;
pub mod types;

pub use guid::EFI_SMBUS_HC_PROTOCOL_GUID;
pub use protocol::{
    EfiSmbusHcExecuteOperation, EfiSmbusHcProtocol, EfiSmbusHcProtocolArpDevice,
    EfiSmbusHcProtocolGetArpMap, EfiSmbusHcProtocolNotify, EfiSmbusNotifyFunction,
};
pub use types::{
    EfiSmbusDeviceAddress, EfiSmbusDeviceCommand, EfiSmbusDeviceMap, EfiSmbusOperation,
    EfiSmbusUdid,
};
