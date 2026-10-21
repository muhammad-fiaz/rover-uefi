//! # Fundamental types
//!
//! Core `UEFI` types that appear in system tables, boot
//! services, runtime services and protocols.
//!
//! The module provides status codes, identifiers, opaque
//! handles and events, physical and virtual addresses,
//! time representations and the integer enumerations
//! used for memory types and allocation types.
//!
//! These types are the vocabulary for the rest of the crate.
//! Tables in the `tables` module and services and protocols
//! in their own modules use the types defined here.

pub mod bytes;
pub mod crc;
pub mod driver;
pub mod event;
pub mod guid;
pub mod handle;
pub mod physical_address;
pub mod status;
pub mod time;
pub mod types;
pub mod virtual_address;

pub use event::EfiEvent;
pub use guid::{EfiGuid, GuidParseError};
pub use handle::EfiHandle;
pub use physical_address::EfiPhysicalAddress;
pub use status::EfiStatus;
pub use time::{
    EFI_TIME_ADJUST_DAYLIGHT, EFI_TIME_IN_DAYLIGHT, EFI_UNSPECIFIED_TIMEZONE, EfiTime,
    EfiTimeCapabilities,
};
pub use types::{
    AllocationType, EFI_MEMORY_DESCRIPTOR_VERSION, EfiMemoryDescriptor, EfiTpl, InterfaceType,
    LocateSearchType, MemoryType, ResetType, TPL_APPLICATION, TPL_CALLBACK, TPL_HIGH_LEVEL,
    TPL_NOTIFY, TimerDelay,
};
pub use virtual_address::EfiVirtualAddress;
