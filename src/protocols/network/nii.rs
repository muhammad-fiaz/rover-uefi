//! # Network Interface Identifier (NII / UNDI) Protocol
//!
//! Defined in UEFI Specification Section 24 and EFI 1.10.
//!
//! An optional low-level hardware interface protocol used to describe details
//! about the software layer (Universal Network Driver Interface, UNDI) that is
//! used to produce the Simple Network Protocol (`SNP`).

use crate::abi::Bool;
use crate::base::EfiGuid;
use core::ffi::c_void;

/// Network Interface Identifier Protocol GUID (EFI 1.10 / UEFI 2.0).
pub const EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE185_41CD,
    0xF755,
    0x4F73,
    [0x92, 0x8D, 0x64, 0x3C, 0x8A, 0x79, 0xB2, 0x29],
);

/// Network Interface Identifier Protocol GUID (UEFI 2.1b+ / UNDI 3.1).
pub const EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_GUID_31: EfiGuid = EfiGuid::new(
    0x1ACE_D566,
    0x76ED,
    0x4218,
    [0xBC, 0x81, 0x76, 0x7F, 0x1F, 0x97, 0x7A, 0x89],
);

/// Protocol revision defined in UEFI Specification 2.4+.
pub const EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_REVISION: u64 = 0x0002_0000;

/// Network Interface Types for UNDI drivers.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiNetworkInterfaceType {
    /// Universal Network Driver Interface (UNDI).
    Undi = 1,
}

/// Raw ABI structure for `EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL`.
#[repr(C)]
#[derive(Debug)]
pub struct EfiNetworkInterfaceIdentifierProtocol {
    /// Revision of the protocol interface.
    pub revision: u64,
    /// Address of identifying structure when started; 0 if not started.
    pub id: u64,
    /// Address of unrelocated network interface image in memory.
    pub image_addr: u64,
    /// Size of unrelocated network interface image in bytes.
    pub image_size: u32,
    /// 4-character ASCII string (e.g. b"UNDI") sent in DHCP option 60.
    pub string_id: [u8; 4],
    /// Network interface type code (`EfiNetworkInterfaceType`).
    pub interface_type: u8,
    /// Major version number of the UNDI specification supported.
    pub major_ver: u8,
    /// Minor version number of the UNDI specification supported.
    pub minor_ver: u8,
    /// True if the network interface supports IPv6 operations.
    pub ipv6_supported: Bool,
    /// Interface instance number identified by this protocol.
    pub if_num: u16,
}

/// UNDI Configuration Table header.
#[repr(C)]
#[derive(Debug)]
pub struct UndiConfigTable {
    /// Number of NIC devices controlled by this UNDI.
    pub number_of_interfaces: u32,
    /// Reserved field.
    pub reserved: u32,
    /// Pointer to next linked UNDI configuration table.
    pub next_link: *mut UndiConfigTable,
    /// Pointer to NIC descriptor list.
    pub nic_list: *mut c_void,
}

/// Safe client wrapper around [`EfiNetworkInterfaceIdentifierProtocol`].
pub struct NetworkInterfaceIdentifier<'a> {
    proto: &'a EfiNetworkInterfaceIdentifierProtocol,
}

impl<'a> NetworkInterfaceIdentifier<'a> {
    /// Creates a new `NetworkInterfaceIdentifier` wrapper.
    #[must_use]
    pub fn new(proto: &'a EfiNetworkInterfaceIdentifierProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *const EfiNetworkInterfaceIdentifierProtocol {
        core::ptr::from_ref(self.proto)
    }

    /// Returns the protocol revision.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.proto.revision
    }

    /// Returns whether the network interface is currently started.
    #[must_use]
    pub fn is_started(&self) -> bool {
        self.proto.id != 0
    }

    /// Returns whether the interface supports IPv6.
    #[must_use]
    pub fn is_ipv6_supported(&self) -> bool {
        self.proto.ipv6_supported != 0
    }

    /// Returns the 4-byte ASCII class identifier string (e.g. "UNDI").
    #[must_use]
    pub fn string_id(&self) -> &[u8; 4] {
        &self.proto.string_id
    }

    /// Returns the major and minor version numbers as `(major, minor)`.
    #[must_use]
    pub fn version(&self) -> (u8, u8) {
        (self.proto.major_ver, self.proto.minor_ver)
    }

    /// Returns the interface index number.
    #[must_use]
    pub fn interface_num(&self) -> u16 {
        self.proto.if_num
    }
}
