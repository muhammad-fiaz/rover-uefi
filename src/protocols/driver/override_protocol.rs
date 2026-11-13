//! # Driver Override Protocols
//!
//! Defined by the UEFI Specification:
//! - Section 11.4: Platform Driver Override Protocol
//! - Section 11.5: Bus Specific Driver Override Protocol
//! - Section 11.6: Driver Family Override Protocol
//!
//! Allows platforms, bus drivers, and driver families to supply prioritized driver
//! image handles and version rankings for controller binding.

use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;

/// The `EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID`.
pub const EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6B30_C738,
    0xA391,
    0x401E,
    [0x92, 0x60, 0x8A, 0x76, 0x79, 0x90, 0x67, 0xEB],
);

/// The `EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID`.
pub const EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3BC1_B285,
    0x8A15,
    0x4A82,
    [0xAA, 0xBF, 0x4D, 0x7D, 0x13, 0xFB, 0x32, 0x65],
);

/// The `EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL_GUID`.
pub const EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB1EE_129E,
    0xDA36,
    0x4181,
    [0x91, 0xF8, 0x04, 0xA4, 0x92, 0x37, 0x66, 0xA7],
);

/// Function pointer for `EFI_PLATFORM_DRIVER_OVERRIDE_GET_DRIVER`.
pub type EfiPlatformDriverOverrideGetDriver = unsafe extern "efiapi" fn(
    this: *const EfiPlatformDriverOverrideProtocol,
    controller_handle: EfiHandle,
    driver_image_handle: *mut EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_PLATFORM_DRIVER_OVERRIDE_GET_DRIVER_PATH`.
pub type EfiPlatformDriverOverrideGetDriverPath = unsafe extern "efiapi" fn(
    this: *const EfiPlatformDriverOverrideProtocol,
    controller_handle: EfiHandle,
    driver_image_path: *mut *mut EfiDevicePath,
) -> EfiStatus;

/// Function pointer for `EFI_PLATFORM_DRIVER_OVERRIDE_DRIVER_LOADED`.
pub type EfiPlatformDriverOverrideDriverLoaded = unsafe extern "efiapi" fn(
    this: *const EfiPlatformDriverOverrideProtocol,
    controller_handle: EfiHandle,
    driver_image_path: *const EfiDevicePath,
    driver_image_handle: EfiHandle,
) -> EfiStatus;

/// The raw ABI structure for `EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPlatformDriverOverrideProtocol {
    /// Retrieves next override driver image handle.
    pub get_driver: EfiPlatformDriverOverrideGetDriver,
    /// Retrieves next override driver device path.
    pub get_driver_path: EfiPlatformDriverOverrideGetDriverPath,
    /// Notifies that an override driver was successfully loaded.
    pub driver_loaded: EfiPlatformDriverOverrideDriverLoaded,
}

/// Function pointer for `EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_GET_DRIVER`.
pub type EfiBusSpecificDriverOverrideGetDriver = unsafe extern "efiapi" fn(
    this: *const EfiBusSpecificDriverOverrideProtocol,
    driver_image_handle: *mut EfiHandle,
) -> EfiStatus;

/// The raw ABI structure for `EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBusSpecificDriverOverrideProtocol {
    /// Retrieves next bus-specific driver override image handle.
    pub get_driver: EfiBusSpecificDriverOverrideGetDriver,
}

/// Function pointer for `EFI_DRIVER_FAMILY_OVERRIDE_GET_VERSION`.
pub type EfiDriverFamilyOverrideGetVersion =
    unsafe extern "efiapi" fn(this: *const EfiDriverFamilyOverrideProtocol) -> u32;

/// Interface structure for Driver Family Override Protocol (`EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL`).
#[derive(Debug)]
#[repr(C)]
pub struct EfiDriverFamilyOverrideProtocol {
    /// Retrieves the version value of this driver in its driver family.
    pub get_version: EfiDriverFamilyOverrideGetVersion,
}

// ---------------------------------------------------------------------------
// Safe Native Rust Client Wrappers
// ---------------------------------------------------------------------------

/// Safe wrapper around [`EfiPlatformDriverOverrideProtocol`].
pub struct PlatformDriverOverride<'a> {
    proto: &'a EfiPlatformDriverOverrideProtocol,
}

impl<'a> PlatformDriverOverride<'a> {
    /// Wraps a reference to [`EfiPlatformDriverOverrideProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiPlatformDriverOverrideProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiPlatformDriverOverrideProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiPlatformDriverOverrideProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Retrieves an image handle for an override driver for `controller_handle`.
    ///
    /// Returns `Ok(Some(driver_handle))` if a driver image handle is available,
    /// `Ok(None)` if no further override driver handles exist (`NOT_FOUND`),
    /// or `Err(status)` on error.
    ///
    /// # Errors
    ///
    /// Returns an error status if retrieving driver handle fails.
    pub fn get_driver(&self, controller_handle: EfiHandle) -> Result<Option<EfiHandle>, EfiStatus> {
        let mut driver_image_handle = EfiHandle::from_ptr(core::ptr::null_mut());
        let status = unsafe {
            (self.proto.get_driver)(
                self.proto,
                controller_handle,
                core::ptr::from_mut(&mut driver_image_handle),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(Some(driver_image_handle))
        } else if status == EfiStatus::NOT_FOUND {
            Ok(None)
        } else {
            Err(status)
        }
    }

    /// Retrieves an image device path for an override driver for `controller_handle`.
    ///
    /// # Errors
    ///
    /// Returns an error status if retrieving driver path fails.
    pub fn get_driver_path(
        &self,
        controller_handle: EfiHandle,
    ) -> Result<Option<*mut EfiDevicePath>, EfiStatus> {
        let mut driver_image_path = core::ptr::null_mut();
        let status = unsafe {
            (self.proto.get_driver_path)(
                self.proto,
                controller_handle,
                core::ptr::from_mut(&mut driver_image_path),
            )
        };
        if status == EfiStatus::SUCCESS {
            if driver_image_path.is_null() {
                Ok(None)
            } else {
                Ok(Some(driver_image_path))
            }
        } else if status == EfiStatus::NOT_FOUND {
            Ok(None)
        } else {
            Err(status)
        }
    }

    /// Informs the platform driver override provider that `driver_image_handle` was loaded.
    ///
    /// # Safety
    ///
    /// If non-null, `driver_image_path` must point to a valid device path.
    ///
    /// # Errors
    ///
    /// Returns an error status if notification fails.
    pub unsafe fn driver_loaded(
        &self,
        controller_handle: EfiHandle,
        driver_image_path: *const EfiDevicePath,
        driver_image_handle: EfiHandle,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.driver_loaded)(
                self.proto,
                controller_handle,
                driver_image_path,
                driver_image_handle,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper around [`EfiBusSpecificDriverOverrideProtocol`].
pub struct BusSpecificDriverOverride<'a> {
    proto: &'a EfiBusSpecificDriverOverrideProtocol,
}

impl<'a> BusSpecificDriverOverride<'a> {
    /// Wraps a reference to [`EfiBusSpecificDriverOverrideProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiBusSpecificDriverOverrideProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiBusSpecificDriverOverrideProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiBusSpecificDriverOverrideProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Retrieves an image handle for a bus-specific override driver.
    ///
    /// Returns `Ok(Some(driver_handle))` if a driver image handle is available,
    /// `Ok(None)` if no further override driver handles exist (`NOT_FOUND`),
    /// or `Err(status)` on error.
    ///
    /// # Errors
    ///
    /// Returns an error status if retrieving driver handle fails.
    pub fn get_driver(&self) -> Result<Option<EfiHandle>, EfiStatus> {
        let mut driver_image_handle = EfiHandle::from_ptr(core::ptr::null_mut());
        let status = unsafe {
            (self.proto.get_driver)(self.proto, core::ptr::from_mut(&mut driver_image_handle))
        };
        if status == EfiStatus::SUCCESS {
            Ok(Some(driver_image_handle))
        } else if status == EfiStatus::NOT_FOUND {
            Ok(None)
        } else {
            Err(status)
        }
    }
}

/// Safe wrapper around [`EfiDriverFamilyOverrideProtocol`].
pub struct DriverFamilyOverride<'a> {
    proto: &'a EfiDriverFamilyOverrideProtocol,
}

impl<'a> DriverFamilyOverride<'a> {
    /// Wraps a reference to [`EfiDriverFamilyOverrideProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiDriverFamilyOverrideProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiDriverFamilyOverrideProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiDriverFamilyOverrideProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Retrieves the version value of this driver in its driver family.
    #[must_use]
    pub fn version(&self) -> u32 {
        unsafe { (self.proto.get_version)(self.proto) }
    }
}
