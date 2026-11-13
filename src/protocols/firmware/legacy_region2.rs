//! # Legacy Region 2 Protocol
//!
//! Provides hardware abstraction for controlling read, write, and boot-lock attributes
//! for the legacy memory shadowing region `0xC0000` to `0xFFFFF` (PAM registers).
//! Defined in UEFI Platform Initialization (PI) Specification Volume 5.

use crate::base::{EfiGuid, EfiStatus};

/// Legacy Region 2 Protocol GUID: `{70101eaf-0085-440c-b356-8ee36fef24f0}`.
pub const EFI_LEGACY_REGION2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x7010_1EAF,
    0x0085,
    0x440C,
    [0xB3, 0x56, 0x8E, 0xE3, 0x6F, 0xEF, 0x24, 0xF0],
);

/// Starting physical address of the legacy option ROM shadow region (0xC0000).
pub const LEGACY_REGION_START: u32 = 0x000C_0000;

/// Ending physical address of the legacy option ROM shadow region (0xFFFFF).
pub const LEGACY_REGION_END: u32 = 0x000F_FFFF;

/// Total size of the legacy region in bytes (256 KiB).
pub const LEGACY_REGION_SIZE: u32 = 0x0004_0000;

/// Current attribute of a legacy memory region.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LegacyRegionAttribute {
    /// Region is currently set to allow reads.
    Decoded = 0,
    /// Region is currently set to disallow reads.
    NotDecoded = 1,
    /// Region is currently set to allow writes.
    WriteEnabled = 2,
    /// Region is currently write-protected.
    WriteDisabled = 3,
    /// Region attributes are locked until after a power cycle.
    BootLocked = 4,
    /// Region attributes are not locked.
    NotLocked = 5,
}

/// Describes the attributes and granularity of a legacy memory region.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct LegacyRegionDescriptor {
    /// Physical starting address of this region.
    pub start: u32,
    /// Number of bytes in this region.
    pub length: u32,
    /// Capability and current status attribute.
    pub attribute: LegacyRegionAttribute,
    /// Byte length programmability associated with this start address and setting.
    pub granularity: u32,
}

/// Function pointer for `EFI_LEGACY_REGION2_DECODE`.
pub type EfiLegacyRegion2Decode = unsafe extern "efiapi" fn(
    this: *mut EfiLegacyRegion2Protocol,
    start: u32,
    length: u32,
    granularity: *mut u32,
    on: *const bool,
) -> EfiStatus;

/// Function pointer for `EFI_LEGACY_REGION2_LOCK`.
pub type EfiLegacyRegion2Lock = unsafe extern "efiapi" fn(
    this: *mut EfiLegacyRegion2Protocol,
    start: u32,
    length: u32,
    granularity: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_LEGACY_REGION2_BOOT_LOCK`.
pub type EfiLegacyRegion2BootLock = unsafe extern "efiapi" fn(
    this: *mut EfiLegacyRegion2Protocol,
    start: u32,
    length: u32,
    granularity: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_LEGACY_REGION2_UNLOCK`.
pub type EfiLegacyRegion2Unlock = unsafe extern "efiapi" fn(
    this: *mut EfiLegacyRegion2Protocol,
    start: u32,
    length: u32,
    granularity: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_LEGACY_REGION_GET_INFO`.
pub type EfiLegacyRegionGetInfo = unsafe extern "efiapi" fn(
    this: *mut EfiLegacyRegion2Protocol,
    descriptor_count: *mut u32,
    descriptor: *mut *mut LegacyRegionDescriptor,
) -> EfiStatus;

/// Raw ABI structure for `EFI_LEGACY_REGION2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiLegacyRegion2Protocol {
    /// Modifies hardware to allow or disallow memory reads in a region.
    pub decode: EfiLegacyRegion2Decode,
    /// Modifies hardware to disallow memory writes in a region.
    pub lock: EfiLegacyRegion2Lock,
    /// Modifies hardware to disallow memory attribute changes in a region until reboot.
    pub boot_lock: EfiLegacyRegion2BootLock,
    /// Modifies hardware to allow memory writes in a region.
    pub unlock: EfiLegacyRegion2Unlock,
    /// Retrieves region descriptor information and granularity.
    pub get_info: EfiLegacyRegionGetInfo,
}

/// Safe client wrapper around [`EfiLegacyRegion2Protocol`].
pub struct LegacyRegion2<'a> {
    proto: &'a mut EfiLegacyRegion2Protocol,
}

impl<'a> LegacyRegion2<'a> {
    /// Creates a new `LegacyRegion2` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiLegacyRegion2Protocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiLegacyRegion2Protocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiLegacyRegion2Protocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiLegacyRegion2Protocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiLegacyRegion2Protocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Enables or disables memory reads in `[start, start + length)`.
    ///
    /// Returns the granularity in bytes of the affected region.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if `start` or `length` is outside `0xC0000..=0xFFFFF`,
    /// or another firmware error status.
    pub fn decode(&mut self, start: u32, length: u32, on: bool) -> Result<u32, EfiStatus> {
        let mut granularity = 0u32;
        let status = unsafe {
            (self.proto.decode)(
                self.proto,
                start,
                length,
                core::ptr::from_mut(&mut granularity),
                core::ptr::from_ref(&on),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(granularity)
        } else {
            Err(status)
        }
    }

    /// Disallows memory writes in `[start, start + length)`.
    ///
    /// Returns the granularity in bytes of the affected region.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if outside the legacy region, or an error status.
    pub fn lock(&mut self, start: u32, length: u32) -> Result<u32, EfiStatus> {
        let mut granularity = 0u32;
        let status = unsafe {
            (self.proto.lock)(
                self.proto,
                start,
                length,
                core::ptr::from_mut(&mut granularity),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(granularity)
        } else {
            Err(status)
        }
    }

    /// Boot-locks `[start, start + length)` against further attribute changes until reboot.
    ///
    /// Returns the granularity in bytes of the affected region.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if outside the legacy region, or `UNSUPPORTED`.
    pub fn boot_lock(&mut self, start: u32, length: u32) -> Result<u32, EfiStatus> {
        let mut granularity = 0u32;
        let status = unsafe {
            (self.proto.boot_lock)(
                self.proto,
                start,
                length,
                core::ptr::from_mut(&mut granularity),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(granularity)
        } else {
            Err(status)
        }
    }

    /// Allows memory writes in `[start, start + length)`.
    ///
    /// Returns the granularity in bytes of the affected region.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if outside the legacy region, or an error status.
    pub fn unlock(&mut self, start: u32, length: u32) -> Result<u32, EfiStatus> {
        let mut granularity = 0u32;
        let status = unsafe {
            (self.proto.unlock)(
                self.proto,
                start,
                length,
                core::ptr::from_mut(&mut granularity),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(granularity)
        } else {
            Err(status)
        }
    }

    /// Retrieves legacy region descriptors allocated by firmware.
    ///
    /// # Safety
    ///
    /// The firmware allocates and populates the descriptor buffer pointed to by `descriptors`.
    ///
    /// # Errors
    ///
    /// Returns an error status if query fails or is unsupported.
    pub unsafe fn get_info_raw(
        &mut self,
        descriptor_count: &mut u32,
        descriptors: &mut *mut LegacyRegionDescriptor,
    ) -> Result<(), EfiStatus> {
        let mut count = *descriptor_count;
        let mut desc = *descriptors;
        let status = unsafe {
            (self.proto.get_info)(
                self.proto,
                core::ptr::from_mut(&mut count),
                core::ptr::from_mut(&mut desc),
            )
        };
        if status == EfiStatus::SUCCESS {
            *descriptor_count = count;
            *descriptors = desc;
            Ok(())
        } else {
            Err(status)
        }
    }
}
