//! # Bluetooth Protocols
//!
//! UEFI Specification 2.5+ Bluetooth host controller and I/O interfaces.

pub mod guid;
pub mod hc;
pub mod io;

pub use guid::{
    EFI_BLUETOOTH_CONFIG_PROTOCOL_GUID, EFI_BLUETOOTH_HC_PROTOCOL_GUID,
    EFI_BLUETOOTH_IO_PROTOCOL_GUID,
};
pub use hc::{
    EfiBluetoothHcAsyncFuncCallback, EfiBluetoothHcAsyncReceiveAclData,
    EfiBluetoothHcAsyncReceiveEvent, EfiBluetoothHcAsyncReceiveScoData, EfiBluetoothHcProtocol,
    EfiBluetoothHcReceiveAclData, EfiBluetoothHcReceiveEvent, EfiBluetoothHcReceiveScoData,
    EfiBluetoothHcSendAclData, EfiBluetoothHcSendCommand, EfiBluetoothHcSendScoData,
};
pub use io::{
    BluetoothAddress, EfiBluetoothIoAsyncFuncCallback, EfiBluetoothIoGetDeviceInfo,
    EfiBluetoothIoGetSdpInfo, EfiBluetoothIoL2CapRawAsyncReceive, EfiBluetoothIoL2CapRawReceive,
    EfiBluetoothIoL2CapRawSend, EfiBluetoothIoProtocol,
};
