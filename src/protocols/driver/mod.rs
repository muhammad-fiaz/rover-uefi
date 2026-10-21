//! # Driver model
//!
//! Driver binding, component naming and loaded images.

pub mod binding;
pub mod component_name;
pub mod guid;
pub mod loaded_image;
pub mod scmi;
pub mod virtio;

pub use binding::{DriverBinding, EfiDriverBindingProtocol};
pub use component_name::{
    ComponentName, ComponentName2, EfiComponentName2Protocol, EfiComponentNameProtocol,
};
pub use guid::{
    COMPONENT_NAME_GUID, COMPONENT_NAME2_GUID, DRIVER_BINDING_GUID, LOADED_IMAGE_DEVICE_PATH_GUID,
    LOADED_IMAGE_GUID,
};
pub use loaded_image::{EfiLoadedImageProtocol, LoadedImage};
pub use scmi::{
    ARM_SCMI_BASE_PROTOCOL_GUID, ARM_SCMI_CLOCK_PROTOCOL_GUID, ARM_SCMI_PERFORMANCE_PROTOCOL_GUID,
    EfiScmiBaseProtocol, EfiScmiClockProtocol, EfiScmiPerformanceProtocol,
};
pub use virtio::{
    EfiVirtioDeviceProtocol, VIRTIO_DEVICE_PROTOCOL_GUID, device_id as virtio_device_id,
    status as virtio_status,
};
