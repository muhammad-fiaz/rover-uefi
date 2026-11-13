//! # Boot Logo 2 Protocol
//!
//! Convey information about the graphics logo displayed during boot.
//! If a platform produces both `BootLogo` and `BootLogo2`, `BootLogo2` must be used.
//!
//! Defined in EDK II `MdeModulePkg/Include/Protocol/BootLogo2.h`.

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;

/// EDK II Boot Logo 2 Protocol GUID.
pub const EDKII_BOOT_LOGO2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4B5D_C1DF,
    0x1EAA,
    0x48B2,
    [0xA7, 0xE9, 0xEA, 0xC4, 0x89, 0xA0, 0x0B, 0x5C],
);

/// Function pointer prototype to update logo image drawn on screen.
pub type EdkiiSetBootLogo2 = unsafe extern "efiapi" fn(
    this: *mut EdkiiBootLogo2Protocol,
    blt_buffer: *const EfiGraphicsOutputBltPixel,
    destination_x: usize,
    destination_y: usize,
    width: usize,
    height: usize,
) -> EfiStatus;

/// Function pointer prototype to retrieve the position and buffer of the logo.
pub type EdkiiGetBootLogo2 = unsafe extern "efiapi" fn(
    this: *mut EdkiiBootLogo2Protocol,
    blt_buffer: *mut *mut EfiGraphicsOutputBltPixel,
    destination_x: *mut usize,
    destination_y: *mut usize,
    width: *mut usize,
    height: *mut usize,
) -> EfiStatus;

/// EDK II Boot Logo 2 Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EdkiiBootLogo2Protocol {
    /// Registers or clears boot logo on screen.
    pub set_boot_logo: Option<EdkiiSetBootLogo2>,
    /// Retrieves current boot logo registration.
    pub get_boot_logo: Option<EdkiiGetBootLogo2>,
}
