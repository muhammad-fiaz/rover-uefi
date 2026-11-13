//! # Boot Logo 2 Protocol and Client API
//!
//! Convey information about the graphics logo displayed during boot.
//! If a platform produces both `BootLogo` and `BootLogo2`, `BootLogo2` must be used.

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;

/// Boot Logo 2 Protocol GUID.
pub const BOOT_LOGO2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4B5D_C1DF,
    0x1EAA,
    0x48B2,
    [0xA7, 0xE9, 0xEA, 0xC4, 0x89, 0xA0, 0x0B, 0x5C],
);

/// Function pointer prototype to update logo image drawn on screen.
pub type SetBootLogo2 = unsafe extern "efiapi" fn(
    this: *mut BootLogo2Protocol,
    blt_buffer: *const EfiGraphicsOutputBltPixel,
    destination_x: usize,
    destination_y: usize,
    width: usize,
    height: usize,
) -> EfiStatus;

/// Function pointer prototype to retrieve the position and buffer of the logo.
pub type GetBootLogo2 = unsafe extern "efiapi" fn(
    this: *mut BootLogo2Protocol,
    blt_buffer: *mut *mut EfiGraphicsOutputBltPixel,
    destination_x: *mut usize,
    destination_y: *mut usize,
    width: *mut usize,
    height: *mut usize,
) -> EfiStatus;

/// Boot Logo 2 Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BootLogo2Protocol {
    /// Registers or clears boot logo on screen.
    pub set_boot_logo: Option<SetBootLogo2>,
    /// Retrieves current boot logo registration.
    pub get_boot_logo: Option<GetBootLogo2>,
}

/// Safe client wrapper for the Boot Logo 2 protocol.
#[derive(Debug)]
pub struct BootLogo2<'a> {
    protocol: &'a mut BootLogo2Protocol,
}

impl<'a> BootLogo2<'a> {
    /// Create a new safe view of the Boot Logo 2 protocol.
    #[must_use]
    pub fn new(protocol: &'a mut BootLogo2Protocol) -> Self {
        Self { protocol }
    }

    /// Update or register the boot logo image.
    ///
    /// # Safety
    /// Caller must ensure `blt_buffer` points to valid memory of at least `width * height` pixels.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub unsafe fn set_logo(
        &mut self,
        blt_buffer: *const EfiGraphicsOutputBltPixel,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Result<(), EfiStatus> {
        if let Some(set_fn) = self.protocol.set_boot_logo {
            let status = unsafe { set_fn(self.protocol, blt_buffer, x, y, width, height) };
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
