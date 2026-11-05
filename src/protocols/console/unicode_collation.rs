//! # Unicode Collation Protocols
//!
//! Defined by the UEFI Specification Section 24.2: Unicode Collation Protocol.
//!
//! Performs case-insensitive Unicode character comparisons, pattern meta-matching,
//! case conversions, and 8.3 FAT filename string conversions.

use crate::abi::types::{Bool, UIntN};
use crate::base::EfiGuid;

/// The `EFI_UNICODE_COLLATION2_PROTOCOL_GUID` (RFC 4646 language tags).
pub const EFI_UNICODE_COLLATION2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA4C7_51FC,
    0x23AE,
    0x4C3E,
    [0x92, 0xE9, 0x49, 0x64, 0xCF, 0x63, 0xF3, 0x49],
);

/// The `EFI_UNICODE_COLLATION_PROTOCOL_GUID` (ISO 639-2 language codes).
pub const EFI_UNICODE_COLLATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1D85_6302,
    0x7FFC,
    0x11D3,
    [0x9A, 0x06, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Function pointer for `EFI_UNICODE_COLLATION.StriColl`.
pub type EfiUnicodeCollationStriColl = unsafe extern "efiapi" fn(
    this: *const EfiUnicodeCollation2Protocol,
    s1: *const u16,
    s2: *const u16,
) -> i64;

/// Function pointer for `EFI_UNICODE_COLLATION.MetaiMatch`.
pub type EfiUnicodeCollationMetaiMatch = unsafe extern "efiapi" fn(
    this: *const EfiUnicodeCollation2Protocol,
    string: *const u16,
    pattern: *const u16,
) -> Bool;

/// Function pointer for `EFI_UNICODE_COLLATION.StrLwr`.
pub type EfiUnicodeCollationStrLwr =
    unsafe extern "efiapi" fn(this: *const EfiUnicodeCollation2Protocol, str: *mut u16);

/// Function pointer for `EFI_UNICODE_COLLATION.StrUpr`.
pub type EfiUnicodeCollationStrUpr =
    unsafe extern "efiapi" fn(this: *const EfiUnicodeCollation2Protocol, str: *mut u16);

/// Function pointer for `EFI_UNICODE_COLLATION.FatToStr`.
pub type EfiUnicodeCollationFatToStr = unsafe extern "efiapi" fn(
    this: *const EfiUnicodeCollation2Protocol,
    fat_size: UIntN,
    fat: *const u8,
    string: *mut u16,
);

/// Function pointer for `EFI_UNICODE_COLLATION.StrToFat`.
pub type EfiUnicodeCollationStrToFat = unsafe extern "efiapi" fn(
    this: *const EfiUnicodeCollation2Protocol,
    string: *const u16,
    fat_size: UIntN,
    fat: *mut u8,
) -> Bool;

/// The raw ABI structure for `EFI_UNICODE_COLLATION2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiUnicodeCollation2Protocol {
    /// Case-insensitive comparison of two null-terminated Unicode strings.
    pub stri_coll: EfiUnicodeCollationStriColl,
    /// Pattern matching with wildcard characters ('*' and '?').
    pub metai_match: EfiUnicodeCollationMetaiMatch,
    /// Converts a Unicode string to lower case in-place.
    pub str_lwr: EfiUnicodeCollationStrLwr,
    /// Converts a Unicode string to upper case in-place.
    pub str_upr: EfiUnicodeCollationStrUpr,
    /// Converts an 8.3 FAT filename to a Unicode string.
    pub fat_to_str: EfiUnicodeCollationFatToStr,
    /// Converts a Unicode string to an 8.3 FAT filename.
    pub str_to_fat: EfiUnicodeCollationStrToFat,
    /// Null-terminated list of RFC 4646 language tags supported.
    pub supported_languages: *const u8,
}

/// The raw ABI structure for `EFI_UNICODE_COLLATION_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiUnicodeCollationProtocol {
    /// Case-insensitive comparison of two null-terminated Unicode strings.
    pub stri_coll: unsafe extern "efiapi" fn(
        this: *const EfiUnicodeCollationProtocol,
        s1: *const u16,
        s2: *const u16,
    ) -> i64,
    /// Pattern matching with wildcard characters ('*' and '?').
    pub metai_match: unsafe extern "efiapi" fn(
        this: *const EfiUnicodeCollationProtocol,
        string: *const u16,
        pattern: *const u16,
    ) -> Bool,
    /// Converts a Unicode string to lower case in-place.
    pub str_lwr: unsafe extern "efiapi" fn(this: *const EfiUnicodeCollationProtocol, str: *mut u16),
    /// Converts a Unicode string to upper case in-place.
    pub str_upr: unsafe extern "efiapi" fn(this: *const EfiUnicodeCollationProtocol, str: *mut u16),
    /// Converts an 8.3 FAT filename to a Unicode string.
    pub fat_to_str: unsafe extern "efiapi" fn(
        this: *const EfiUnicodeCollationProtocol,
        fat_size: UIntN,
        fat: *const u8,
        string: *mut u16,
    ),
    /// Converts a Unicode string to an 8.3 FAT filename.
    pub str_to_fat: unsafe extern "efiapi" fn(
        this: *const EfiUnicodeCollationProtocol,
        string: *const u16,
        fat_size: UIntN,
        fat: *mut u8,
    ) -> Bool,
    /// Null-terminated list of ISO 639-2 language codes supported.
    pub supported_languages: *const u8,
}
