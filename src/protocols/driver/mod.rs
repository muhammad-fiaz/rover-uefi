//! # Driver model
//!
//! Driver binding, component naming and loaded images.

pub mod bds;
pub mod binding;
pub mod component_name;
pub mod diagnostics;
pub mod guid;
pub mod health;
pub mod loaded_image;
pub mod override_protocol;
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
pub use loaded_image::{EfiLoadedImageProtocol, LoadedImage};
pub use override_protocol::{
    EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID, EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID,
    EfiBusSpecificDriverOverrideGetDriver, EfiBusSpecificDriverOverrideProtocol,
    EfiPlatformDriverOverrideDriverLoaded, EfiPlatformDriverOverrideGetDriver,
    EfiPlatformDriverOverrideGetDriverPath, EfiPlatformDriverOverrideProtocol,
};
pub use scmi::{
    ARM_SCMI_BASE_PROTOCOL_GUID, ARM_SCMI_CLOCK_PROTOCOL_GUID, ARM_SCMI_PERFORMANCE_PROTOCOL_GUID,
    EfiScmiBaseProtocol, EfiScmiClockProtocol, EfiScmiPerformanceProtocol,
};
pub use supported_version::{
    EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL_GUID, EfiDriverSupportedEfiVersionProtocol,
};
pub use virtio::{
    EfiVirtioDeviceProtocol, VIRTIO_DEVICE_PROTOCOL_GUID, device_id as virtio_device_id,
    status as virtio_status,
};
