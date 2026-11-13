//! # Android Boot Image Protocol and Client API
//!
//! Provides services to append command-line arguments to the kernel command line
//! and to update or augment the flattened device tree (`DTB`) when booting Android
//! kernel and recovery images (`boot.img`, `vendor_boot.img`, `init_boot.img`).

use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};

/// Android Boot Image Protocol GUID.
pub const ANDROID_BOOTIMG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9859_BB19,
    0x407C,
    0x4F8B,
    [0xBC, 0xE1, 0xF8, 0xDA, 0x65, 0x65, 0xF4, 0xA5],
);

/// Function pointer to append kernel arguments.
pub type AndroidBootImgAppendKernelArgs =
    unsafe extern "efiapi" fn(args: *const u16, size: usize) -> EfiStatus;

/// Function pointer to update or relocate a device tree blob (`DTB`).
pub type AndroidBootImgUpdateDtb = unsafe extern "efiapi" fn(
    orig_dtb_base: EfiPhysicalAddress,
    new_dtb_base: *mut EfiPhysicalAddress,
) -> EfiStatus;

/// Android Boot Image Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AndroidBootImgProtocol {
    /// Append arguments to the kernel command line.
    pub append_args: Option<AndroidBootImgAppendKernelArgs>,
    /// Update or replace device tree blob (`DTB`).
    pub update_dtb: Option<AndroidBootImgUpdateDtb>,
}

/// Safe client wrapper for the Android Boot Image protocol.
#[derive(Debug)]
pub struct AndroidBootImg<'a> {
    protocol: &'a AndroidBootImgProtocol,
}

impl<'a> AndroidBootImg<'a> {
    /// Create a new safe view of the Android Boot Image protocol.
    #[must_use]
    pub fn new(protocol: &'a AndroidBootImgProtocol) -> Self {
        Self { protocol }
    }

    /// Append UTF-16 kernel command-line arguments.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if the protocol hook is not installed, or
    /// the underlying `EfiStatus` reported by firmware.
    pub fn append_kernel_args(&self, args: &[u16]) -> Result<(), EfiStatus> {
        if let Some(append_fn) = self.protocol.append_args {
            let status = unsafe { append_fn(args.as_ptr(), core::mem::size_of_val(args)) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Update device tree blob (`DTB`) address.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if the protocol hook is not installed, or
    /// the underlying `EfiStatus` reported by firmware.
    pub fn update_dtb(
        &self,
        orig_dtb: EfiPhysicalAddress,
    ) -> Result<EfiPhysicalAddress, EfiStatus> {
        if let Some(update_fn) = self.protocol.update_dtb {
            let mut new_dtb = EfiPhysicalAddress::NULL;
            let status = unsafe { update_fn(orig_dtb, &raw mut new_dtb) };
            if status == EfiStatus::SUCCESS {
                Ok(new_dtb)
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
