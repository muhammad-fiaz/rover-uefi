//! # Redfish Discover Protocol
//!
//! Provides service discovery for Redfish servers over local network interfaces
//! using SMBIOS Host Interface or SSDP.
//!
//! Defined in the UEFI Specification, Section 31.1.

use crate::base::{EfiEvent, EfiHandle, EfiStatus};
use crate::protocols::network::types::MacAddress;

pub use super::guid::REDFISH_DISCOVER_PROTOCOL_GUID;

/// Discover Redfish server reported in SMBIOS type 42h.
pub const EFI_REDFISH_DISCOVER_HOST_INTERFACE: u32 = 0x0000_0001;
/// Discover Redfish server using `UPnP` / SSDP search method.
pub const EFI_REDFISH_DISCOVER_SSDP: u32 = 0x0000_0002;
/// Use UDP version 6 for SSDP.
pub const EFI_REDFISH_DISCOVER_SSDP_UDP6: u32 = 0x0000_0004;
/// Keep sending `UPnP` search in the duration indicated by mask.
pub const EFI_REDFISH_DISCOVER_KEEP_ALIVE: u32 = 0x0000_0008;
/// Renew search and notify caller of all found Redfish servers.
pub const EFI_REDFISH_DISCOVER_RENEW: u32 = 0x0000_0010;
/// Validate Redfish service for host interface instance.
pub const EFI_REDFISH_DISCOVER_VALIDATION: u32 = 0x8000_0000;
/// Mask for search duration (2^N seconds).
pub const EFI_REDFISH_DISCOVER_DURATION_MASK: u32 = 0x0F00_0000;

/// Redfish token signature: `'RFTS'`.
pub const REDFISH_DISCOVER_TOKEN_SIGNATURE: u32 = 0x5354_4652;

/// IP address union for IPv4 or IPv6.
#[repr(C)]
#[derive(Clone, Copy)]
pub union EfiIpAddress {
    /// 32-bit IPv4 address words.
    pub addr: [u32; 4],
    /// Raw bytes.
    pub bytes: [u8; 16],
}

impl core::fmt::Debug for EfiIpAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // SAFETY: reading raw bytes of union
        let b = unsafe { self.bytes };
        write!(
            f,
            "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]
        )
    }
}

/// Discovered Redfish service information.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRedfishDiscoveredInformation {
    /// REST EX handle associated with this Redfish service.
    pub redfish_rest_ex_handle: EfiHandle,
    /// Indicates IPv6.
    pub is_udp6: bool,
    /// IP address of Redfish service.
    pub redfish_host_ip_address: EfiIpAddress,
    /// Redfish service version.
    pub redfish_version: usize,
    /// Location string (UTF-16).
    pub location: *mut u16,
    /// UUID string (UTF-16).
    pub uuid: *mut u16,
    /// OS string (UTF-16).
    pub os: *mut u16,
    /// OS version string (UTF-16).
    pub os_version: *mut u16,
    /// Product name string (UTF-16).
    pub product: *mut u16,
    /// Product version string (UTF-16).
    pub product_ver: *mut u16,
    /// Whether HTTPS is used.
    pub use_https: bool,
}

/// Discovered Redfish instance item.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRedfishDiscoveredInstance {
    /// Discovery status.
    pub status: EfiStatus,
    /// Discovered service information.
    pub information: EfiRedfishDiscoveredInformation,
}

/// Discovered Redfish service list.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRedfishDiscoveredList {
    /// Number of services found.
    pub number_of_service_found: usize,
    /// Pointer to array of discovered instances.
    pub redfish_instances: *mut EfiRedfishDiscoveredInstance,
}

/// Target network interface descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRedfishDiscoverNetworkInterface {
    /// MAC address.
    pub mac_address: MacAddress,
    /// Indicates IPv6.
    pub is_ipv6: bool,
    /// Subnet ID.
    pub subnet_id: EfiIpAddress,
    /// Subnet prefix length.
    pub subnet_prefix_length: u8,
    /// VLAN ID.
    pub vlan_id: u16,
}

/// Discovery token.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRedfishDiscoveredToken {
    /// Signature ('RFTS').
    pub signature: u32,
    /// Discovered list.
    pub discover_list: EfiRedfishDiscoveredList,
    /// Notification event.
    pub event: EfiEvent,
    /// Timeout in seconds (1 to 5, or 0 for synchronous).
    pub timeout: usize,
}

/// Gets the NIC list where Redfish discovery is supported.
pub type EfiRedfishDiscoverNetworkList = unsafe extern "efiapi" fn(
    this: *mut EfiRedfishDiscoverProtocol,
    image_handle: EfiHandle,
    number_of_network_interfaces: *mut usize,
    network_interfaces: *mut *mut EfiRedfishDiscoverNetworkInterface,
) -> EfiStatus;

/// Acquires Redfish services via static configuration or SSDP.
pub type EfiRedfishDiscoverAcquireService = unsafe extern "efiapi" fn(
    this: *mut EfiRedfishDiscoverProtocol,
    image_handle: EfiHandle,
    target_network_interface: *mut EfiRedfishDiscoverNetworkInterface,
    flags: u32,
    token: *mut EfiRedfishDiscoveredToken,
) -> EfiStatus;

/// Aborts Redfish service discovery on a network interface.
pub type EfiRedfishDiscoverAbortAcquire = unsafe extern "efiapi" fn(
    this: *mut EfiRedfishDiscoverProtocol,
    target_network_interface: *mut EfiRedfishDiscoverNetworkInterface,
) -> EfiStatus;

/// Releases discovered Redfish services list.
pub type EfiRedfishDiscoverReleaseService = unsafe extern "efiapi" fn(
    this: *mut EfiRedfishDiscoverProtocol,
    list: *mut EfiRedfishDiscoveredList,
) -> EfiStatus;

/// The EFI Redfish Discover Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiRedfishDiscoverProtocol {
    /// Gets NIC list capable of Redfish discovery.
    pub get_network_interface_list: EfiRedfishDiscoverNetworkList,
    /// Acquires Redfish services.
    pub acquire_redfish_service: EfiRedfishDiscoverAcquireService,
    /// Aborts Redfish service acquisition.
    pub abort_acquire_redfish_service: EfiRedfishDiscoverAbortAcquire,
    /// Releases Redfish services list.
    pub release_redfish_service: EfiRedfishDiscoverReleaseService,
}
