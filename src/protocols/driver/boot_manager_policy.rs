//! # Boot Manager Policy Protocol
//!
//! Defined in the UEFI Specification, this protocol is used by UEFI applications
//! to request that the platform Boot Manager connect controllers and devices
//! according to platform boot policy.

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::device_path::types::EfiDevicePath;

/// The `EFI_BOOT_MANAGER_POLICY_PROTOCOL_GUID`.
pub const EFI_BOOT_MANAGER_POLICY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xFEDF_8E0C,
    0xE147,
    0x11E3,
    [0x99, 0x03, 0xB8, 0xE8, 0x56, 0x2C, 0xBA, 0xFA],
);

/// GUID for requesting the Boot Manager to connect system console devices.
pub const EFI_BOOT_MANAGER_POLICY_CONSOLE_GUID: EfiGuid = EfiGuid::new(
    0xCAB0_E94C,
    0xE15F,
    0x11E3,
    [0x91, 0x8D, 0xB8, 0xE8, 0x56, 0x2C, 0xBA, 0xFA],
);

/// GUID for requesting the Boot Manager to connect network devices and protocols.
pub const EFI_BOOT_MANAGER_POLICY_NETWORK_GUID: EfiGuid = EfiGuid::new(
    0xD041_59DC,
    0xE15F,
    0x11E3,
    [0xB2, 0x61, 0xB8, 0xE8, 0x56, 0x2C, 0xBA, 0xFA],
);

/// GUID for requesting the Boot Manager to connect all platform devices and drivers.
pub const EFI_BOOT_MANAGER_POLICY_CONNECT_ALL_GUID: EfiGuid = EfiGuid::new(
    0x113B_2126,
    0xFC8A,
    0x11E3,
    [0xBD, 0x6C, 0xB8, 0xE8, 0x56, 0x2C, 0xBA, 0xFA],
);

/// GUID for requesting the Boot Manager to connect discoverable storage disks and controllers.
pub const EFI_BOOT_MANAGER_POLICY_STORAGE_GUID: EfiGuid = EfiGuid::new(
    0xCD68_FE79,
    0xD3CB,
    0x436E,
    [0xA8, 0x50, 0xF4, 0x43, 0xC8, 0x8C, 0xFB, 0x49],
);

/// Boot Manager Policy Protocol specification revision 1.0.
pub const EFI_BOOT_MANAGER_POLICY_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Function pointer for `EFI_BOOT_MANAGER_POLICY_CONNECT_DEVICE_PATH`.
pub type EfiBootManagerPolicyConnectDevicePath = unsafe extern "efiapi" fn(
    this: *const EfiBootManagerPolicyProtocol,
    device_path: *const EfiDevicePath,
    recursive: bool,
) -> EfiStatus;

/// Function pointer for `EFI_BOOT_MANAGER_POLICY_CONNECT_DEVICE_CLASS`.
pub type EfiBootManagerPolicyConnectDeviceClass = unsafe extern "efiapi" fn(
    this: *const EfiBootManagerPolicyProtocol,
    class: *const EfiGuid,
) -> EfiStatus;

/// Raw ABI protocol structure for Boot Manager Policy (`EFI_BOOT_MANAGER_POLICY_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiBootManagerPolicyProtocol {
    /// Protocol revision version.
    pub revision: u64,
    /// Connects a specific device path according to platform boot manager policy.
    pub connect_device_path: EfiBootManagerPolicyConnectDevicePath,
    /// Connects a device class according to platform boot manager policy.
    pub connect_device_class: EfiBootManagerPolicyConnectDeviceClass,
}

/// Safe client wrapper around [`EfiBootManagerPolicyProtocol`].
pub struct BootManagerPolicy<'a> {
    proto: &'a EfiBootManagerPolicyProtocol,
}

impl<'a> BootManagerPolicy<'a> {
    /// Wraps a reference to [`EfiBootManagerPolicyProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiBootManagerPolicyProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiBootManagerPolicyProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiBootManagerPolicyProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.proto.revision
    }

    /// Connects a device path using platform boot manager policy.
    ///
    /// If `device_path` is `None`, all controllers in the system will be connected.
    ///
    /// # Errors
    ///
    /// Returns an error status if the device path cannot be found or connection fails.
    pub fn connect_device_path(
        &self,
        device_path: Option<&EfiDevicePath>,
        recursive: bool,
    ) -> Result<(), EfiStatus> {
        let dev_ptr = device_path.map_or(core::ptr::null(), core::ptr::from_ref);
        let status = unsafe { (self.proto.connect_device_path)(self.proto, dev_ptr, recursive) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Connects a device class using platform boot manager policy.
    ///
    /// # Errors
    ///
    /// Returns an error status if class connection is unsupported or fails.
    pub fn connect_device_class(&self, class: &EfiGuid) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.connect_device_class)(self.proto, class) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Connects all platform consoles using platform boot policy.
    ///
    /// # Errors
    ///
    /// Returns an error status if console connection fails.
    pub fn connect_consoles(&self) -> Result<(), EfiStatus> {
        self.connect_device_class(&EFI_BOOT_MANAGER_POLICY_CONSOLE_GUID)
    }

    /// Connects platform networking devices using platform boot policy.
    ///
    /// # Errors
    ///
    /// Returns an error status if network connection fails.
    pub fn connect_network(&self) -> Result<(), EfiStatus> {
        self.connect_device_class(&EFI_BOOT_MANAGER_POLICY_NETWORK_GUID)
    }

    /// Connects all drivers and devices using platform boot policy.
    ///
    /// # Errors
    ///
    /// Returns an error status if connecting all devices fails.
    pub fn connect_all(&self) -> Result<(), EfiStatus> {
        self.connect_device_class(&EFI_BOOT_MANAGER_POLICY_CONNECT_ALL_GUID)
    }

    /// Connects discoverable storage controllers and disks using platform boot policy.
    ///
    /// # Errors
    ///
    /// Returns an error status if storage connection fails.
    pub fn connect_storage(&self) -> Result<(), EfiStatus> {
        self.connect_device_class(&EFI_BOOT_MANAGER_POLICY_STORAGE_GUID)
    }
}
