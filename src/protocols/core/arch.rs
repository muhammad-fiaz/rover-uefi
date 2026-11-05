//! # PI architectural protocols
//!
//! Defined in PI Specification Volume 2: DXE.
//!
//! Architectural protocol `GUID` constants registered with NULL interface
//! pointers to signal that DXE core services or runtime services are initialized.

use crate::base::EfiGuid;

/// The Monotonic Counter architectural protocol `GUID`.
pub const MONOTONIC_COUNTER_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1DA9_7072,
    0xBDDC,
    0x4B30,
    [0x99, 0xF1, 0x72, 0xA0, 0xB5, 0x6F, 0xFF, 0x2A],
);

/// The Reset architectural protocol `GUID`.
pub const RESET_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x27CF_AC88,
    0x46CC,
    0x11D4,
    [0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// The Variable architectural protocol `GUID`.
pub const VARIABLE_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1E56_68E2,
    0x8481,
    0x11D4,
    [0xBC, 0xF1, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

/// The Variable Write architectural protocol `GUID`.
pub const VARIABLE_WRITE_ARCH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6441_F818,
    0x6362,
    0x4E44,
    [0xB5, 0x70, 0x7D, 0xBA, 0x31, 0xDD, 0x24, 0x53],
);
