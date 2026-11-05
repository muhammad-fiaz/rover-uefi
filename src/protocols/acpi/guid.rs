//! # ACPI identifiers
//!
//! Configuration table `GUID` constants.

use crate::base::EfiGuid;

/// The ACPI 1.0 RSDP `GUID`.
pub const ACPI_10_GUID: EfiGuid = EfiGuid::new(
    0xEB9D_2D30,
    0x2D88,
    0x11D3,
    [0x9A, 0x16, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// The ACPI 2.0+ RSDP `GUID`.
pub const ACPI_20_GUID: EfiGuid = EfiGuid::new(
    0x8868_E871,
    0xE4F1,
    0x11D3,
    [0xBC, 0x22, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
);

/// The ACPI Table protocol `GUID` (UEFI 2.3+).
pub const EFI_ACPI_TABLE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xFFE0_6BDD,
    0x6107,
    0x46A6,
    [0x7B, 0xB2, 0x5A, 0x9C, 0x7E, 0xC5, 0x27, 0x5C],
);
