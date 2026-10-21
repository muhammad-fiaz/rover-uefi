//! # Firmware formats and protocols
//!
//! Typed parsing for firmware image and volume formats and PI protocols.
//! Parsers are bounded and borrow from caller supplied slices.

pub mod depex;
pub mod firmvol;
pub mod fmp;
pub mod fsp;
pub mod hii;
pub mod hob;
pub mod ifr;
pub mod mm_communication;
pub mod pecoff;
pub mod prm;
pub mod universal_payload;

pub use fsp::{
    FSP_IA32, FSP_INFO_HEADER_OFFSET, FSP_INFO_HEADER_SIGNATURE, FSP_X64, FspInfoHeader,
    FspMemoryInit, FspNotifyParams, FspNotifyPhase, FspPhase, FspSiliconInit, FspTempRamExit,
    FspTempRamInit, IMAGE_ATTRIBUTE_64BIT_MODE_SUPPORT, IMAGE_ATTRIBUTE_DISPATCH_MODE_SUPPORT,
    IMAGE_ATTRIBUTE_GRAPHICS_SUPPORT,
};
pub use mm_communication::{
    EfiMmCommunicate, EfiMmCommunicate2, EfiMmCommunicateHeader, EfiMmCommunication2Protocol,
    EfiMmCommunicationProtocol, MM_COMMUNICATION_PROTOCOL_GUID, MM_COMMUNICATION2_PROTOCOL_GUID,
};
pub use prm::{
    AcpiParameterBufferDescriptor, PRM_ACPI_PARAMETER_DATA_BUFFER_HEADER_SIGNATURE,
    PRM_CONFIG_PROTOCOL_GUID, PRM_CONFIG_PROTOCOL_SIGNATURE, PRM_CONFIG_PROTOCOL_VERSION,
    PRM_CONTEXT_BUFFER_INTERFACE_VERSION, PRM_CONTEXT_BUFFER_SIGNATURE,
    PRM_DATA_BUFFER_HEADER_SIGNATURE, PRM_HANDLER_NAME_MAXIMUM_LENGTH, PrmConfigProtocol,
    PrmContextBuffer, PrmDataBufferHeader, PrmHandler, PrmModuleContextBuffers,
    PrmRuntimeMmioRange, PrmRuntimeMmioRanges,
};
pub use universal_payload::{
    UNIVERSAL_PAYLOAD_IDENTIFIER, UniversalPayloadEntryPoint, UniversalPayloadGenericHeader,
    UniversalPayloadInfoHeader,
};
