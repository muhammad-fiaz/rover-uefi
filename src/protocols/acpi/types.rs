//! # ACPI types
//!
//! Signature and table type constants.

/// RSDT signature.
pub const SIG_RSDT: [u8; 4] = *b"RSDT";

/// XSDT signature.
pub const SIG_XSDT: [u8; 4] = *b"XSDT";

/// FADT signature.
pub const SIG_FADT: [u8; 4] = *b"FACP";

/// MADT signature.
pub const SIG_MADT: [u8; 4] = *b"APIC";

/// MCFG signature.
pub const SIG_MCFG: [u8; 4] = *b"MCFG";

/// FACS signature.
pub const SIG_FACS: [u8; 4] = *b"FACS";

/// DSDT signature.
pub const SIG_DSDT: [u8; 4] = *b"DSDT";

/// SSDT signature.
pub const SIG_SSDT: [u8; 4] = *b"SSDT";

/// HPET signature.
pub const SIG_HPET: [u8; 4] = *b"HPET";

/// SRAT signature.
pub const SIG_SRAT: [u8; 4] = *b"SRAT";

/// SLIT signature.
pub const SIG_SLIT: [u8; 4] = *b"SLIT";

/// DMAR signature.
pub const SIG_DMAR: [u8; 4] = *b"DMAR";

/// GTDT signature.
pub const SIG_GTDT: [u8; 4] = *b"GTDT";

/// SPCR signature.
pub const SIG_SPCR: [u8; 4] = *b"SPCR";

/// BGRT signature.
pub const SIG_BGRT: [u8; 4] = *b"BGRT";
