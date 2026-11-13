//! Adapter Information Protocol (AIP) definition and safe wrapper.
//!
//! Defined in UEFI Specification 2.4+ Section 29.
//! The Adapter Information Protocol provides dynamic discovery and configuration
//! of network adapter hardware properties, media state, offload capabilities,
//! SAN addresses, and firmware boot options.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::network::types::EfiMacAddress;
use core::ffi::c_void;

/// UEFI Adapter Information Protocol GUID: `{E5DD1403-D622-C24E-8488-C71B17F5E802}`.
pub const EFI_ADAPTER_INFORMATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE5DD_1403,
    0xD622,
    0xC24E,
    [0x84, 0x88, 0xC7, 0x1B, 0x17, 0xF5, 0xE8, 0x02],
);

/// Adapter Media State Information Type GUID: `{D7C74207-A831-4A26-B1F5-D193065CE8B6}`.
pub const EFI_ADAPTER_INFO_MEDIA_STATE_GUID: EfiGuid = EfiGuid::new(
    0xD7C7_4207,
    0xA831,
    0x4A26,
    [0xB1, 0xF5, 0xD1, 0x93, 0x06, 0x5C, 0xE8, 0xB6],
);

/// Adapter Network Boot Information Type GUID: `{1FBD2960-4130-41E5-94AC-D2CF037FB37C}`.
pub const EFI_ADAPTER_INFO_NETWORK_BOOT_GUID: EfiGuid = EfiGuid::new(
    0x1FBD_2960,
    0x4130,
    0x41E5,
    [0x94, 0xAC, 0xD2, 0xCF, 0x03, 0x7F, 0xB3, 0x7C],
);

/// Adapter SAN MAC Address Information Type GUID: `{114DA5EF-2CF1-4E12-9BBB-C470B55205D9}`.
pub const EFI_ADAPTER_INFO_SAN_MAC_ADDRESS_GUID: EfiGuid = EfiGuid::new(
    0x114D_A5EF,
    0x2CF1,
    0x4E12,
    [0x9B, 0xBB, 0xC4, 0x70, 0xB5, 0x52, 0x05, 0xD9],
);

/// Adapter UNDI IPv6 Support Information Type GUID: `{4BD56BE3-4975-4D8A-A0AD-C491204B5D4D}`.
pub const EFI_ADAPTER_INFO_UNDI_IPV6_SUPPORT_GUID: EfiGuid = EfiGuid::new(
    0x4BD5_6BE3,
    0x4975,
    0x4D8A,
    [0xA0, 0xAD, 0xC4, 0x91, 0x20, 0x4B, 0x5D, 0x4D],
);

/// Adapter Media Type Information Type GUID: `{8484472F-71EC-411A-B39C-62CD94D9916E}`.
pub const EFI_ADAPTER_INFO_MEDIA_TYPE_GUID: EfiGuid = EfiGuid::new(
    0x8484_472F,
    0x71EC,
    0x411A,
    [0xB3, 0x9C, 0x62, 0xCD, 0x94, 0xD9, 0x91, 0x6E],
);

/// Coherent Device Attribute Table (CDAT) Information Type GUID: `{77AF24D1-B6F0-42B9-83F5-8FE6E83EB6F0}`.
pub const EFI_ADAPTER_INFO_CDAT_TYPE_GUID: EfiGuid = EfiGuid::new(
    0x77AF_24D1,
    0xB6F0,
    0x42B9,
    [0x83, 0xF5, 0x8F, 0xE6, 0xE8, 0x3E, 0xB6, 0xF0],
);

/// Adapter Media Type: 802.3 Ethernet Network Adapter.
pub const EFI_ADAPTER_INFO_MEDIA_TYPE_ETHERNET: u8 = 1;

/// Adapter Media Type: 802.11 Wireless Network Adapter.
pub const EFI_ADAPTER_INFO_MEDIA_TYPE_WIRELESS: u8 = 2;

/// Current network media attachment state.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiAdapterInfoMediaState {
    /// Media state status (`SUCCESS`, `NOT_READY`, or `NO_MEDIA`).
    pub media_state: EfiStatus,
}

/// Current network adapter physical media type.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiAdapterInfoMediaType {
    /// Physical media type identifier (`1` for Ethernet, `2` for Wireless).
    pub media_type: u8,
}

/// Network adapter boot capabilities and active configuration.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EfiAdapterInfoNetworkBoot {
    /// True if the adapter supports booting from iSCSI IPv4 targets.
    pub iscsi_ipv4_boot_capability: bool,
    /// True if the adapter supports booting from iSCSI IPv6 targets.
    pub iscsi_ipv6_boot_capability: bool,
    /// True if the adapter supports booting from `FCoE` targets.
    pub fcoe_boot_capability: bool,
    /// True if the adapter supports offload engine (TOE).
    pub offload_capability: bool,
    /// True if the adapter supports multipath I/O (MPIO).
    pub iscsi_mpio_capability: bool,
    /// True if currently configured to boot from iSCSI IPv4 targets.
    pub iscsi_ipv4_boot: bool,
    /// True if currently configured to boot from iSCSI IPv6 targets.
    pub iscsi_ipv6_boot: bool,
    /// True if currently configured to boot from `FCoE` targets.
    pub fcoe_boot: bool,
}

/// Storage Area Network (SAN) MAC address.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiAdapterInfoSanMacAddress {
    /// SAN MAC address for `FCoE` operations.
    pub san_mac_address: EfiMacAddress,
}

/// UNDI driver IPv6 traffic support.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiAdapterInfoUndiIpv6Support {
    /// True if UNDI driver supports IPv6.
    pub ipv6_support: bool,
}

/// Function prototype for querying adapter information by type GUID.
pub type EfiAdapterInfoGetInfo = unsafe extern "efiapi" fn(
    this: *const EfiAdapterInformationProtocol,
    information_type: *const EfiGuid,
    information_block: *mut *mut c_void,
    information_block_size: *mut UIntN,
) -> EfiStatus;

/// Function prototype for setting adapter information by type GUID.
pub type EfiAdapterInfoSetInfo = unsafe extern "efiapi" fn(
    this: *const EfiAdapterInformationProtocol,
    information_type: *const EfiGuid,
    information_block: *const c_void,
    information_block_size: UIntN,
) -> EfiStatus;

/// Function prototype for enumerating supported information type GUIDs.
pub type EfiAdapterInfoGetSupportedTypes = unsafe extern "efiapi" fn(
    this: *const EfiAdapterInformationProtocol,
    info_types_buffer: *mut *mut EfiGuid,
    info_types_buffer_count: *mut UIntN,
) -> EfiStatus;

/// UEFI Adapter Information Protocol structure.
#[repr(C)]
pub struct EfiAdapterInformationProtocol {
    /// Retrieves current state information for the adapter.
    pub get_information: EfiAdapterInfoGetInfo,
    /// Configures state information for the adapter.
    pub set_information: EfiAdapterInfoSetInfo,
    /// Enumerates supported information type GUIDs.
    pub get_supported_types: EfiAdapterInfoGetSupportedTypes,
}

/// Safe idiomatic wrapper around `EfiAdapterInformationProtocol`.
pub struct AdapterInformation {
    proto: *const EfiAdapterInformationProtocol,
}

impl AdapterInformation {
    /// Creates a new `AdapterInformation` wrapper.
    #[must_use]
    pub const fn new(proto: *const EfiAdapterInformationProtocol) -> Self {
        Self { proto }
    }

    /// Returns the underlying raw protocol pointer.
    #[must_use]
    pub const fn raw(&self) -> *const EfiAdapterInformationProtocol {
        self.proto
    }

    /// Retrieves an allocated information block pointer and size for `info_type`.
    ///
    /// # Errors
    ///
    /// Returns `UNSUPPORTED` if the type is unknown, `NOT_FOUND` if unavailable,
    /// or an error status on device failure.
    pub fn get_information_raw(
        &self,
        info_type: &EfiGuid,
    ) -> Result<(*mut c_void, usize), EfiStatus> {
        let mut block = core::ptr::null_mut();
        let mut size = 0 as UIntN;

        let status = unsafe {
            ((*self.proto).get_information)(
                self.proto,
                info_type,
                core::ptr::from_mut(&mut block),
                core::ptr::from_mut(&mut size),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((block, size as usize))
        } else {
            Err(status)
        }
    }

    /// Configures adapter information block for `info_type`.
    ///
    /// # Errors
    ///
    /// Returns `UNSUPPORTED` if not writable or unknown, or an error status.
    pub fn set_information(&self, info_type: &EfiGuid, info_block: &[u8]) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).set_information)(
                self.proto,
                info_type,
                info_block.as_ptr().cast::<c_void>(),
                info_block.len() as UIntN,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves the list of supported information type GUIDs allocated by the driver.
    ///
    /// Returns `(pointer_to_guids, count)`.
    ///
    /// # Errors
    ///
    /// Returns an error status if enumeration fails.
    pub fn get_supported_types_raw(&self) -> Result<(*mut EfiGuid, usize), EfiStatus> {
        let mut types_buf = core::ptr::null_mut();
        let mut count = 0 as UIntN;

        let status = unsafe {
            ((*self.proto).get_supported_types)(
                self.proto,
                core::ptr::from_mut(&mut types_buf),
                core::ptr::from_mut(&mut count),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((types_buf, count as usize))
        } else {
            Err(status)
        }
    }
}
