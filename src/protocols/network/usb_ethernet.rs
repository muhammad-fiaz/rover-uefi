//! # `USB` Ethernet Protocol and Client API
//!
//! Provides the `USB` Ethernet protocol used to bind and drive `USB` network
//! adapters (such as `CDC-ECM`, `CDC-NCM`, `RNDIS`, ASIX AX88772, and Realtek RTL8152)
//! across laptops, tablets, mobile and Android devices, and virtual environments.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// `USB` Ethernet Protocol GUID.
pub const USB_ETHERNET_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x8D89_69CC,
    0xFEB0,
    0x4303,
    [0xB2, 0x1A, 0x1F, 0x11, 0x6F, 0x38, 0x56, 0x43],
);

/// `USB` Communications Device Class code (`CDC`).
pub const USB_CDC_CLASS: u8 = 0x02;
/// `USB` `CDC` Abstract Control Model subclass.
pub const USB_CDC_ACM_SUBCLASS: u8 = 0x02;
/// `USB` `CDC` Ethernet Control Model subclass (`ECM`).
pub const USB_CDC_ECM_SUBCLASS: u8 = 0x06;
/// `USB` `CDC` Network Control Model subclass (`NCM`).
pub const USB_CDC_NCM_SUBCLASS: u8 = 0x0D;
/// `USB` `CDC` Data Class code.
pub const USB_CDC_DATA_CLASS: u8 = 0x0A;
/// `USB` Miscellaneous Class code.
pub const USB_MISC_CLASS: u8 = 0xEF;
/// `USB` `RNDIS` subclass code.
pub const USB_RNDIS_SUBCLASS: u8 = 0x04;
/// `USB` `RNDIS` Ethernet protocol code.
pub const USB_RNDIS_ETHERNET_PROTOCOL: u8 = 0x01;

/// Functional descriptor subtypes.
pub mod desc_subtype {
    /// Header functional descriptor.
    pub const HEADER: u8 = 0x00;
    /// Union functional descriptor.
    pub const UNION: u8 = 0x06;
    /// Ethernet functional descriptor.
    pub const ETHERNET: u8 = 0x0F;
}

/// Packet filter bitmask flags for `USB` Ethernet devices.
pub mod packet_filter {
    /// Promiscuous mode: receive all packets.
    pub const PROMISCUOUS: u16 = 1 << 0;
    /// Receive all multicast packets.
    pub const ALL_MULTICAST: u16 = 1 << 1;
    /// Directed packets matching station `MAC`.
    pub const DIRECTED: u16 = 1 << 2;
    /// Broadcast packets.
    pub const BROADCAST: u16 = 1 << 3;
    /// Multicast packets matching multicast address list.
    pub const MULTICAST: u16 = 1 << 4;
}

/// `USB` Header Functional Descriptor.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UsbHeaderFunDescriptor {
    /// Size of this descriptor in bytes.
    pub function_length: u8,
    /// Descriptor type (`CS_INTERFACE` 0x24).
    pub descriptor_type: u8,
    /// Descriptor subtype (`desc_subtype::HEADER`).
    pub descriptor_subtype: u8,
    /// `CDC` specification release number in `BCD`.
    pub bcd_cdc: u16,
}

/// `USB` Union Functional Descriptor.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UsbUnionFunDescriptor {
    /// Size of this descriptor in bytes.
    pub function_length: u8,
    /// Descriptor type (`CS_INTERFACE` 0x24).
    pub descriptor_type: u8,
    /// Descriptor subtype (`desc_subtype::UNION`).
    pub descriptor_subtype: u8,
    /// Controlling interface number.
    pub master_interface: u8,
    /// Subordinate/data interface number.
    pub slave_interface: u8,
}

/// `USB` Ethernet Functional Descriptor.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UsbEthernetFunDescriptor {
    /// Size of this descriptor in bytes.
    pub function_length: u8,
    /// Descriptor type (`CS_INTERFACE` 0x24).
    pub descriptor_type: u8,
    /// Descriptor subtype (`desc_subtype::ETHERNET`).
    pub descriptor_subtype: u8,
    /// Index of string descriptor containing station `MAC` address.
    pub mac_address_index: u8,
    /// Bitmap of ethernet statistics supported by device.
    pub ethernet_statistics: u32,
    /// Maximum segment size (typically 1514).
    pub max_segment_size: u16,
    /// Number of multicast filters supported.
    pub number_mc_filters: u16,
    /// Number of wake pattern filters.
    pub number_power_filters: u8,
}

/// Universal Network Driver Interface (`UNDI`) table for `USB` Ethernet.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UsbEthernetUndi {
    /// Pointer to undi get state function.
    pub get_state: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi start function.
    pub start: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi stop function.
    pub stop: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi get init info function.
    pub get_init_info: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi get config info function.
    pub get_config_info: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi initialize function.
    pub initialize: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi reset function.
    pub reset: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi shutdown function.
    pub shutdown: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi interrupt enable function.
    pub interrupt_enable: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi receive filter function.
    pub receive_filter: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi station address function.
    pub station_address: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi statistics function.
    pub statistics: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi mcast ip-to-mac function.
    pub mcast_ip_to_mac: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi nvram data function.
    pub nv_data: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi status function.
    pub get_status: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi fill header function.
    pub fill_header: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi transmit function.
    pub transmit: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
    /// Pointer to undi receive function.
    pub receive: Option<unsafe extern "efiapi" fn(cpb: u64) -> u32>,
}

/// `USB` Ethernet Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UsbEthernetProtocol {
    /// `UNDI` function dispatch table.
    pub usb_eth_undi: UsbEthernetUndi,
    /// Initialize device hardware.
    pub usb_eth_initialize:
        Option<unsafe extern "efiapi" fn(this: *mut UsbEthernetProtocol) -> EfiStatus>,
    /// Retrieve device statistics.
    pub usb_eth_statistics:
        Option<unsafe extern "efiapi" fn(this: *mut UsbEthernetProtocol) -> EfiStatus>,
    /// Receive frame from network.
    pub usb_eth_receive:
        Option<unsafe extern "efiapi" fn(this: *mut UsbEthernetProtocol) -> EfiStatus>,
    /// Transmit frame onto network.
    pub usb_eth_transmit:
        Option<unsafe extern "efiapi" fn(this: *mut UsbEthernetProtocol) -> EfiStatus>,
    /// Handle interrupt status.
    pub usb_eth_interrupt:
        Option<unsafe extern "efiapi" fn(this: *mut UsbEthernetProtocol) -> EfiStatus>,
    /// Retrieve station `MAC` address.
    pub usb_eth_mac_address: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            mac_addr: *mut [u8; 6],
        ) -> EfiStatus,
    >,
    /// Retrieve maximum bulk transfer size.
    pub usb_eth_max_bulk_size: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            max_bulk_size: *mut usize,
        ) -> EfiStatus,
    >,
    /// Read header functional descriptor.
    pub usb_header_fun_descriptor: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            desc: *mut UsbHeaderFunDescriptor,
        ) -> EfiStatus,
    >,
    /// Read union functional descriptor.
    pub usb_union_fun_descriptor: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            desc: *mut UsbUnionFunDescriptor,
        ) -> EfiStatus,
    >,
    /// Read ethernet functional descriptor.
    pub usb_eth_fun_descriptor: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            desc: *mut UsbEthernetFunDescriptor,
        ) -> EfiStatus,
    >,
    /// Set multicast filters.
    pub set_usb_eth_mcast_filter: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            value: u16,
            mcast_addr: *const c_void,
        ) -> EfiStatus,
    >,
    /// Set power management pattern filter.
    pub set_usb_eth_power_pattern_filter: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            value: u16,
            length: u16,
            pattern_filter: *const c_void,
        ) -> EfiStatus,
    >,
    /// Get power management pattern filter.
    pub get_usb_eth_power_pattern_filter: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            value: u16,
            pattern_active: *mut bool,
        ) -> EfiStatus,
    >,
    /// Set ethernet packet filter bitmap.
    pub set_usb_eth_packet_filter:
        Option<unsafe extern "efiapi" fn(this: *mut UsbEthernetProtocol, value: u16) -> EfiStatus>,
    /// Get ethernet device statistic.
    pub get_usb_eth_statistic: Option<
        unsafe extern "efiapi" fn(
            this: *mut UsbEthernetProtocol,
            feature_selector: u16,
            statistic: *mut c_void,
        ) -> EfiStatus,
    >,
}

/// Safe client-side wrapper for `USB` Ethernet.
#[derive(Debug)]
pub struct UsbEthernet<'a> {
    protocol: &'a mut UsbEthernetProtocol,
}

impl<'a> UsbEthernet<'a> {
    /// Create a new safe view of a `USB` Ethernet device.
    #[must_use]
    pub fn new(protocol: &'a mut UsbEthernetProtocol) -> Self {
        Self { protocol }
    }

    /// Initialize the `USB` Ethernet hardware.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn initialize(&mut self) -> Result<(), EfiStatus> {
        if let Some(init_fn) = self.protocol.usb_eth_initialize {
            let status = unsafe { init_fn(self.protocol) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Retrieve the station `MAC` address.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn mac_address(&mut self) -> Result<[u8; 6], EfiStatus> {
        if let Some(mac_fn) = self.protocol.usb_eth_mac_address {
            let mut mac = [0u8; 6];
            let status = unsafe { mac_fn(self.protocol, &raw mut mac) };
            if status == EfiStatus::SUCCESS {
                Ok(mac)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Set the packet filter bitmask.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn set_packet_filter(&mut self, filter: u16) -> Result<(), EfiStatus> {
        if let Some(set_fn) = self.protocol.set_usb_eth_packet_filter {
            let status = unsafe { set_fn(self.protocol, filter) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Retrieve maximum bulk transfer size.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn max_bulk_size(&mut self) -> Result<usize, EfiStatus> {
        if let Some(bulk_fn) = self.protocol.usb_eth_max_bulk_size {
            let mut size = 0usize;
            let status = unsafe { bulk_fn(self.protocol, &raw mut size) };
            if status == EfiStatus::SUCCESS {
                Ok(size)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
