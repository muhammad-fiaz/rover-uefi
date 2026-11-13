//! # Firmware formats and protocols
//!
//! Typed parsing for firmware image and volume formats and PI protocols.
//! Parsers are bounded and borrow from caller supplied slices.

pub mod arch_protocols;
pub mod capsule_arch;
pub mod decompress;
pub mod depex;
pub mod dt_fixup;
pub mod dxe_services;
pub mod esrt;
pub mod firmvol;
pub mod fmp;
pub mod form_browser_ex;
pub mod fsp;
pub mod ftw;
pub mod hii;
pub mod hob;
pub mod ifr;
pub mod lock_box;
pub mod memory_accept;
pub mod memory_attribute;
pub mod memory_test;
pub mod mm_base;
pub mod mm_communication;
pub mod mm_system_table;
pub mod pecoff;
pub mod pei;
pub mod ppi;
pub mod prm;
pub mod progress;
pub mod sections;
pub mod smm_access2;
pub mod smm_control2;
pub mod smm_cpu;
pub mod smm_events;
pub mod swap_address_range;
pub mod universal_payload;

pub use arch_protocols::{
    EFI_CPU_ARCH_PROTOCOL_GUID, EFI_METRONOME_ARCH_PROTOCOL_GUID,
    EFI_MONOTONIC_COUNTER_ARCH_PROTOCOL_GUID, EFI_REAL_TIME_CLOCK_ARCH_PROTOCOL_GUID,
    EFI_RESET_ARCH_PROTOCOL_GUID, EFI_RUNTIME_ARCH_PROTOCOL_GUID, EFI_SECURITY_ARCH_PROTOCOL_GUID,
    EFI_SECURITY2_ARCH_PROTOCOL_GUID, EFI_TIMER_ARCH_PROTOCOL_GUID,
    EFI_VARIABLE_ARCH_PROTOCOL_GUID, EFI_VARIABLE_WRITE_ARCH_PROTOCOL_GUID,
    EFI_WATCHDOG_TIMER_ARCH_PROTOCOL_GUID, EfiCpuArchProtocol, EfiCpuFlushType, EfiCpuInitType,
    EfiMetronomeArchProtocol, EfiSecurity2ArchProtocol, EfiSecurityArchProtocol,
    EfiTimerArchProtocol, EfiWatchdogTimerArchProtocol,
};
pub use capsule_arch::EFI_CAPSULE_ARCH_PROTOCOL_GUID;
pub use decompress::{
    DecompressInfo, EFI_DECOMPRESS_PROTOCOL_GUID, EfiDecompress, EfiDecompressGetInfo,
    EfiDecompressProtocol, UEFI_DECOMPRESS_SCRATCH_SIZE, decompress_uefi, get_decompress_info,
};
pub use dt_fixup::{
    DT_FIXUP_PROTOCOL_GUID, DT_FIXUP_PROTOCOL_REVISION, DeviceTreeFixup, DtFixup, DtFixupProtocol,
};
pub use dxe_services::{
    DXE_SERVICES_REVISION, DXE_SERVICES_SIGNATURE, DXE_SERVICES_TABLE_GUID, EfiAddIoSpace,
    EfiAddMemorySpace, EfiAllocateIoSpace, EfiAllocateMemorySpace, EfiDxeDispatch, EfiDxeSchedule,
    EfiDxeServices, EfiDxeTrust, EfiFreeIoSpace, EfiFreeMemorySpace, EfiGcdAllocateType,
    EfiGcdIoSpaceDescriptor, EfiGcdIoType, EfiGcdMemorySpaceDescriptor, EfiGcdMemoryType,
    EfiGetIoSpaceDescriptor, EfiGetIoSpaceMap, EfiGetMemorySpaceDescriptor, EfiGetMemorySpaceMap,
    EfiProcessFirmwareVolume, EfiRemoveIoSpace, EfiRemoveMemorySpace, EfiSetMemorySpaceAttributes,
    EfiSetMemorySpaceCapabilities,
};
pub use esrt::{
    ESRT_FW_TYPE_DEVICEFIRMWARE, ESRT_FW_TYPE_SYSTEMFIRMWARE, ESRT_FW_TYPE_UEFIDRIVER,
    ESRT_FW_TYPE_UNKNOWN, ESRT_MANAGEMENT_PROTOCOL_GUID, EfiSystemResourceEntry,
    EfiSystemResourceTable, EsrtGetEntry, EsrtLockRepository, EsrtManagement,
    EsrtManagementProtocol, EsrtRegisterEntry, EsrtSyncFmp, EsrtUnregisterEntry, EsrtUpdateEntry,
    LAST_ATTEMPT_STATUS_ERROR_AUTH_ERROR, LAST_ATTEMPT_STATUS_ERROR_INCORRECT_VERSION,
    LAST_ATTEMPT_STATUS_ERROR_INSUFFICIENT_RESOURCES, LAST_ATTEMPT_STATUS_ERROR_INVALID_FORMAT,
    LAST_ATTEMPT_STATUS_ERROR_PWR_EVT_AC, LAST_ATTEMPT_STATUS_ERROR_PWR_EVT_BATT,
    LAST_ATTEMPT_STATUS_ERROR_UNSATISFIED_DEPENDENCIES, LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL,
    LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL_VENDOR_RANGE_MAX,
    LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL_VENDOR_RANGE_MIN, LAST_ATTEMPT_STATUS_SUCCESS,
};
pub use form_browser_ex::{
    BROWSER_ACTION_DEFAULT, BROWSER_ACTION_DISCARD, BROWSER_ACTION_EXIT, BROWSER_ACTION_GOTO,
    BROWSER_ACTION_RESET, BROWSER_ACTION_SUBMIT, BROWSER_ACTION_UNREGISTER,
    BROWSER_EXTENSION2_VERSION_1, BROWSER_EXTENSION2_VERSION_1_1, BrowserExecuteAction,
    BrowserIsDataModified, BrowserIsResetRequired, BrowserRegisterExitHandler,
    BrowserRegisterHotKey, BrowserSetScope, BrowserSettingScope, EfiFormBrowserExtension2Protocol,
    EfiFormBrowserExtensionProtocol, FORM_BROWSER_EXTENSION_PROTOCOL_GUID,
    FORM_BROWSER_EXTENSION2_PROTOCOL_GUID, FormBrowserExtension, FormBrowserExtension2,
};
pub use fsp::{
    FSP_IA32, FSP_INFO_HEADER_OFFSET, FSP_INFO_HEADER_SIGNATURE, FSP_X64, FspInfoHeader,
    FspMemoryInit, FspNotifyParams, FspNotifyPhase, FspPhase, FspSiliconInit, FspTempRamExit,
    FspTempRamInit, IMAGE_ATTRIBUTE_64BIT_MODE_SUPPORT, IMAGE_ATTRIBUTE_DISPATCH_MODE_SUPPORT,
    IMAGE_ATTRIBUTE_GRAPHICS_SUPPORT,
};
pub use ftw::{
    EfiFaultTolerantWriteAbort, EfiFaultTolerantWriteAllocate, EfiFaultTolerantWriteGetLastWrite,
    EfiFaultTolerantWriteGetMaxBlockSize, EfiFaultTolerantWriteProtocol,
    EfiFaultTolerantWriteRestart, EfiFaultTolerantWriteWrite, FAULT_TOLERANT_WRITE_PROTOCOL_GUID,
    FAULT_TOLERANT_WRITE_SMM_PROTOCOL_GUID, FaultTolerantWrite,
};
pub use lock_box::{
    LOCK_BOX_ATTRIBUTE_RESTORE_IN_PLACE, LOCK_BOX_ATTRIBUTE_RESTORE_IN_S3_ONLY,
    LOCK_BOX_PROTOCOL_GUID, LockBoxDescriptor, SMM_LOCK_BOX_COMMAND_RESTORE,
    SMM_LOCK_BOX_COMMAND_RESTORE_ALL_IN_PLACE, SMM_LOCK_BOX_COMMAND_SAVE,
    SMM_LOCK_BOX_COMMAND_SET_ATTRIBUTES, SMM_LOCK_BOX_COMMAND_UPDATE,
    SMM_LOCK_BOX_COMMUNICATION_GUID, SmmLockBoxParameterHeader, SmmLockBoxParameterRestore,
    SmmLockBoxParameterRestoreAllInPlace, SmmLockBoxParameterSave,
    SmmLockBoxParameterSetAttributes, SmmLockBoxParameterUpdate,
};
pub use memory_accept::{
    AcceptMemory, MEMORY_ACCEPT_PROTOCOL_GUID, MemoryAccept, MemoryAcceptProtocol,
};
pub use memory_attribute::{
    EFI_MEMORY_ATTRIBUTE_PROTOCOL_GUID, EfiClearMemoryAttributes, EfiGetMemoryAttributes,
    EfiMemoryAttributeProtocol, EfiSetMemoryAttributes, MemoryAttribute,
    SMM_MEMORY_ATTRIBUTE_PROTOCOL_GUID, SmmClearMemoryAttributes, SmmGetMemoryAttributes,
    SmmMemoryAttribute, SmmMemoryAttributeProtocol, SmmSetMemoryAttributes,
    memory_attribute as memory_attr_flags,
};
pub use memory_test::{
    EfiGenericMemoryTestProtocol, EfiInitializeMemoryTest, EfiMemoryTestFinished,
    EfiPerformMemoryTest, GENERIC_MEMORY_TEST_PROTOCOL_GUID, GenericMemoryTest, MemoryTestLevel,
    MemoryTestProgress,
};
pub use mm_base::{
    EfiMmBaseProtocol, EfiMmGetMmstLocation, EfiMmInsideOut, EfiSmmBase2Protocol,
    MM_BASE_PROTOCOL_GUID, MmBaseProtocol, SMM_BASE2_PROTOCOL_GUID,
};
pub use mm_communication::{
    EfiMmCommunicate, EfiMmCommunicate2, EfiMmCommunicateHeader, EfiMmCommunication2Protocol,
    EfiMmCommunicationProtocol, MM_COMMUNICATION_PROTOCOL_GUID, MM_COMMUNICATION2_PROTOCOL_GUID,
    MmCommunication, MmCommunication2,
};
pub use mm_system_table::{
    EFI_MM_SYSTEM_TABLE_GUID, EFI_SMM_SYSTEM_TABLE2_GUID, EfiMmAllocatePages, EfiMmAllocatePool,
    EfiMmFreePages, EfiMmFreePool, EfiMmInstallConfigurationTable, EfiMmSystemTable, EfiMmiHandler,
    EfiMmiHandlerRegister, EfiMmiHandlerUnRegister, EfiMmiManage, MM_SYSTEM_TABLE_REVISION,
    MM_SYSTEM_TABLE_SIGNATURE,
};
pub use pei::{
    BOOT_ASSUMING_NO_CONFIGURATION_CHANGES, BOOT_IN_RECOVERY_MODE, BOOT_ON_FLASH_UPDATE,
    BOOT_ON_S2_RESUME, BOOT_ON_S3_RESUME, BOOT_ON_S4_RESUME, BOOT_ON_S5_RESUME,
    BOOT_WITH_DEFAULT_SETTINGS, BOOT_WITH_FULL_CONFIGURATION,
    BOOT_WITH_FULL_CONFIGURATION_PLUS_DIAGNOSTICS, BOOT_WITH_MINIMAL_CONFIGURATION,
    EFI_PEI_DECOMPRESS_PPI_GUID, EFI_PEI_END_OF_PEI_PHASE_PPI_GUID,
    EFI_PEI_PPI_DESCRIPTOR_NOTIFY_CALLBACK, EFI_PEI_PPI_DESCRIPTOR_NOTIFY_DISPATCH,
    EFI_PEI_PPI_DESCRIPTOR_PPI, EFI_PEI_PPI_DESCRIPTOR_TERMINATE_LIST,
    EFI_PEI_READ_ONLY_VARIABLE2_PPI_GUID, EFI_PEI_SECURITY2_PPI_GUID, EFI_PEI_STALL_PPI_GUID,
    EFI_PEI_TEMPORARY_RAM_DONE_PPI_GUID, EFI_PEI_TEMPORARY_RAM_SUPPORT_PPI_GUID,
    EfiPeiAllocatePages, EfiPeiAllocatePool, EfiPeiCreateHob, EfiPeiDecompressPpi,
    EfiPeiFfsFindNextFile, EfiPeiFfsFindNextVolume, EfiPeiFfsFindSectionData, EfiPeiGetBootMode,
    EfiPeiGetHobList, EfiPeiInstallPeiMemory, EfiPeiInstallPpi, EfiPeiLocatePpi,
    EfiPeiNotifyDescriptor, EfiPeiNotifyPpi, EfiPeiPpiDescriptor, EfiPeiReadOnlyVariable2Ppi,
    EfiPeiReinstallPpi, EfiPeiReportStatusCode, EfiPeiReset2System, EfiPeiResetSystem,
    EfiPeiServices, EfiPeiSetBootMode, EfiPeiStallPpi, EfiPeimNotifyEntry, PEI_SERVICES_REVISION,
    PEI_SERVICES_SIGNATURE,
};
pub use ppi::{
    EFI_DXE_IPL_PPI_GUID, EFI_PEI_BOOT_IN_RECOVERY_MODE_PPI_GUID,
    EFI_PEI_CPU_IO_PPI_INSTALLED_GUID, EFI_PEI_FIRMWARE_VOLUME_INFO_PPI_GUID,
    EFI_PEI_FIRMWARE_VOLUME_INFO2_PPI_GUID, EFI_PEI_GRAPHICS_PPI_GUID,
    EFI_PEI_MASTER_BOOT_MODE_PEIM_PPI_GUID, EFI_PEI_MP_SERVICES_PPI_GUID,
    EFI_PEI_PERMANENT_MEMORY_INSTALLED_PPI_GUID, EFI_PEI_RECOVERY_BLOCK_IO_PPI_GUID, EfiDxeIplPpi,
    EfiPeiCpuIoPpi, EfiPeiCpuIoPpiAccess, EfiPeiCpuIoPpiWidth, EfiPeiFirmwareVolumeInfo2Ppi,
    EfiPeiFirmwareVolumeInfoPpi, EfiPeiGraphicsPpi, EfiPeiMpServicesPpi,
};
pub use prm::{
    AcpiParameterBufferDescriptor, PRM_ACPI_PARAMETER_DATA_BUFFER_HEADER_SIGNATURE,
    PRM_CONFIG_PROTOCOL_GUID, PRM_CONFIG_PROTOCOL_SIGNATURE, PRM_CONFIG_PROTOCOL_VERSION,
    PRM_CONTEXT_BUFFER_INTERFACE_VERSION, PRM_CONTEXT_BUFFER_SIGNATURE,
    PRM_DATA_BUFFER_HEADER_SIGNATURE, PRM_HANDLER_NAME_MAXIMUM_LENGTH, PrmConfig,
    PrmConfigProtocol, PrmContextBuffer, PrmDataBufferHeader, PrmHandler, PrmModuleContextBuffers,
    PrmRuntimeMmioRange, PrmRuntimeMmioRanges,
};
pub use progress::{
    FIRMWARE_MANAGEMENT_PROGRESS_PROTOCOL_GUID, FIRMWARE_MANAGEMENT_PROGRESS_VERSION,
    FirmwareManagementProgress, FirmwareManagementProgressProtocol,
};
pub use sections::{
    CompressionSectionHeader, EFI_NOT_COMPRESSED, EFI_SECTION_ALL, EFI_SECTION_COMPATIBILITY16,
    EFI_SECTION_COMPRESSION, EFI_SECTION_DISPOSABLE, EFI_SECTION_DXE_DEPEX,
    EFI_SECTION_FIRMWARE_VOLUME_IMAGE, EFI_SECTION_FREEFORM_SUBTYPE_GUID, EFI_SECTION_GUID_DEFINED,
    EFI_SECTION_PE32, EFI_SECTION_PEI_DEPEX, EFI_SECTION_PIC, EFI_SECTION_RAW,
    EFI_SECTION_SMM_DEPEX, EFI_SECTION_TE, EFI_SECTION_USER_INTERFACE, EFI_SECTION_VERSION,
    EFI_STANDARD_COMPRESSION, FfsSection, FfsSectionHeader, FfsSectionIter,
    GuidDefinedSectionHeader, parse_compression_header, parse_guid_defined_header, parse_section,
    parse_section_header,
};
pub use smm_access2::{
    EfiSmmAccess2Protocol, EfiSmmCapabilities2, EfiSmmClose2, EfiSmmLock2, EfiSmmOpen2,
    EfiSmramDescriptor, SMM_ACCESS2_PROTOCOL_GUID, SMRAM_ALLOCATED, SMRAM_CACHEABLE, SMRAM_CLOSED,
    SMRAM_LOCKED, SMRAM_OPEN, SmmAccess2,
};
pub use smm_control2::{
    EfiSmmActivate2, EfiSmmControl2Protocol, EfiSmmDeactivate2, SMM_CONTROL2_PROTOCOL_GUID,
    SmmControl2,
};
pub use smm_cpu::{
    EfiSmmCpuProtocol, EfiSmmReadSaveState, EfiSmmSaveStateIoInfo, EfiSmmSaveStateIoType,
    EfiSmmSaveStateIoWidth, EfiSmmSaveStateRegister, EfiSmmWriteSaveState, SMM_CPU_PROTOCOL_GUID,
    SmmCpu,
};
pub use smm_events::{
    SMM_EXIT_BOOT_SERVICES_PROTOCOL_GUID, SMM_LEGACY_BOOT_PROTOCOL_GUID,
    SMM_READY_TO_BOOT_PROTOCOL_GUID,
};
pub use swap_address_range::{
    EfiGetRangeLocation, EfiGetRtcPowerStatus, EfiGetSwapLockCapability, EfiGetSwapState,
    EfiSetSwapLock, EfiSetSwapState, EfiSwapAddressRangeProtocol,
    SMM_SWAP_ADDRESS_RANGE_PROTOCOL_GUID, SWAP_ADDRESS_RANGE_PROTOCOL_GUID, SwapAddressRange,
    SwapRangeLocation, swap_lock,
};
pub use universal_payload::{
    UNIVERSAL_PAYLOAD_IDENTIFIER, UniversalPayloadEntryPoint, UniversalPayloadGenericHeader,
    UniversalPayloadInfoHeader,
};
