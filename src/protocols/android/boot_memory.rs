//! # Android Boot Memory Protocol
//!
//! Provides APIs for reserving, querying, and synchronizing kernel, ramdisk,
//! device tree, and partition image buffers for Android fastboot and bootloader flow.

#![allow(clippy::doc_markdown)]

use crate::abi::types::{Bool, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Protocol GUID for the Android Boot Memory interface.
///
/// `{309f2874-ad59-4fd2-af5e-ce0f4ab401a6}`
pub const GBL_EFI_BOOT_MEMORY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x309f_2874,
    0xad59,
    0x4fd2,
    [0xaf, 0x5e, 0xce, 0x0f, 0x4a, 0xb4, 0x01, 0xa6],
);

/// Initial revision of the Boot Memory protocol.
pub const GBL_EFI_BOOT_MEMORY_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Android Boot Buffer types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GblEfiBootBufferType {
    /// General image load buffer.
    GeneralLoad = 0,
    /// Linux/Android OS kernel buffer.
    Kernel = 1,
    /// Initial RAM disk (`ramdisk.img`).
    Ramdisk = 2,
    /// Flattened Device Tree (`FDT`).
    Fdt = 3,
    /// Protected Virtual Machine Firmware (`pvmfw`).
    PvmfwData = 4,
    /// Fastboot sparse/raw download buffer.
    FastbootDownload = 5,
}

/// Partition buffer attribute flags.
pub mod partition_buffer_flag {
    /// Indicates the partition buffer is already preloaded in memory.
    pub const PRELOADED: u32 = 1 << 0;
}

/// Raw function pointer for querying a partition buffer.
pub type GblEfiBootMemoryGetPartitionBuffer = unsafe extern "efiapi" fn(
    this: *mut GblEfiBootMemoryProtocol,
    base_name: *const u8,
    size: *mut UIntN,
    addr: *mut *mut c_void,
    flag: *mut u32,
) -> EfiStatus;

/// Raw function pointer for synchronizing partition buffers back to storage.
pub type GblEfiBootMemorySyncPartitionBuffer = unsafe extern "efiapi" fn(
    this: *mut GblEfiBootMemoryProtocol,
    sync_preloaded: Bool,
) -> EfiStatus;

/// Raw function pointer for querying dedicated boot buffers.
pub type GblEfiBootMemoryGetBootBuffer = unsafe extern "efiapi" fn(
    this: *mut GblEfiBootMemoryProtocol,
    buf_type: u32,
    size: *mut UIntN,
    addr: *mut *mut c_void,
) -> EfiStatus;

/// Raw Android Boot Memory Protocol structure.
#[derive(Debug)]
#[repr(C)]
pub struct GblEfiBootMemoryProtocol {
    /// Revision of the protocol.
    pub revision: u64,
    /// Function to retrieve a partition buffer.
    pub get_partition_buffer: GblEfiBootMemoryGetPartitionBuffer,
    /// Function to flush/sync partition buffers to media.
    pub sync_partition_buffer: GblEfiBootMemorySyncPartitionBuffer,
    /// Function to allocate or locate dedicated boot buffers.
    pub get_boot_buffer: GblEfiBootMemoryGetBootBuffer,
}

/// High-level safe wrapper around [`GblEfiBootMemoryProtocol`].
pub struct BootMemory<'a> {
    proto: &'a mut GblEfiBootMemoryProtocol,
}

impl<'a> BootMemory<'a> {
    /// Creates a safe wrapper around an active protocol instance.
    #[must_use]
    pub fn new(proto: &'a mut GblEfiBootMemoryProtocol) -> Self {
        Self { proto }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.proto.revision
    }

    /// Locates the buffer for a named partition (e.g. `boot`, `vendor_boot`, `init_boot`).
    ///
    /// Returns `(base_address, size_bytes, flags)` on success.
    ///
    /// # Errors
    ///
    /// Returns an error status if the requested partition is not found or unsupported.
    pub fn get_partition_buffer(
        &mut self,
        partition_name: &[u8],
    ) -> Result<(*mut u8, usize, u32), EfiStatus> {
        let mut size: UIntN = 0;
        let mut addr: *mut c_void = core::ptr::null_mut();
        let mut flag: u32 = 0;

        let status = unsafe {
            (self.proto.get_partition_buffer)(
                self.proto,
                partition_name.as_ptr(),
                &raw mut size,
                &raw mut addr,
                &raw mut flag,
            )
        };

        if status.is_success() {
            Ok((addr.cast::<u8>(), size, flag))
        } else {
            Err(status)
        }
    }

    /// Synchronizes partition buffer modifications back to flash/disk.
    ///
    /// # Errors
    ///
    /// Returns an error status if flushing changes back to storage fails.
    pub fn sync_partition_buffer(&mut self, sync_preloaded: bool) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.proto.sync_partition_buffer)(self.proto, Bool::from(sync_preloaded)) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries the dedicated boot buffer for a specific payload type.
    ///
    /// Returns `(base_address, size_bytes)` on success.
    ///
    /// # Errors
    ///
    /// Returns an error status if the requested buffer type is unavailable or cannot be allocated.
    pub fn get_boot_buffer(
        &mut self,
        buf_type: GblEfiBootBufferType,
    ) -> Result<(*mut u8, usize), EfiStatus> {
        let mut size: UIntN = 0;
        let mut addr: *mut c_void = core::ptr::null_mut();

        let status = unsafe {
            (self.proto.get_boot_buffer)(self.proto, buf_type as u32, &raw mut size, &raw mut addr)
        };

        if status.is_success() {
            Ok((addr.cast::<u8>(), size))
        } else {
            Err(status)
        }
    }
}
