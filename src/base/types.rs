//! # Enumerations
//!
//! Integer enumerations used by memory services.
//!
//! The specification expresses memory types and
//! allocation types as `C` enums. Firmware may
//! return values that do not match a named variant,
//! so the Rust types store the raw `u32` value and
//! provide named constants plus checked conversion.
//! No Rust enum is used for these values, which
//! avoids invalid discriminants.
//!
//! The module also defines the task priority level
//! type and the small enumerations that appear in
//! boot services and runtime services table function
//! signatures, together with the memory descriptor.

use crate::abi::types::UIntN;
use crate::base::physical_address::EfiPhysicalAddress;
use crate::base::virtual_address::EfiVirtualAddress;

/// A memory descriptor type.
///
/// The value classifies a region of the firmware
/// memory map. Named associated constants cover
/// the values defined by the specification. Values
/// from `0x7000_0000` to `0x7FFF_FFFF` are reserved
/// for platform (OEM) use, and values from
/// `0x8000_0000` to `0xFFFF_FFFF` are reserved for
/// operating system use. Use
/// [`MemoryType::from_raw_checked`] to validate a value
/// obtained from firmware.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MemoryType(u32);

impl MemoryType {
    /// Reserved memory, with no defined use.
    pub const RESERVED: Self = Self(0);

    /// Code loaded from a `UEFI` image.
    pub const LOADER_CODE: Self = Self(1);

    /// Data loaded from a `UEFI` image.
    pub const LOADER_DATA: Self = Self(2);

    /// Code used during `ExitBootServices`.
    pub const EXIT_BOOT_SERVICES_CODE: Self = Self(3);

    /// Data used during `ExitBootServices`.
    pub const EXIT_BOOT_SERVICES_DATA: Self = Self(4);

    /// Code that remains available at runtime.
    pub const RUNTIME_SERVICES_CODE: Self = Self(5);

    /// Data that remains available at runtime.
    pub const RUNTIME_SERVICES_DATA: Self = Self(6);

    /// Memory that must not be used.
    pub const UNUSABLE_MEMORY: Self = Self(7);

    /// Code used by the `ACPI` platform.
    pub const ACPI_RECLAIM_MEMORY: Self = Self(8);

    /// Table data owned by the `ACPI` platform.
    pub const ACPI_MEMORY_MAP: Self = Self(9);

    /// Memory-mapped `I/O` registers.
    pub const MEMORY_MAPPED_IO: Self = Self(10);

    /// Memory-mapped `I/O` port space.
    pub const MEMORY_MAPPED_IO_PORT_SPACE: Self = Self(11);

    /// Pal code.
    pub const PAL_CODE: Self = Self(12);

    /// Persistent memory.
    pub const PERSISTENT_MEMORY: Self = Self(13);

    /// Memory that has not been accepted by the
    /// underlying isolation architecture.
    pub const UNACCEPTED_MEMORY: Self = Self(14);

    /// The first value after the highest defined
    /// normal type.
    pub const MAX_MEMORY_TYPE: Self = Self(15);

    /// The first platform-reserved value.
    pub const OEM_RESERVED_MIN: Self = Self(0x7000_0000);

    /// The last platform-reserved value.
    pub const OEM_RESERVED_MAX: Self = Self(0x7FFF_FFFF);

    /// The first operating-system-reserved value.
    pub const OS_RESERVED_MIN: Self = Self(0x8000_0000);

    /// The last operating-system-reserved value.
    pub const OS_RESERVED_MAX: Self = Self(0xFFFF_FFFF);

    /// Creates a value from a raw firmware integer.
    ///
    /// The value is preserved without validation.
    /// Use [`MemoryType::from_raw_checked`] when a
    /// value from firmware must be constrained to
    /// the defined range.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a value from a raw integer if it is in
    /// the normal defined range.
    ///
    /// Returns `None` for values outside `0` through
    /// [`MemoryType::MAX_MEMORY_TYPE`] and for the
    /// reserved ranges, which require separate
    /// platform or operating system handling.
    #[must_use]
    pub const fn from_raw_checked(raw: u32) -> Option<Self> {
        if raw < 15 { Some(Self(raw)) } else { None }
    }

    /// Returns the raw integer value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

impl core::fmt::Debug for MemoryType {
    /// Formats the memory type as its raw value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.debug_tuple("MemoryType").field(&self.0).finish()
    }
}

/// A page allocation type.
///
/// The value selects how boot services choose
/// pages for an allocation request. Named
/// associated constants cover the values defined
/// by the specification. Use
/// [`AllocationType::from_raw_checked`] to validate
/// a value obtained from firmware.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AllocationType(u32);

impl AllocationType {
    /// Allocate any available range of pages.
    pub const ANY_PAGES: Self = Self(0);

    /// Allocate any available range whose
    /// uppermost address is at most a limit.
    pub const MAX_ADDRESS: Self = Self(1);

    /// Allocate pages at a specified address.
    pub const ADDRESS: Self = Self(2);

    /// The first value after the highest defined
    /// allocation type.
    pub const MAX_ALLOCATE_TYPE: Self = Self(3);

    /// Creates a value from a raw firmware integer.
    ///
    /// The value is preserved without validation.
    /// Use [`AllocationType::from_raw_checked`] when
    /// a value from firmware must be constrained
    /// to the defined range.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a value from a raw integer if it is
    /// a defined allocation type.
    ///
    /// Returns `None` for values at or above
    /// [`AllocationType::MAX_ALLOCATE_TYPE`].
    #[must_use]
    pub const fn from_raw_checked(raw: u32) -> Option<Self> {
        if raw < 3 { Some(Self(raw)) } else { None }
    }

    /// Returns the raw integer value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

impl core::fmt::Debug for AllocationType {
    /// Formats the allocation type as its raw value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_tuple("AllocationType")
            .field(&self.0)
            .finish()
    }
}

/// The task priority level type.
///
/// The value matches `EFI_TPL` in the specification,
/// which is a pointer-sized unsigned integer. It
/// identifies the priority required for event
/// notifications and for raising or restoring the
/// current priority.
pub type EfiTpl = UIntN;

/// The application task priority level.
///
/// Code running at the application level can wait
/// for events.
pub const TPL_APPLICATION: EfiTpl = 4;

/// The callback task priority level.
pub const TPL_CALLBACK: EfiTpl = 8;

/// The notify task priority level.
pub const TPL_NOTIFY: EfiTpl = 16;

/// The high task priority level.
pub const TPL_HIGH_LEVEL: EfiTpl = 31;

/// A timer delay type.
///
/// The value selects whether a timer event is
/// cancelled, signalled once, or signalled
/// periodically. The raw `u32` value is preserved
/// so values supplied by firmware outside the
/// defined range remain representable.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TimerDelay(u32);

impl TimerDelay {
    /// Cancel the timer.
    pub const CANCEL: Self = Self(0);

    /// Signal periodically at the specified interval.
    pub const PERIODIC: Self = Self(1);

    /// Signal once at the specified interval.
    pub const RELATIVE: Self = Self(2);

    /// Creates a value from a raw firmware integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a value if the raw integer names a
    /// defined timer delay.
    #[must_use]
    pub const fn from_raw_checked(raw: u32) -> Option<Self> {
        if raw < 3 { Some(Self(raw)) } else { None }
    }

    /// Returns the raw integer value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

impl core::fmt::Debug for TimerDelay {
    /// Formats the timer delay as its raw value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.debug_tuple("TimerDelay").field(&self.0).finish()
    }
}

/// A handle search type.
///
/// The value selects which handles a locate
/// operation returns.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LocateSearchType(u32);

impl LocateSearchType {
    /// Return all handles in the database.
    pub const ALL_HANDLES: Self = Self(0);

    /// Return the next handle from a protocol
    /// notification event.
    pub const BY_REGISTER_NOTIFY: Self = Self(1);

    /// Return handles that support a protocol.
    pub const BY_PROTOCOL: Self = Self(2);

    /// Creates a value from a raw firmware integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a value if the raw integer names a
    /// defined search type.
    #[must_use]
    pub const fn from_raw_checked(raw: u32) -> Option<Self> {
        if raw < 3 { Some(Self(raw)) } else { None }
    }

    /// Returns the raw integer value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

impl core::fmt::Debug for LocateSearchType {
    /// Formats the search type as its raw value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_tuple("LocateSearchType")
            .field(&self.0)
            .finish()
    }
}

/// A protocol interface type.
///
/// The value indicates whether an interface is
/// supplied in native form or in a transitional
/// form. Only the native form is defined by the
/// current specification.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterfaceType(u32);

impl InterfaceType {
    /// The interface is supplied in native form.
    pub const NATIVE_INTERFACE: Self = Self(0);

    /// Creates a value from a raw firmware integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a value if the raw integer names a
    /// defined interface type.
    #[must_use]
    pub const fn from_raw_checked(raw: u32) -> Option<Self> {
        if raw == 0 { Some(Self(raw)) } else { None }
    }

    /// Returns the raw integer value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

impl core::fmt::Debug for InterfaceType {
    /// Formats the interface type as its raw value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_tuple("InterfaceType")
            .field(&self.0)
            .finish()
    }
}

/// A system reset type.
///
/// The value selects the kind of reset performed
/// by the reset system runtime service.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ResetType(u32);

impl ResetType {
    /// Reset to the initial hardware state.
    pub const COLD: Self = Self(0);

    /// Reset processors while preserving pending
    /// cycles.
    pub const WARM: Self = Self(1);

    /// Power off the system.
    pub const SHUTDOWN: Self = Self(2);

    /// Reset to a platform-defined state.
    pub const PLATFORM_SPECIFIC: Self = Self(3);

    /// Creates a value from a raw firmware integer.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a value if the raw integer names a
    /// defined reset type.
    #[must_use]
    pub const fn from_raw_checked(raw: u32) -> Option<Self> {
        if raw < 4 { Some(Self(raw)) } else { None }
    }

    /// Returns the raw integer value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

impl core::fmt::Debug for ResetType {
    /// Formats the reset type as its raw value.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.debug_tuple("ResetType").field(&self.0).finish()
    }
}

/// The memory descriptor version.
///
/// The specification defines version `1` for
/// memory descriptors.
pub const EFI_MEMORY_DESCRIPTOR_VERSION: u32 = 1;

/// A firmware memory descriptor.
///
/// The struct is `#[repr(C)]` so field order, size
/// and alignment match the firmware definition:
/// a memory type, a physical start, a virtual
/// start, a page count, and an attribute mask.
/// The total size is 40 bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    /// The type of the memory region.
    pub memory_type: MemoryType,
    /// The physical address of the first byte.
    pub physical_start: EfiPhysicalAddress,
    /// The virtual address of the first byte.
    pub virtual_start: EfiVirtualAddress,
    /// The number of 4 KiB pages in the region.
    pub number_of_pages: u64,
    /// The attribute mask of the region.
    pub attribute: u64,
}
