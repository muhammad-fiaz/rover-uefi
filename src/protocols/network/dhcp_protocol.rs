//! # DHCP Protocols
//!
//! Defined by the UEFI Specification Section 29.5: `DHCPv4` Protocol and
//! Section 29.6: `DHCPv6` Protocol.
//!
//! Provides dynamic host configuration and address lease acquisition services.

use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use crate::protocols::network::types::{Ipv4Address, Ipv6Address};
use core::ffi::c_void;

/// The `EFI_DHCP4_PROTOCOL_GUID`.
pub const EFI_DHCP4_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x8A21_9110,
    0x4A52,
    0x41E3,
    [0x85, 0xB5, 0xD2, 0x5C, 0x1B, 0x64, 0x70, 0x16],
);

/// The `EFI_DHCP6_PROTOCOL_GUID`.
pub const EFI_DHCP6_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x87C8_8741,
    0x05AD,
    0x4D7E,
    [0xBB, 0xC4, 0xA1, 0x43, 0x8A, 0xDE, 0x22, 0x90],
);

/// `DHCPv4` client state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiDhcp4State {
    /// Client is stopped.
    Dhcp4Stopped = 0,
    /// Client is initializing.
    Dhcp4Init = 1,
    /// Client is selecting an offered lease.
    Dhcp4Selecting = 2,
    /// Client is requesting lease assignment.
    Dhcp4Requesting = 3,
    /// Lease is bound and active.
    Dhcp4Bound = 4,
    /// Client is renewing its lease.
    Dhcp4Renewing = 5,
    /// Client is rebinding its lease.
    Dhcp4Rebinding = 6,
    /// Client is initializing after restart.
    Dhcp4InitReboot = 7,
    /// Client is in reboot state.
    Dhcp4Rebooting = 8,
}

/// Operational mode data for `DHCPv4`.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiDhcp4ModeData {
    /// Current state of the `DHCPv4` client state machine.
    pub state: EfiDhcp4State,
    /// Assigned client IPv4 address.
    pub client_address: Ipv4Address,
    /// Assigned subnet mask.
    pub subnet_mask: Ipv4Address,
    /// Assigned router / gateway address.
    pub router_address: Ipv4Address,
    /// Assigned server IPv4 address.
    pub server_address: Ipv4Address,
    /// Lease duration in seconds.
    pub lease_time: u32,
}

/// The raw ABI structure for `EFI_DHCP4_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDhcp4Protocol {
    /// Retrieves current operational mode and lease details.
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *const EfiDhcp4Protocol,
        mode_data: *mut EfiDhcp4ModeData,
    ) -> EfiStatus,
    /// Configures or resets the `DHCPv4` client.
    pub configure: unsafe extern "efiapi" fn(
        this: *const EfiDhcp4Protocol,
        dhcp4_cfg_data: *const c_void,
    ) -> EfiStatus,
    /// Starts the `DHCPv4` lease negotiation state machine.
    pub start: unsafe extern "efiapi" fn(
        this: *const EfiDhcp4Protocol,
        completion_event: EfiEvent,
    ) -> EfiStatus,
    /// Renews or rebinds an existing lease.
    pub renew_rebind: unsafe extern "efiapi" fn(
        this: *const EfiDhcp4Protocol,
        rebind_request: u8,
        completion_event: EfiEvent,
    ) -> EfiStatus,
    /// Releases the active lease and stops client.
    pub release: unsafe extern "efiapi" fn(this: *const EfiDhcp4Protocol) -> EfiStatus,
    /// Stops the `DHCPv4` client without releasing lease.
    pub stop: unsafe extern "efiapi" fn(this: *const EfiDhcp4Protocol) -> EfiStatus,
    /// Builds a raw DHCP packet.
    pub build: unsafe extern "efiapi" fn(
        this: *const EfiDhcp4Protocol,
        seed_packet: *const c_void,
        delete_count: u32,
        delete_list: *const u8,
        append_count: u32,
        append_list: *const c_void,
        new_packet: *mut *mut c_void,
    ) -> EfiStatus,
    /// Transmits a raw DHCP request packet.
    pub transmit:
        unsafe extern "efiapi" fn(this: *const EfiDhcp4Protocol, token: *mut c_void) -> EfiStatus,
}

/// `DHCPv6` client state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiDhcp6State {
    /// Client is stopped.
    Dhcp6Stopped = 0,
    /// Client is initializing.
    Dhcp6Init = 1,
    /// Client is selecting an offered lease.
    Dhcp6Selecting = 2,
    /// Client is requesting lease assignment.
    Dhcp6Requesting = 3,
    /// Client is confirming previous lease.
    Dhcp6Confirming = 4,
    /// Lease is bound and active.
    Dhcp6Bound = 5,
    /// Client is renewing its lease.
    Dhcp6Renewing = 6,
    /// Client is rebinding its lease.
    Dhcp6Rebinding = 7,
    /// Client is declining invalid addresses.
    Dhcp6Declining = 8,
    /// Client is releasing its lease.
    Dhcp6Releasing = 9,
}

/// Operational mode data for `DHCPv6`.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiDhcp6ModeData {
    /// Current state of the `DHCPv6` client.
    pub state: EfiDhcp6State,
    /// Client link-local IPv6 address.
    pub client_address: Ipv6Address,
    /// Server IPv6 address.
    pub server_address: Ipv6Address,
    /// Identity association count.
    pub ia_count: u32,
}

/// The raw ABI structure for `EFI_DHCP6_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDhcp6Protocol {
    /// Retrieves current operational mode and lease details.
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *const EfiDhcp6Protocol,
        mode_data: *mut EfiDhcp6ModeData,
        dhcp6_config_data: *mut c_void,
    ) -> EfiStatus,
    /// Configures or resets the `DHCPv6` client.
    pub configure: unsafe extern "efiapi" fn(
        this: *const EfiDhcp6Protocol,
        dhcp6_cfg_data: *const c_void,
    ) -> EfiStatus,
    /// Starts `DHCPv6` negotiation.
    pub start: unsafe extern "efiapi" fn(this: *const EfiDhcp6Protocol) -> EfiStatus,
    /// Renews or rebinds `DHCPv6` identity associations.
    pub renew_rebind:
        unsafe extern "efiapi" fn(this: *const EfiDhcp6Protocol, rebind_request: u8) -> EfiStatus,
    /// Releases `DHCPv6` identity associations.
    pub release: unsafe extern "efiapi" fn(
        this: *const EfiDhcp6Protocol,
        ia_count: u32,
        ia_list: *const c_void,
    ) -> EfiStatus,
    /// Stops the `DHCPv6` client.
    pub stop: unsafe extern "efiapi" fn(this: *const EfiDhcp6Protocol) -> EfiStatus,
    /// Parses options out of a `DHCPv6` packet.
    pub parse: unsafe extern "efiapi" fn(
        this: *const EfiDhcp6Protocol,
        packet: *const c_void,
        option_count: *mut u32,
        packet_option_list: *mut *mut c_void,
    ) -> EfiStatus,
}
