//! # Graphics validation
//!
//! Pure validation and size helpers.
//!
//! All arithmetic is checked. No firmware
//! state is dereferenced.

use crate::abi::types::UIntN;

/// Returns the checked framebuffer size.
///
/// The size is `stride * height * 4`. Returns
/// `None` on overflow.
#[must_use]
pub fn framebuffer_size(pixels_per_scan_line: u32, vertical_resolution: u32) -> Option<UIntN> {
    let stride = UIntN::try_from(pixels_per_scan_line).ok()?;
    let height = UIntN::try_from(vertical_resolution).ok()?;
    stride.checked_mul(height)?.checked_mul(4)
}

/// Returns the checked byte offset of a pixel.
///
/// The offset is `(y * stride + x) * 4`.
#[must_use]
pub fn pixel_offset(x: UIntN, y: UIntN, pixels_per_scan_line: u32) -> Option<UIntN> {
    let stride = UIntN::try_from(pixels_per_scan_line).ok()?;
    y.checked_mul(stride)?.checked_add(x)?.checked_mul(4)
}

/// Returns the checked BLT buffer size.
///
/// The size is `width * height * 4` for a
/// tightly packed buffer, or `delta * height`
/// when a stride is given. Returns `None` on
/// overflow.
#[must_use]
pub fn blt_buffer_size(width: UIntN, height: UIntN, full_stride: UIntN) -> Option<UIntN> {
    if full_stride == 0 {
        width.checked_mul(height)?.checked_mul(4)
    } else {
        full_stride.checked_mul(height)
    }
}

/// Returns whether a rectangle fits in the
/// display.
///
/// The check uses checked addition.
#[must_use]
pub fn rectangle_in_display(
    x: UIntN,
    y: UIntN,
    width: UIntN,
    height: UIntN,
    horizontal_resolution: u32,
    vertical_resolution: u32,
) -> bool {
    let Some(max_x) = x.checked_add(width) else {
        return false;
    };
    let Some(max_y) = y.checked_add(height) else {
        return false;
    };
    let Ok(end_x) = u32::try_from(max_x) else {
        return false;
    };
    let Ok(end_y) = u32::try_from(max_y) else {
        return false;
    };
    end_x <= horizontal_resolution && end_y <= vertical_resolution
}

/// Returns whether a mode index is valid.
#[must_use]
pub fn validate_mode_index(mode: u32, max_mode: u32) -> bool {
    max_mode != 0 && mode < max_mode
}
