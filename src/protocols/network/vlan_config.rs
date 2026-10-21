//! # VLAN Config Protocol
//!
//! Declarations for `EFI_VLAN_CONFIG_PROTOCOL` according to the UEFI Specification.
//!
//! Provides manageability interface for IEEE 802.1Q VLAN configuration on Ethernet interfaces.

use crate::base::{EfiGuid, EfiStatus};

/// The VLAN config protocol GUID: `{9e23d768-d2f3-4366-9fc3-3a7aba864374}`.
pub const VLAN_CONFIG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9e23_d768,
    0xd2f3,
    0x4366,
    [0x9f, 0xc3, 0x3a, 0x7a, 0xba, 0x86, 0x43, 0x74],
);

/// VLAN find configuration entry data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct EfiVlanFindData {
    /// VLAN identifier (1-4094, or 0 for untagged).
    pub vlan_id: u16,
    /// Priority (0-7).
    pub priority: u8,
}

/// Function pointer for `EFI_VLAN_CONFIG_SET`.
pub type EfiVlanConfigSet = unsafe extern "efiapi" fn(
    this: *mut EfiVlanConfigProtocol,
    vlan_id: u16,
    priority: u8,
) -> EfiStatus;

/// Function pointer for `EFI_VLAN_CONFIG_FIND`.
pub type EfiVlanConfigFind = unsafe extern "efiapi" fn(
    this: *mut EfiVlanConfigProtocol,
    vlan_id: *const u16,
    number_of_vlan: *mut u16,
    entries: *mut *mut EfiVlanFindData,
) -> EfiStatus;

/// Function pointer for `EFI_VLAN_CONFIG_REMOVE`.
pub type EfiVlanConfigRemove =
    unsafe extern "efiapi" fn(this: *mut EfiVlanConfigProtocol, vlan_id: u16) -> EfiStatus;

/// `EFI_VLAN_CONFIG_PROTOCOL` interface.
#[derive(Debug)]
#[repr(C)]
pub struct EfiVlanConfigProtocol {
    /// Creates or updates a VLAN configuration.
    pub set: EfiVlanConfigSet,
    /// Finds configured VLANs matching criteria.
    pub find: EfiVlanConfigFind,
    /// Removes a configured VLAN device.
    pub remove: EfiVlanConfigRemove,
}
