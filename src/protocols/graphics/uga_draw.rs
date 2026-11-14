//! # Universal Graphics Adapter (UGA) Draw Protocol
//!
//! Provides the legacy EFI 1.10 UGA Draw protocol for controlling display modes
//! and performing bit-block transfers (BLT) on UGA-compliant display hardware.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};

/// UGA Draw Protocol GUID: `{982c058b-0073-4183-acbb-69b42c11ce7f}`.
pub const EFI_UGA_DRAW_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x982c_058b,
    0x0073,
    0x4183,
    [0xac, 0xbb, 0x69, 0xb4, 0x2c, 0x11, 0xce, 0x7f],
);

/// UGA pixel color representation in 32-bit BGR format.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EfiUgaPixel {
    /// Blue color intensity (0-255).
    pub blue: u8,
    /// Green color intensity (0-255).
    pub green: u8,
    /// Red color intensity (0-255).
    pub red: u8,
    /// Reserved byte (must be zero).
    pub reserved: u8,
}

impl EfiUgaPixel {
    /// Creates a new pixel with specified RGB channels.
    #[must_use]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            blue,
            green,
            red,
            reserved: 0,
        }
    }
}

/// Bit-block transfer (BLT) operation types for UGA display surfaces.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EfiUgaBltOperation {
    /// Fills destination rectangle on video display with a single color.
    VideoFill = 0,
    /// Reads video display rectangle into a host pixel buffer.
    VideoToBltBuffer = 1,
    /// Writes host pixel buffer rectangle onto video display.
    BufferToVideo = 2,
    /// Copies rectangular area within video display memory.
    VideoToVideo = 3,
    /// Maximum valid BLT operation delimiter.
    MaxBltOperation = 4,
}

/// Mode geometry and timing parameters for a UGA display device.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UgaModeInfo {
    /// Horizontal resolution in pixels.
    pub horizontal_resolution: u32,
    /// Vertical resolution in pixels.
    pub vertical_resolution: u32,
    /// Color depth in bits per pixel.
    pub color_depth: u32,
    /// Refresh rate in Hertz.
    pub refresh_rate: u32,
}

/// Retrieves the current display mode parameters.
pub type EfiUgaDrawGetMode = unsafe extern "efiapi" fn(
    this: *const EfiUgaDrawProtocol,
    horizontal_resolution: *mut u32,
    vertical_resolution: *mut u32,
    color_depth: *mut u32,
    refresh_rate: *mut u32,
) -> EfiStatus;

/// Sets the current display mode parameters.
pub type EfiUgaDrawSetMode = unsafe extern "efiapi" fn(
    this: *const EfiUgaDrawProtocol,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    color_depth: u32,
    refresh_rate: u32,
) -> EfiStatus;

/// Performs a bit-block transfer between memory and video surface.
pub type EfiUgaDrawBlt = unsafe extern "efiapi" fn(
    this: *const EfiUgaDrawProtocol,
    blt_buffer: *mut EfiUgaPixel,
    blt_operation: EfiUgaBltOperation,
    source_x: UIntN,
    source_y: UIntN,
    destination_x: UIntN,
    destination_y: UIntN,
    width: UIntN,
    height: UIntN,
    delta: UIntN,
) -> EfiStatus;

/// Raw interface table for the Universal Graphics Adapter (UGA) Draw Protocol.
#[repr(C)]
pub struct EfiUgaDrawProtocol {
    /// Retrieves current resolution, color depth, and refresh rate.
    pub get_mode: EfiUgaDrawGetMode,
    /// Configures resolution, color depth, and refresh rate.
    pub set_mode: EfiUgaDrawSetMode,
    /// Executes a bit-block raster transfer.
    pub blt: EfiUgaDrawBlt,
}

/// Safe client wrapper around [`EfiUgaDrawProtocol`].
pub struct UgaDraw<'a> {
    proto: &'a EfiUgaDrawProtocol,
}

impl UgaDraw<'_> {
    /// Protocol GUID.
    pub const GUID: EfiGuid = EFI_UGA_DRAW_PROTOCOL_GUID;

    /// Wraps a raw pointer to an `EfiUgaDrawProtocol`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `proto` points to a valid, live `EfiUgaDrawProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiUgaDrawProtocol) -> Self {
        unsafe { Self { proto: &*proto } }
    }

    /// Queries the current display mode information.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if query failed.
    pub fn get_mode(&self) -> Result<UgaModeInfo, EfiStatus> {
        let mut h_res: u32 = 0;
        let mut v_res: u32 = 0;
        let mut color_depth: u32 = 0;
        let mut refresh_rate: u32 = 0;

        let status = unsafe {
            (self.proto.get_mode)(
                self.proto,
                &raw mut h_res,
                &raw mut v_res,
                &raw mut color_depth,
                &raw mut refresh_rate,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(UgaModeInfo {
                horizontal_resolution: h_res,
                vertical_resolution: v_res,
                color_depth,
                refresh_rate,
            })
        } else {
            Err(status)
        }
    }

    /// Configures the display mode.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if the requested mode is unsupported or configuration failed.
    pub fn set_mode(&self, info: &UgaModeInfo) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.set_mode)(
                self.proto,
                info.horizontal_resolution,
                info.vertical_resolution,
                info.color_depth,
                info.refresh_rate,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Performs a `BitBLT` operation using a pixel slice.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if blitting operation failed or coordinates were invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn blt(
        &self,
        blt_buffer: Option<&mut [EfiUgaPixel]>,
        blt_operation: EfiUgaBltOperation,
        source_x: UIntN,
        source_y: UIntN,
        destination_x: UIntN,
        destination_y: UIntN,
        width: UIntN,
        height: UIntN,
        delta: UIntN,
    ) -> Result<(), EfiStatus> {
        let buf_ptr = blt_buffer.map_or(core::ptr::null_mut(), <[EfiUgaPixel]>::as_mut_ptr);

        let status = unsafe {
            (self.proto.blt)(
                self.proto,
                buf_ptr,
                blt_operation,
                source_x,
                source_y,
                destination_x,
                destination_y,
                width,
                height,
                delta,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Performs a `BitBLT` operation using raw pointers.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `blt_buffer` (if not null) points to a valid buffer of
    /// sufficient size for the transfer.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if transfer failed.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn blt_raw(
        &self,
        blt_buffer: *mut EfiUgaPixel,
        blt_operation: EfiUgaBltOperation,
        source_x: UIntN,
        source_y: UIntN,
        destination_x: UIntN,
        destination_y: UIntN,
        width: UIntN,
        height: UIntN,
        delta: UIntN,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.blt)(
                self.proto,
                blt_buffer,
                blt_operation,
                source_x,
                source_y,
                destination_x,
                destination_y,
                width,
                height,
                delta,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
