//! # Redfish Host Interface Specifications
//!
//! Provides data structures and constants for SMBIOS Type 42 (Management Controller
//! Host Interface) parsing and configuration for Redfish over IP over USB or PCI/PCIe.

/// Redfish Host Interface device type: USB network interface V2.
pub const REDFISH_HOST_INTERFACE_DEVICE_TYPE_USB_V2: u8 = 0x04;

/// Redfish Host Interface device type: PCI / `PCIe` network interface V2.
pub const REDFISH_HOST_INTERFACE_DEVICE_TYPE_PCI_PCIE_V2: u8 = 0x05;

/// Redfish Host Interface device type: OEM minimum identifier.
pub const REDFISH_HOST_INTERFACE_DEVICE_TYPE_OEM_MIN: u8 = 0x80;

/// Redfish Host Interface device type: OEM maximum identifier.
pub const REDFISH_HOST_INTERFACE_DEVICE_TYPE_OEM_MAX: u8 = 0xFF;

/// Redfish Host IP assignment: Unknown.
pub const REDFISH_HOST_IP_ASSIGNMENT_UNKNOWN: u8 = 0x00;
/// Redfish Host IP assignment: Static configuration.
pub const REDFISH_HOST_IP_ASSIGNMENT_STATIC: u8 = 0x01;
/// Redfish Host IP assignment: DHCP acquired.
pub const REDFISH_HOST_IP_ASSIGNMENT_DHCP: u8 = 0x02;
/// Redfish Host IP assignment: Auto-configured (e.g. IPv6 SLAAC or link-local).
pub const REDFISH_HOST_IP_ASSIGNMENT_AUTO_CONFIGURE: u8 = 0x03;
/// Redfish Host IP assignment: Selected by the host OS.
pub const REDFISH_HOST_IP_ASSIGNMENT_HOST_SELECTED: u8 = 0x04;

/// Redfish Host IP address format: Unknown.
pub const REDFISH_HOST_IP_FORMAT_UNKNOWN: u8 = 0x00;
/// Redfish Host IP address format: IPv4.
pub const REDFISH_HOST_IP_FORMAT_IPV4: u8 = 0x01;
/// Redfish Host IP address format: IPv6.
pub const REDFISH_HOST_IP_FORMAT_IPV6: u8 = 0x02;

/// USB Network Interface Device Descriptor V2 (SMBIOS Type 42 payload).
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbInterfaceDeviceDescriptorV2 {
    /// Structure length in bytes.
    pub length: u8,
    /// USB vendor ID from device descriptor.
    pub id_vendor: u16,
    /// USB product ID from device descriptor.
    pub id_product: u16,
    /// Serial number string index in SMBIOS string table.
    pub serial_number_str: u8,
    /// MAC address of the virtual USB network interface.
    pub mac_address: [u8; 6],
    /// Device characteristics bitmask.
    pub characteristics: u16,
    /// Credential bootstrapping handle.
    pub credential_bootstrapping_handle: u16,
}

/// PCI / `PCIe` Network Interface Device Descriptor V2 (SMBIOS Type 42 payload).
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PciInterfaceDeviceDescriptorV2 {
    /// Structure length in bytes.
    pub length: u8,
    /// PCI vendor ID.
    pub vendor_id: u16,
    /// PCI device ID.
    pub device_id: u16,
    /// PCI subsystem vendor ID.
    pub subsystem_vendor_id: u16,
    /// PCI subsystem ID.
    pub subsystem_id: u16,
    /// MAC address of the network interface.
    pub mac_address: [u8; 6],
    /// PCI segment group number.
    pub segment_group_number: u16,
    /// PCI bus number.
    pub bus_number: u8,
    /// PCI device and function number encoded as `(device << 3) | function`.
    pub device_function_number: u8,
}
