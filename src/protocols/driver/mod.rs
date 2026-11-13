//! # Driver model
//!
//! Driver binding, component naming and loaded images.

pub mod bds;
pub mod binding;
pub mod component_name;
pub mod diagnostics;
pub mod guid;
pub mod health;
pub mod iommu;
pub mod loaded_image;
pub mod non_discoverable;
pub mod override_protocol;
pub mod qemu_fw_cfg;
pub mod scmi;
pub mod supported_version;
pub mod virtio;

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
pub use guid::{
    COMPONENT_NAME_GUID, COMPONENT_NAME2_GUID, DRIVER_BINDING_GUID, LOADED_IMAGE_DEVICE_PATH_GUID,
    LOADED_IMAGE_GUID,
};
pub use health::{
    EFI_DRIVER_HEALTH_PROTOCOL_GUID, EfiDriverHealthGetHealthStatus, EfiDriverHealthHiiMessage,
    EfiDriverHealthNotify, EfiDriverHealthProtocol, EfiDriverHealthRepair, EfiDriverHealthStatus,
};
pub use iommu::{
    EDKII_IOMMU_ACCESS_READ, EDKII_IOMMU_ACCESS_WRITE, EDKII_IOMMU_ATTRIBUTE_DUAL_ADDRESS_CYCLE,
    EDKII_IOMMU_ATTRIBUTE_MEMORY_CACHED, EDKII_IOMMU_ATTRIBUTE_MEMORY_WRITE_COMBINE,
    EDKII_IOMMU_PROTOCOL_GUID, EDKII_IOMMU_PROTOCOL_REVISION, EdkiiIoMmuAllocateBuffer,
    EdkiiIoMmuFreeBuffer, EdkiiIoMmuMap, EdkiiIoMmuOperation, EdkiiIoMmuProtocol,
    EdkiiIoMmuSetAttribute, EdkiiIoMmuUnmap,
};
pub use loaded_image::{EfiLoadedImageProtocol, LoadedImage};
pub use non_discoverable::{
    EDKII_NON_DISCOVERABLE_AHCI_DEVICE_GUID, EDKII_NON_DISCOVERABLE_AMBA_DEVICE_GUID,
    EDKII_NON_DISCOVERABLE_DEVICE_PROTOCOL_GUID, EDKII_NON_DISCOVERABLE_EHCI_DEVICE_GUID,
    EDKII_NON_DISCOVERABLE_NVME_DEVICE_GUID, EDKII_NON_DISCOVERABLE_OHCI_DEVICE_GUID,
    EDKII_NON_DISCOVERABLE_SDHCI_DEVICE_GUID, EDKII_NON_DISCOVERABLE_UFS_DEVICE_GUID,
    EDKII_NON_DISCOVERABLE_UHCI_DEVICE_GUID, EDKII_NON_DISCOVERABLE_XHCI_DEVICE_GUID,
    EdkiiNonDiscoverableDeviceProtocol, NonDiscoverableDeviceDmaType, NonDiscoverableDeviceInit,
};
pub use override_protocol::{
    EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID, EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID,
    EfiBusSpecificDriverOverrideGetDriver, EfiBusSpecificDriverOverrideProtocol,
    EfiPlatformDriverOverrideDriverLoaded, EfiPlatformDriverOverrideGetDriver,
    EfiPlatformDriverOverrideGetDriverPath, EfiPlatformDriverOverrideProtocol,
};
pub use qemu_fw_cfg::{
    FW_CFG_F_DMA, FW_CFG_IO_DATA, FW_CFG_IO_DMA_ADDRESS, FW_CFG_IO_SELECTOR, FwCfgDmaAccess,
    FwCfgFile, QEMU_FW_CFG_FNAME_SIZE, QEMU_RAMFB_GUID, QemuRamfbConfig, RAMFB_FOURCC_XRGB8888,
    dma_ctl as fw_cfg_dma_ctl, item_key as fw_cfg_item_key,
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
