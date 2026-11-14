//! # Bluetooth Protocols
//!
//! UEFI Specification 2.5+ Bluetooth host controller and I/O interfaces.

pub mod attribute;
pub mod guid;
pub mod hc;
pub mod io;
pub mod le_config;

pub use attribute::{
    BluetoothAttribute, BluetoothAttributeHeader, BluetoothAttributePermission,
    BluetoothGattCharacteristicInfo, BluetoothGattPrimaryServiceInfo, BluetoothLeAddress,
    BluetoothUuid, EFI_BLUETOOTH_ATTRIBUTE_PROTOCOL_GUID,
    EFI_BLUETOOTH_ATTRIBUTE_SERVICE_BINDING_PROTOCOL_GUID, EfiBluetoothAttributeCallbackFunction,
    EfiBluetoothAttributeGetDeviceInfo, EfiBluetoothAttributeGetServiceInfo,
    EfiBluetoothAttributeProtocol, EfiBluetoothAttributeRegisterForServerNotification,
    EfiBluetoothAttributeSendRequest, EfiBluetoothLeDeviceInfo, UUID_16BIT_TYPE_LEN,
    UUID_32BIT_TYPE_LEN, UUID_128BIT_TYPE_LEN,
};
pub use guid::{
    EFI_BLUETOOTH_CONFIG_PROTOCOL_GUID, EFI_BLUETOOTH_HC_PROTOCOL_GUID,
    EFI_BLUETOOTH_IO_PROTOCOL_GUID, EFI_BLUETOOTH_LE_CONFIG_PROTOCOL_GUID,
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
pub use le_config::{
    BluetoothLeConfig, EfiBluetoothLeConfigConnect, EfiBluetoothLeConfigDisconnect,
    EfiBluetoothLeConfigGetData, EfiBluetoothLeConfigInit, EfiBluetoothLeConfigProtocol,
    EfiBluetoothLeConfigScan, EfiBluetoothLeConfigScanCallbackFunction,
    EfiBluetoothLeConfigScanParameter, EfiBluetoothLeConfigSetData,
    EfiBluetoothLeScanCallbackInformation,
};
