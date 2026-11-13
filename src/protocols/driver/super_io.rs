//! # Super I/O and Super I/O Control Protocols
//!
//! Provides hardware abstraction, register access, resource assignment, and logical device
//! enablement for legacy motherboard peripheral controllers per UEFI Platform Initialization (PI)
//! Specification 1.2.1.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_SIO_PROTOCOL_GUID`.
pub const EFI_SIO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x215F_DD18,
    0xBD50,
    0x4FEB,
    [0x89, 0x0B, 0x58, 0xCA, 0x0B, 0x47, 0x39, 0xE9],
);

/// The `EFI_SIO_CONTROL_PROTOCOL_GUID`.
pub const EFI_SIO_CONTROL_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB919_78DF,
    0x9FC1,
    0x427D,
    [0xBB, 0x05, 0x4C, 0x82, 0x84, 0x55, 0xCA, 0x27],
);

/// Super I/O register modify entry for table-based programming.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EfiSioRegisterModify {
    /// Super I/O register index.
    pub register: u8,
    /// Bitwise AND mask applied before OR.
    pub and_mask: u8,
    /// Bitwise OR mask applied after AND.
    pub or_mask: u8,
}

/// Pointer container for ACPI Small or Large resource descriptor headers.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AcpiResourceHeaderPtr {
    /// Pointer to small ACPI resource header.
    pub small_header: *mut c_void,
    /// Pointer to large ACPI resource header.
    pub large_header: *mut c_void,
}

impl core::fmt::Debug for AcpiResourceHeaderPtr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AcpiResourceHeaderPtr")
            .field("raw", &unsafe { self.small_header })
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Super I/O Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_SIO_REGISTER_ACCESS`.
pub type EfiSioRegisterAccess = unsafe extern "efiapi" fn(
    this: *const EfiSioProtocol,
    write: bool,
    exit_cfg_mode: bool,
    register: u8,
    value: *mut u8,
) -> EfiStatus;

/// Function pointer for `EFI_SIO_GET_RESOURCES`.
pub type EfiSioGetResources = unsafe extern "efiapi" fn(
    this: *const EfiSioProtocol,
    resource_list: *mut AcpiResourceHeaderPtr,
) -> EfiStatus;

/// Function pointer for `EFI_SIO_SET_RESOURCES`.
pub type EfiSioSetResources = unsafe extern "efiapi" fn(
    this: *const EfiSioProtocol,
    resource_list: AcpiResourceHeaderPtr,
) -> EfiStatus;

/// Function pointer for `EFI_SIO_POSSIBLE_RESOURCES`.
pub type EfiSioPossibleResources = unsafe extern "efiapi" fn(
    this: *const EfiSioProtocol,
    resource_collection: *mut AcpiResourceHeaderPtr,
) -> EfiStatus;

/// Function pointer for `EFI_SIO_MODIFY`.
pub type EfiSioModify = unsafe extern "efiapi" fn(
    this: *const EfiSioProtocol,
    command: *const EfiSioRegisterModify,
    number_of_commands: UIntN,
) -> EfiStatus;

/// Interface structure for Super I/O Protocol (`EFI_SIO_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiSioProtocol {
    /// Provides low-level register read/write access.
    pub register_access: EfiSioRegisterAccess,
    /// Returns ACPI resource descriptors currently consumed by the device.
    pub get_resources: EfiSioGetResources,
    /// Sets resource allocation for the device.
    pub set_resources: EfiSioSetResources,
    /// Returns possible resource combinations for the device.
    pub possible_resources: EfiSioPossibleResources,
    /// Performs table-based atomic register modifications.
    pub modify: EfiSioModify,
}

/// Safe wrapper around [`EfiSioProtocol`].
pub struct SuperIo<'a> {
    proto: &'a EfiSioProtocol,
}

impl<'a> SuperIo<'a> {
    /// Wraps a reference to [`EfiSioProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiSioProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiSioProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiSioProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Reads a single Super I/O register byte.
    ///
    /// # Errors
    ///
    /// Returns an error status if register access fails.
    pub fn read_register(&self, register: u8, exit_cfg_mode: bool) -> Result<u8, EfiStatus> {
        let mut value = 0u8;
        let status = unsafe {
            (self.proto.register_access)(
                self.proto,
                false,
                exit_cfg_mode,
                register,
                core::ptr::from_mut(&mut value),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(value)
        } else {
            Err(status)
        }
    }

    /// Writes a single Super I/O register byte.
    ///
    /// # Errors
    ///
    /// Returns an error status if register access fails.
    pub fn write_register(
        &self,
        register: u8,
        mut value: u8,
        exit_cfg_mode: bool,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.register_access)(
                self.proto,
                true,
                exit_cfg_mode,
                register,
                core::ptr::from_mut(&mut value),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves current ACPI resource descriptors allocated for this device.
    ///
    /// # Errors
    ///
    /// Returns an error status if retrieving resources fails.
    pub fn get_resources(&self) -> Result<AcpiResourceHeaderPtr, EfiStatus> {
        let mut res = AcpiResourceHeaderPtr {
            small_header: core::ptr::null_mut(),
        };
        let status =
            unsafe { (self.proto.get_resources)(self.proto, core::ptr::from_mut(&mut res)) };
        if status == EfiStatus::SUCCESS {
            Ok(res)
        } else {
            Err(status)
        }
    }

    /// Configures active resources for this device.
    ///
    /// # Errors
    ///
    /// Returns an error status if setting resources fails.
    pub fn set_resources(&self, resources: AcpiResourceHeaderPtr) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set_resources)(self.proto, resources) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves possible resource descriptor collections for this device.
    ///
    /// # Errors
    ///
    /// Returns an error status if querying possible resources fails.
    pub fn possible_resources(&self) -> Result<AcpiResourceHeaderPtr, EfiStatus> {
        let mut res = AcpiResourceHeaderPtr {
            small_header: core::ptr::null_mut(),
        };
        let status =
            unsafe { (self.proto.possible_resources)(self.proto, core::ptr::from_mut(&mut res)) };
        if status == EfiStatus::SUCCESS {
            Ok(res)
        } else {
            Err(status)
        }
    }

    /// Performs table-based batch modifications of Super I/O registers.
    ///
    /// # Errors
    ///
    /// Returns an error status if register modification fails.
    pub fn modify(&self, commands: &[EfiSioRegisterModify]) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.modify)(self.proto, commands.as_ptr(), commands.len()) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// ---------------------------------------------------------------------------
// Super I/O Control Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_SIO_CONTROL_ENABLE`.
pub type EfiSioControlEnable =
    unsafe extern "efiapi" fn(this: *const EfiSioControlProtocol) -> EfiStatus;

/// Function pointer for `EFI_SIO_CONTROL_DISABLE`.
pub type EfiSioControlDisable =
    unsafe extern "efiapi" fn(this: *const EfiSioControlProtocol) -> EfiStatus;

/// Interface structure for Super I/O Control Protocol (`EFI_SIO_CONTROL_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiSioControlProtocol {
    /// Protocol version.
    pub version: u32,
    /// Enables a logical ISA device.
    pub enable_device: EfiSioControlEnable,
    /// Disables a logical ISA device.
    pub disable_device: EfiSioControlDisable,
}

/// Safe wrapper around [`EfiSioControlProtocol`].
pub struct SuperIoControl<'a> {
    proto: &'a EfiSioControlProtocol,
}

impl<'a> SuperIoControl<'a> {
    /// Wraps a reference to [`EfiSioControlProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiSioControlProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiSioControlProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiSioControlProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns the Super I/O control protocol version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.proto.version
    }

    /// Enables the logical ISA device and allocates default resources.
    ///
    /// # Errors
    ///
    /// Returns an error status if device enable fails.
    pub fn enable(&self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.enable_device)(self.proto) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Disables the logical ISA device and releases system resources.
    ///
    /// # Errors
    ///
    /// Returns an error status if device disable fails.
    pub fn disable(&self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.disable_device)(self.proto) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
