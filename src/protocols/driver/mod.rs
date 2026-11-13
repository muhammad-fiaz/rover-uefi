//! # Driver model
//!
//! Driver binding, component naming and loaded images.

pub mod ata_policy;
pub mod bds;
pub mod binding;
pub mod component_name;
pub mod config_manager;
pub mod diagnostics;
pub mod embedded;
pub mod emulator;
pub mod fdt_client;
pub mod gpio;
pub mod guid;
pub mod hardware_interrupt;
pub mod health;
pub mod image_loader;
pub mod iommu;
pub mod isa;
pub mod loaded_image;
pub mod non_discoverable;
pub mod override_protocol;
pub mod ovmf;
pub mod platform_boot_manager;
pub mod ps2_policy;
pub mod qemu_fw_cfg;
pub mod realm_aperture;
pub mod reset_filter;
pub mod scmi;
pub mod sd_mmc_override;
pub mod supported_version;
pub mod virtio;
pub mod xen;

pub use ata_policy::{
    ATA_ATAPI_POLICY_PROTOCOL_GUID, ATA_ATAPI_POLICY_VERSION, AtaAtapiPolicy,
    AtaAtapiPolicyProtocol,
};
pub use bds::{BdsArchProtocol, EFI_BDS_ARCH_PROTOCOL_GUID, EfiBdsArchProtocol, EfiBdsEntry};
pub use binding::{DriverBinding, EfiDriverBindingProtocol};
pub use component_name::{
    ComponentName, ComponentName2, EfiComponentName2Protocol, EfiComponentNameProtocol,
};
pub use config_manager::{
    CM_NAMESPACE_ID_MASK, CM_NAMESPACE_ID_SHIFT, CM_NULL_TOKEN, CM_OBJECT_ID_MASK,
    CONFIGURATION_MANAGER_PROTOCOL_GUID, CONFIGURATION_MANAGER_PROTOCOL_REVISION, CmObjDescriptor,
    CmStdObjAcpiTableInfo, CmStdObjConfigurationManagerInfo, CmStdObjSmbiosTableInfo,
    CmX64ObjIoApicInfo, CmX64ObjLocalApicInfo, ConfigurationManager, ConfigurationManagerGetObject,
    ConfigurationManagerProtocol, ConfigurationManagerSetObject,
    DYNAMIC_TABLE_FACTORY_PROTOCOL_GUID, DynamicTableFactory, DynamicTableFactoryProtocol,
    arm_object_id as cm_arm_object_id, get_cm_namespace_id, get_cm_object_id, make_cm_object_id,
    namespace as cm_namespace, std_object_id as cm_std_object_id,
    x64_object_id as cm_x64_object_id,
};
pub use diagnostics::{
    EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID, EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID,
    EfiDriverDiagnosticType, EfiDriverDiagnostics2Protocol, EfiDriverDiagnostics2RunDiagnostics,
    EfiDriverDiagnosticsProtocol, EfiDriverDiagnosticsRunDiagnostics,
};
pub use embedded::{
    EMBEDDED_DEVICE_PROTOCOL_GUID, EMBEDDED_EXTERNAL_DEVICE_PROTOCOL_GUID, EmbeddedDeviceProtocol,
    EmbeddedExternalDevice, EmbeddedExternalDeviceProtocol, EmbeddedExternalDeviceRead,
    EmbeddedExternalDeviceWrite, USB_DEVICE_PROTOCOL_GUID, UsbDevice, UsbDeviceProtocol,
    UsbDeviceRxCallback, UsbDeviceSend, UsbDeviceStart, UsbDeviceTxCallback,
};
pub use emulator::{
    EMU_BLOCK_IO_PROTOCOL_GUID, EMU_GRAPHICS_WINDOW_PROTOCOL_GUID, EMU_THREAD_THUNK_PROTOCOL_GUID,
    EMU_THUNK_PROTOCOL_GUID, EmuBlockIo, EmuBlockIoProtocol, EmuGraphicsWindow,
    EmuGraphicsWindowProtocol, EmuHostMutex, EmuThreadEntry, EmuThreadThunk,
    EmuThreadThunkProtocol, EmuThunk, EmuThunkProtocol,
};
pub use fdt_client::{
    FDT_CLIENT_PROTOCOL_GUID, FdtClient, FdtClientFindCompatibleNode,
    FdtClientFindCompatibleNodeProperty, FdtClientFindCompatibleNodeReg,
    FdtClientFindMemoryNodeReg, FdtClientFindNextCompatibleNode, FdtClientFindNextMemoryNodeReg,
    FdtClientGetNodeProperty, FdtClientGetOrInsertChosenNode, FdtClientProtocol,
    FdtClientSetNodeProperty,
};
pub use gpio::{
    EMBEDDED_GPIO_PROTOCOL_GUID, EmbeddedGpio, EmbeddedGpioGet, EmbeddedGpioGetMode,
    EmbeddedGpioProtocol, EmbeddedGpioSet, EmbeddedGpioSetPull, GpioMode, GpioPin, GpioPull,
    PLATFORM_GPIO_PROTOCOL_GUID,
};
pub use guid::{
    COMPONENT_NAME_GUID, COMPONENT_NAME2_GUID, DRIVER_BINDING_GUID, LOADED_IMAGE_DEVICE_PATH_GUID,
    LOADED_IMAGE_GUID,
};
pub use hardware_interrupt::{
    HARDWARE_INTERRUPT_PROTOCOL_GUID, HARDWARE_INTERRUPT2_PROTOCOL_GUID, HardwareInterrupt,
    HardwareInterrupt2, HardwareInterrupt2Disable, HardwareInterrupt2Enable,
    HardwareInterrupt2EndOfInterrupt, HardwareInterrupt2GetState, HardwareInterrupt2GetTriggerType,
    HardwareInterrupt2Protocol, HardwareInterrupt2Register, HardwareInterrupt2SetTriggerType,
    HardwareInterruptDisable, HardwareInterruptEnable, HardwareInterruptEndOfInterrupt,
    HardwareInterruptGetState, HardwareInterruptHandler, HardwareInterruptProtocol,
    HardwareInterruptRegister, HardwareInterruptSource, HardwareInterruptTriggerType,
};
pub use health::{
    EFI_DRIVER_HEALTH_PROTOCOL_GUID, EfiDriverHealthGetHealthStatus, EfiDriverHealthHiiMessage,
    EfiDriverHealthNotify, EfiDriverHealthProtocol, EfiDriverHealthRepair, EfiDriverHealthStatus,
};
pub use image_loader::{
    DEBUGGER_CONFIGURATION_PROTOCOL_GUID, DEBUGGER_CONFIGURATION_REVISION, DebuggerConfiguration,
    EfiDebuggerConfigurationProtocol, EfiLoadPeImage, EfiPe32ImageProtocol,
    EfiPeCoffImageEmulatorProtocol, EfiUnloadPeImage, PE32_IMAGE_PROTOCOL_GUID,
    PECOFF_IMAGE_EMULATOR_PROTOCOL_GUID, PECOFF_IMAGE_EMULATOR_VERSION, Pe32Image,
    PeCoffImageEmulator, PeCoffImageEmulatorIsImageSupported, PeCoffImageEmulatorRegisterImage,
    PeCoffImageEmulatorUnregisterImage, load_pe_image_attributes,
};
pub use iommu::{
    IOMMU_ACCESS_READ, IOMMU_ACCESS_WRITE, IOMMU_ATTRIBUTE_DUAL_ADDRESS_CYCLE,
    IOMMU_ATTRIBUTE_MEMORY_CACHED, IOMMU_ATTRIBUTE_MEMORY_WRITE_COMBINE, IOMMU_PROTOCOL_GUID,
    IOMMU_PROTOCOL_REVISION, IoMmu, IoMmuAllocateBuffer, IoMmuFreeBuffer, IoMmuMap, IoMmuOperation,
    IoMmuProtocol, IoMmuSetAttribute, IoMmuUnmap,
};
pub use isa::{
    EFI_ISA_HC_PROTOCOL_GUID, EFI_ISA_HC_SERVICE_BINDING_PROTOCOL_GUID, EfiIsaHcCloseIo,
    EfiIsaHcOpenIo, EfiIsaHcProtocol,
};
pub use loaded_image::{EfiLoadedImageProtocol, LoadedImage};
pub use non_discoverable::{
    NON_DISCOVERABLE_AHCI_DEVICE_GUID, NON_DISCOVERABLE_AMBA_DEVICE_GUID,
    NON_DISCOVERABLE_DEVICE_PROTOCOL_GUID, NON_DISCOVERABLE_EHCI_DEVICE_GUID,
    NON_DISCOVERABLE_NVME_DEVICE_GUID, NON_DISCOVERABLE_OHCI_DEVICE_GUID,
    NON_DISCOVERABLE_SDHCI_DEVICE_GUID, NON_DISCOVERABLE_UFS_DEVICE_GUID,
    NON_DISCOVERABLE_UHCI_DEVICE_GUID, NON_DISCOVERABLE_XHCI_DEVICE_GUID, NonDiscoverableDevice,
    NonDiscoverableDeviceDmaType, NonDiscoverableDeviceInit, NonDiscoverableDeviceProtocol,
};
pub use override_protocol::{
    BusSpecificDriverOverride, DriverFamilyOverride,
    EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID, EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL_GUID,
    EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID, EfiBusSpecificDriverOverrideGetDriver,
    EfiBusSpecificDriverOverrideProtocol, EfiDriverFamilyOverrideGetVersion,
    EfiDriverFamilyOverrideProtocol, EfiPlatformDriverOverrideDriverLoaded,
    EfiPlatformDriverOverrideGetDriver, EfiPlatformDriverOverrideGetDriverPath,
    EfiPlatformDriverOverrideProtocol, PlatformDriverOverride,
};
pub use ovmf::{
    OVMF_LOADED_X86_LINUX_KERNEL_PROTOCOL_GUID, OVMF_SEV_MEMORY_ACCEPTANCE_PROTOCOL_GUID,
    OvmfLoadedX86LinuxKernel, OvmfSevAllowUnacceptedMemory, OvmfSevMemoryAcceptanceProtocol,
};
pub use platform_boot_manager::{
    BootManagerLoadOptionType, EfiBootManagerLoadOption, EfiPlatformBootManagerProtocol,
    PLATFORM_BOOT_MANAGER_PROTOCOL_GUID, PLATFORM_BOOT_MANAGER_PROTOCOL_REVISION,
    PlatformBootManager, PlatformBootManagerRefreshAllBootOptions, load_option_attributes,
};
pub use ps2_policy::{
    KEYBOARD_LIGHT_CAPSLOCK, KEYBOARD_LIGHT_NUMLOCK, KEYBOARD_LIGHT_SCROLLLOCK,
    PS2_POLICY_PROTOCOL_GUID, Ps2InitHardwareFn, Ps2Policy, Ps2PolicyProtocol,
};
pub use qemu_fw_cfg::{
    FW_CFG_F_DMA, FW_CFG_IO_DATA, FW_CFG_IO_DMA_ADDRESS, FW_CFG_IO_SELECTOR, FwCfgDmaAccess,
    FwCfgFile, QEMU_FW_CFG_FNAME_SIZE, QEMU_RAMFB_GUID, QemuRamfbConfig, RAMFB_FOURCC_XRGB8888,
    dma_ctl as fw_cfg_dma_ctl, item_key as fw_cfg_item_key,
};
pub use realm_aperture::{
    REALM_APERTURE_MANAGEMENT_PROTOCOL_GUID, REALM_APERTURE_MANAGEMENT_PROTOCOL_REVISION,
    RealmApertureClose, RealmApertureManagement, RealmApertureManagementProtocol,
    RealmApertureOpen,
};
pub use reset_filter::{
    EfiPlatformSpecificResetFilterProtocol, EfiPlatformSpecificResetHandlerProtocol,
    EfiResetFilter, PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID,
    PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID, PlatformSpecificResetFilter,
    PlatformSpecificResetHandler,
};
pub use scmi::{
    ARM_SCMI_BASE_PROTOCOL_GUID, ARM_SCMI_CLOCK_PROTOCOL_GUID, ARM_SCMI_PERFORMANCE_PROTOCOL_GUID,
    EfiScmiBaseProtocol, EfiScmiClockProtocol, EfiScmiPerformanceProtocol,
};
pub use sd_mmc_override::{
    EmmcDriverStrengthType, SD_MMC_BUS_WIDTH_IGNORE, SD_MMC_CLOCK_FREQ_IGNORE,
    SD_MMC_DRIVER_STRENGTH_IGNORE, SD_MMC_OVERRIDE_PROTOCOL_GUID, SD_MMC_OVERRIDE_PROTOCOL_VERSION,
    SdDriverStrengthType, SdMmcBusMode, SdMmcCapabilityFn, SdMmcNotifyPhaseFn,
    SdMmcOperatingParameters, SdMmcOverride, SdMmcOverrideProtocol, SdMmcPhaseType,
};
pub use supported_version::{
    EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL_GUID, EfiDriverSupportedEfiVersionProtocol,
};
pub use virtio::{
    EfiVirtioDeviceProtocol, VIRTIO_DEVICE_PROTOCOL_GUID, VIRTIO_GPU_CONTROL_QUEUE,
    VIRTIO_GPU_FLAG_FENCE, VIRTIO_GPU_MAX_SCANOUTS, VirtioAllocateSharedPages, VirtioDeviceRead,
    VirtioDeviceWrite, VirtioFreeSharedPages, VirtioGetDeviceFeatures, VirtioGetDeviceStatus,
    VirtioGetQueueNumMax, VirtioGpuControlHeader, VirtioGpuCtrlType, VirtioGpuFormat,
    VirtioGpuMemEntry, VirtioGpuPmode, VirtioGpuRectangle, VirtioGpuResourceAttachBacking,
    VirtioGpuResourceCreate2d, VirtioGpuResourceDetachBacking, VirtioGpuResourceFlush,
    VirtioGpuResourceUnref, VirtioGpuRespDisplayInfo, VirtioGpuSetScanout,
    VirtioGpuTransferToHost2d, VirtioMapOperation, VirtioMapShared, VirtioSetDeviceStatus,
    VirtioSetGuestFeatures, VirtioSetPageSize, VirtioSetQueueAddress, VirtioSetQueueAlign,
    VirtioSetQueueNotify, VirtioSetQueueNum, VirtioSetQueueSel, VirtioUnmapShared,
    device_id as virtio_device_id, status as virtio_status, virtio_spec_revision,
};
pub use xen::{
    XENBUS_PROTOCOL_GUID, XENIO_PROTOCOL_GUID, XenBusProtocol, XenBusState, XenIoProtocol,
    XenStoreStatus,
};
