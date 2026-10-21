//! # PCI access
//!
//! Root bridge and PCI I/O wrappers.
//!
//! Safe methods take slices and use stack
//! locals. Unsafe methods expose DMA mapping
//! and raw resource pointers.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::{AllocationType, EfiHandle, EfiPhysicalAddress, EfiStatus, MemoryType};
use crate::protocols::pci::types::{
    AttributeOperation, PciIoOperation, PciWidth, RootBridgeOperation,
};

/// Polls memory or I/O.
pub type RootPoll = unsafe extern "efiapi" fn(
    *mut EfiPciRootBridgeIoProtocol,
    PciWidth,
    u64,
    u64,
    u64,
    u64,
    *mut u64,
) -> EfiStatus;

/// Transfers memory or I/O.
pub type RootIoMem = unsafe extern "efiapi" fn(
    *mut EfiPciRootBridgeIoProtocol,
    PciWidth,
    u64,
    UIntN,
    *mut c_void,
) -> EfiStatus;

/// A root bridge access pair.
#[derive(Debug)]
#[repr(C)]
pub struct RootAccess {
    /// Reads.
    pub read: RootIoMem,
    /// Writes.
    pub write: RootIoMem,
}

/// The root bridge protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPciRootBridgeIoProtocol {
    /// The parent handle.
    pub parent_handle: EfiHandle,
    /// Polls memory.
    pub poll_mem: RootPoll,
    /// Polls I/O.
    pub poll_io: RootPoll,
    /// Memory access.
    pub mem: RootAccess,
    /// I/O access.
    pub io: RootAccess,
    /// Configuration access.
    pub pci: RootAccess,
    /// Copies memory.
    pub copy_mem: unsafe extern "efiapi" fn(
        *mut EfiPciRootBridgeIoProtocol,
        PciWidth,
        u64,
        u64,
        UIntN,
    ) -> EfiStatus,
    /// Maps DMA memory.
    pub map: unsafe extern "efiapi" fn(
        *mut EfiPciRootBridgeIoProtocol,
        RootBridgeOperation,
        *mut c_void,
        *mut UIntN,
        *mut EfiPhysicalAddress,
        *mut *mut c_void,
    ) -> EfiStatus,
    /// Unmaps DMA memory.
    pub unmap: unsafe extern "efiapi" fn(*mut EfiPciRootBridgeIoProtocol, *mut c_void) -> EfiStatus,
    /// Allocates a DMA buffer.
    pub allocate_buffer: unsafe extern "efiapi" fn(
        *mut EfiPciRootBridgeIoProtocol,
        AllocationType,
        MemoryType,
        UIntN,
        *mut *mut c_void,
        u64,
    ) -> EfiStatus,
    /// Frees a DMA buffer.
    pub free_buffer:
        unsafe extern "efiapi" fn(*mut EfiPciRootBridgeIoProtocol, UIntN, *mut c_void) -> EfiStatus,
    /// Flushes writes.
    pub flush: unsafe extern "efiapi" fn(*mut EfiPciRootBridgeIoProtocol) -> EfiStatus,
    /// Gets attributes.
    pub get_attributes:
        unsafe extern "efiapi" fn(*mut EfiPciRootBridgeIoProtocol, *mut u64, *mut u64) -> EfiStatus,
    /// Sets attributes.
    pub set_attributes: unsafe extern "efiapi" fn(
        *mut EfiPciRootBridgeIoProtocol,
        u64,
        *mut u64,
        *mut u64,
    ) -> EfiStatus,
    /// Gets configuration.
    pub configuration:
        unsafe extern "efiapi" fn(*mut EfiPciRootBridgeIoProtocol, *mut *mut c_void) -> EfiStatus,
    /// The segment number.
    pub segment_number: u32,
}

/// Polls PCI I/O memory.
pub type PciPoll = unsafe extern "efiapi" fn(
    *mut EfiPciIoProtocol,
    PciWidth,
    u8,
    u64,
    u64,
    u64,
    u64,
    *mut u64,
) -> EfiStatus;

/// Transfers PCI memory or I/O.
pub type PciIoMem = unsafe extern "efiapi" fn(
    *mut EfiPciIoProtocol,
    PciWidth,
    u8,
    u64,
    UIntN,
    *mut c_void,
) -> EfiStatus;

/// A PCI access pair.
#[derive(Debug)]
#[repr(C)]
pub struct PciAccess {
    /// Reads.
    pub read: PciIoMem,
    /// Writes.
    pub write: PciIoMem,
}

/// Transfers PCI configuration.
pub type PciConfig = unsafe extern "efiapi" fn(
    *mut EfiPciIoProtocol,
    PciWidth,
    u32,
    UIntN,
    *mut c_void,
) -> EfiStatus;

/// A configuration pair.
#[derive(Debug)]
#[repr(C)]
pub struct PciConfigAccess {
    /// Reads.
    pub read: PciConfig,
    /// Writes.
    pub write: PciConfig,
}

/// The PCI I/O protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPciIoProtocol {
    /// Polls memory.
    pub poll_mem: PciPoll,
    /// Polls I/O.
    pub poll_io: PciPoll,
    /// Memory access.
    pub mem: PciAccess,
    /// I/O access.
    pub io: PciAccess,
    /// Configuration access.
    pub pci: PciConfigAccess,
    /// Copies memory.
    pub copy_mem: unsafe extern "efiapi" fn(
        *mut EfiPciIoProtocol,
        PciWidth,
        u8,
        u64,
        u8,
        u64,
        UIntN,
    ) -> EfiStatus,
    /// Maps DMA memory.
    pub map: unsafe extern "efiapi" fn(
        *mut EfiPciIoProtocol,
        PciIoOperation,
        *mut c_void,
        *mut UIntN,
        *mut EfiPhysicalAddress,
        *mut *mut c_void,
    ) -> EfiStatus,
    /// Unmaps DMA memory.
    pub unmap: unsafe extern "efiapi" fn(*mut EfiPciIoProtocol, *mut c_void) -> EfiStatus,
    /// Allocates a DMA buffer.
    pub allocate_buffer: unsafe extern "efiapi" fn(
        *mut EfiPciIoProtocol,
        AllocationType,
        MemoryType,
        UIntN,
        *mut *mut c_void,
        u64,
    ) -> EfiStatus,
    /// Frees a DMA buffer.
    pub free_buffer:
        unsafe extern "efiapi" fn(*mut EfiPciIoProtocol, UIntN, *mut c_void) -> EfiStatus,
    /// Flushes writes.
    pub flush: unsafe extern "efiapi" fn(*mut EfiPciIoProtocol) -> EfiStatus,
    /// Gets the location.
    pub get_location: unsafe extern "efiapi" fn(
        *mut EfiPciIoProtocol,
        *mut UIntN,
        *mut UIntN,
        *mut UIntN,
        *mut UIntN,
    ) -> EfiStatus,
    /// Performs an attribute operation.
    pub attributes: unsafe extern "efiapi" fn(
        *mut EfiPciIoProtocol,
        AttributeOperation,
        u64,
        *mut u64,
    ) -> EfiStatus,
    /// Gets BAR attributes.
    pub get_bar_attributes: unsafe extern "efiapi" fn(
        *mut EfiPciIoProtocol,
        u8,
        *mut u64,
        *mut *mut c_void,
    ) -> EfiStatus,
    /// Sets BAR attributes.
    pub set_bar_attributes:
        unsafe extern "efiapi" fn(*mut EfiPciIoProtocol, u64, u8, *mut u64, *mut u64) -> EfiStatus,
    /// The ROM size.
    pub rom_size: u64,
    /// The ROM image pointer.
    pub rom_image: *mut c_void,
}

/// A typed root bridge view.
pub struct PciRootBridgeIo<'a> {
    protocol: &'a EfiPciRootBridgeIoProtocol,
}

impl<'a> PciRootBridgeIo<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiPciRootBridgeIoProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiPciRootBridgeIoProtocol {
        self.protocol
    }

    /// Reads root memory.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn mem_read(
        &self,
        width: PciWidth,
        address: u64,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        // SAFETY: The slice pointer is valid for the call.
        let status = unsafe {
            (self.protocol.mem.read)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                width,
                address,
                buffer.len(),
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Writes root memory.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn mem_write(&self, width: PciWidth, address: u64, buffer: &[u8]) -> Result<(), EfiStatus> {
        // SAFETY: The slice pointer is valid for the call.
        let status = unsafe {
            (self.protocol.mem.write)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                width,
                address,
                buffer.len(),
                buffer.as_ptr().cast::<c_void>().cast_mut(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Flushes writes.
    #[must_use]
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn flush(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.flush)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }

    /// Gets attributes.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn get_attributes(&self) -> Result<(u64, u64), EfiStatus> {
        let mut supports: u64 = 0;
        let mut attributes: u64 = 0;
        // SAFETY: Stack locals are valid for out parameters.
        let status = unsafe {
            (self.protocol.get_attributes)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut supports,
                &raw mut attributes,
            )
        };
        if status.is_success() {
            Ok((supports, attributes))
        } else {
            Err(status)
        }
    }

    /// Maps DMA memory.
    ///
    /// # Safety
    ///
    /// The host address must be valid, the
    /// size pointer must be valid, and both
    /// out pointers must be valid.
    #[must_use]
    pub unsafe fn map(
        &self,
        operation: RootBridgeOperation,
        host_address: *mut c_void,
        number_of_bytes: *mut UIntN,
        device_address: *mut EfiPhysicalAddress,
        mapping: *mut *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity.
        unsafe {
            (self.protocol.map)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                operation,
                host_address,
                number_of_bytes,
                device_address,
                mapping,
            )
        }
    }

    /// Unmaps DMA memory.
    ///
    /// # Safety
    ///
    /// The mapping must come from `map`.
    #[must_use]
    pub unsafe fn unmap(&self, mapping: *mut c_void) -> EfiStatus {
        // SAFETY: The caller guarantees the mapping is valid.
        unsafe { (self.protocol.unmap)(core::ptr::from_ref(self.protocol).cast_mut(), mapping) }
    }
}

impl core::fmt::Debug for PciRootBridgeIo<'_> {
    /// Formats without dereferencing firmware
    /// state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("PciRootBridgeIo")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}

/// A typed PCI I/O view.
pub struct PciIo<'a> {
    protocol: &'a EfiPciIoProtocol,
}

impl<'a> PciIo<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiPciIoProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiPciIoProtocol {
        self.protocol
    }

    /// Reads memory through a BAR.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn mem_read(
        &self,
        width: PciWidth,
        bar: u8,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        // SAFETY: The slice pointer is valid for the call.
        let status = unsafe {
            (self.protocol.mem.read)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                width,
                bar,
                offset,
                buffer.len(),
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Writes memory through a BAR.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn mem_write(
        &self,
        width: PciWidth,
        bar: u8,
        offset: u64,
        buffer: &[u8],
    ) -> Result<(), EfiStatus> {
        // SAFETY: The slice pointer is valid for the call.
        let status = unsafe {
            (self.protocol.mem.write)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                width,
                bar,
                offset,
                buffer.len(),
                buffer.as_ptr().cast::<c_void>().cast_mut(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads configuration space.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn pci_read(
        &self,
        width: PciWidth,
        offset: u32,
        buffer: &mut [u8],
    ) -> Result<(), EfiStatus> {
        // SAFETY: The slice pointer is valid for the call.
        let status = unsafe {
            (self.protocol.pci.read)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                width,
                offset,
                buffer.len(),
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Writes configuration space.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn pci_write(&self, width: PciWidth, offset: u32, buffer: &[u8]) -> Result<(), EfiStatus> {
        // SAFETY: The slice pointer is valid for the call.
        let status = unsafe {
            (self.protocol.pci.write)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                width,
                offset,
                buffer.len(),
                buffer.as_ptr().cast::<c_void>().cast_mut(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Gets the device location.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn get_location(&self) -> Result<(UIntN, UIntN, UIntN, UIntN), EfiStatus> {
        let mut segment: UIntN = 0;
        let mut bus: UIntN = 0;
        let mut device: UIntN = 0;
        let mut function: UIntN = 0;
        // SAFETY: Stack locals are valid for out parameters.
        let status = unsafe {
            (self.protocol.get_location)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                &raw mut segment,
                &raw mut bus,
                &raw mut device,
                &raw mut function,
            )
        };
        if status.is_success() {
            Ok((segment, bus, device, function))
        } else {
            Err(status)
        }
    }

    /// Performs an attribute operation.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// operation fails.
    pub fn attributes(
        &self,
        operation: AttributeOperation,
        attributes: u64,
    ) -> Result<u64, EfiStatus> {
        let mut result: u64 = 0;
        // SAFETY: The stack local is valid when required.
        let status = unsafe {
            (self.protocol.attributes)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                operation,
                attributes,
                &raw mut result,
            )
        };
        if status.is_success() {
            Ok(result)
        } else {
            Err(status)
        }
    }

    /// Flushes writes.
    #[must_use]
    pub fn flush(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.flush)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }
}

impl core::fmt::Debug for PciIo<'_> {
    /// Formats without dereferencing firmware
    /// state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("PciIo")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
