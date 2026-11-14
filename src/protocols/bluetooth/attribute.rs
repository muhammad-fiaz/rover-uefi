//! # Bluetooth Attribute Protocol
//!
//! Provides services for Bluetooth ATT (Attribute Protocol) and GATT (Generic Attribute Profile)
//! interfaces as defined in UEFI Specification 2.7+.

pub use super::guid::{
    EFI_BLUETOOTH_ATTRIBUTE_PROTOCOL_GUID, EFI_BLUETOOTH_ATTRIBUTE_SERVICE_BINDING_PROTOCOL_GUID,
};
use crate::abi::types::UIntN;
use crate::base::EfiStatus;
use core::ffi::c_void;

/// UUID length constants.
pub const UUID_16BIT_TYPE_LEN: u8 = 2;
/// 32-bit UUID length.
pub const UUID_32BIT_TYPE_LEN: u8 = 4;
/// 128-bit UUID length.
pub const UUID_128BIT_TYPE_LEN: u8 = 16;

/// Bluetooth UUID structure supporting 16-bit, 32-bit, or 128-bit UUIDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct BluetoothUuid {
    /// Length of UUID in bytes (2, 4, or 16).
    pub length: u8,
    /// UUID byte representation.
    pub data: [u8; 16],
}

impl BluetoothUuid {
    /// Creates a 16-bit Bluetooth UUID.
    #[must_use]
    pub const fn from_u16(val: u16) -> Self {
        let mut data = [0u8; 16];
        let bytes = val.to_le_bytes();
        data[0] = bytes[0];
        data[1] = bytes[1];
        Self {
            length: UUID_16BIT_TYPE_LEN,
            data,
        }
    }

    /// Creates a 32-bit Bluetooth UUID.
    #[must_use]
    pub const fn from_u32(val: u32) -> Self {
        let mut data = [0u8; 16];
        let bytes = val.to_le_bytes();
        data[0] = bytes[0];
        data[1] = bytes[1];
        data[2] = bytes[2];
        data[3] = bytes[3];
        Self {
            length: UUID_32BIT_TYPE_LEN,
            data,
        }
    }

    /// Creates a 128-bit Bluetooth UUID from raw bytes.
    #[must_use]
    pub const fn from_u128_bytes(bytes: [u8; 16]) -> Self {
        Self {
            length: UUID_128BIT_TYPE_LEN,
            data: bytes,
        }
    }
}

/// Bluetooth attribute permission bitfield container.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct BluetoothAttributePermission(pub u32);

impl BluetoothAttributePermission {
    /// Readable attribute permission bit.
    pub const READABLE: u32 = 1 << 0;
    /// Read encryption required.
    pub const READ_ENCRYPTION: u32 = 1 << 1;
    /// Read authentication required.
    pub const READ_AUTHENTICATION: u32 = 1 << 2;
    /// Read authorization required.
    pub const READ_AUTHORIZATION: u32 = 1 << 3;
    /// Writable attribute permission bit.
    pub const WRITABLE: u32 = 1 << 16;
    /// Write encryption required.
    pub const WRITE_ENCRYPTION: u32 = 1 << 17;
    /// Write authentication required.
    pub const WRITE_AUTHENTICATION: u32 = 1 << 18;
    /// Write authorization required.
    pub const WRITE_AUTHORIZATION: u32 = 1 << 19;

    /// Checks if readable.
    #[must_use]
    pub const fn is_readable(self) -> bool {
        (self.0 & Self::READABLE) != 0
    }

    /// Checks if writable.
    #[must_use]
    pub const fn is_writable(self) -> bool {
        (self.0 & Self::WRITABLE) != 0
    }
}

/// Bluetooth attribute common header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct BluetoothAttributeHeader {
    /// Attribute type UUID.
    pub r#type: BluetoothUuid,
    /// Length of the attribute in bytes.
    pub length: u16,
    /// Unique 16-bit attribute handle.
    pub attribute_handle: u16,
    /// Permission flags for this attribute.
    pub attribute_permission: BluetoothAttributePermission,
}

/// GATT Primary Service Info structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct BluetoothGattPrimaryServiceInfo {
    /// Attribute header.
    pub header: BluetoothAttributeHeader,
    /// Handle of the last attribute in this service group.
    pub end_group_handle: u16,
    /// Service UUID.
    pub service_uuid: BluetoothUuid,
}

/// GATT Characteristic Info structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct BluetoothGattCharacteristicInfo {
    /// Attribute header.
    pub header: BluetoothAttributeHeader,
    /// Characteristic properties mask.
    pub characteristic_properties: u8,
    /// Handle of the characteristic value attribute.
    pub characteristic_value_handle: u16,
    /// Characteristic UUID.
    pub characteristic_uuid: BluetoothUuid,
}

/// Bluetooth Low Energy (LE) device address including address type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct BluetoothLeAddress {
    /// Address type (0 = public device address, 1 = random device address).
    pub r#type: u8,
    /// 48-bit device address bytes in little-endian order.
    pub address: [u8; 6],
}

/// Bluetooth LE device information structure.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiBluetoothLeDeviceInfo {
    /// Version of structure.
    pub version: u32,
    /// Device address.
    pub bd_addr: BluetoothLeAddress,
    /// Directed address.
    pub direct_address: BluetoothLeAddress,
    /// Received Signal Strength Indication.
    pub rssi: u8,
    /// Size of advertisement data in bytes.
    pub advertisement_data_size: UIntN,
    /// Pointer to raw advertisement payload.
    pub advertisement_data: *mut c_void,
}

/// Callback function when a response is received from the remote server.
pub type EfiBluetoothAttributeCallbackFunction = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothAttributeProtocol,
    data: *mut c_void,
    data_length: UIntN,
    context: *mut c_void,
) -> EfiStatus;

/// Function pointer for sending an attribute request or command.
pub type EfiBluetoothAttributeSendRequest = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothAttributeProtocol,
    data: *mut c_void,
    data_length: UIntN,
    callback: Option<EfiBluetoothAttributeCallbackFunction>,
    context: *mut c_void,
) -> EfiStatus;

/// Function pointer for registering server notification callbacks.
pub type EfiBluetoothAttributeRegisterForServerNotification =
    unsafe extern "efiapi" fn(
        this: *mut EfiBluetoothAttributeProtocol,
        callback_parameter: *mut c_void,
        callback: Option<EfiBluetoothAttributeCallbackFunction>,
        context: *mut c_void,
    ) -> EfiStatus;

/// Function pointer for getting discovered service information.
pub type EfiBluetoothAttributeGetServiceInfo = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothAttributeProtocol,
    service_info_size: *mut UIntN,
    service_info: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for getting device information.
pub type EfiBluetoothAttributeGetDeviceInfo = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothAttributeProtocol,
    device_info_size: *mut UIntN,
    device_info: *mut *mut c_void,
) -> EfiStatus;

/// Raw ABI structure for `EFI_BLUETOOTH_ATTRIBUTE_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBluetoothAttributeProtocol {
    /// Sends a REQUEST or COMMAND PDU to remote GATT server.
    pub send_request: EfiBluetoothAttributeSendRequest,
    /// Registers or unregisters for server notifications/indications.
    pub register_for_server_notification: EfiBluetoothAttributeRegisterForServerNotification,
    /// Retrieves discovered GATT service information.
    pub get_service_info: EfiBluetoothAttributeGetServiceInfo,
    /// Retrieves device information for connected Bluetooth device.
    pub get_device_info: EfiBluetoothAttributeGetDeviceInfo,
}

/// Safe client wrapper around [`EfiBluetoothAttributeProtocol`].
pub struct BluetoothAttribute<'a> {
    proto: &'a mut EfiBluetoothAttributeProtocol,
}

impl<'a> BluetoothAttribute<'a> {
    /// Creates a new `BluetoothAttribute` client wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiBluetoothAttributeProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiBluetoothAttributeProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Sends a request or command buffer to the remote server.
    ///
    /// # Safety
    ///
    /// `buffer` must contain a valid Attribute Protocol PDU.
    ///
    /// # Errors
    ///
    /// Returns an error status if transmission fails.
    pub unsafe fn send_request_raw(
        &mut self,
        buffer: &mut [u8],
        callback: Option<EfiBluetoothAttributeCallbackFunction>,
        context: *mut c_void,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.send_request)(
                self.proto,
                buffer.as_mut_ptr().cast::<c_void>(),
                buffer.len(),
                callback,
                context,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
