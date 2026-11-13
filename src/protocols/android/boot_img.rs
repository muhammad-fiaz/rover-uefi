//! # Android Boot Image Protocol and Client API
//!
//! Provides services to append command-line arguments to the kernel command line
//! and to update or augment the flattened device tree (`DTB`) when booting Android
//! kernel and recovery images (`boot.img`, `vendor_boot.img`, `init_boot.img`),
//! as well as data structures for parsing and validating Android boot image headers.

use crate::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};
use core::mem::size_of;

/// Android Boot Image Protocol GUID.
pub const ANDROID_BOOTIMG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9859_BB19,
    0x407C,
    0x4F8B,
    [0xBC, 0xE1, 0xF8, 0xDA, 0x65, 0x65, 0xF4, 0xA5],
);

/// Magic byte sequence for Android `boot.img` headers (`"ANDROID!"`).
pub const ANDROID_BOOT_MAGIC: &[u8; 8] = b"ANDROID!";

/// Magic byte sequence for Android `vendor_boot.img` headers (`"VNDRBOOT"`).
pub const VENDOR_BOOT_MAGIC: &[u8; 8] = b"VNDRBOOT";

/// Size of kernel command line in Android boot image header (512 bytes).
pub const ANDROID_BOOTIMG_KERNEL_ARGS_SIZE: usize = 512;

/// Size of extra command line arguments in Android boot image header v1+ (1024 bytes).
pub const ANDROID_BOOTIMG_EXTRA_ARGS_SIZE: usize = 1024;

/// Size of kernel command line in Android vendor boot image header (2048 bytes).
pub const ANDROID_VENDOR_BOOT_ARGS_SIZE: usize = 2048;

/// Standard Android `boot.img` header (versions v0, v1, v2, v3, v4).
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AndroidBootImgHeader {
    /// Magic string (`ANDROID_BOOT_MAGIC`, `"ANDROID!"`).
    pub magic: [u8; 8],
    /// Size of kernel binary in bytes.
    pub kernel_size: u32,
    /// Physical load address for kernel.
    pub kernel_addr: u32,
    /// Size of initial RAM disk in bytes.
    pub ramdisk_size: u32,
    /// Physical load address for initial RAM disk.
    pub ramdisk_addr: u32,
    /// Size of second-stage bootloader in bytes.
    pub second_size: u32,
    /// Physical load address for second-stage bootloader.
    pub second_addr: u32,
    /// Physical address for kernel tags / ATAGS.
    pub tags_addr: u32,
    /// Page size for image layout (typically 2048 or 4096).
    pub page_size: u32,
    /// Header version (0, 1, 2, 3, or 4).
    pub header_version: u32,
    /// OS version and security patch level bitfield.
    pub os_version: u32,
    /// Product name string.
    pub name: [u8; 16],
    /// Kernel command line string.
    pub cmdline: [u8; ANDROID_BOOTIMG_KERNEL_ARGS_SIZE],
    /// SHA-1 / SHA-256 digest ID of image components.
    pub id: [u32; 8],
    /// Extra command line arguments (v1+).
    pub extra_cmdline: [u8; ANDROID_BOOTIMG_EXTRA_ARGS_SIZE],
    /// Size of recovery DTBO image in bytes (v1+).
    pub recovery_dtbo_size: u32,
    /// Physical offset of recovery DTBO image in image file (v1+).
    pub recovery_dtbo_offset: u64,
    /// Size of this header in bytes (v1+).
    pub header_size: u32,
    /// Size of device tree blob (`DTB`) in bytes (v2+).
    pub dtb_size: u32,
    /// Physical load address for `DTB` (v2+).
    pub dtb_addr: u64,
}

impl AndroidBootImgHeader {
    /// Validate that the magic matches `ANDROID!`.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        &self.magic == ANDROID_BOOT_MAGIC
    }

    /// Check if the page size is valid (power of two, non-zero, and at least 2048).
    #[must_use]
    pub fn is_valid_page_size(&self) -> bool {
        let ps = self.page_size;
        ps >= 2048 && ps.is_power_of_two()
    }

    /// Extract the OS version as `(major, minor, patch)`.
    #[must_use]
    pub fn unpack_os_version(&self) -> (u32, u32, u32) {
        let os = self.os_version >> 11;
        let major = (os >> 14) & 0x7F;
        let minor = (os >> 7) & 0x7F;
        let patch = os & 0x7F;
        (major, minor, patch)
    }

    /// Extract the OS security patch level as `(year, month)`.
    #[must_use]
    pub fn unpack_patch_level(&self) -> (u32, u32) {
        let os = self.os_version;
        let year = ((os >> 4) & 0x7F) + 2000;
        let month = os & 0x0F;
        (year, month)
    }

    /// Calculates the total image size including page alignment padding.
    #[must_use]
    pub fn calculate_total_size(&self) -> usize {
        let ps = self.page_size as usize;
        if ps == 0 {
            return 0;
        }

        let align_to_page = |val: usize| -> usize { (val + ps - 1) & !(ps - 1) };

        let mut total = align_to_page(size_of::<Self>());
        total = total.saturating_add(align_to_page(self.kernel_size as usize));
        total = total.saturating_add(align_to_page(self.ramdisk_size as usize));
        total = total.saturating_add(align_to_page(self.second_size as usize));

        if self.header_version >= 1 {
            total = total.saturating_add(align_to_page(self.recovery_dtbo_size as usize));
        }
        if self.header_version >= 2 {
            total = total.saturating_add(align_to_page(self.dtb_size as usize));
        }

        total
    }
}

/// Android `vendor_boot.img` header (version v3/v4).
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AndroidVendorBootHeader {
    /// Magic string (`VENDOR_BOOT_MAGIC`, `"VNDRBOOT"`).
    pub magic: [u8; 8],
    /// Header version (3 or 4).
    pub header_version: u32,
    /// Page size in bytes.
    pub page_size: u32,
    /// Physical load address for kernel.
    pub kernel_addr: u32,
    /// Physical load address for ramdisk.
    pub ramdisk_addr: u32,
    /// Size of vendor RAM disk in bytes.
    pub vendor_ramdisk_size: u32,
    /// Vendor boot command line string.
    pub cmdline: [u8; ANDROID_VENDOR_BOOT_ARGS_SIZE],
    /// Physical load address for tags.
    pub tags_addr: u32,
    /// Product / board name string.
    pub name: [u8; 16],
    /// Size of this header structure in bytes.
    pub header_size: u32,
    /// Size of device tree blob (`DTB`) in bytes.
    pub dtb_size: u32,
    /// Physical load address for `DTB`.
    pub dtb_addr: u64,
}

impl AndroidVendorBootHeader {
    /// Validate that the magic matches `VNDRBOOT`.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        &self.magic == VENDOR_BOOT_MAGIC
    }

    /// Check if the page size is valid.
    #[must_use]
    pub fn is_valid_page_size(&self) -> bool {
        let ps = self.page_size;
        ps >= 2048 && ps.is_power_of_two()
    }

    /// Calculate total vendor boot image size with page alignment.
    #[must_use]
    pub fn calculate_total_size(&self) -> usize {
        let ps = self.page_size as usize;
        if ps == 0 {
            return 0;
        }

        let align_to_page = |val: usize| -> usize { (val + ps - 1) & !(ps - 1) };

        let mut total = align_to_page(size_of::<Self>());
        total = total.saturating_add(align_to_page(self.vendor_ramdisk_size as usize));
        total = total.saturating_add(align_to_page(self.dtb_size as usize));
        total
    }
}

/// Parses an Android `boot.img` header from a raw buffer.
#[must_use]
pub fn parse_boot_image(buf: &[u8]) -> Option<&AndroidBootImgHeader> {
    if buf.len() < size_of::<AndroidBootImgHeader>() {
        return None;
    }
    #[allow(clippy::cast_ptr_alignment)]
    let header = unsafe { &*(buf.as_ptr().cast::<AndroidBootImgHeader>()) };
    if header.is_valid() {
        Some(header)
    } else {
        None
    }
}

/// Parses an Android `vendor_boot.img` header from a raw buffer.
#[must_use]
pub fn parse_vendor_boot_image(buf: &[u8]) -> Option<&AndroidVendorBootHeader> {
    if buf.len() < size_of::<AndroidVendorBootHeader>() {
        return None;
    }
    #[allow(clippy::cast_ptr_alignment)]
    let header = unsafe { &*(buf.as_ptr().cast::<AndroidVendorBootHeader>()) };
    if header.is_valid() {
        Some(header)
    } else {
        None
    }
}

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
