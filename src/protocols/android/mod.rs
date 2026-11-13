//! # Android and Mobile Boot Protocols
//!
//! Protocols and interfaces powering Android boot flow, Fastboot transport
//! and flashing, Android Verified Boot (`AVB`), A/B slot switching (`GBL` Boot Control),
//! and device-tree/bootconfig handoff across mobile platforms, tablets, and emulators.

pub mod avb;
pub mod avf;
pub mod boot_control;
pub mod boot_img;
pub mod boot_memory;
pub mod debug;
pub mod fastboot;
pub mod os_config;

pub use avf::{
    Avf, GBL_EFI_AVF_PROTOCOL_GUID, GBL_EFI_AVF_PROTOCOL_REVISION, GblEfiAvfProtocol,
    GblEfiAvfReadSecretKeeperPublicKey, GblEfiAvfReadVendorDiceHandover,
};
pub use boot_memory::{
    BootMemory, GBL_EFI_BOOT_MEMORY_PROTOCOL_GUID, GBL_EFI_BOOT_MEMORY_PROTOCOL_REVISION,
    GblEfiBootBufferType, GblEfiBootMemoryGetBootBuffer, GblEfiBootMemoryGetPartitionBuffer,
    GblEfiBootMemoryProtocol, GblEfiBootMemorySyncPartitionBuffer, partition_buffer_flag,
};
pub use debug::{
    GBL_EFI_DEBUG_PROTOCOL_GUID, GBL_EFI_DEBUG_PROTOCOL_REVISION, GblDebug, GblEfiDebugErrorTag,
    GblEfiDebugFatalError, GblEfiDebugProtocol,
};

pub use avb::{
    Avb, GBL_EFI_AVB_PROTOCOL_GUID, GBL_EFI_AVB_PROTOCOL_REVISION, GblEfiAvbKeyValidationStatus,
    GblEfiAvbLockState, GblEfiAvbLockType, GblEfiAvbProtocol, GblEfiAvbReadLockState,
    GblEfiAvbReadRollbackIndex, GblEfiAvbValidatePublicKey, GblEfiAvbWriteLockState,
    GblEfiAvbWriteRollbackIndex, boot_color as avb_boot_color, device_status as avb_device_status,
    partition_flag as avb_partition_flag,
};
pub use boot_control::{
    BootControl, GBL_EFI_BOOT_CONTROL_PROTOCOL_GUID, GBL_EFI_BOOT_CONTROL_PROTOCOL_REVISION,
    GblEfiBootControlGetCurrentSlot, GblEfiBootControlGetOneShotBootMode,
    GblEfiBootControlGetSlotCount, GblEfiBootControlGetSlotInfo, GblEfiBootControlHandleLoadedOs,
    GblEfiBootControlProtocol, GblEfiBootControlSetActiveSlot, GblEfiLoadedOs,
    GblEfiOneShotBootMode, GblEfiSlotInfo, GblEfiUnbootableReason,
};
pub use boot_img::{
    ANDROID_BOOTIMG_PROTOCOL_GUID, AndroidBootImg, AndroidBootImgAppendKernelArgs,
    AndroidBootImgProtocol, AndroidBootImgUpdateDtb,
};
pub use fastboot::{
    ANDROID_FASTBOOT_PLATFORM_PROTOCOL_GUID, ANDROID_FASTBOOT_TRANSPORT_PROTOCOL_GUID,
    FastbootPlatform, FastbootPlatformDoOemCommand, FastbootPlatformErase, FastbootPlatformFlash,
    FastbootPlatformGetVar, FastbootPlatformInit, FastbootPlatformProtocol, FastbootPlatformUnInit,
    FastbootTransportProtocol, FastbootTransportReceive, FastbootTransportSend,
};
pub use os_config::{
    GBL_EFI_OS_CONFIGURATION_PROTOCOL_GUID, GBL_EFI_OS_CONFIGURATION_PROTOCOL_REVISION,
    GblEfiDeviceTreeMetadata, GblEfiDeviceTreeSource, GblEfiDeviceTreeType, GblEfiFixupBootConfig,
    GblEfiOsConfigurationProtocol, GblEfiSelectDeviceTrees, GblEfiSelectFitConfiguration,
    GblEfiVerifiedDeviceTree, OsConfiguration,
};
