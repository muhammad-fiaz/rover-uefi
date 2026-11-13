//! # Driver model
//!
//! Driver binding, component naming and loaded images.

pub mod bds;
pub mod binding;
pub mod component_name;
pub mod diagnostics;
pub mod fdt_client;
pub mod gpio;
pub mod guid;
pub mod hardware_interrupt;
pub mod health;
pub mod iommu;
pub mod isa;
pub mod loaded_image;
pub mod non_discoverable;
pub mod override_protocol;
pub mod ovmf;
pub mod qemu_fw_cfg;
pub mod reset_filter;
pub mod scmi;
pub mod supported_version;
pub mod virtio;
pub mod xen;

pub use bds::{BdsArchProtocol, EFI_BDS_ARCH_PROTOCOL_GUID, EfiBdsArchProtocol, EfiBdsEntry};
pub use binding::{DriverBinding, EfiDriverBindingProtocol};
pub use component_name::{
    ComponentName, ComponentName2, EfiComponentName2Protocol, EfiComponentNameProtocol,
};
pub use diagnostics::{
    EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID, EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID,
    EfiDriverDiagnosticType, EfiDriverDiagnostics2Protocol, EfiDriverDiagnostics2RunDiagnostics,
    EfiDriverDiagnosticsProtocol, EfiDriverDiagnosticsRunDiagnostics,
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
    EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID, EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID,
    EfiBusSpecificDriverOverrideGetDriver, EfiBusSpecificDriverOverrideProtocol,
    EfiPlatformDriverOverrideDriverLoaded, EfiPlatformDriverOverrideGetDriver,
    EfiPlatformDriverOverrideGetDriverPath, EfiPlatformDriverOverrideProtocol,
};
pub use ovmf::{
    OVMF_LOADED_X86_LINUX_KERNEL_PROTOCOL_GUID, OVMF_SEV_MEMORY_ACCEPTANCE_PROTOCOL_GUID,
    OvmfLoadedX86LinuxKernel, OvmfSevAllowUnacceptedMemory, OvmfSevMemoryAcceptanceProtocol,
};
pub use qemu_fw_cfg::{
    FW_CFG_F_DMA, FW_CFG_IO_DATA, FW_CFG_IO_DMA_ADDRESS, FW_CFG_IO_SELECTOR, FwCfgDmaAccess,
    FwCfgFile, QEMU_FW_CFG_FNAME_SIZE, QEMU_RAMFB_GUID, QemuRamfbConfig, RAMFB_FOURCC_XRGB8888,
    dma_ctl as fw_cfg_dma_ctl, item_key as fw_cfg_item_key,
};
pub use reset_filter::{
    EfiPlatformSpecificResetFilterProtocol, EfiResetFilter,
    PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID, PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID,
    PlatformSpecificResetFilter,
};
pub use scmi::{
    ARM_SCMI_BASE_PROTOCOL_GUID, ARM_SCMI_CLOCK_PROTOCOL_GUID, ARM_SCMI_PERFORMANCE_PROTOCOL_GUID,
    EfiScmiBaseProtocol, EfiScmiClockProtocol, EfiScmiPerformanceProtocol,
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
