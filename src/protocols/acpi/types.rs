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

/// DBG2 signature.
pub const SIG_DBG2: [u8; 4] = *b"DBG2";

/// TPM2 signature.
pub const SIG_TPM2: [u8; 4] = *b"TPM2";

/// PRMT signature.
pub const SIG_PRMT: [u8; 4] = *b"PRMT";

/// BERT signature.
pub const SIG_BERT: [u8; 4] = *b"BERT";

/// HEST signature.
pub const SIG_HEST: [u8; 4] = *b"HEST";

/// NFIT signature.
pub const SIG_NFIT: [u8; 4] = *b"NFIT";

/// PPTT signature.
pub const SIG_PPTT: [u8; 4] = *b"PPTT";

/// CEDT signature.
pub const SIG_CEDT: [u8; 4] = *b"CEDT";

/// IORT signature.
pub const SIG_IORT: [u8; 4] = *b"IORT";

/// FPDT signature.
pub const SIG_FPDT: [u8; 4] = *b"FPDT";
