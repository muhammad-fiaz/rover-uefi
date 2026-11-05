//! # DXE Services Table and Global Coherency Domain (GCD)
//!
//! Defined by the UEFI Platform Initialization (PI) Specification Volume 2:
//! Driver Execution Environment Core Interface.
//!
//! DXE Services manage the global memory and I/O architectural maps (GCD)
//! and provide dispatcher and firmware volume registration capabilities.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};
use crate::tables::header::EfiTableHeader;
use core::ffi::c_void;

/// DXE Services Table signature.
pub const DXE_SERVICES_SIGNATURE: u64 = 0x5652_4553_2045_5844; // "DXE SERV"

/// DXE Services specification revision 1.80.
pub const DXE_SERVICES_REVISION: u32 = (1 << 16) | 0x0050;

/// The configuration table GUID used to locate the DXE Services Table.
pub const DXE_SERVICES_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x0547_A072,
    0x3E82,
    0x49A7,
    [0xAE, 0xDD, 0x4D, 0x10, 0x56, 0x01, 0x0E, 0x24],
);

/// GCD memory space type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiGcdMemoryType {
    /// Non-existent memory space.
    NonExistent = 0,
    /// Reserved memory space.
    Reserved = 1,
    /// System memory (RAM).
    SystemMemory = 2,
    /// Memory mapped I/O space.
    MemoryMappedIo = 3,
    /// Persistent memory space.
    PersistentMemory = 4,
    /// Reliable memory space.
    ReliableMemory = 5,
    /// Maximum memory space type count.
    MaximumMemoryType = 6,
}

/// GCD memory allocation search algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiGcdAllocateType {
    /// Search bottom-up for any address.
    AllocateAnySearchBottomUp = 0,
    /// Search bottom-up for address below max address.
    AllocateMaxAddressSearchBottomUp = 1,
    /// Allocate at specific address.
    AllocateAddress = 2,
    /// Search top-down for any address.
    AllocateAnySearchTopDown = 3,
    /// Search top-down for address below max address.
    AllocateMaxAddressSearchTopDown = 4,
    /// Maximum allocation search type count.
    MaxAllocateType = 5,
}

/// GCD memory space descriptor.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiGcdMemorySpaceDescriptor {
    /// Base address of the memory space.
    pub base_address: EfiPhysicalAddress,
    /// Length of the memory space in bytes.
    pub length: u64,
    /// Capabilities bitmask of the memory space.
    pub capabilities: u64,
    /// Current attributes bitmask of the memory space.
    pub attributes: u64,
    /// GCD memory type.
    pub gcd_memory_type: EfiGcdMemoryType,
    /// Handle of the image that allocated this space.
    pub image_handle: EfiHandle,
    /// Device handle associated with this memory space.
    pub device_handle: EfiHandle,
}

/// GCD I/O space type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiGcdIoType {
    /// Non-existent I/O space.
    NonExistent = 0,
    /// Reserved I/O space.
    Reserved = 1,
    /// Standard I/O space.
    Io = 2,
    /// Maximum I/O space type count.
    MaximumIoType = 3,
}

/// GCD I/O space descriptor.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiGcdIoSpaceDescriptor {
    /// Base address of the I/O space.
    pub base_address: EfiPhysicalAddress,
    /// Length of the I/O space in bytes.
    pub length: u64,
    /// GCD I/O space type.
    pub gcd_io_type: EfiGcdIoType,
    /// Handle of the image that allocated this space.
    pub image_handle: EfiHandle,
    /// Device handle associated with this I/O space.
    pub device_handle: EfiHandle,
}

/// Function pointer for `EFI_ADD_MEMORY_SPACE`.
pub type EfiAddMemorySpace = unsafe extern "efiapi" fn(
    gcd_memory_type: EfiGcdMemoryType,
    base_address: EfiPhysicalAddress,
    length: u64,
    capabilities: u64,
) -> EfiStatus;

/// Function pointer for `EFI_ALLOCATE_MEMORY_SPACE`.
pub type EfiAllocateMemorySpace = unsafe extern "efiapi" fn(
    gcd_allocate_type: EfiGcdAllocateType,
    gcd_memory_type: EfiGcdMemoryType,
    alignment: UIntN,
    length: u64,
    base_address: *mut EfiPhysicalAddress,
    image_handle: EfiHandle,
    device_handle: EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_FREE_MEMORY_SPACE`.
pub type EfiFreeMemorySpace =
    unsafe extern "efiapi" fn(base_address: EfiPhysicalAddress, length: u64) -> EfiStatus;

/// Function pointer for `EFI_REMOVE_MEMORY_SPACE`.
pub type EfiRemoveMemorySpace =
    unsafe extern "efiapi" fn(base_address: EfiPhysicalAddress, length: u64) -> EfiStatus;

/// Function pointer for `EFI_GET_MEMORY_SPACE_DESCRIPTOR`.
pub type EfiGetMemorySpaceDescriptor = unsafe extern "efiapi" fn(
    base_address: EfiPhysicalAddress,
    descriptor: *mut EfiGcdMemorySpaceDescriptor,
) -> EfiStatus;

/// Function pointer for `EFI_SET_MEMORY_SPACE_ATTRIBUTES`.
pub type EfiSetMemorySpaceAttributes = unsafe extern "efiapi" fn(
    base_address: EfiPhysicalAddress,
    length: u64,
    attributes: u64,
) -> EfiStatus;

/// Function pointer for `EFI_SET_MEMORY_SPACE_CAPABILITIES`.
pub type EfiSetMemorySpaceCapabilities = unsafe extern "efiapi" fn(
    base_address: EfiPhysicalAddress,
    length: u64,
    capabilities: u64,
) -> EfiStatus;

/// Function pointer for `EFI_GET_MEMORY_SPACE_MAP`.
pub type EfiGetMemorySpaceMap = unsafe extern "efiapi" fn(
    number_of_descriptors: *mut UIntN,
    space_map: *mut *mut EfiGcdMemorySpaceDescriptor,
) -> EfiStatus;

/// Function pointer for `EFI_ADD_IO_SPACE`.
pub type EfiAddIoSpace = unsafe extern "efiapi" fn(
    gcd_io_type: EfiGcdIoType,
    base_address: EfiPhysicalAddress,
    length: u64,
) -> EfiStatus;

/// Function pointer for `EFI_ALLOCATE_IO_SPACE`.
pub type EfiAllocateIoSpace = unsafe extern "efiapi" fn(
    gcd_allocate_type: EfiGcdAllocateType,
    gcd_io_type: EfiGcdIoType,
    alignment: UIntN,
    length: u64,
    base_address: *mut EfiPhysicalAddress,
    image_handle: EfiHandle,
    device_handle: EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_FREE_IO_SPACE`.
pub type EfiFreeIoSpace =
    unsafe extern "efiapi" fn(base_address: EfiPhysicalAddress, length: u64) -> EfiStatus;

/// Function pointer for `EFI_REMOVE_IO_SPACE`.
pub type EfiRemoveIoSpace =
    unsafe extern "efiapi" fn(base_address: EfiPhysicalAddress, length: u64) -> EfiStatus;

/// Function pointer for `EFI_GET_IO_SPACE_DESCRIPTOR`.
pub type EfiGetIoSpaceDescriptor = unsafe extern "efiapi" fn(
    base_address: EfiPhysicalAddress,
    descriptor: *mut EfiGcdIoSpaceDescriptor,
) -> EfiStatus;

/// Function pointer for `EFI_GET_IO_SPACE_MAP`.
pub type EfiGetIoSpaceMap = unsafe extern "efiapi" fn(
    number_of_descriptors: *mut UIntN,
    space_map: *mut *mut EfiGcdIoSpaceDescriptor,
) -> EfiStatus;

/// Function pointer for `EFI_DXE_DISPATCH`.
pub type EfiDxeDispatch = unsafe extern "efiapi" fn() -> EfiStatus;

/// Function pointer for `EFI_DXE_SCHEDULE`.
pub type EfiDxeSchedule = unsafe extern "efiapi" fn(
    firmware_volume_handle: EfiHandle,
    driver_name: *const EfiGuid,
) -> EfiStatus;

/// Function pointer for `EFI_DXE_TRUST`.
pub type EfiDxeTrust = unsafe extern "efiapi" fn(
    firmware_volume_handle: EfiHandle,
    driver_name: *const EfiGuid,
) -> EfiStatus;

/// Function pointer for `EFI_PROCESS_FIRMWARE_VOLUME`.
pub type EfiProcessFirmwareVolume = unsafe extern "efiapi" fn(
    firmware_volume_header: *const c_void,
    size: UIntN,
    firmware_volume_handle: *mut EfiHandle,
) -> EfiStatus;

/// The DXE Services Table defined by PI 1.8 Volume 2.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDxeServices {
    /// Table header with signature and revision.
    pub hdr: EfiTableHeader,
    /// Adds memory space to the GCD.
    pub add_memory_space: EfiAddMemorySpace,
    /// Allocates memory space from the GCD.
    pub allocate_memory_space: EfiAllocateMemorySpace,
    /// Frees previously allocated memory space back to the GCD.
    pub free_memory_space: EfiFreeMemorySpace,
    /// Removes memory space from the GCD.
    pub remove_memory_space: EfiRemoveMemorySpace,
    /// Gets descriptor for a given memory address.
    pub get_memory_space_descriptor: EfiGetMemorySpaceDescriptor,
    /// Modifies architectural attributes of a memory region.
    pub set_memory_space_attributes: EfiSetMemorySpaceAttributes,
    /// Modifies architectural capabilities of a memory region.
    pub set_memory_space_capabilities: EfiSetMemorySpaceCapabilities,
    /// Retrieves the complete GCD memory map.
    pub get_memory_space_map: EfiGetMemorySpaceMap,
    /// Adds I/O space to the GCD.
    pub add_io_space: EfiAddIoSpace,
    /// Allocates I/O space from the GCD.
    pub allocate_io_space: EfiAllocateIoSpace,
    /// Frees previously allocated I/O space back to the GCD.
    pub free_io_space: EfiFreeIoSpace,
    /// Removes I/O space from the GCD.
    pub remove_io_space: EfiRemoveIoSpace,
    /// Gets descriptor for a given I/O address.
    pub get_io_space_descriptor: EfiGetIoSpaceDescriptor,
    /// Retrieves the complete GCD I/O map.
    pub get_io_space_map: EfiGetIoSpaceMap,
    /// Dispatches eligible DXE drivers whose dependency expressions are satisfied.
    pub dispatch: EfiDxeDispatch,
    /// Inserts a driver into the execution schedule.
    pub schedule: EfiDxeSchedule,
    /// Marks a driver as trusted.
    pub trust: EfiDxeTrust,
    /// Registers a newly discovered firmware volume image with the DXE Core.
    pub process_firmware_volume: EfiProcessFirmwareVolume,
}
