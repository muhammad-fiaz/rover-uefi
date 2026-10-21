//! # Framebuffer metadata
//!
//! Typed framebuffer information.
//!
//! The framebuffer base is a physical address,
//! not a Rust pointer. Callers must map the
//! physical memory through the operating
//! system before use. The metadata may become
//! invalid when the mode changes.

use crate::abi::types::UIntN;
use crate::base::EfiPhysicalAddress;
use crate::protocols::graphics::mode::{
    EfiGraphicsOutputModeInformation, EfiGraphicsOutputProtocolMode,
};
use crate::protocols::graphics::pixel::{EfiPixelBitmask, PixelFormat};
use crate::protocols::graphics::validation::framebuffer_size;

/// Framebuffer information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FramebufferInfo {
    /// The physical framebuffer base.
    pub base: EfiPhysicalAddress,
    /// The framebuffer size in bytes.
    pub size: UIntN,
    /// The horizontal resolution.
    pub horizontal_resolution: u32,
    /// The vertical resolution.
    pub vertical_resolution: u32,
    /// Pixels per scan line.
    pub pixels_per_scan_line: u32,
    /// The pixel format.
    pub pixel_format: PixelFormat,
    /// The pixel bitmask.
    pub pixel_information: EfiPixelBitmask,
}

/// Builds framebuffer information from mode
/// data.
///
/// Returns `None` when the size calculation
/// overflows or the format is BLT-only.
#[must_use]
pub fn framebuffer_from_mode(
    mode: &EfiGraphicsOutputProtocolMode,
    info: &EfiGraphicsOutputModeInformation,
) -> Option<FramebufferInfo> {
    if info.pixel_format == PixelFormat::BLT_ONLY {
        return None;
    }
    let size = framebuffer_size(info.pixels_per_scan_line, info.vertical_resolution)?;
    Some(FramebufferInfo {
        base: mode.framebuffer_base,
        size,
        horizontal_resolution: info.horizontal_resolution,
        vertical_resolution: info.vertical_resolution,
        pixels_per_scan_line: info.pixels_per_scan_line,
        pixel_format: info.pixel_format,
        pixel_information: info.pixel_information,
    })
}
