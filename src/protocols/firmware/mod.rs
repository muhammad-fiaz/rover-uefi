//! # Firmware formats and protocols
//!
//! Typed parsing for firmware image and volume formats and PI protocols.
//! Parsers are bounded and borrow from caller supplied slices.

pub mod arch_protocols;
pub mod capsule_arch;
pub mod capsule_policy;
pub mod decompress;
pub mod depex;
pub mod dt_fixup;
pub mod dxe_services;
pub mod esrt;
pub mod firmvol;
pub mod fmp;
pub mod fmp_depex;
pub mod form_browser;
pub mod form_browser_ex;
pub mod fsp;
pub mod ftw;
pub mod hii;
pub mod hob;
pub mod ifr;
pub mod lock_box;
pub mod memory_accept;
pub mod memory_attribute;
pub mod memory_profile;
pub mod memory_test;
pub mod mm_base;
pub mod mm_communication;
pub mod mm_dispatch;
pub mod mm_system_table;
pub mod pecoff;
pub mod pei;
pub mod ppi;
pub mod prm;
pub mod progress;
pub mod psa_fwu;
pub mod s3_save_state;
pub mod sections;
pub mod smm_access2;
pub mod smm_control2;
pub mod smm_cpu;
pub mod smm_events;
pub mod smm_variable;
pub mod standalone_mm;
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
pub use capsule_policy::{
    CAPSULE_UPDATE_POLICY_PROTOCOL_GUID, CapsuleUpdatePolicy,
    CapsuleUpdatePolicyCheckSystemEnvironment, CapsuleUpdatePolicyCheckSystemPower,
    CapsuleUpdatePolicyCheckSystemThermal,
    CapsuleUpdatePolicyIsLockFmpDeviceAtLockEventGuidRequired,
    CapsuleUpdatePolicyIsLowestSupportedVersionCheckRequired, CapsuleUpdatePolicyProtocol,
    last_attempt_status,
};
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
pub use fmp::{
    EFI_FIRMWARE_MANAGEMENT_PROTOCOL_GUID, EfiFirmwareImageDep, EfiFirmwareImageDescriptor,
    EfiFirmwareManagementCheckImage, EfiFirmwareManagementGetImage,
    EfiFirmwareManagementGetImageInfo, EfiFirmwareManagementGetPackageInfo,
    EfiFirmwareManagementProtocol, EfiFirmwareManagementSetImage,
    EfiFirmwareManagementSetPackageInfo, EfiFirmwareManagementUpdateImageProgress,
    FirmwareManagement, FmpCapsuleHeader, is_reserved_flags, parse_fmp_capsule_header,
};
pub use fmp_depex::{
    EFI_FMP_DEP_AND, EFI_FMP_DEP_DECLARE_LENGTH, EFI_FMP_DEP_END, EFI_FMP_DEP_EQ,
    EFI_FMP_DEP_FALSE, EFI_FMP_DEP_GT, EFI_FMP_DEP_GTE, EFI_FMP_DEP_LT, EFI_FMP_DEP_LTE,
    EFI_FMP_DEP_NOT, EFI_FMP_DEP_OR, EFI_FMP_DEP_PUSH_GUID, EFI_FMP_DEP_PUSH_VERSION,
    EFI_FMP_DEP_TRUE, EFI_FMP_DEP_VERSION_STR, FmpDepexError, FmpDepexValue,
    LAST_ATTEMPT_STATUS_DEPENDENCY_CHECK_MAX, LAST_ATTEMPT_STATUS_DEPENDENCY_CHECK_MIN,
    LAST_ATTEMPT_STATUS_DEPENDENCY_LIB_MAX, LAST_ATTEMPT_STATUS_DEPENDENCY_LIB_MIN,
    LAST_ATTEMPT_STATUS_DRIVER_MAX, LAST_ATTEMPT_STATUS_DRIVER_MIN,
    LAST_ATTEMPT_STATUS_FMP_RESERVED_MAX, LAST_ATTEMPT_STATUS_FMP_RESERVED_MIN,
    MAX_FMP_DEPEX_STACK_DEPTH, evaluate_fmp_depex, validate_fmp_depex,
};
pub use form_browser::{
    ConfirmDataChangeFn, EFI_FORM_BROWSER2_PROTOCOL_GUID, EFI_HII_PLATFORM_SETUP_FORMSET_GUID,
    EFI_HII_USER_CREDENTIAL_FORMSET_GUID, EfiBrowserCallback2, EfiFormBrowser2Protocol,
    EfiFormDisplayEngineProtocol, EfiFormId, EfiHiiValue, EfiScreenDescriptor, EfiSendForm2,
    ExitDisplayFn, FORM_DISPLAY_ENGINE_PROTOCOL_GUID, FormBrowser2, FormDisplayEngine,
    FormDisplayFn, UserInput, browser_action_request, display_status,
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
    FSP_COMPONENT_I, FSP_COMPONENT_M, FSP_COMPONENT_O, FSP_COMPONENT_S, FSP_COMPONENT_T, FSP_IA32,
    FSP_INFO_HEADER_OFFSET, FSP_INFO_HEADER_SIGNATURE, FSP_STATUS_RESET_REQUIRED_3,
    FSP_STATUS_RESET_REQUIRED_4, FSP_STATUS_RESET_REQUIRED_5, FSP_STATUS_RESET_REQUIRED_6,
    FSP_STATUS_RESET_REQUIRED_7, FSP_STATUS_RESET_REQUIRED_8, FSP_STATUS_RESET_REQUIRED_COLD,
    FSP_STATUS_RESET_REQUIRED_WARM, FSP_STATUS_VARIABLE_REQUEST, FSP_X64, FspError, FspImage,
    FspInfoHeader, FspMemoryInit, FspNotifyParams, FspNotifyPhase, FspPhase, FspSiliconInit,
    FspTempRamExit, FspTempRamInit, FspUpdHeader, FspiArchUpd, FspmArchUpd, FspsArchUpd,
    FsptArchUpd, IMAGE_ATTRIBUTE_64BIT_MODE_SUPPORT, IMAGE_ATTRIBUTE_DISPATCH_MODE_SUPPORT,
    IMAGE_ATTRIBUTE_GRAPHICS_SUPPORT, IMAGE_ATTRIBUTE_VARIABLE_SERVICES_SUPPORT,
    encode_fsp_reset_request,
};
pub use ftw::{
    EfiFaultTolerantWriteAbort, EfiFaultTolerantWriteAllocate, EfiFaultTolerantWriteGetLastWrite,
    EfiFaultTolerantWriteGetMaxBlockSize, EfiFaultTolerantWriteProtocol,
    EfiFaultTolerantWriteRestart, EfiFaultTolerantWriteWrite, FAULT_TOLERANT_WRITE_PROTOCOL_GUID,
    FaultTolerantWrite, SMM_FAULT_TOLERANT_WRITE_PROTOCOL_GUID,
};
pub use hii::{
    EFI_HII_CONFIG_ROUTING_PROTOCOL_GUID, EFI_HII_DATABASE_PROTOCOL_GUID,
    EFI_HII_FONT_PROTOCOL_GUID, EFI_HII_IMAGE_DECODER_NAME_JPEG_GUID,
    EFI_HII_IMAGE_DECODER_NAME_PNG_GUID, EFI_HII_IMAGE_DECODER_PROTOCOL_GUID,
    EFI_HII_IMAGE_PROTOCOL_GUID, EFI_HII_POPUP_PROTOCOL_GUID,
    EFI_HII_SET_KEYBOARD_LAYOUT_EVENT_GUID, EFI_HII_STRING_PROTOCOL_GUID, EFI_STRING_ID_NULL,
    EfiHiiDatabaseNotify, EfiHiiDatabaseProtocol, EfiHiiFontProtocol, EfiHiiHandle,
    EfiHiiImageDecoderProtocol, EfiHiiKeyboardLayout, EfiHiiPackageListHeader, EfiHiiPopupProtocol,
    EfiHiiStringProtocol, EfiImageOutput, EfiKeyDescriptor, EfiStringId, FontDisplayInfo, FontInfo,
    HII_CONFIG_ACCESS_GUID, HII_DATABASE_GUID, HII_FONT_ALTER_GUID, HII_FORMS_GUID,
    HII_STRING_PACKAGE_GUID, HiiConfigRouting, HiiDatabase, HiiFont, HiiImageDecoder,
    HiiImageDecoderColorType, HiiImageDecoderHeader, HiiPackageHeader, HiiPackageType, HiiPopup,
    HiiPopupSelection, HiiPopupStyle, HiiPopupType, HiiRowInfo, HiiString, hii_out_flags,
    is_known_package_type, notify_type as hii_notify_type, parse_package_header,
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
pub use memory_profile::{
    EfiMemoryProfileGetData, EfiMemoryProfileGetRecordingState, EfiMemoryProfileProtocol,
    EfiMemoryProfileRecord, EfiMemoryProfileRegisterImage, EfiMemoryProfileSetRecordingState,
    EfiMemoryProfileUnregisterImage, EfiSmmMemoryProfileProtocol, MEMORY_PROFILE_CONTEXT_REVISION,
    MEMORY_PROFILE_CONTEXT_SIGNATURE, MEMORY_PROFILE_DRIVER_INFO_REVISION,
    MEMORY_PROFILE_DRIVER_INFO_SIGNATURE, MEMORY_PROFILE_PROTOCOL_GUID, MemoryProfile,
    MemoryProfileAction, MemoryProfileCommonHeader, MemoryProfileContext,
    SMM_MEMORY_PROFILE_PROTOCOL_GUID,
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
    EFI_MM_COMMUNICATION_PROTOCOL_GUID, EFI_MM_COMMUNICATION2_PROTOCOL_GUID,
    EFI_SMM_COMMUNICATION_PROTOCOL_GUID, EfiMmCommunicate, EfiMmCommunicate2,
    EfiMmCommunicateHeader, EfiMmCommunication2Protocol, EfiMmCommunicationProtocol,
    MM_COMMUNICATION_PROTOCOL_GUID, MM_COMMUNICATION2_PROTOCOL_GUID, MmCommunication,
    MmCommunication2,
};
pub use mm_dispatch::{
    EFI_MM_GPI_DISPATCH_PROTOCOL_GUID, EFI_MM_IO_TRAP_DISPATCH_PROTOCOL_GUID,
    EFI_MM_PERIODIC_TIMER_DISPATCH_PROTOCOL_GUID, EFI_MM_POWER_BUTTON_DISPATCH_PROTOCOL_GUID,
    EFI_MM_STANDBY_BUTTON_DISPATCH_PROTOCOL_GUID, EFI_MM_SW_DISPATCH_PROTOCOL_GUID,
    EFI_MM_SX_DISPATCH_PROTOCOL_GUID, EFI_MM_USB_DISPATCH_PROTOCOL_GUID,
    EFI_SMM_GPI_DISPATCH2_PROTOCOL_GUID, EFI_SMM_IO_TRAP_DISPATCH2_PROTOCOL_GUID,
    EFI_SMM_PERIODIC_TIMER_DISPATCH2_PROTOCOL_GUID, EFI_SMM_POWER_BUTTON_DISPATCH2_PROTOCOL_GUID,
    EFI_SMM_STANDBY_BUTTON_DISPATCH2_PROTOCOL_GUID, EFI_SMM_SW_DISPATCH2_PROTOCOL_GUID,
    EFI_SMM_SX_DISPATCH2_PROTOCOL_GUID, EFI_SMM_USB_DISPATCH2_PROTOCOL_GUID, MmGpiDispatch,
    MmGpiDispatchProtocol, MmGpiRegisterContext, MmIoTrapContext, MmIoTrapDispatch,
    MmIoTrapDispatchProtocol, MmIoTrapRegisterContext, MmIoTrapType, MmPeriodicTimerContext,
    MmPeriodicTimerDispatch, MmPeriodicTimerDispatchProtocol, MmPeriodicTimerRegisterContext,
    MmPowerButtonDispatch, MmPowerButtonDispatchProtocol, MmPowerButtonRegisterContext,
    MmStandbyButtonDispatch, MmStandbyButtonDispatchProtocol, MmStandbyButtonRegisterContext,
    MmSwContext, MmSwDispatch, MmSwDispatchProtocol, MmSwRegisterContext, MmSxDispatch,
    MmSxDispatchProtocol, MmSxRegisterContext, MmUsbDispatch, MmUsbDispatchProtocol,
    MmUsbRegisterContext, PowerButtonPhase, SleepPhase, SleepType, StandbyButtonPhase, UsbMmiType,
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
pub use psa_fwu::{
    PSA_FWU_IMAGE_DIRECTORY_GUID, PSA_FWU_METADATA_GUID, PSA_FWU_UPDATE_AGENT_GUID,
    PSA_MM_FWU_AUTH_FAIL, PSA_MM_FWU_BUSY, PSA_MM_FWU_COMMAND_ACCEPT_IMAGE,
    PSA_MM_FWU_COMMAND_BEGIN_STAGING, PSA_MM_FWU_COMMAND_CANCEL_STAGING, PSA_MM_FWU_COMMAND_COMMIT,
    PSA_MM_FWU_COMMAND_DISCOVER, PSA_MM_FWU_COMMAND_END_STAGING, PSA_MM_FWU_COMMAND_OPEN,
    PSA_MM_FWU_COMMAND_READ_STREAM, PSA_MM_FWU_COMMAND_SELECT_PREVIOUS,
    PSA_MM_FWU_COMMAND_WRITE_STREAM, PSA_MM_FWU_DENIED, PSA_MM_FWU_NO_PERMISSION,
    PSA_MM_FWU_NOT_AVAILABLE, PSA_MM_FWU_OUT_OF_BOUNDS, PSA_MM_FWU_RESUME, PSA_MM_FWU_SUCCESS,
    PSA_MM_FWU_UNKNOWN, PsaFwuDiscoveryResult, PsaFwuImageProperty, PsaFwuStreamHeader,
    SERVICE_STATUS_ERR_INIT, SERVICE_STATUS_OPERATIVE, parse_psa_version, psa_fwu_is_success,
};
pub use s3_save_state::{
    EFI_BOOT_SCRIPT_DISPATCH_2_OPCODE, EFI_BOOT_SCRIPT_DISPATCH_OPCODE,
    EFI_BOOT_SCRIPT_INFORMATION_OPCODE, EFI_BOOT_SCRIPT_IO_POLL_OPCODE,
    EFI_BOOT_SCRIPT_IO_READ_WRITE_OPCODE, EFI_BOOT_SCRIPT_IO_WRITE_OPCODE,
    EFI_BOOT_SCRIPT_MEM_POLL_OPCODE, EFI_BOOT_SCRIPT_MEM_READ_WRITE_OPCODE,
    EFI_BOOT_SCRIPT_MEM_WRITE_OPCODE, EFI_BOOT_SCRIPT_PCI_CONFIG_POLL_OPCODE,
    EFI_BOOT_SCRIPT_PCI_CONFIG_READ_WRITE_OPCODE, EFI_BOOT_SCRIPT_PCI_CONFIG_WRITE_OPCODE,
    EFI_BOOT_SCRIPT_PCI_CONFIG2_POLL_OPCODE, EFI_BOOT_SCRIPT_PCI_CONFIG2_READ_WRITE_OPCODE,
    EFI_BOOT_SCRIPT_PCI_CONFIG2_WRITE_OPCODE, EFI_BOOT_SCRIPT_SMBUS_EXECUTE_OPCODE,
    EFI_BOOT_SCRIPT_STALL_OPCODE, EFI_S3_SAVE_STATE_PROTOCOL_GUID,
    EFI_S3_SMM_SAVE_STATE_PROTOCOL_GUID, EfiBootScriptWidth, EfiS3BootScriptPosition,
    EfiS3SaveStateCompare, EfiS3SaveStateInsert, EfiS3SaveStateLabel, EfiS3SaveStateProtocol,
    EfiS3SaveStateWrite, EfiS3SmmSaveStateProtocol, S3BootScriptRecord, S3BootScriptTable,
    S3BootScriptTarget, S3SaveState, pci_address as s3_boot_script_pci_address,
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
    EfiSmmSaveStateIoWidth, EfiSmmSaveStateRegister, EfiSmmWriteSaveState,
    SM_MONITOR_INIT_PROTOCOL_GUID, SM_MONITOR_STATE_ACTIVATED, SM_MONITOR_STATE_ENABLED,
    SMM_CPU_PROTOCOL_GUID, SMM_CPU_RENDEZVOUS_PROTOCOL_GUID, SMM_CPU_SERVICE_PROTOCOL_GUID,
    SmMonitorInit, SmMonitorInitProtocol, SmmCpu, SmmCpuRendezvous, SmmCpuRendezvousProtocol,
    SmmCpuService, SmmCpuServiceProtocol,
};
pub use smm_events::{
    SMM_EXIT_BOOT_SERVICES_PROTOCOL_GUID, SMM_LEGACY_BOOT_PROTOCOL_GUID,
    SMM_READY_TO_BOOT_PROTOCOL_GUID,
};
pub use smm_variable::{
    EFI_SMM_FIRMWARE_VOLUME_BLOCK_PROTOCOL_GUID, EFI_SMM_VARIABLE_PROTOCOL_GUID,
    EfiSmmFirmwareVolumeBlockProtocol, EfiSmmVariableProtocol, SmmVariable,
};
pub use standalone_mm::{
    CORE_TYPE_EFFICIENT, CORE_TYPE_PERFORMANCE, CORE_TYPE_STANDARD,
    EFI_MM_PEI_MMRAM_MEMORY_RESERVE_GUID, EfiMmCommunicateFvDispatch,
    EfiMmCommunicateFvDispatchData, EfiMmramHobDescriptorBlockHeader, MM_FV_DISPATCH_GUID,
    MP_INFORMATION_HOB_GUID, MP_INFORMATION2_GUID, MP_INFORMATION2_HOB_REVISION,
    MmFoundationEntryPoint, MmImageEntryPoint, MpInformation2Entry, MpInformation2HobData,
    MpInformationHobHeader,
};
pub use swap_address_range::{
    EfiGetRangeLocation, EfiGetRtcPowerStatus, EfiGetSwapLockCapability, EfiGetSwapState,
    EfiSetSwapLock, EfiSetSwapState, EfiSwapAddressRangeProtocol,
    SMM_SWAP_ADDRESS_RANGE_PROTOCOL_GUID, SWAP_ADDRESS_RANGE_PROTOCOL_GUID, SwapAddressRange,
    SwapRangeLocation, swap_lock,
};
pub use universal_payload::{
    NON_RELOCATABLE, PREFETCHABLE, SS_32BIT_MEMORY_SPACE, SS_64BIT_MEMORY_SPACE,
    SS_CONFIGURATION_SPACE, SS_IO_SPACE, UNIVERSAL_PAYLOAD_ACPI_TABLE_GUID,
    UNIVERSAL_PAYLOAD_ACPI_TABLE_REVISION, UNIVERSAL_PAYLOAD_BASE_GUID,
    UNIVERSAL_PAYLOAD_BASE_REVISION, UNIVERSAL_PAYLOAD_COMMAND_LINE_GUID,
    UNIVERSAL_PAYLOAD_DEVICE_TREE_GUID, UNIVERSAL_PAYLOAD_EXTRA_DATA_GUID,
    UNIVERSAL_PAYLOAD_EXTRA_SEC_NAME_PREFIX, UNIVERSAL_PAYLOAD_IDENTIFIER,
    UNIVERSAL_PAYLOAD_INFO_SEC_NAME, UNIVERSAL_PAYLOAD_PCI_ROOT_BRIDGE_INFO_GUID,
    UNIVERSAL_PAYLOAD_PCI_ROOT_BRIDGES_REVISION, UNIVERSAL_PAYLOAD_PCI_SEGMENT_INFO_HOB_GUID,
    UNIVERSAL_PAYLOAD_SERIAL_PORT_INFO_GUID, UNIVERSAL_PAYLOAD_SERIAL_PORT_INFO_REVISION,
    UNIVERSAL_PAYLOAD_SERIAL_PORT_PARENT_DEVICE_INFO_GUID, UNIVERSAL_PAYLOAD_SMBIOS_TABLE_GUID,
    UNIVERSAL_PAYLOAD_SMBIOS_TABLE_REVISION, UNIVERSAL_PAYLOAD_SMBIOS3_TABLE_GUID,
    UniversalPayloadAcpiTable, UniversalPayloadBase, UniversalPayloadEntryPoint,
    UniversalPayloadError, UniversalPayloadGenericHeader, UniversalPayloadInfoHeader,
    UniversalPayloadPciRootBridge, UniversalPayloadPciRootBridgeAperture,
    UniversalPayloadSerialPortInfo, UniversalPayloadSmbiosTable, parse_acpi_table,
    parse_generic_header, parse_info_header, parse_serial_port_info, parse_smbios_table,
};
