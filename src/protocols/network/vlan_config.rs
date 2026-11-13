//! # VLAN Config Protocol
//!
//! Declarations for `EFI_VLAN_CONFIG_PROTOCOL` according to the UEFI Specification.
//!
//! Provides manageability interface for IEEE 802.1Q VLAN configuration on Ethernet interfaces.

use crate::base::{EfiGuid, EfiStatus};

/// The VLAN config protocol GUID: `{9e23d768-d2f3-4366-9fc3-3a7aba864374}`.
pub const VLAN_CONFIG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9E23_D768,
    0xD2F3,
    0x4366,
    [0x9F, 0xC3, 0x3A, 0x7A, 0xBA, 0x86, 0x43, 0x74],
);

/// The `EFI_VLAN_CONFIG_PROTOCOL_GUID`.
pub const EFI_VLAN_CONFIG_PROTOCOL_GUID: EfiGuid = VLAN_CONFIG_PROTOCOL_GUID;

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

/// Safe wrapper around [`EfiVlanConfigProtocol`].
pub struct VlanConfig<'a> {
    proto: &'a mut EfiVlanConfigProtocol,
}

impl<'a> VlanConfig<'a> {
    /// Wraps a mutable reference to [`EfiVlanConfigProtocol`].
    pub fn new(proto: &'a mut EfiVlanConfigProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw mutable pointer to [`EfiVlanConfigProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiVlanConfigProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Creates a new VLAN or modifies an existing VLAN's priority on the interface.
    ///
    /// `vlan_id` must be between 0 and 4094, and `priority` must be between 0 and 7.
    ///
    /// # Errors
    ///
    /// Returns an error status if parameters are invalid or memory allocation fails.
    pub fn set(&mut self, vlan_id: u16, priority: u8) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set)(self.proto, vlan_id, priority) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Finds configured VLANs matching `vlan_id`, or all configured VLANs if `vlan_id` is `None`.
    ///
    /// Returns a tuple containing a pointer to the buffer of entries allocated by firmware
    /// and the number of returned entries. The caller is responsible for freeing the buffer.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if no matching VLANs are configured.
    pub fn find(&mut self, vlan_id: Option<u16>) -> Result<(*mut EfiVlanFindData, u16), EfiStatus> {
        let id_val = vlan_id.unwrap_or(0);
        let id_ptr = if vlan_id.is_some() {
            core::ptr::from_ref(&id_val)
        } else {
            core::ptr::null()
        };

        let mut count: u16 = 0;
        let mut entries_ptr: *mut EfiVlanFindData = core::ptr::null_mut();

        let status = unsafe {
            (self.proto.find)(
                self.proto,
                id_ptr,
                core::ptr::from_mut(&mut count),
                core::ptr::from_mut(&mut entries_ptr),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok((entries_ptr, count))
        } else {
            Err(status)
        }
    }

    /// Removes a configured VLAN by identifier (0-4094).
    ///
    /// # Errors
    ///
    /// Returns an error status if the VLAN does not exist or parameters are invalid.
    pub fn remove(&mut self, vlan_id: u16) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.remove)(self.proto, vlan_id) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
