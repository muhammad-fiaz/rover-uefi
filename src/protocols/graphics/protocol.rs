//! # Graphics output protocol
//!
//! The `EFI_GRAPHICS_OUTPUT_PROTOCOL`.
//!
//! The wrapper borrows the firmware protocol
//! instance. Query mode returns a
//! firmware-allocated buffer that must be freed
//! with boot services free pool. The safe query
//! method copies the information and frees the
//! buffer before returning.

use crate::abi::types::UIntN;
use crate::base::EfiStatus;
use crate::protocols::graphics::blt::{BltOperation, BltRectangle, BltRequest};
use crate::protocols::graphics::framebuffer::{FramebufferInfo, framebuffer_from_mode};
use crate::protocols::graphics::mode::{
    EfiGraphicsOutputModeInformation, EfiGraphicsOutputProtocolMode,
};
use crate::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;
use crate::services::boot::BootServices;

/// Queries mode information.
pub type EfiGraphicsOutputProtocolQueryMode = unsafe extern "efiapi" fn(
    *mut EfiGraphicsOutputProtocol,
    u32,
    *mut UIntN,
    *mut *mut EfiGraphicsOutputModeInformation,
) -> EfiStatus;

/// Sets the mode.
pub type EfiGraphicsOutputProtocolSetMode =
    unsafe extern "efiapi" fn(*mut EfiGraphicsOutputProtocol, u32) -> EfiStatus;

/// Performs a BLT operation.
pub type EfiGraphicsOutputProtocolBlt = unsafe extern "efiapi" fn(
    *mut EfiGraphicsOutputProtocol,
    *mut EfiGraphicsOutputBltPixel,
    BltOperation,
    UIntN,
    UIntN,
    UIntN,
    UIntN,
    UIntN,
    UIntN,
    UIntN,
) -> EfiStatus;

/// The graphics output protocol.
///
/// The struct is `#[repr(C)]` so field order
/// and offsets match the specification.
#[derive(Debug)]
#[repr(C)]
pub struct EfiGraphicsOutputProtocol {
    /// Queries mode information.
    pub query_mode: EfiGraphicsOutputProtocolQueryMode,
    /// Sets the mode.
    pub set_mode: EfiGraphicsOutputProtocolSetMode,
    /// Performs a BLT operation.
    pub blt: EfiGraphicsOutputProtocolBlt,
    /// The mode data pointer.
    pub mode: *mut EfiGraphicsOutputProtocolMode,
}

const _: () = {
    assert!(core::mem::size_of::<EfiGraphicsOutputProtocol>() == 4 * core::mem::size_of::<usize>());
    assert!(
        core::mem::offset_of!(EfiGraphicsOutputProtocol, mode) == 3 * core::mem::size_of::<usize>()
    );
};

/// A typed graphics output view.
pub struct GraphicsOutput<'a> {
    protocol: &'a EfiGraphicsOutputProtocol,
}

impl<'a> GraphicsOutput<'a> {
    /// Creates a view from a protocol reference.
    #[must_use]
    pub const fn new(protocol: &'a EfiGraphicsOutputProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol reference.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiGraphicsOutputProtocol {
        self.protocol
    }

    /// Queries mode information and frees the
    /// firmware buffer.
    ///
    /// The buffer is copied before it is freed
    /// with boot services free pool, so no
    /// allocation leaks.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// mode cannot be queried.
    pub fn query_mode(
        &self,
        boot: &BootServices<'_>,
        mode_number: u32,
    ) -> Result<EfiGraphicsOutputModeInformation, EfiStatus> {
        let mut size_of_info: UIntN = 0;
        let mut info: *mut EfiGraphicsOutputModeInformation = core::ptr::null_mut();
        // SAFETY: Stack locals are valid for out parameters.
        let status = unsafe {
            (self.protocol.query_mode)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                mode_number,
                &raw mut size_of_info,
                &raw mut info,
            )
        };
        if !status.is_success() {
            return Err(status);
        }
        if info.is_null() {
            return Err(EfiStatus::NOT_FOUND);
        }
        // SAFETY: Firmware guarantees a valid buffer of `size_of_info` bytes after success.
        let copied = unsafe { *info };
        // SAFETY: The buffer was allocated by firmware for QueryMode and must be freed with FreePool.
        let free_status = unsafe { boot.free_pool(info.cast()) };
        if free_status.is_success() || free_status.is_warning() {
            Ok(copied)
        } else {
            Err(free_status)
        }
    }

    /// Sets the mode.
    #[must_use]
    pub fn set_mode(&self, mode_number: u32) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe {
            (self.protocol.set_mode)(core::ptr::from_ref(self.protocol).cast_mut(), mode_number)
        }
    }

    /// Performs a BLT operation.
    ///
    /// # Safety
    ///
    /// The buffer pointer must be valid for
    /// the operation. For video fill the
    /// buffer must hold one pixel. For buffer
    /// operations the buffer must hold the
    /// rectangle. The rectangle coordinates
    /// must be within the display.
    #[must_use]
    pub unsafe fn blt(
        &self,
        buffer: *mut EfiGraphicsOutputBltPixel,
        request: BltRequest,
    ) -> EfiStatus {
        let rect = request.rectangle;
        // SAFETY: The caller guarantees pointer and rectangle validity.
        unsafe {
            (self.protocol.blt)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                buffer,
                request.operation,
                rect.source_x,
                rect.source_y,
                rect.destination_x,
                rect.destination_y,
                rect.width,
                rect.height,
                rect.delta,
            )
        }
    }

    /// Fills a video rectangle with one pixel.
    #[must_use]
    pub fn blt_fill(&self, pixel: EfiGraphicsOutputBltPixel, rect: BltRectangle) -> EfiStatus {
        let mut pixel_copy = pixel;
        // SAFETY: The stack local holds one pixel for the call.
        unsafe {
            (self.protocol.blt)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut pixel_copy,
                BltOperation::VIDEO_FILL,
                rect.source_x,
                rect.source_y,
                rect.destination_x,
                rect.destination_y,
                rect.width,
                rect.height,
                rect.delta,
            )
        }
    }

    /// Copies video to a BLT buffer.
    #[must_use]
    pub fn blt_video_to_buffer(
        &self,
        buffer: &mut [EfiGraphicsOutputBltPixel],
        rect: BltRectangle,
    ) -> EfiStatus {
        // SAFETY: The slice pointer is valid for the call.
        unsafe {
            (self.protocol.blt)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                buffer.as_mut_ptr(),
                BltOperation::VIDEO_TO_BLT_BUFFER,
                rect.source_x,
                rect.source_y,
                rect.destination_x,
                rect.destination_y,
                rect.width,
                rect.height,
                rect.delta,
            )
        }
    }

    /// Copies a BLT buffer to video.
    #[must_use]
    pub fn blt_buffer_to_video(
        &self,
        buffer: &[EfiGraphicsOutputBltPixel],
        rect: BltRectangle,
    ) -> EfiStatus {
        // SAFETY: The slice pointer is valid for the call.
        unsafe {
            (self.protocol.blt)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                buffer.as_ptr().cast_mut(),
                BltOperation::BUFFER_TO_VIDEO,
                rect.source_x,
                rect.source_y,
                rect.destination_x,
                rect.destination_y,
                rect.width,
                rect.height,
                rect.delta,
            )
        }
    }

    /// Copies video to video without a buffer.
    #[must_use]
    pub fn blt_video_to_video(&self, rect: BltRectangle) -> EfiStatus {
        // SAFETY: No buffer is used for this operation.
        unsafe {
            (self.protocol.blt)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                core::ptr::null_mut(),
                BltOperation::VIDEO_TO_VIDEO,
                rect.source_x,
                rect.source_y,
                rect.destination_x,
                rect.destination_y,
                rect.width,
                rect.height,
                rect.delta,
            )
        }
    }

    /// Returns the current mode number.
    #[must_use]
    pub fn current_mode(&self) -> Option<u32> {
        if self.protocol.mode.is_null() {
            return None;
        }
        // SAFETY: Firmware guarantees the mode pointer is valid when non-null.
        let mode = unsafe { (*self.protocol.mode).mode };
        Some(mode)
    }

    /// Returns a copy of the current mode information.
    #[must_use]
    pub fn mode_information(&self) -> Option<EfiGraphicsOutputModeInformation> {
        if self.protocol.mode.is_null() {
            return None;
        }
        // SAFETY: Firmware guarantees the mode data and info pointers are valid when non-null.
        let mode = unsafe { &*self.protocol.mode };
        if mode.info.is_null() {
            return None;
        }
        // SAFETY: The info buffer is firmware-owned and valid for the current mode.
        Some(unsafe { *mode.info })
    }

    /// Returns framebuffer metadata for the
    /// current mode.
    #[must_use]
    pub fn framebuffer_information(&self) -> Option<FramebufferInfo> {
        if self.protocol.mode.is_null() {
            return None;
        }
        // SAFETY: Firmware guarantees the mode data is valid when non-null.
        let mode = unsafe { &*self.protocol.mode };
        if mode.info.is_null() {
            return None;
        }
        // SAFETY: The info buffer is firmware-owned and valid for the current mode.
        let info = unsafe { &*mode.info };
        framebuffer_from_mode(mode, info)
    }
}

impl core::fmt::Debug for GraphicsOutput<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("GraphicsOutput")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// The EDID discovered protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiEdidDiscoveredProtocol {
    /// The EDID size in bytes.
    pub size_of_edid: u32,
    /// The EDID buffer pointer.
    pub edid: *const u8,
}

/// The EDID active protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiEdidActiveProtocol {
    /// The EDID size in bytes.
    pub size_of_edid: u32,
    /// The EDID buffer pointer.
    pub edid: *const u8,
}

const _: () = {
    assert!(
        core::mem::size_of::<EfiEdidDiscoveredProtocol>()
            == if core::mem::size_of::<usize>() == 8 {
                16
            } else {
                8
            }
    );
    assert!(
        core::mem::size_of::<EfiEdidActiveProtocol>()
            == if core::mem::size_of::<usize>() == 8 {
                16
            } else {
                8
            }
    );
};

/// A typed EDID view.
pub struct Edid<'a> {
    size: u32,
    pointer: *const u8,
    _marker: core::marker::PhantomData<&'a ()>,
}

impl Edid<'_> {
    /// Creates an EDID view from raw parts.
    ///
    /// # Safety
    ///
    /// The pointer and size must describe a
    /// valid buffer for the lifetime of the
    /// view.
    #[must_use]
    pub const unsafe fn from_raw_parts(pointer: *const u8, size: u32) -> Self {
        Self {
            size,
            pointer,
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the EDID bytes.
    ///
    /// Returns `None` when the pointer is
    /// null with a non-zero size.
    #[must_use]
    pub fn bytes(&self) -> Option<&[u8]> {
        if self.pointer.is_null() {
            if self.size == 0 {
                return Some(&[]);
            }
            return None;
        }
        let Ok(length) = usize::try_from(self.size) else {
            return None;
        };
        // SAFETY: Firmware guarantees the buffer is valid for `length` bytes.
        Some(unsafe { core::slice::from_raw_parts(self.pointer, length) })
    }

    /// Returns the EDID size.
    #[must_use]
    pub const fn size(&self) -> u32 {
        self.size
    }
}

impl core::fmt::Debug for Edid<'_> {
    /// Formats the view without dereferencing
    /// firmware state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Edid")
            .field("size", &self.size)
            .finish()
    }
}
