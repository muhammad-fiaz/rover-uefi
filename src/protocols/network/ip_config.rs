//! # IP Configuration Protocols
//!
//! Defined by the UEFI Specification Section 27.5: IP4 Config2 Protocol and
//! Section 28.5: IP6 Config Protocol.
//!
//! Manages network interface static/DHCP address configuration policies,
//! DNS nameserver lists, and default gateways.

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use crate::protocols::network::types::Ipv4Address;
use core::ffi::c_void;

/// The `EFI_IP4_CONFIG2_PROTOCOL_GUID`.
pub const EFI_IP4_CONFIG2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5B44_6304,
    0x4AA8,
    0x460B,
    [0x90, 0x2D, 0xDC, 0x46, 0x59, 0x51, 0x82, 0x2F],
);

/// The `EFI_IP6_CONFIG_PROTOCOL_GUID`.
pub const EFI_IP6_CONFIG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x937D_C2A3,
    0x1607,
    0x414F,
    [0x83, 0x70, 0x7C, 0xF9, 0x04, 0x9E, 0x84, 0xD4],
);

/// IP4 Configuration policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiIp4Config2Policy {
    /// Static IPv4 configuration.
    Static = 0,
    /// Dynamic IPv4 configuration via DHCP.
    Dhcp = 1,
    /// Maximum policy count.
    Max = 2,
}

/// IP4 Configuration data types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiIp4Config2DataType {
    /// Interface information data type.
    InterfaceInfo = 0,
    /// Policy data type (`EfiIp4Config2Policy`).
    Policy = 1,
    /// Manual static IPv4 address.
    ManualAddress = 2,
    /// Default gateway address list.
    Gateway = 3,
    /// DNS server address list.
    DnsServer = 4,
    /// Maximum data type count.
    Maximum = 5,
}

/// Static manual address entry for IPv4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiIp4Config2ManualAddress {
    /// Station IPv4 address.
    pub address: Ipv4Address,
    /// Subnet mask.
    pub subnet_mask: Ipv4Address,
}

/// The raw ABI structure for `EFI_IP4_CONFIG2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiIp4Config2Protocol {
    /// Sets configuration data for the specified data type.
    pub set_data: unsafe extern "efiapi" fn(
        this: *const EfiIp4Config2Protocol,
        data_type: EfiIp4Config2DataType,
        data_size: UIntN,
        data: *const c_void,
    ) -> EfiStatus,
    /// Gets configuration data for the specified data type.
    pub get_data: unsafe extern "efiapi" fn(
        this: *const EfiIp4Config2Protocol,
        data_type: EfiIp4Config2DataType,
        data_size: *mut UIntN,
        data: *mut c_void,
    ) -> EfiStatus,
    /// Registers notification event upon data modification.
    pub register_data_notify: unsafe extern "efiapi" fn(
        this: *const EfiIp4Config2Protocol,
        data_type: EfiIp4Config2DataType,
        event: EfiEvent,
    ) -> EfiStatus,
    /// Unregisters notification event.
    pub unregister_data_notify: unsafe extern "efiapi" fn(
        this: *const EfiIp4Config2Protocol,
        data_type: EfiIp4Config2DataType,
        event: EfiEvent,
    ) -> EfiStatus,
}

/// IP6 Configuration policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiIp6ConfigPolicy {
    /// Manual static address assignment.
    Manual = 0,
    /// Automatic address configuration (SLAAC / `DHCPv6`).
    Automatic = 1,
}

/// The raw ABI structure for `EFI_IP6_CONFIG_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiIp6ConfigProtocol {
    /// Sets IPv6 configuration parameters.
    pub set_data: unsafe extern "efiapi" fn(
        this: *const EfiIp6ConfigProtocol,
        data_type: u32,
        data_size: UIntN,
        data: *const c_void,
    ) -> EfiStatus,
    /// Gets IPv6 configuration parameters.
    pub get_data: unsafe extern "efiapi" fn(
        this: *const EfiIp6ConfigProtocol,
        data_type: u32,
        data_size: *mut UIntN,
        data: *mut c_void,
    ) -> EfiStatus,
    /// Registers event notification.
    pub register_data_notify: unsafe extern "efiapi" fn(
        this: *const EfiIp6ConfigProtocol,
        data_type: u32,
        event: EfiEvent,
    ) -> EfiStatus,
    /// Unregisters event notification.
    pub unregister_data_notify: unsafe extern "efiapi" fn(
        this: *const EfiIp6ConfigProtocol,
        data_type: u32,
        event: EfiEvent,
    ) -> EfiStatus,
}
