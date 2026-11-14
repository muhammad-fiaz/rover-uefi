//! # Bluetooth Low Energy (LE) Configuration Protocol
//!
//! Provides services for user interface configuration and device pairing for Bluetooth LE
//! devices as defined in UEFI Specification 2.7+.

pub use super::guid::EFI_BLUETOOTH_LE_CONFIG_PROTOCOL_GUID;
use crate::abi::types::UIntN;
use crate::base::EfiStatus;
use crate::protocols::bluetooth::attribute::BluetoothLeAddress;
use core::ffi::c_void;

/// Scan parameters for Bluetooth LE discovery.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiBluetoothLeConfigScanParameter {
    /// Structure version (0).
    pub version: u32,
    /// Passive scanning (0) or active scanning (1).
    pub scan_type: u8,
    /// Scan interval in units of 0.625ms.
    pub scan_interval: u16,
    /// Scan window in units of 0.625ms.
    pub scan_window: u16,
    /// Scanning filter policy.
    pub scanning_filter_policy: u8,
    /// Advertisement flag filter bitmask.
    pub advertisement_flag_filter: u8,
}

/// Information passed to scan callback upon device discovery.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiBluetoothLeScanCallbackInformation {
    /// Remote device address.
    pub bd_addr: BluetoothLeAddress,
    /// Direct address if directed advertisement.
    pub direct_address: BluetoothLeAddress,
    /// State of the remote device.
    pub remote_device_state: u8,
    /// Received Signal Strength Indication (RSSI) in dBm.
    pub rssi: i8,
    /// Advertisement data payload size in bytes.
    pub advertisement_data_size: UIntN,
    /// Pointer to advertisement data payload.
    pub advertisement_data: *mut c_void,
}

/// Callback function invoked when an LE device is discovered during scan.
pub type EfiBluetoothLeConfigScanCallbackFunction = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothLeConfigProtocol,
    context: *mut c_void,
    callback_info: *mut EfiBluetoothLeScanCallbackInformation,
) -> EfiStatus;

/// Function pointer for initializing the Bluetooth LE controller.
pub type EfiBluetoothLeConfigInit =
    unsafe extern "efiapi" fn(this: *mut EfiBluetoothLeConfigProtocol) -> EfiStatus;

/// Function pointer for scanning for Bluetooth LE devices.
pub type EfiBluetoothLeConfigScan = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothLeConfigProtocol,
    re_scan: bool,
    timeout: u32,
    scan_parameter: *const EfiBluetoothLeConfigScanParameter,
    callback: EfiBluetoothLeConfigScanCallbackFunction,
    context: *mut c_void,
) -> EfiStatus;

/// Function pointer for connecting to a remote LE device.
pub type EfiBluetoothLeConfigConnect = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothLeConfigProtocol,
    auto_reconnect: bool,
    do_bonding: bool,
    connect_parameter: *const c_void,
    bd_addr: *const BluetoothLeAddress,
) -> EfiStatus;

/// Function pointer for disconnecting from a remote LE device.
pub type EfiBluetoothLeConfigDisconnect = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothLeConfigProtocol,
    bd_addr: *const BluetoothLeAddress,
    reason: u8,
) -> EfiStatus;

/// Function pointer for getting configuration data.
pub type EfiBluetoothLeConfigGetData = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothLeConfigProtocol,
    data_type: u32,
    data_size: *mut UIntN,
    data: *mut c_void,
) -> EfiStatus;

/// Function pointer for setting configuration data.
pub type EfiBluetoothLeConfigSetData = unsafe extern "efiapi" fn(
    this: *mut EfiBluetoothLeConfigProtocol,
    data_type: u32,
    data_size: UIntN,
    data: *const c_void,
) -> EfiStatus;

/// Raw ABI structure for `EFI_BLUETOOTH_LE_CONFIG_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBluetoothLeConfigProtocol {
    /// Initializes Bluetooth LE host controller and local radio.
    pub init: EfiBluetoothLeConfigInit,
    /// Scans for remote Bluetooth LE devices.
    pub scan: EfiBluetoothLeConfigScan,
    /// Connects to a remote LE device.
    pub connect: EfiBluetoothLeConfigConnect,
    /// Disconnects an active link with a remote LE device.
    pub disconnect: EfiBluetoothLeConfigDisconnect,
    /// Retrieves local Bluetooth LE configuration data.
    pub get_data: EfiBluetoothLeConfigGetData,
    /// Sets local Bluetooth LE configuration data.
    pub set_data: EfiBluetoothLeConfigSetData,
    /// Retrieves remote device configuration data.
    pub get_remote_data: *mut c_void,
    /// Registers Security Manager Protocol (SMP) auth callback.
    pub register_smp_auth_callback: *mut c_void,
    /// Sends SMP authentication data.
    pub send_smp_auth_data: *mut c_void,
    /// Registers SMP get-data callback.
    pub register_smp_get_data_callback: *mut c_void,
    /// Registers SMP set-data callback.
    pub register_smp_set_data_callback: *mut c_void,
    /// Registers link connect complete notification callback.
    pub register_link_connect_complete_callback: *mut c_void,
}

/// Safe client wrapper around [`EfiBluetoothLeConfigProtocol`].
pub struct BluetoothLeConfig<'a> {
    proto: &'a mut EfiBluetoothLeConfigProtocol,
}

impl<'a> BluetoothLeConfig<'a> {
    /// Creates a new `BluetoothLeConfig` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiBluetoothLeConfigProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiBluetoothLeConfigProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Initializes the Bluetooth LE host controller.
    ///
    /// # Errors
    ///
    /// Returns an error status if initialization fails.
    pub fn init(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.init)(self.proto) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Disconnects from the remote LE device identified by `bd_addr`.
    ///
    /// # Errors
    ///
    /// Returns an error status if disconnection request fails.
    pub fn disconnect(
        &mut self,
        bd_addr: &BluetoothLeAddress,
        reason: u8,
    ) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.proto.disconnect)(self.proto, core::ptr::from_ref(bd_addr), reason) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
