//! # System tables
//!
//! `UEFI` system tables and their shared headers.
//!
//! The system table is passed to every image entry
//! point. The boot services table is available only
//! before `ExitBootServices`, while the runtime
//! services table remains available afterwards under
//! the virtual memory map. The configuration table
//! array carries firmware vendor data such as `ACPI`
//! and `SMBIOS` tables. All three tables begin with
//! the shared `EFI_TABLE_HEADER`. Field ordering in
//! each struct matches the UEFI 2.11 specification.

pub mod boot;
pub mod boot_option;
pub mod configuration;
pub mod esrt;
pub mod header;
pub mod runtime;
pub mod system;

pub use boot::{
    EfiAllocatePages, EfiAllocatePool, EfiBootServices, EfiCalculateCrc32, EfiCheckEvent,
    EfiCloseEvent, EfiCloseProtocol, EfiConnectController, EfiCopyMem, EfiCreateEvent,
    EfiCreateEventEx, EfiDevicePath, EfiDisconnectController, EfiEventNotify, EfiExit,
    EfiExitBootServices, EfiFreePages, EfiFreePool, EfiGetMemoryMap, EfiGetNextMonotonicCount,
    EfiHandleProtocol, EfiInstallConfigurationTable, EfiInstallMultipleProtocolInterfaces,
    EfiInstallProtocolInterface, EfiLoadImage, EfiLocateDevicePath, EfiLocateHandle,
    EfiLocateHandleBuffer, EfiLocateProtocol, EfiOpenProtocol, EfiOpenProtocolInformation,
    EfiOpenProtocolInformationEntry, EfiProtocolsPerHandle, EfiRaiseTpl, EfiRegisterProtocolNotify,
    EfiReinstallProtocolInterface, EfiRestoreTpl, EfiSetMem, EfiSetTimer, EfiSetWatchdogTimer,
    EfiSignalEvent, EfiStall, EfiStartImage, EfiUninstallMultipleProtocolInterfaces,
    EfiUninstallProtocolInterface, EfiUnloadImage, EfiWaitForEvent,
    OPEN_PROTOCOL_BY_CHILD_CONTROLLER, OPEN_PROTOCOL_BY_DRIVER, OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    OPEN_PROTOCOL_EXCLUSIVE, OPEN_PROTOCOL_GET_PROTOCOL, OPEN_PROTOCOL_TEST_PROTOCOL,
};
pub use boot_option::{
    BOOT_MANAGER_ALT_PRESSED, BOOT_MANAGER_CONTROL_PRESSED, BOOT_MANAGER_LOGO_PRESSED,
    BOOT_MANAGER_MENU_KEY_PRESSED, BOOT_MANAGER_SHIFT_PRESSED, BOOT_MANAGER_SYS_REQ_PRESSED,
    BootOrderIter, BootOrderView, ConsoleType, EFI_BOOT_CURRENT_VARIABLE_NAME,
    EFI_BOOT_NEXT_VARIABLE_NAME, EFI_BOOT_OPTION_SUPPORT_APP, EFI_BOOT_OPTION_SUPPORT_COUNT,
    EFI_BOOT_OPTION_SUPPORT_KEY, EFI_BOOT_OPTION_SUPPORT_SYSPREP,
    EFI_BOOT_OPTION_SUPPORT_VARIABLE_NAME, EFI_BOOT_ORDER_VARIABLE_NAME,
    EFI_CAPSULE_FILE_DIRECTORY, EFI_CON_IN_DEV_VARIABLE_NAME, EFI_CON_IN_VARIABLE_NAME,
    EFI_CON_OUT_DEV_VARIABLE_NAME, EFI_CON_OUT_VARIABLE_NAME, EFI_DRIVER_ORDER_VARIABLE_NAME,
    EFI_ERR_OUT_DEV_VARIABLE_NAME, EFI_ERR_OUT_VARIABLE_NAME, EFI_LANG_CODES_VARIABLE_NAME,
    EFI_LANG_VARIABLE_NAME, EFI_PLATFORM_LANG_CODES_VARIABLE_NAME, EFI_PLATFORM_LANG_VARIABLE_NAME,
    EFI_PLATFORM_RECOVERY_ORDER_VARIABLE_NAME, EFI_REMOVABLE_MEDIA_FILE_NAME_AARCH64,
    EFI_REMOVABLE_MEDIA_FILE_NAME_IA32, EFI_REMOVABLE_MEDIA_FILE_NAME_LOONGARCH64,
    EFI_REMOVABLE_MEDIA_FILE_NAME_RISCV64, EFI_REMOVABLE_MEDIA_FILE_NAME_X64,
    EFI_SYS_PREP_ORDER_VARIABLE_NAME, EFI_TIMEOUT_VARIABLE_NAME, EfiBootKeyData,
    EfiKeyOptionHeader, EfiLoadOptionHeader, KeyOptionView, LOAD_OPTION_ACTIVE,
    LOAD_OPTION_CATEGORY_APP, LOAD_OPTION_CATEGORY_BOOT, LOAD_OPTION_CATEGORY_MASK,
    LOAD_OPTION_FORCE_RECONNECT, LOAD_OPTION_HIDDEN, LOAD_OPTION_NUMBER_MAX,
    LOAD_OPTION_NUMBER_UNASSIGNED, LoadOptionError, LoadOptionType, LoadOptionView,
    add_option_number_to_order, find_free_option_number, format_boot_option_name,
    format_driver_option_name, format_key_option_name, format_load_option_name,
    format_platform_recovery_option_name, format_sysprep_option_name,
    is_valid_key_option_variable_name, is_valid_load_option_variable_name, parse_boot_option_index,
    remove_option_number_from_order, validate_load_option,
};
pub use configuration::EfiConfigurationTable;
pub use esrt::{
    EFI_SYSTEM_RESOURCE_TABLE_FIRMWARE_RESOURCE_VERSION, EFI_SYSTEM_RESOURCE_TABLE_GUID,
    ESRT_FW_TYPE_DEVICEFIRMWARE, ESRT_FW_TYPE_SYSTEMFIRMWARE, ESRT_FW_TYPE_UEFIDRIVER,
    ESRT_FW_TYPE_UNKNOWN, ESRT_MANAGEMENT_PROTOCOL_GUID, EfiSystemResourceEntry,
    EfiSystemResourceTable, EsrtGetEntry, EsrtLockRepository, EsrtManagementProtocol,
    EsrtRegisterEntry, EsrtSyncFmp, EsrtUnregisterEntry, EsrtUpdateEntry,
    LAST_ATTEMPT_STATUS_ERROR_AUTH_ERROR, LAST_ATTEMPT_STATUS_ERROR_INCORRECT_VERSION,
    LAST_ATTEMPT_STATUS_ERROR_INSUFFICIENT_RESOURCES, LAST_ATTEMPT_STATUS_ERROR_INVALID_FORMAT,
    LAST_ATTEMPT_STATUS_ERROR_PWR_EVT_AC, LAST_ATTEMPT_STATUS_ERROR_PWR_EVT_BATT,
    LAST_ATTEMPT_STATUS_ERROR_UNSATISFIED_DEPENDENCIES, LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL,
    LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL_VENDOR_RANGE_MAX,
    LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL_VENDOR_RANGE_MIN, LAST_ATTEMPT_STATUS_SUCCESS,
};
pub use header::{
    BOOT_SERVICES_SIGNATURE, EFI_2_00_SYSTEM_TABLE_REVISION, EFI_2_10_SYSTEM_TABLE_REVISION,
    EFI_2_20_SYSTEM_TABLE_REVISION, EFI_2_30_SYSTEM_TABLE_REVISION, EFI_2_31_SYSTEM_TABLE_REVISION,
    EFI_2_40_SYSTEM_TABLE_REVISION, EFI_2_50_SYSTEM_TABLE_REVISION, EFI_2_60_SYSTEM_TABLE_REVISION,
    EFI_2_70_SYSTEM_TABLE_REVISION, EFI_2_80_SYSTEM_TABLE_REVISION, EFI_2_90_SYSTEM_TABLE_REVISION,
    EFI_2_100_SYSTEM_TABLE_REVISION, EFI_2_110_SYSTEM_TABLE_REVISION, EFI_SPECIFICATION_VERSION,
    EfiTableHeader, RUNTIME_SERVICES_SIGNATURE, SYSTEM_TABLE_SIGNATURE,
};
pub use runtime::{
    CAPSULE_FLAGS_INITIATE_RESET, CAPSULE_FLAGS_PERSIST_ACROSS_RESET,
    CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE, EfiCapsuleHeader, EfiConvertPointer,
    EfiGetNextHighMonotonicCount, EfiGetNextVariableName, EfiGetTime, EfiGetVariable,
    EfiGetWakeupTime, EfiQueryCapsuleCapabilities, EfiQueryVariableInfo, EfiResetSystem,
    EfiRuntimeServices, EfiSetTime, EfiSetVariable, EfiSetVirtualAddressMap, EfiSetWakeupTime,
    EfiUpdateCapsule,
};
pub use system::{
    EfiImageEntryPoint, EfiSimpleTextInputProtocol, EfiSimpleTextOutputProtocol, EfiSystemTable,
};
