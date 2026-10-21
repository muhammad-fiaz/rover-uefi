//! # Display modes
//!
//! Mode information and protocol mode.
//!
//! Both structs are `#[repr(C)]` and match the
//! specification field order.

use crate::abi::types::UIntN;
use crate::base::EfiPhysicalAddress;
use crate::protocols::graphics::pixel::{EfiPixelBitmask, PixelFormat};

/// Mode information.
///
/// The struct is `#[repr(C)]` with size 36.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiGraphicsOutputModeInformation {
    /// The structure version.
    pub version: u32,
    /// The horizontal resolution.
    pub horizontal_resolution: u32,
    /// The vertical resolution.
    pub vertical_resolution: u32,
    /// The pixel format.
    pub pixel_format: PixelFormat,
    /// The pixel bitmask.
    pub pixel_information: EfiPixelBitmask,
    /// Pixels per scan line.
    pub pixels_per_scan_line: u32,
}

/// The protocol mode.
///
/// The struct is `#[repr(C)]`. The info pointer
/// is firmware-owned and becomes invalid when
/// the mode changes.
#[derive(Debug)]
#[repr(C)]
pub struct EfiGraphicsOutputProtocolMode {
    /// The number of supported modes.
    pub max_mode: u32,
    /// The current mode number.
    pub mode: u32,
    /// The current mode information pointer.
    pub info: *mut EfiGraphicsOutputModeInformation,
    /// The size of the info structure.
    pub size_of_info: UIntN,
    /// The framebuffer base address.
    pub framebuffer_base: EfiPhysicalAddress,
    /// The framebuffer size in bytes.
    pub framebuffer_size: UIntN,
}

const _: () = {
    assert!(core::mem::size_of::<EfiGraphicsOutputModeInformation>() == 36);
    assert!(core::mem::offset_of!(EfiGraphicsOutputModeInformation, horizontal_resolution) == 4);
    assert!(core::mem::offset_of!(EfiGraphicsOutputModeInformation, pixel_format) == 12);
    assert!(core::mem::offset_of!(EfiGraphicsOutputModeInformation, pixels_per_scan_line) == 32);
};
