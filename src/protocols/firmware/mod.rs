//! # Firmware formats and protocols
//!
//! Typed parsing for firmware image and volume formats and PI protocols.
//! Parsers are bounded and borrow from caller supplied slices.

pub mod arch_protocols;
pub mod capsule_arch;
pub mod decompress;
pub mod depex;
pub mod dxe_services;
pub mod firmvol;
pub mod fmp;
pub mod fsp;
pub mod hii;
pub mod hob;
pub mod ifr;
pub mod mm_base;
pub mod mm_communication;
pub mod mm_system_table;
pub mod pecoff;
pub mod pei;
pub mod ppi;
pub mod prm;
pub mod sections;
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
pub use dxe_services::{
    DXE_SERVICES_REVISION, DXE_SERVICES_SIGNATURE, DXE_SERVICES_TABLE_GUID, EfiAddIoSpace,
    EfiAddMemorySpace, EfiAllocateIoSpace, EfiAllocateMemorySpace, EfiDxeDispatch, EfiDxeSchedule,
    EfiDxeServices, EfiDxeTrust, EfiFreeIoSpace, EfiFreeMemorySpace, EfiGcdAllocateType,
    EfiGcdIoSpaceDescriptor, EfiGcdIoType, EfiGcdMemorySpaceDescriptor, EfiGcdMemoryType,
    EfiGetIoSpaceDescriptor, EfiGetIoSpaceMap, EfiGetMemorySpaceDescriptor, EfiGetMemorySpaceMap,
    EfiProcessFirmwareVolume, EfiRemoveIoSpace, EfiRemoveMemorySpace, EfiSetMemorySpaceAttributes,
    EfiSetMemorySpaceCapabilities,
};
pub use fsp::{
    FSP_IA32, FSP_INFO_HEADER_OFFSET, FSP_INFO_HEADER_SIGNATURE, FSP_X64, FspInfoHeader,
    FspMemoryInit, FspNotifyParams, FspNotifyPhase, FspPhase, FspSiliconInit, FspTempRamExit,
    FspTempRamInit, IMAGE_ATTRIBUTE_64BIT_MODE_SUPPORT, IMAGE_ATTRIBUTE_DISPATCH_MODE_SUPPORT,
    IMAGE_ATTRIBUTE_GRAPHICS_SUPPORT,
};
pub use mm_base::{
    EfiMmBaseProtocol, EfiMmGetMmstLocation, EfiMmInsideOut, EfiSmmBase2Protocol,
    MM_BASE_PROTOCOL_GUID, MmBaseProtocol, SMM_BASE2_PROTOCOL_GUID,
};
pub use mm_communication::{
    EfiMmCommunicate, EfiMmCommunicate2, EfiMmCommunicateHeader, EfiMmCommunication2Protocol,
    EfiMmCommunicationProtocol, MM_COMMUNICATION_PROTOCOL_GUID, MM_COMMUNICATION2_PROTOCOL_GUID,
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
    PRM_DATA_BUFFER_HEADER_SIGNATURE, PRM_HANDLER_NAME_MAXIMUM_LENGTH, PrmConfigProtocol,
    PrmContextBuffer, PrmDataBufferHeader, PrmHandler, PrmModuleContextBuffers,
    PrmRuntimeMmioRange, PrmRuntimeMmioRanges,
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
pub use universal_payload::{
    UNIVERSAL_PAYLOAD_IDENTIFIER, UniversalPayloadEntryPoint, UniversalPayloadGenericHeader,
    UniversalPayloadInfoHeader,
};
