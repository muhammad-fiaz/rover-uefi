//! # Capsule architectural protocol
//!
//! Defined in PI Specification Volume 2: DXE.
//!
//! The DXE driver producing this protocol initializes `UpdateCapsule` and
//! `QueryCapsuleCapabilities` in the UEFI Runtime Services Table and installs
//! this protocol on a new handle with a NULL interface pointer to signal
//! that capsule services are available.

use crate::base::EfiGuid;

/// The capsule architectural protocol `GUID`.
pub const EFI_CAPSULE_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5053_697E,
    0x2CBC,
    0x4819,
    [0x90, 0xD9, 0x05, 0x80, 0xDE, 0xEE, 0x57, 0x54],
);
