//! # `SMBus` Host Controller Protocol
//!
//! Defined in UEFI Platform Initialization (PI) Specification Volume 5.
//!
//! Provides hardware management and transactions for motherboard thermal sensors,
//! SPD EEPROMs (memory modules), power supplies, and voltage controllers.

use super::types::{
    EfiSmbusDeviceAddress, EfiSmbusDeviceCommand, EfiSmbusDeviceMap, EfiSmbusOperation,
    EfiSmbusUdid,
};
use crate::abi::types::{Bool, UIntN};
use crate::base::EfiStatus;

/// Executes an `SMBus` hardware transaction.
pub type EfiSmbusHcExecuteOperation = unsafe extern "efiapi" fn(
    this: *const EfiSmbusHcProtocol,
    slave_address: EfiSmbusDeviceAddress,
    command: EfiSmbusDeviceCommand,
    operation: EfiSmbusOperation,
    pec_check: Bool,
    length: *mut UIntN,
    buffer: *mut u8,
) -> EfiStatus;

/// Enumerate `SMBus` devices via `ARP` (Address Resolution Protocol).
pub type EfiSmbusHcProtocolArpDevice = unsafe extern "efiapi" fn(
    this: *const EfiSmbusHcProtocol,
    arp_all: Bool,
    smbus_udid: *mut EfiSmbusUdid,
    slave_address: *mut EfiSmbusDeviceAddress,
) -> EfiStatus;

/// Returns current mapping of enumerated `SMBus` devices.
pub type EfiSmbusHcProtocolGetArpMap = unsafe extern "efiapi" fn(
    this: *const EfiSmbusHcProtocol,
    length: *mut UIntN,
    smbus_device_map: *mut *mut EfiSmbusDeviceMap,
) -> EfiStatus;

/// Host notification callback function.
pub type EfiSmbusNotifyFunction =
    unsafe extern "efiapi" fn(slave_address: EfiSmbusDeviceAddress, data: UIntN) -> EfiStatus;

/// Registers a callback for `SMBus` Host Notify events.
pub type EfiSmbusHcProtocolNotify = unsafe extern "efiapi" fn(
    this: *const EfiSmbusHcProtocol,
    slave_address: EfiSmbusDeviceAddress,
    data: UIntN,
    notify_function: EfiSmbusNotifyFunction,
) -> EfiStatus;

/// EFI `SMBus` Host Controller Protocol interface (`EFI_SMBUS_HC_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiSmbusHcProtocol {
    /// Executes an `SMBus` bus transaction.
    pub execute: EfiSmbusHcExecuteOperation,
    /// Enumerates devices using the `SMBus` `ARP` protocol.
    pub arp_device: EfiSmbusHcProtocolArpDevice,
    /// Retrieves the `ARP` device map.
    pub get_arp_map: EfiSmbusHcProtocolGetArpMap,
    /// Registers a Host Notify callback.
    pub notify: EfiSmbusHcProtocolNotify,
}
