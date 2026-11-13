//! # Bluetooth I/O Protocol
//!
//! Defined in UEFI Specification 2.5+ Section 26.
//!
//! Provides `L2CAP` channel management, device info retrieval, and `SDP` querying.

use crate::abi::types::{Bool, UIntN};
use crate::base::EfiStatus;

/// 48-bit Bluetooth device hardware address (`BD_ADDR`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BluetoothAddress {
    /// 6-byte IEEE 802 MAC / `BD_ADDR`.
    pub address: [u8; 6],
}

impl BluetoothAddress {
    /// Creates a new Bluetooth address from 6 bytes.
    #[must_use]
    pub const fn new(address: [u8; 6]) -> Self {
        Self { address }
    }
}

/// Retrieves Bluetooth device information.
pub type EfiBluetoothIoGetDeviceInfo = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothIoProtocol,
    device_info_size: *mut UIntN,
    device_info: *mut u8,
) -> EfiStatus;

/// Retrieves Service Discovery Protocol (`SDP`) record information.
pub type EfiBluetoothIoGetSdpInfo = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothIoProtocol,
    sdp_info_size: *mut UIntN,
    sdp_info: *mut u8,
) -> EfiStatus;

/// Sends raw `L2CAP` data packet.
pub type EfiBluetoothIoL2CapRawSend = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothIoProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut u8,
    timeout: UIntN,
) -> EfiStatus;

/// Receives raw `L2CAP` data packet synchronously.
pub type EfiBluetoothIoL2CapRawReceive = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothIoProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut u8,
    timeout: UIntN,
) -> EfiStatus;

/// Asynchronous callback for `L2CAP` packets.
pub type EfiBluetoothIoAsyncFuncCallback =
    unsafe extern "efiapi" fn(data: *mut u8, data_length: UIntN, context: *mut ()) -> EfiStatus;

/// Receives raw `L2CAP` data packet asynchronously.
pub type EfiBluetoothIoL2CapRawAsyncReceive = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothIoProtocol,
    is_new_transfer: Bool,
    polling_interval: UIntN,
    data_length: UIntN,
    callback: EfiBluetoothIoAsyncFuncCallback,
    context: *mut (),
) -> EfiStatus;

/// EFI Bluetooth I/O Protocol interface (`EFI_BLUETOOTH_IO_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiBluetoothIoProtocol {
    /// Retrieves device info.
    pub get_device_info: EfiBluetoothIoGetDeviceInfo,
    /// Retrieves `SDP` records.
    pub get_sdp_info: EfiBluetoothIoGetSdpInfo,
    /// Sends raw `L2CAP` packet.
    pub l2cap_raw_send: EfiBluetoothIoL2CapRawSend,
    /// Receives raw `L2CAP` packet synchronously.
    pub l2cap_raw_receive: EfiBluetoothIoL2CapRawReceive,
    /// Receives raw `L2CAP` packet asynchronously.
    pub l2cap_raw_async_receive: EfiBluetoothIoL2CapRawAsyncReceive,
}
