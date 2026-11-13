//! # Android Fastboot Protocols and Client API
//!
//! Provides platform hooks and transport abstractions for the Android Fastboot
//! protocol, facilitating partition flashing, erasing, OEM command processing,
//! and variable retrieval over `USB` or network endpoints.

use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Android Fastboot Platform Protocol GUID.
pub const ANDROID_FASTBOOT_PLATFORM_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5246_85A0,
    0x89A0,
    0x11E3,
    [0x9D, 0x4D, 0xBF, 0xA9, 0xF6, 0xA4, 0x03, 0x08],
);

/// Android Fastboot Transport Protocol GUID.
pub const ANDROID_FASTBOOT_TRANSPORT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x74BD_9FE0,
    0x8902,
    0x11E3,
    [0xB9, 0xD3, 0xF7, 0x22, 0x38, 0xFC, 0x9A, 0x31],
);

/// Initialise platform resources for fastboot.
pub type FastbootPlatformInit = unsafe extern "efiapi" fn() -> EfiStatus;

/// Uninitialise platform resources when exiting fastboot.
pub type FastbootPlatformUnInit = unsafe extern "efiapi" fn();

/// Flash a named partition with image data.
pub type FastbootPlatformFlash = unsafe extern "efiapi" fn(
    partition_name: *const u8,
    buffer_size: usize,
    buffer: *const c_void,
) -> EfiStatus;

/// Erase a named partition.
pub type FastbootPlatformErase = unsafe extern "efiapi" fn(partition_name: *const u8) -> EfiStatus;

/// Retrieve a fastboot variable string.
pub type FastbootPlatformGetVar =
    unsafe extern "efiapi" fn(name: *const u8, value: *mut u8) -> EfiStatus;

/// Execute an OEM-specific fastboot command.
pub type FastbootPlatformDoOemCommand = unsafe extern "efiapi" fn(command: *const u8) -> EfiStatus;

/// Android Fastboot Platform Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FastbootPlatformProtocol {
    /// Initialise platform for fastboot mode.
    pub init: Option<FastbootPlatformInit>,
    /// Uninitialise platform resources.
    pub un_init: Option<FastbootPlatformUnInit>,
    /// Flash a designated partition.
    pub flash_partition: Option<FastbootPlatformFlash>,
    /// Erase a designated partition.
    pub erase_partition: Option<FastbootPlatformErase>,
    /// Get variable value.
    pub get_var: Option<FastbootPlatformGetVar>,
    /// Execute OEM command.
    pub do_oem_command: Option<FastbootPlatformDoOemCommand>,
}

/// Send data over the fastboot transport.
pub type FastbootTransportSend = unsafe extern "efiapi" fn(
    buffer_size: usize,
    buffer: *const c_void,
    callback_event: EfiEvent,
) -> EfiStatus;

/// Receive data from the fastboot transport.
pub type FastbootTransportReceive = unsafe extern "efiapi" fn(
    buffer_size: usize,
    buffer: *mut c_void,
    callback_event: EfiEvent,
) -> EfiStatus;

/// Android Fastboot Transport Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FastbootTransportProtocol {
    /// Transmit data over transport channel.
    pub send: Option<FastbootTransportSend>,
    /// Receive incoming data from transport channel.
    pub receive: Option<FastbootTransportReceive>,
}

/// Safe client wrapper for Android Fastboot platform operations.
#[derive(Debug)]
pub struct FastbootPlatform<'a> {
    protocol: &'a FastbootPlatformProtocol,
}

impl<'a> FastbootPlatform<'a> {
    /// Create a new safe view of the Fastboot Platform protocol.
    #[must_use]
    pub fn new(protocol: &'a FastbootPlatformProtocol) -> Self {
        Self { protocol }
    }

    /// Initialize fastboot platform resources.
    ///
    /// # Errors
    /// Returns an error if the platform initialization hook fails.
    pub fn init(&self) -> Result<(), EfiStatus> {
        if let Some(init_fn) = self.protocol.init {
            let status = unsafe { init_fn() };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Ok(())
        }
    }

    /// Flash an image buffer to the named partition.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn flash_partition(&self, name: &[u8], buffer: &[u8]) -> Result<(), EfiStatus> {
        if let Some(flash_fn) = self.protocol.flash_partition {
            let status = unsafe {
                flash_fn(
                    name.as_ptr(),
                    buffer.len(),
                    buffer.as_ptr().cast::<c_void>(),
                )
            };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Erase the named partition.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn erase_partition(&self, name: &[u8]) -> Result<(), EfiStatus> {
        if let Some(erase_fn) = self.protocol.erase_partition {
            let status = unsafe { erase_fn(name.as_ptr()) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
