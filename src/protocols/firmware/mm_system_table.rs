//! # Management Mode System Table (MMST)
//!
//! Defined by the UEFI Platform Initialization (PI) Specification Volume 4:
//! Management Mode Core Interface.
//!
//! The MM System Table provides execution environment services inside MM / SMM,
//! including MMI registration, memory allocation, configuration tables, and
//! protocol handling.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};
use crate::tables::header::EfiTableHeader;
use core::ffi::c_void;

/// MM System Table signature.
pub const MM_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453_4D4D; // "MMST"

/// MM System Table revision 1.80.
pub const MM_SYSTEM_TABLE_REVISION: u32 = (1 << 16) | 0x0050;

/// Configuration table GUID for locating the MM System Table.
pub const EFI_MM_SYSTEM_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x7E12_F0CD,
    0x1B19,
    0x4879,
    [0xA2, 0xD4, 0xBB, 0x8A, 0x47, 0x62, 0x42, 0x72],
);

/// SMM System Table 2 GUID.
pub const EFI_SMM_SYSTEM_TABLE2_GUID: EfiGuid = EfiGuid::new(
    0x7E12_F0CD,
    0x1B19,
    0x4879,
    [0xA2, 0xD4, 0xBB, 0x8A, 0x47, 0x62, 0x42, 0x72],
);

/// Function pointer type for MMI handler callbacks.
pub type EfiMmiHandler = unsafe extern "efiapi" fn(
    dispatch_handle: EfiHandle,
    context: *const c_void,
    comm_buffer: *mut c_void,
    comm_buffer_size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_MM_INSTALL_CONFIGURATION_TABLE`.
pub type EfiMmInstallConfigurationTable = unsafe extern "efiapi" fn(
    system_table: *const EfiMmSystemTable,
    guid: *const EfiGuid,
    table: *const c_void,
    table_size: UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_MM_ALLOCATE_POOL`.
pub type EfiMmAllocatePool =
    unsafe extern "efiapi" fn(pool_type: u32, size: UIntN, buffer: *mut *mut c_void) -> EfiStatus;

/// Function pointer for `EFI_MM_FREE_POOL`.
pub type EfiMmFreePool = unsafe extern "efiapi" fn(buffer: *mut c_void) -> EfiStatus;

/// Function pointer for `EFI_MM_ALLOCATE_PAGES`.
pub type EfiMmAllocatePages = unsafe extern "efiapi" fn(
    allocate_type: u32,
    memory_type: u32,
    pages: UIntN,
    memory: *mut EfiPhysicalAddress,
) -> EfiStatus;

/// Function pointer for `EFI_MM_FREE_PAGES`.
pub type EfiMmFreePages =
    unsafe extern "efiapi" fn(memory: EfiPhysicalAddress, pages: UIntN) -> EfiStatus;

/// Function pointer for `EFI_MMI_MANAGE`.
pub type EfiMmiManage = unsafe extern "efiapi" fn(
    handler_type: *const EfiGuid,
    context: *const c_void,
    comm_buffer: *mut c_void,
    comm_buffer_size: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_MMI_HANDLER_REGISTER`.
pub type EfiMmiHandlerRegister = unsafe extern "efiapi" fn(
    handler: EfiMmiHandler,
    handler_type: *const EfiGuid,
    dispatch_handle: *mut EfiHandle,
) -> EfiStatus;

/// Function pointer for `EFI_MMI_HANDLER_UNREGISTER`.
pub type EfiMmiHandlerUnRegister =
    unsafe extern "efiapi" fn(dispatch_handle: EfiHandle) -> EfiStatus;

/// The MM System Table defined by PI 1.8 Volume 4.
#[derive(Debug)]
#[repr(C)]
pub struct EfiMmSystemTable {
    /// Table header with signature and revision.
    pub hdr: EfiTableHeader,
    /// Unicode string identifying firmware vendor.
    pub mm_firmware_vendor: *const u16,
    /// Firmware revision number.
    pub mm_firmware_revision: u32,
    /// Installs a configuration table in the MM environment.
    pub mm_install_configuration_table: EfiMmInstallConfigurationTable,
    /// MM CPU I/O services pointer.
    pub mm_cpu_io: *const c_void,
    /// Allocates memory pool pages from SMRAM.
    pub mm_allocate_pool: EfiMmAllocatePool,
    /// Frees memory pool allocated from SMRAM.
    pub mm_free_pool: EfiMmFreePool,
    /// Allocates pages in SMRAM.
    pub mm_allocate_pages: EfiMmAllocatePages,
    /// Frees pages in SMRAM.
    pub mm_free_pages: EfiMmFreePages,
    /// Executes a procedure on the specified application processor.
    pub mm_startup_this_ap: unsafe extern "efiapi" fn(
        procedure: unsafe extern "efiapi" fn(*mut c_void) -> EfiStatus,
        cpu_number: UIntN,
        proc_arguments: *mut c_void,
    ) -> EfiStatus,
    /// Number of processors currently in MM mode.
    pub currently_executing_cpu: UIntN,
    /// Total number of processors in the system.
    pub number_of_cpus: UIntN,
    /// Optional pointer to processor status flags.
    pub cpu_save_state_size: *mut UIntN,
    /// Pointer to processor save state areas.
    pub cpu_save_state: *mut *mut c_void,
    /// Number of configuration table entries.
    pub number_of_table_entries: UIntN,
    /// Pointer to configuration table entries.
    pub mm_configuration_table: *mut c_void,
    /// Installs a protocol interface in MM.
    pub mm_install_protocol_interface: unsafe extern "efiapi" fn(
        user_handle: *mut EfiHandle,
        protocol: *const EfiGuid,
        interface_type: u32,
        interface: *const c_void,
    ) -> EfiStatus,
    /// Uninstalls a protocol interface from MM.
    pub mm_uninstall_protocol_interface: unsafe extern "efiapi" fn(
        user_handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: *const c_void,
    ) -> EfiStatus,
    /// Reinstalls a protocol interface in MM.
    pub mm_reinstall_protocol_interface: unsafe extern "efiapi" fn(
        user_handle: EfiHandle,
        protocol: *const EfiGuid,
        old_interface: *const c_void,
        new_interface: *const c_void,
    ) -> EfiStatus,
    /// Registers a notification callback on MM protocol installation.
    pub mm_register_protocol_notify: unsafe extern "efiapi" fn(
        protocol: *const EfiGuid,
        function: unsafe extern "efiapi" fn(*const EfiGuid, *mut c_void, EfiHandle) -> EfiStatus,
        registration: *mut *mut c_void,
    ) -> EfiStatus,
    /// Locates handles supporting a protocol in MM.
    pub mm_locate_handle: unsafe extern "efiapi" fn(
        search_type: u32,
        protocol: *const EfiGuid,
        search_key: *const c_void,
        buffer_size: *mut UIntN,
        buffer: *mut EfiHandle,
    ) -> EfiStatus,
    /// Locates the first instance of a protocol in MM.
    pub mm_locate_protocol: unsafe extern "efiapi" fn(
        protocol: *const EfiGuid,
        registration: *const c_void,
        interface: *mut *mut c_void,
    ) -> EfiStatus,
    /// Dispatches MMI handlers for a specified root MMI source.
    pub mmi_manage: EfiMmiManage,
    /// Registers a handler for an MMI source.
    pub mmi_handler_register: EfiMmiHandlerRegister,
    /// Unregisters an MMI handler.
    pub mmi_handler_unregister: EfiMmiHandlerUnRegister,
}
