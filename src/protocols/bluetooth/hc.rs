//! # Bluetooth Host Controller Protocol
//!
//! Defined in UEFI Specification 2.5+ Section 26.
//!
//! Abstracts the Bluetooth HCI (Host Controller Interface) layer for sending
//! and receiving HCI commands, events, `ACL` data packets, and `SCO` audio packets.

use crate::abi::types::{Bool, UIntN};
use crate::base::EfiStatus;

/// Asynchronous callback for Bluetooth HCI packets.
pub type EfiBluetoothHcAsyncFuncCallback =
    unsafe extern "efiapi" fn(data: *mut u8, data_length: UIntN, context: *mut ()) -> EfiStatus;

/// Sends an HCI command packet.
pub type EfiBluetoothHcSendCommand = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut u8,
    timeout: UIntN,
) -> EfiStatus;

/// Receives an HCI event packet synchronously.
pub type EfiBluetoothHcReceiveEvent = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut u8,
    timeout: UIntN,
) -> EfiStatus;

/// Submits or cancels an asynchronous HCI event receive transfer.
pub type EfiBluetoothHcAsyncReceiveEvent = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    is_new_transfer: Bool,
    polling_interval: UIntN,
    data_length: UIntN,
    callback: EfiBluetoothHcAsyncFuncCallback,
    context: *mut (),
) -> EfiStatus;

/// Sends an HCI `ACL` data packet.
pub type EfiBluetoothHcSendAclData = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut u8,
    timeout: UIntN,
) -> EfiStatus;

/// Receives an HCI `ACL` data packet synchronously.
pub type EfiBluetoothHcReceiveAclData = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut u8,
    timeout: UIntN,
) -> EfiStatus;

/// Submits or cancels an asynchronous HCI `ACL` data receive transfer.
pub type EfiBluetoothHcAsyncReceiveAclData = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    is_new_transfer: Bool,
    polling_interval: UIntN,
    data_length: UIntN,
    callback: EfiBluetoothHcAsyncFuncCallback,
    context: *mut (),
) -> EfiStatus;

/// Sends an HCI `SCO` (synchronous connection-oriented audio) data packet.
pub type EfiBluetoothHcSendScoData = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut u8,
    timeout: UIntN,
) -> EfiStatus;

/// Receives an HCI `SCO` audio data packet synchronously.
pub type EfiBluetoothHcReceiveScoData = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut u8,
    timeout: UIntN,
) -> EfiStatus;

/// Submits or cancels an asynchronous HCI `SCO` receive transfer.
pub type EfiBluetoothHcAsyncReceiveScoData = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothHcProtocol,
    is_new_transfer: Bool,
    polling_interval: UIntN,
    data_length: UIntN,
    callback: EfiBluetoothHcAsyncFuncCallback,
    context: *mut (),
) -> EfiStatus;

/// EFI Bluetooth Host Controller Protocol interface (`EFI_BLUETOOTH_HC_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiBluetoothHcProtocol {
    /// Sends an HCI command packet.
    pub send_command: EfiBluetoothHcSendCommand,
    /// Receives an HCI event packet.
    pub receive_event: EfiBluetoothHcReceiveEvent,
    /// Receives an HCI event asynchronously.
    pub async_receive_event: EfiBluetoothHcAsyncReceiveEvent,
    /// Sends an HCI `ACL` data packet.
    pub send_acl_data: EfiBluetoothHcSendAclData,
    /// Receives an HCI `ACL` data packet.
    pub receive_acl_data: EfiBluetoothHcReceiveAclData,
    /// Receives an HCI `ACL` data packet asynchronously.
    pub async_receive_acl_data: EfiBluetoothHcAsyncReceiveAclData,
    /// Sends an HCI `SCO` voice/audio data packet.
    pub send_sco_data: EfiBluetoothHcSendScoData,
    /// Receives an HCI `SCO` voice/audio data packet.
    pub receive_sco_data: EfiBluetoothHcReceiveScoData,
    /// Receives an HCI `SCO` packet asynchronously.
    pub async_receive_sco_data: EfiBluetoothHcAsyncReceiveScoData,
}
