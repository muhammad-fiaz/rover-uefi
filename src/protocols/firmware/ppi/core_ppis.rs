//! # PEI Core and Architectural PPIs
//!
//! Defined in UEFI Platform Initialization (PI) Specification Volume 1:
//! Pre-EFI Initialization Core Interface.
//!
//! PPIs (PEIM-to-PEIM Interfaces) provide modular communication between
//! early pre-memory and post-memory initialization modules.

use core::ffi::c_void;

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::firmware::pei::EfiPeiServices;
use crate::protocols::graphics::EfiGraphicsOutputProtocolMode;

/// Memory Discovered / Permanent Memory Installed PPI GUID.
pub const EFI_PEI_PERMANENT_MEMORY_INSTALLED_PPI_GUID: EfiGuid = EfiGuid::new(
    0xF894_643D,
    0xC449,
    0x42D1,
    [0x8E, 0xA8, 0x85, 0xBD, 0xD8, 0xC6, 0x5B, 0xDE],
);

/// Master Boot Mode PPI GUID.
pub const EFI_PEI_MASTER_BOOT_MODE_PEIM_PPI_GUID: EfiGuid = EfiGuid::new(
    0x7408_D748,
    0xFC8C,
    0x4EE6,
    [0x92, 0x88, 0xC4, 0xBE, 0xC0, 0x92, 0xA4, 0x10],
);

/// Boot In Recovery Mode PPI GUID.
pub const EFI_PEI_BOOT_IN_RECOVERY_MODE_PPI_GUID: EfiGuid = EfiGuid::new(
    0x17EE_496A,
    0xD8E4,
    0x4B9A,
    [0x94, 0xD1, 0xCE, 0x82, 0x72, 0x30, 0x08, 0x50],
);

/// DXE Initial Program Load (IPL) PPI GUID.
pub const EFI_DXE_IPL_PPI_GUID: EfiGuid = EfiGuid::new(
    0x0AE8_CE5D,
    0xE448,
    0x4437,
    [0xA8, 0xD7, 0xEB, 0xF5, 0xF1, 0x94, 0xF7, 0x31],
);

/// PEI CPU I/O PPI GUID.
pub const EFI_PEI_CPU_IO_PPI_INSTALLED_GUID: EfiGuid = EfiGuid::new(
    0xE6AF_1F7B,
    0xFC3F,
    0x46DA,
    [0xA8, 0x28, 0xA3, 0xB4, 0x57, 0xA4, 0x42, 0x82],
);

/// PEI Firmware Volume Info PPI GUID.
pub const EFI_PEI_FIRMWARE_VOLUME_INFO_PPI_GUID: EfiGuid = EfiGuid::new(
    0x49ED_B1C1,
    0xBF21,
    0x4761,
    [0xBB, 0x12, 0xEB, 0x00, 0x31, 0xAA, 0xBB, 0x39],
);

/// PEI Firmware Volume Info 2 PPI GUID.
pub const EFI_PEI_FIRMWARE_VOLUME_INFO2_PPI_GUID: EfiGuid = EfiGuid::new(
    0xEA7C_A24B,
    0xDED5,
    0x4DAD,
    [0xA3, 0x89, 0xBF, 0x82, 0x7E, 0x8F, 0x9B, 0x38],
);

/// PEI Recovery Block I/O PPI GUID.
pub const EFI_PEI_RECOVERY_BLOCK_IO_PPI_GUID: EfiGuid = EfiGuid::new(
    0x695D_8AA1,
    0x42EE,
    0x4C46,
    [0x80, 0x5C, 0x6E, 0xA6, 0xBC, 0xE7, 0x99, 0xE3],
);

/// PEI Graphics PPI GUID.
pub const EFI_PEI_GRAPHICS_PPI_GUID: EfiGuid = EfiGuid::new(
    0x6ECD_1463,
    0x4A4A,
    0x461B,
    [0xAF, 0x5F, 0x5A, 0x33, 0xE3, 0xB2, 0x16, 0x2B],
);

/// PEI MP Services PPI GUID.
pub const EFI_PEI_MP_SERVICES_PPI_GUID: EfiGuid = EfiGuid::new(
    0xEE16_160A,
    0xE8BE,
    0x47A6,
    [0x82, 0x0A, 0xC6, 0x90, 0x0D, 0xB0, 0x25, 0x0A],
);

/// PEI MP Services 2 PPI GUID (PI 1.8+).
pub const EFI_PEI_MP_SERVICES2_PPI_GUID: EfiGuid = EfiGuid::new(
    0x5CB9_CB3D,
    0x31A4,
    0x480C,
    [0x94, 0x98, 0x29, 0xD2, 0x69, 0xBA, 0xCF, 0xBA],
);

// --- DXE IPL PPI ---

/// Entry point to transfer control from PEI to the DXE Foundation.
pub type EfiDxeIplEntry = unsafe extern "efiapi" fn(
    this: *const EfiDxeIplPpi,
    pei_services: *const *const EfiPeiServices,
    hob_list: *const c_void,
) -> EfiStatus;

/// DXE Initial Program Load (IPL) PPI interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiDxeIplPpi {
    /// Dispatches the DXE foundation or invokes recovery/S3 modules.
    pub entry: EfiDxeIplEntry,
}

// --- PEI CPU I/O PPI ---

/// Access width for PEI CPU I/O operations.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiPeiCpuIoPpiWidth {
    /// 8-bit memory/IO access.
    Uint8 = 0,
    /// 16-bit memory/IO access.
    Uint16 = 1,
    /// 32-bit memory/IO access.
    Uint32 = 2,
    /// 64-bit memory/IO access.
    Uint64 = 3,
    /// FIFO 8-bit access.
    FifoUint8 = 4,
    /// FIFO 16-bit access.
    FifoUint16 = 5,
    /// FIFO 32-bit access.
    FifoUint32 = 6,
    /// FIFO 64-bit access.
    FifoUint64 = 7,
    /// Fill 8-bit access.
    FillUint8 = 8,
    /// Fill 16-bit access.
    FillUint16 = 9,
    /// Fill 32-bit access.
    FillUint32 = 10,
    /// Fill 64-bit access.
    FillUint64 = 11,
}

/// Generic CPU I/O access function pointer.
pub type EfiPeiCpuIoPpiAccess = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    this: *const EfiPeiCpuIoPpi,
    width: EfiPeiCpuIoPpiWidth,
    address: u64,
    count: usize,
    buffer: *mut c_void,
) -> EfiStatus;

/// CPU I/O read function pointer.
pub type EfiPeiCpuIoReadFn<T> = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    this: *const EfiPeiCpuIoPpi,
    address: u64,
) -> T;

/// CPU I/O write function pointer.
pub type EfiPeiCpuIoWriteFn<T> = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    this: *const EfiPeiCpuIoPpi,
    address: u64,
    data: T,
);

/// PEI CPU I/O PPI interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiCpuIoPpi {
    /// General memory access services.
    pub mem: EfiPeiCpuIoPpiAccess,
    /// General I/O port access services.
    pub io: EfiPeiCpuIoPpiAccess,

    /// 8-bit I/O read.
    pub io_read8: EfiPeiCpuIoReadFn<u8>,
    /// 16-bit I/O read.
    pub io_read16: EfiPeiCpuIoReadFn<u16>,
    /// 32-bit I/O read.
    pub io_read32: EfiPeiCpuIoReadFn<u32>,
    /// 64-bit I/O read.
    pub io_read64: EfiPeiCpuIoReadFn<u64>,

    /// 8-bit I/O write.
    pub io_write8: EfiPeiCpuIoWriteFn<u8>,
    /// 16-bit I/O write.
    pub io_write16: EfiPeiCpuIoWriteFn<u16>,
    /// 32-bit I/O write.
    pub io_write32: EfiPeiCpuIoWriteFn<u32>,
    /// 64-bit I/O write.
    pub io_write64: EfiPeiCpuIoWriteFn<u64>,

    /// 8-bit Memory read.
    pub mem_read8: EfiPeiCpuIoReadFn<u8>,
    /// 16-bit Memory read.
    pub mem_read16: EfiPeiCpuIoReadFn<u16>,
    /// 32-bit Memory read.
    pub mem_read32: EfiPeiCpuIoReadFn<u32>,
    /// 64-bit Memory read.
    pub mem_read64: EfiPeiCpuIoReadFn<u64>,

    /// 8-bit Memory write.
    pub mem_write8: EfiPeiCpuIoWriteFn<u8>,
    /// 16-bit Memory write.
    pub mem_write16: EfiPeiCpuIoWriteFn<u16>,
    /// 32-bit Memory write.
    pub mem_write32: EfiPeiCpuIoWriteFn<u32>,
    /// 64-bit Memory write.
    pub mem_write64: EfiPeiCpuIoWriteFn<u64>,
}

// --- PEI Firmware Volume Info PPIs ---

/// PEI Firmware Volume Info PPI.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiFirmwareVolumeInfoPpi {
    /// Format identifier of the firmware volume.
    pub fv_format: EfiGuid,
    /// Pointer to buffer allowing processing of the firmware volume.
    pub fv_info: *const c_void,
    /// Size of the firmware volume data.
    pub fv_info_size: u32,
    /// Optional parent firmware volume name.
    pub parent_fv_name: *const EfiGuid,
    /// Optional parent firmware volume file name.
    pub parent_file_name: *const EfiGuid,
}

/// PEI Firmware Volume Info 2 PPI.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiFirmwareVolumeInfo2Ppi {
    /// Format identifier of the firmware volume.
    pub fv_format: EfiGuid,
    /// Pointer to buffer allowing processing of the firmware volume.
    pub fv_info: *const c_void,
    /// Size of the firmware volume data.
    pub fv_info_size: u32,
    /// Optional parent firmware volume name.
    pub parent_fv_name: *const EfiGuid,
    /// Optional parent firmware volume file name.
    pub parent_file_name: *const EfiGuid,
    /// Authentication status of the firmware volume.
    pub authentication_status: u32,
}

// --- PEI Graphics PPI ---

/// Initializes graphics in PEI.
pub type EfiPeiGraphicsInit =
    unsafe extern "efiapi" fn(graphics_policy_ptr: *const c_void) -> EfiStatus;

/// Gets current graphics mode in PEI.
pub type EfiPeiGraphicsGetMode =
    unsafe extern "efiapi" fn(mode: *mut EfiGraphicsOutputProtocolMode) -> EfiStatus;

/// PEI Graphics PPI interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiGraphicsPpi {
    /// Initializes graphics controller hardware in PEI.
    pub graphics_ppi_init: EfiPeiGraphicsInit,
    /// Retrieves active framebuffer and display mode parameters.
    pub graphics_ppi_get_mode: EfiPeiGraphicsGetMode,
}

// --- PEI MP Services PPI ---

/// Multi-processor AP procedure callback.
pub type EfiPeiApProcedure =
    unsafe extern "efiapi" fn(pei_services: *const *const EfiPeiServices, buffer: *mut c_void);

/// Get CPU count in PEI.
pub type EfiPeiMpServicesGetNumberOfProcessors = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    this: *mut EfiPeiMpServicesPpi,
    number_of_processors: *mut usize,
    number_of_enabled_processors: *mut usize,
) -> EfiStatus;

/// Startup all APs in PEI.
pub type EfiPeiMpServicesStartupAllAps = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    this: *mut EfiPeiMpServicesPpi,
    procedure: EfiPeiApProcedure,
    single_thread: bool,
    timeout_in_microseconds: usize,
    procedure_argument: *mut c_void,
) -> EfiStatus;

/// Query executing processor index in PEI.
pub type EfiPeiMpServicesWhoAmI = unsafe extern "efiapi" fn(
    pei_services: *const *const EfiPeiServices,
    this: *mut EfiPeiMpServicesPpi,
    processor_number: *mut usize,
) -> EfiStatus;

/// PEI Multi-Processor Services PPI interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiMpServicesPpi {
    /// Retrieves processor count and enabled processor count.
    pub get_number_of_processors: EfiPeiMpServicesGetNumberOfProcessors,
    /// Reserved function pointer for processor info lookup.
    pub get_processor_info: unsafe extern "efiapi" fn(
        *const *const EfiPeiServices,
        *mut EfiPeiMpServicesPpi,
        usize,
        *mut c_void,
    ) -> EfiStatus,
    /// Executes a routine across all application processors in PEI.
    pub startup_all_aps: EfiPeiMpServicesStartupAllAps,
    /// Executes a routine on a designated application processor.
    pub startup_this_ap: unsafe extern "efiapi" fn(
        *const *const EfiPeiServices,
        *mut EfiPeiMpServicesPpi,
        EfiPeiApProcedure,
        usize,
        usize,
        *mut c_void,
    ) -> EfiStatus,
    /// Switches the Bootstrap Processor (BSP) in PEI.
    pub switch_bsp: unsafe extern "efiapi" fn(
        *const *const EfiPeiServices,
        *mut EfiPeiMpServicesPpi,
        usize,
        bool,
    ) -> EfiStatus,
    /// Enables or disables an application processor in PEI.
    pub enable_disable_ap: unsafe extern "efiapi" fn(
        *const *const EfiPeiServices,
        *mut EfiPeiMpServicesPpi,
        usize,
        bool,
        *mut u32,
    ) -> EfiStatus,
    /// Returns the calling logical processor number.
    pub who_am_i: EfiPeiMpServicesWhoAmI,
}

// --- PEI MP Services 2 PPI (PI 1.8+) ---

/// Multi-processor AP procedure callback for MP Services 2 (PI 1.8+).
pub type EfiPeiMpServices2Procedure = unsafe extern "efiapi" fn(buffer: *mut c_void);

/// Get CPU count in PEI (PI 1.8+).
pub type EfiPeiMpServices2GetNumberOfProcessors = unsafe extern "efiapi" fn(
    this: *const EfiPeiMpServices2Ppi,
    number_of_processors: *mut usize,
    number_of_enabled_processors: *mut usize,
) -> EfiStatus;

/// Get processor information in PEI (PI 1.8+).
pub type EfiPeiMpServices2GetProcessorInfo = unsafe extern "efiapi" fn(
    this: *const EfiPeiMpServices2Ppi,
    processor_number: usize,
    processor_info_buffer: *mut crate::protocols::core::mp_services::EfiProcessorInformation,
) -> EfiStatus;

/// Startup all APs in PEI (PI 1.8+).
pub type EfiPeiMpServices2StartupAllAps = unsafe extern "efiapi" fn(
    this: *const EfiPeiMpServices2Ppi,
    procedure: EfiPeiMpServices2Procedure,
    single_thread: bool,
    timeout_in_microseconds: usize,
    procedure_argument: *mut c_void,
) -> EfiStatus;

/// Startup a specific AP in PEI (PI 1.8+).
pub type EfiPeiMpServices2StartupThisAp = unsafe extern "efiapi" fn(
    this: *const EfiPeiMpServices2Ppi,
    procedure: EfiPeiMpServices2Procedure,
    processor_number: usize,
    timeout_in_microseconds: usize,
    procedure_argument: *mut c_void,
) -> EfiStatus;

/// Switches the Bootstrap Processor (BSP) in PEI (PI 1.8+).
pub type EfiPeiMpServices2SwitchBsp = unsafe extern "efiapi" fn(
    this: *const EfiPeiMpServices2Ppi,
    processor_number: usize,
    enable_old_bsp: bool,
) -> EfiStatus;

/// Enables or disables an application processor in PEI (PI 1.8+).
pub type EfiPeiMpServices2EnableDisableAp = unsafe extern "efiapi" fn(
    this: *const EfiPeiMpServices2Ppi,
    processor_number: usize,
    enable_ap: bool,
    health_flag: *mut u32,
) -> EfiStatus;

/// Query executing processor index in PEI (PI 1.8+).
pub type EfiPeiMpServices2WhoAmI = unsafe extern "efiapi" fn(
    this: *const EfiPeiMpServices2Ppi,
    processor_number: *mut usize,
) -> EfiStatus;

/// PEI Multi-Processor Services 2 PPI interface (PI 1.8+).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiMpServices2Ppi {
    /// Retrieves processor count and enabled processor count.
    pub get_number_of_processors: EfiPeiMpServices2GetNumberOfProcessors,
    /// Retrieves detailed processor information.
    pub get_processor_info: EfiPeiMpServices2GetProcessorInfo,
    /// Executes a routine across all application processors in PEI.
    pub startup_all_aps: EfiPeiMpServices2StartupAllAps,
    /// Executes a routine on a designated application processor.
    pub startup_this_ap: EfiPeiMpServices2StartupThisAp,
    /// Switches the Bootstrap Processor (BSP) in PEI.
    pub switch_bsp: EfiPeiMpServices2SwitchBsp,
    /// Enables or disables an application processor in PEI.
    pub enable_disable_ap: EfiPeiMpServices2EnableDisableAp,
    /// Returns the calling logical processor number.
    pub who_am_i: EfiPeiMpServices2WhoAmI,
}

// --- PEI Reset & Reset 2 PPIs ---

/// PEI Reset PPI GUID: `{3dae3825-4215-4a86-82dd-94244350ec75}`.
pub const EFI_PEI_RESET_PPI_GUID: EfiGuid = EfiGuid::new(
    0x3DAE_3825,
    0x4215,
    0x4A86,
    [0x82, 0xDD, 0x94, 0x24, 0x43, 0x50, 0xEC, 0x75],
);

/// PEI Reset 2 PPI GUID: `{600e152e-4478-4cdf-a0b1-3d85418c1ab7}`.
pub const EFI_PEI_RESET2_PPI_GUID: EfiGuid = EfiGuid::new(
    0x600E_152E,
    0x4478,
    0x4CDF,
    [0xA0, 0xB1, 0x3D, 0x85, 0x41, 0x8C, 0x1A, 0xB7],
);

/// Function pointer to execute a platform reset in PEI.
pub type EfiPeiResetSystem2 = unsafe extern "efiapi" fn(
    reset_type: crate::base::ResetType,
    reset_status: EfiStatus,
    data_size: usize,
    reset_data: *const c_void,
) -> !;

/// PEI Reset 2 PPI interface table.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiReset2Ppi {
    /// Resets the platform.
    pub reset_system: EfiPeiResetSystem2,
}

// --- PEI S3 Resume 2 PPI ---

/// PEI S3 Resume 2 PPI GUID: `{6d582dbc-db85-4514-8fcc-5adf6227b147}`.
pub const EFI_PEI_S3_RESUME2_PPI_GUID: EfiGuid = EfiGuid::new(
    0x6D58_2DBC,
    0xDB85,
    0x4514,
    [0x8F, 0xCC, 0x5A, 0xDF, 0x62, 0x27, 0xB1, 0x47],
);

/// Function pointer to execute S3 restore boot script in PEI.
pub type EfiPeiS3RestoreConfig2 =
    unsafe extern "efiapi" fn(this: *const EfiPeiS3Resume2Ppi) -> EfiStatus;

/// PEI S3 Resume 2 PPI interface table.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiPeiS3Resume2Ppi {
    /// Restores platform configuration by executing the S3 boot script.
    pub s3_restore_config2: EfiPeiS3RestoreConfig2,
}
