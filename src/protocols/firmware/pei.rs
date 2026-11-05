//! # Pre-EFI Initialization (PEI) services and interfaces
//!
//! Defined by the UEFI Platform Initialization (PI) Specification Volume 1:
//! Pre-EFI Initialization Core Interface.
//!
//! PEI is the execution phase responsible for initializing permanent memory,
//! describing discovered platform resources via HOBs, and dispatching DXE.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};
use crate::tables::header::EfiTableHeader;
use core::ffi::c_void;

/// PEI Services Table signature.
pub const PEI_SERVICES_SIGNATURE: u64 = 0x5652_4553_2049_4550; // "PEI SERV"

/// PEI Services specification revision 1.80.
pub const PEI_SERVICES_REVISION: u32 = (1 << 16) | 0x0050;

// EFI_PEI_PPI_DESCRIPTOR bit flags
/// Flags indicate that this descriptor contains a PPI pointer.
pub const EFI_PEI_PPI_DESCRIPTOR_PPI: u32 = 0x0000_0010;
/// Flags indicate that this descriptor contains a notify callback.
pub const EFI_PEI_PPI_DESCRIPTOR_NOTIFY_CALLBACK: u32 = 0x0000_0020;
/// Flags indicate that this descriptor contains a notify dispatch function.
pub const EFI_PEI_PPI_DESCRIPTOR_NOTIFY_DISPATCH: u32 = 0x0000_0040;
/// Flags indicate that this descriptor is the last entry in a descriptor list.
pub const EFI_PEI_PPI_DESCRIPTOR_TERMINATE_LIST: u32 = 0x8000_0000;

// Boot Mode definitions
/// Boot with full hardware configuration.
pub const BOOT_WITH_FULL_CONFIGURATION: u32 = 0x00;
/// Boot with minimal hardware configuration.
pub const BOOT_WITH_MINIMAL_CONFIGURATION: u32 = 0x01;
/// Boot assuming no hardware configuration changes.
pub const BOOT_ASSUMING_NO_CONFIGURATION_CHANGES: u32 = 0x02;
/// Boot with full configuration plus diagnostics.
pub const BOOT_WITH_FULL_CONFIGURATION_PLUS_DIAGNOSTICS: u32 = 0x03;
/// Boot with default settings.
pub const BOOT_WITH_DEFAULT_SETTINGS: u32 = 0x04;
/// S4 sleep resume boot.
pub const BOOT_ON_S4_RESUME: u32 = 0x05;
/// S5 sleep resume boot.
pub const BOOT_ON_S5_RESUME: u32 = 0x06;
/// S2 sleep resume boot.
pub const BOOT_ON_S2_RESUME: u32 = 0x10;
/// S3 sleep resume boot.
pub const BOOT_ON_S3_RESUME: u32 = 0x11;
/// Flash update boot.
pub const BOOT_ON_FLASH_UPDATE: u32 = 0x12;
/// Recovery mode boot.
pub const BOOT_IN_RECOVERY_MODE: u32 = 0x20;

/// PPI descriptor structure used to register PPIs with PEI Core.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiPeiPpiDescriptor {
    /// Flags indicating the descriptor type and list termination.
    pub flags: u32,
    /// GUID of the PPI interface.
    pub guid: *const EfiGuid,
    /// Pointer to the PPI interface structure or notify callback.
    pub ppi: *const c_void,
}

/// Notification callback function type for PPI installations.
pub type EfiPeimNotifyEntry = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    notify_descriptor: *const EfiPeiNotifyDescriptor,
    ppi: *const c_void,
) -> EfiStatus;

/// PPI notification descriptor structure.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiPeiNotifyDescriptor {
    /// Flags indicating notification mode (`NOTIFY_CALLBACK` or `NOTIFY_DISPATCH`).
    pub flags: u32,
    /// GUID of the PPI to monitor.
    pub guid: *const EfiGuid,
    /// Callback function invoked upon PPI installation.
    pub notify: EfiPeimNotifyEntry,
}

/// Function pointer for `EFI_PEI_INSTALL_PPI`.
pub type EfiPeiInstallPpi = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    ppi_list: *const EfiPeiPpiDescriptor,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_REINSTALL_PPI`.
pub type EfiPeiReinstallPpi = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    old_ppi: *const EfiPeiPpiDescriptor,
    new_ppi: *const EfiPeiPpiDescriptor,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_LOCATE_PPI`.
pub type EfiPeiLocatePpi = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    guid: *const EfiGuid,
    instance: UIntN,
    ppi_descriptor: *mut *const EfiPeiPpiDescriptor,
    ppi: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_NOTIFY_PPI`.
pub type EfiPeiNotifyPpi = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    notify_list: *const EfiPeiNotifyDescriptor,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_GET_BOOT_MODE`.
pub type EfiPeiGetBootMode = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    boot_mode: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_SET_BOOT_MODE`.
pub type EfiPeiSetBootMode = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    boot_mode: u32,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_GET_HOB_LIST`.
pub type EfiPeiGetHobList = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    hob_list: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_CREATE_HOB`.
pub type EfiPeiCreateHob = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    hob_type: u16,
    hob_length: u16,
    hob: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_FFS_FIND_NEXT_VOLUME`.
pub type EfiPeiFfsFindNextVolume = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    instance: UIntN,
    volume_handle: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_FFS_FIND_NEXT_FILE`.
pub type EfiPeiFfsFindNextFile = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    search_type: u8,
    volume_handle: *const c_void,
    file_handle: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_FFS_FIND_SECTION_DATA`.
pub type EfiPeiFfsFindSectionData = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    section_type: u8,
    file_handle: *const c_void,
    section_data: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_INSTALL_PEI_MEMORY`.
pub type EfiPeiInstallPeiMemory = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    memory_begin: EfiPhysicalAddress,
    memory_length: u64,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_ALLOCATE_PAGES`.
pub type EfiPeiAllocatePages = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    memory_type: u32,
    pages: UIntN,
    memory: *mut EfiPhysicalAddress,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_ALLOCATE_POOL`.
pub type EfiPeiAllocatePool = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    size: UIntN,
    buffer: *mut *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_REPORT_STATUS_CODE`.
pub type EfiPeiReportStatusCode = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    code_type: u32,
    value: u32,
    instance: u32,
    caller_id: *const EfiGuid,
    data: *const c_void,
) -> EfiStatus;

/// Function pointer for `EFI_PEI_RESET_SYSTEM`.
pub type EfiPeiResetSystem =
    unsafe extern "efiapi" fn(pei_services: *const *const EfiPeiServices) -> EfiStatus;

/// Function pointer for `EFI_PEI_RESET2_SYSTEM`.
pub type EfiPeiReset2System = unsafe extern "efiapi" fn(
    reset_type: u32,
    reset_status: EfiStatus,
    data_size: UIntN,
    reset_data: *const c_void,
) -> EfiStatus;

/// The PEI Services Table structure defined by PI 1.8 Volume 1.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPeiServices {
    /// Table header with signature and revision.
    pub hdr: EfiTableHeader,
    /// Installs a list of PPIs.
    pub install_ppi: EfiPeiInstallPpi,
    /// Replaces an existing PPI descriptor.
    pub reinstall_ppi: EfiPeiReinstallPpi,
    /// Locates an installed PPI interface.
    pub locate_ppi: EfiPeiLocatePpi,
    /// Registers a PPI notification descriptor.
    pub notify_ppi: EfiPeiNotifyPpi,
    /// Gets the current boot mode.
    pub get_boot_mode: EfiPeiGetBootMode,
    /// Sets the current boot mode.
    pub set_boot_mode: EfiPeiSetBootMode,
    /// Retrieves the pointer to the HOB list.
    pub get_hob_list: EfiPeiGetHobList,
    /// Builds a new HOB entry.
    pub create_hob: EfiPeiCreateHob,
    /// Finds the next firmware volume.
    pub ffs_find_next_volume: EfiPeiFfsFindNextVolume,
    /// Finds the next file in a firmware volume.
    pub ffs_find_next_file: EfiPeiFfsFindNextFile,
    /// Finds section data within a firmware file.
    pub ffs_find_section_data: EfiPeiFfsFindSectionData,
    /// Declares permanent memory to PEI Core.
    pub install_pei_memory: EfiPeiInstallPeiMemory,
    /// Allocates memory pages in permanent memory.
    pub allocate_pages: EfiPeiAllocatePages,
    /// Allocates memory pool from HOB or permanent memory.
    pub allocate_pool: EfiPeiAllocatePool,
    /// Copies memory buffer.
    pub copy_mem:
        unsafe extern "efiapi" fn(destination: *mut c_void, source: *const c_void, length: UIntN),
    /// Fills memory buffer with a constant byte value.
    pub set_mem: unsafe extern "efiapi" fn(buffer: *mut c_void, size: UIntN, value: u8),
    /// Reports architectural status code.
    pub report_status_code: EfiPeiReportStatusCode,
    /// Resets the platform.
    pub reset_system: EfiPeiResetSystem,
    /// CPU I/O primitive pointer.
    pub cpu_io: *const c_void,
    /// PCI Configuration primitive pointer.
    pub pci_cfg: *const c_void,
    /// Searches for a file by GUID name.
    pub ffs_find_file_by_name: unsafe extern "efiapi" fn(
        file_name: *const EfiGuid,
        volume_handle: *const c_void,
        file_handle: *mut *mut c_void,
    ) -> EfiStatus,
    /// Gets file information.
    pub ffs_get_file_info:
        unsafe extern "efiapi" fn(file_handle: *const c_void, file_info: *mut c_void) -> EfiStatus,
    /// Gets volume information.
    pub ffs_get_volume_info: unsafe extern "efiapi" fn(
        volume_handle: *const c_void,
        volume_info: *mut c_void,
    ) -> EfiStatus,
    /// Registers a PEIM for execution from permanent memory (shadowing).
    pub register_for_shadow: unsafe extern "efiapi" fn(file_handle: EfiHandle) -> EfiStatus,
    /// Finds section data with authentication support (PI 1.3+).
    pub ffs_find_section_data3: unsafe extern "efiapi" fn(
        pei_services: *const *const EfiPeiServices,
        section_type: u8,
        section_instance: UIntN,
        file_handle: *const c_void,
        section_data: *mut *mut c_void,
        authentication_status: *mut u32,
    ) -> EfiStatus,
    /// PI 1.3+ `ResetSystem2` function pointer.
    pub reset_system2: EfiPeiReset2System,
}

// Core PPI GUIDs
/// Read-only variable 2 PPI GUID.
pub const EFI_PEI_READ_ONLY_VARIABLE2_PPI_GUID: EfiGuid = EfiGuid::new(
    0x2AB8_68E0,
    0x0749,
    0x4462,
    [0xA6, 0x49, 0x70, 0xA6, 0x31, 0xE1, 0x1A, 0xE4],
);

/// End of PEI phase PPI GUID.
pub const EFI_PEI_END_OF_PEI_PHASE_PPI_GUID: EfiGuid = EfiGuid::new(
    0x605E_A650,
    0xC623,
    0x44AD,
    [0x8B, 0x25, 0x4A, 0x52, 0x09, 0xFB, 0x02, 0x29],
);

/// Stall PPI GUID.
pub const EFI_PEI_STALL_PPI_GUID: EfiGuid = EfiGuid::new(
    0x33CB_9FF2,
    0xB31C,
    0x4E30,
    [0xBA, 0xF9, 0x36, 0x67, 0x6B, 0x0F, 0x6E, 0x37],
);

/// Temporary RAM Support PPI GUID.
pub const EFI_PEI_TEMPORARY_RAM_SUPPORT_PPI_GUID: EfiGuid = EfiGuid::new(
    0xDABF_30A4,
    0x450A,
    0x496E,
    [0x99, 0x82, 0x51, 0xD4, 0x76, 0xF9, 0x1E, 0xCD],
);

/// Temporary RAM Done PPI GUID.
pub const EFI_PEI_TEMPORARY_RAM_DONE_PPI_GUID: EfiGuid = EfiGuid::new(
    0xCE3E_C3F8,
    0x7A1E,
    0x4486,
    [0xA8, 0x07, 0xBE, 0x43, 0x80, 0x48, 0x31, 0xAA],
);

/// Security2 PPI GUID.
pub const EFI_PEI_SECURITY2_PPI_GUID: EfiGuid = EfiGuid::new(
    0x9CA6_3665,
    0x5466,
    0x4D5E,
    [0x8F, 0x86, 0x98, 0x17, 0x69, 0x75, 0x81, 0x6B],
);

/// Decompress PPI GUID.
pub const EFI_PEI_DECOMPRESS_PPI_GUID: EfiGuid = EfiGuid::new(
    0x1A36_E4E7,
    0xFAB6,
    0x476A,
    [0x8E, 0x33, 0x40, 0xB0, 0xFB, 0x89, 0x65, 0x35],
);

/// Read-only variable 2 PPI interface structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPeiReadOnlyVariable2Ppi {
    /// Gets a firmware variable in PEI phase.
    pub get_variable: unsafe extern "efiapi" fn(
        this: *const EfiPeiReadOnlyVariable2Ppi,
        variable_name: *const u16,
        vendor_guid: *const EfiGuid,
        attributes: *mut u32,
        data_size: *mut UIntN,
        data: *mut c_void,
    ) -> EfiStatus,
    /// Enumerates next variable in PEI phase.
    pub next_variable_name: unsafe extern "efiapi" fn(
        this: *const EfiPeiReadOnlyVariable2Ppi,
        variable_name_size: *mut UIntN,
        variable_name: *mut u16,
        vendor_guid: *mut EfiGuid,
    ) -> EfiStatus,
}

/// Stall PPI interface structure for calibrated delays in PEI.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPeiStallPpi {
    /// Delays execution for the specified number of microseconds.
    pub stall: unsafe extern "efiapi" fn(
        pei_services: *const *const EfiPeiServices,
        this: *const EfiPeiStallPpi,
        microseconds: UIntN,
    ) -> EfiStatus,
}

/// Decompress PPI interface structure.
#[derive(Debug)]
#[repr(C)]
pub struct EfiPeiDecompressPpi {
    /// Decompresses section or image data in PEI.
    pub decompress: unsafe extern "efiapi" fn(
        this: *const EfiPeiDecompressPpi,
        source: *const c_void,
        source_size: u32,
        destination: *mut c_void,
        destination_size: u32,
    ) -> EfiStatus,
}
