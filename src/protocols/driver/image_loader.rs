//! # Image Loader and Emulator Protocols
//!
//! Provides protocol definitions and client abstractions for PE/COFF image loading,
//! image emulation across architecture boundaries, and debugger configuration.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;
use crate::tables::system::EfiImageEntryPoint;
use core::ffi::c_void;

// ---------------------------------------------------------------------------
// PE/COFF Image Emulator Protocol
// ---------------------------------------------------------------------------

/// `PE/COFF` Image Emulator Protocol GUID: `{96f46153-97a7-4793-acc1-fa19bf78ea97}`.
pub const PECOFF_IMAGE_EMULATOR_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x96F4_6153,
    0x97A7,
    0x4793,
    [0xAC, 0xC1, 0xFA, 0x19, 0xBF, 0x78, 0xEA, 0x97],
);

/// `PE/COFF` Image Emulator protocol revision version.
pub const PECOFF_IMAGE_EMULATOR_VERSION: u32 = 0x1;

/// Checks whether the emulator supports executing a given image.
pub type PeCoffImageEmulatorIsImageSupported = unsafe extern "efiapi" fn(
    this: *mut EfiPeCoffImageEmulatorProtocol,
    image_type: u16,
    device_path: *const EfiDevicePath,
) -> bool;

/// Registers a supported `PE/COFF` image with the emulator.
pub type PeCoffImageEmulatorRegisterImage = unsafe extern "efiapi" fn(
    this: *mut EfiPeCoffImageEmulatorProtocol,
    image_base: EfiPhysicalAddress,
    image_size: u64,
    entry_point: *mut Option<EfiImageEntryPoint>,
) -> EfiStatus;

/// Unregisters a `PE/COFF` image that was previously registered with the emulator.
pub type PeCoffImageEmulatorUnregisterImage = unsafe extern "efiapi" fn(
    this: *mut EfiPeCoffImageEmulatorProtocol,
    image_base: EfiPhysicalAddress,
) -> EfiStatus;

/// `PE/COFF` Image Emulator Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeCoffImageEmulatorProtocol {
    /// Checks whether the emulator supports executing a given image.
    pub is_image_supported: Option<PeCoffImageEmulatorIsImageSupported>,
    /// Registers a supported image with the emulator.
    pub register_image: Option<PeCoffImageEmulatorRegisterImage>,
    /// Unregisters a previously registered image.
    pub unregister_image: Option<PeCoffImageEmulatorUnregisterImage>,
    /// Protocol revision version.
    pub version: u32,
    /// Target machine type architecture supported by the emulator.
    pub machine_type: u16,
}

/// Safe client wrapper around [`EfiPeCoffImageEmulatorProtocol`].
#[derive(Debug)]
pub struct PeCoffImageEmulator<'a> {
    proto: &'a mut EfiPeCoffImageEmulatorProtocol,
}

impl<'a> PeCoffImageEmulator<'a> {
    /// Creates a new safe wrapper around a mutable reference to [`EfiPeCoffImageEmulatorProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiPeCoffImageEmulatorProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiPeCoffImageEmulatorProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *mut EfiPeCoffImageEmulatorProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Checks whether the emulator supports executing an image of the specified type.
    #[must_use]
    pub fn is_image_supported(
        &mut self,
        image_type: u16,
        device_path: Option<&EfiDevicePath>,
    ) -> bool {
        let Some(func) = self.proto.is_image_supported else {
            return false;
        };
        let dp_ptr = device_path.map_or(core::ptr::null(), core::ptr::from_ref);
        unsafe { func(self.proto, image_type, dp_ptr) }
    }

    /// Registers a supported `PE/COFF` image with the emulator.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if the protocol function is missing, or the firmware error code.
    pub fn register_image(
        &mut self,
        image_base: EfiPhysicalAddress,
        image_size: u64,
        entry_point: &mut Option<EfiImageEntryPoint>,
    ) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.register_image else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe {
            func(
                self.proto,
                image_base,
                image_size,
                core::ptr::from_mut(entry_point),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unregisters an image previously registered with the emulator.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if the protocol function is missing, or the firmware error code.
    pub fn unregister_image(&mut self, image_base: EfiPhysicalAddress) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.unregister_image else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { func(self.proto, image_base) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the protocol version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.proto.version
    }

    /// Returns the target machine type supported by the emulator.
    #[must_use]
    pub const fn machine_type(&self) -> u16 {
        self.proto.machine_type
    }
}

// ---------------------------------------------------------------------------
// Load PE32 Image Protocol
// ---------------------------------------------------------------------------

/// `PE32` Image Protocol GUID: `{5cb5c776-60d5-45ee-883c-452708cd743f}`.
pub const PE32_IMAGE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5CB5_C776,
    0x60D5,
    0x45EE,
    [0x88, 0x3C, 0x45, 0x27, 0x08, 0xCD, 0x74, 0x3F],
);

/// Load `PE` image attribute flags.
pub mod load_pe_image_attributes {
    /// No attribute flags.
    pub const NONE: u32 = 0x00;
    /// Register image for runtime access.
    pub const RUNTIME_REGISTRATION: u32 = 0x01;
    /// Register image in debug image info table.
    pub const DEBUG_IMAGE_INFO_TABLE_REGISTRATION: u32 = 0x02;
}

/// Loads a `PE/COFF` image into memory and returns an image handle.
pub type EfiLoadPeImage = unsafe extern "efiapi" fn(
    this: *mut EfiPe32ImageProtocol,
    parent_image_handle: EfiHandle,
    file_path: *const EfiDevicePath,
    source_buffer: *const c_void,
    source_size: UIntN,
    dst_buffer: EfiPhysicalAddress,
    number_of_pages: *mut UIntN,
    image_handle: *mut EfiHandle,
    entry_point: *mut EfiPhysicalAddress,
    attribute: u32,
) -> EfiStatus;

/// Unloads a previously loaded `PE/COFF` image.
pub type EfiUnloadPeImage = unsafe extern "efiapi" fn(
    this: *mut EfiPe32ImageProtocol,
    image_handle: EfiHandle,
) -> EfiStatus;

/// Low-level `PE32` Image Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPe32ImageProtocol {
    /// Loads a `PE/COFF` image into memory.
    pub load_pe_image: Option<EfiLoadPeImage>,
    /// Unloads a loaded `PE/COFF` image.
    pub unload_pe_image: Option<EfiUnloadPeImage>,
}

/// Safe client wrapper around [`EfiPe32ImageProtocol`].
#[derive(Debug)]
pub struct Pe32Image<'a> {
    proto: &'a mut EfiPe32ImageProtocol,
}

impl<'a> Pe32Image<'a> {
    /// Creates a new safe wrapper around a mutable reference to [`EfiPe32ImageProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiPe32ImageProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiPe32ImageProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *mut EfiPe32ImageProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Unloads an image given its handle.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if the protocol function is missing, or the firmware error code.
    pub fn unload_image(&mut self, image_handle: EfiHandle) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.unload_pe_image else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { func(self.proto, image_handle) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// ---------------------------------------------------------------------------
// Debugger Configuration Protocol
// ---------------------------------------------------------------------------

/// Debugger Configuration Protocol GUID: `{577d959c-e967-4546-8620-c778fae5da05}`.
pub const DEBUGGER_CONFIGURATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x577D_959C,
    0xE967,
    0x4546,
    [0x86, 0x20, 0xC7, 0x78, 0xFA, 0xE5, 0xDA, 0x05],
);

/// Debugger configuration revision constant.
pub const DEBUGGER_CONFIGURATION_REVISION: u32 = 0x0000_0001;

/// Low-level Debugger Configuration Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiDebuggerConfigurationProtocol {
    /// Debugger configuration revision number.
    pub revision: u32,
    /// Private data associated with the debugger.
    pub debugger_private_data: *mut c_void,
}

/// Safe client wrapper around [`EfiDebuggerConfigurationProtocol`].
#[derive(Clone, Copy, Debug)]
pub struct DebuggerConfiguration {
    proto: *const EfiDebuggerConfigurationProtocol,
}

impl DebuggerConfiguration {
    /// Wraps a raw pointer to [`EfiDebuggerConfigurationProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiDebuggerConfigurationProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self { proto })
        }
    }

    /// Returns the debugger configuration revision number.
    #[must_use]
    pub fn revision(&self) -> u32 {
        unsafe { (*self.proto).revision }
    }

    /// Returns the debugger private data pointer.
    #[must_use]
    pub fn private_data(&self) -> *mut c_void {
        unsafe { (*self.proto).debugger_private_data }
    }
}
