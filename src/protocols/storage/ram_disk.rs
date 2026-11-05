//! # RAM Disk Protocol
//!
//! UEFI 2.6 RAM Disk Protocol (`EFI_RAM_DISK_PROTOCOL`).
//! Provides services to register and unregister memory-backed RAM disks.

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;

/// RAM Disk Protocol GUID: `ab38a0df-6873-44a9-87e6-d4eb56148449`
pub const RAM_DISK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xab38_a0df,
    0x6873,
    0x44a9,
    [0x87, 0xe6, 0xd4, 0xeb, 0x56, 0x14, 0x84, 0x49],
);

/// Virtual Disk type GUID: `77ab535a-45fc-624b-5560-f7b281d1f96e`
pub const VIRTUAL_DISK_GUID: EfiGuid = EfiGuid::new(
    0x77ab_535a,
    0x45fc,
    0x624b,
    [0x55, 0x60, 0xf7, 0xb2, 0x81, 0xd1, 0xf9, 0x6e],
);

/// Virtual CD/DVD type GUID: `3d5abd30-4175-87ce-6d64-d2ade523c4bb`
pub const VIRTUAL_CD_GUID: EfiGuid = EfiGuid::new(
    0x3d5a_bd30,
    0x4175,
    0x87ce,
    [0x6d, 0x64, 0xd2, 0xad, 0xe5, 0x23, 0xc4, 0xbb],
);

/// Persistent Virtual Disk type GUID (NVDIMM-backed): `5cea02c9-4d07-69d3-269f-4496fbe096f9`
pub const PERSISTENT_VIRTUAL_DISK_GUID: EfiGuid = EfiGuid::new(
    0x5cea_02c9,
    0x4d07,
    0x69d3,
    [0x26, 0x9f, 0x44, 0x96, 0xfb, 0xe0, 0x96, 0xf9],
);

/// Persistent Virtual CD type GUID: `08018188-42cd-bb48-100f-5387d53ded3d`
pub const PERSISTENT_VIRTUAL_CD_GUID: EfiGuid = EfiGuid::new(
    0x0801_8188,
    0x42cd,
    0xbb48,
    [0x10, 0x0f, 0x53, 0x87, 0xd5, 0x3d, 0xed, 0x3d],
);

/// Function pointer type for `RegisterRamDisk`.
pub type EfiRamDiskRegister = unsafe extern "efiapi" fn(
    ram_disk_base: u64,
    ram_disk_size: u64,
    ram_disk_type: *const EfiGuid,
    parent_device_path: *const EfiDevicePath,
    device_path: *mut *mut EfiDevicePath,
) -> EfiStatus;

/// Function pointer type for `UnregisterRamDisk`.
pub type EfiRamDiskUnregister =
    unsafe extern "efiapi" fn(device_path: *const EfiDevicePath) -> EfiStatus;

/// Raw UEFI RAM Disk Protocol interface (`EFI_RAM_DISK_PROTOCOL`).
#[repr(C)]
pub struct EfiRamDiskProtocol {
    /// Registers a RAM disk into the handle database.
    pub register: EfiRamDiskRegister,
    /// Unregisters a RAM disk.
    pub unregister: EfiRamDiskUnregister,
}

/// Safe wrapper around `EfiRamDiskProtocol`.
pub struct RamDiskProtocol<'a> {
    raw: &'a mut EfiRamDiskProtocol,
}

impl RamDiskProtocol<'_> {
    /// Wraps a raw `EfiRamDiskProtocol` instance.
    ///
    /// # Safety
    /// The caller must ensure that `raw` points to a valid, live protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut EfiRamDiskProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            // SAFETY: Caller guarantees `raw` points to a valid, live protocol instance.
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Registers a RAM disk buffer.
    ///
    /// Returns `(EfiStatus, Option<*mut EfiDevicePath>)`.
    pub fn register_ramdisk(
        &mut self,
        base: u64,
        size: u64,
        disk_type: &EfiGuid,
        parent: Option<*const EfiDevicePath>,
    ) -> (EfiStatus, Option<*mut EfiDevicePath>) {
        if size == 0 {
            return (EfiStatus::INVALID_PARAMETER, None);
        }
        let mut dev_path: *mut EfiDevicePath = core::ptr::null_mut();
        let parent_ptr = parent.unwrap_or(core::ptr::null());
        let status = unsafe {
            (self.raw.register)(
                base,
                size,
                core::ptr::from_ref(disk_type),
                parent_ptr,
                &raw mut dev_path,
            )
        };
        if status == EfiStatus::SUCCESS && !dev_path.is_null() {
            (status, Some(dev_path))
        } else {
            (status, None)
        }
    }

    /// Unregisters a RAM disk given its device path.
    ///
    /// # Safety
    /// Caller must ensure `device_path` points to a valid device path describing a RAM disk
    /// created by `register_ramdisk`.
    pub unsafe fn unregister_ramdisk(&mut self, device_path: *const EfiDevicePath) -> EfiStatus {
        if device_path.is_null() {
            return EfiStatus::INVALID_PARAMETER;
        }
        unsafe { (self.raw.unregister)(device_path) }
    }
}
