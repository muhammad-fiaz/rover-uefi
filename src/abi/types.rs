//! # Primitive types
//!
//! Low-level integer and character types that mirror the
//! primitive `C` types used by the `UEFI` specification (`UINTN`,
//! `INTN`, `CHAR8`, `CHAR16` and `BOOLEAN`).
//!
//! All types in this module are direct aliases for the matching
//! Rust primitive. The mapping preserves exact width and alignment
//! with the `UEFI` definitions on every supported target.

/// The `UEFI` boolean type.
///
/// The specification defines `BOOLEAN` as an unsigned 8-bit
/// value. Only `0` (false) and `1` (true) are meaningful, but
/// firmware returns are compared against [`EFI_FALSE`] and
/// [`EFI_TRUE`] rather than interpreted as a Rust `bool`.
pub type Bool = u8;

/// The `UEFI` 8-bit character type.
///
/// This matches the `CHAR8` type in the specification. It
/// represents a single byte, typically part of an `ASCII`
/// string.
pub type Char8 = u8;

/// The `UEFI` 16-bit character type.
///
/// This matches the `CHAR16` type in the specification. It
/// represents one UTF-16 code unit, as used by text console
/// protocols and firmware vendor strings.
pub type Char16 = u16;

/// The pointer-sized unsigned integer type.
///
/// This matches `UINTN` in the specification. Its width
/// follows the target pointer width: 32 bits on 32-bit
/// targets and 64 bits on 64-bit targets. Protocol and
/// table fields that are declared as `UINTN` must use
/// this alias so the layout adapts to the target.
pub type UIntN = usize;

/// The pointer-sized signed integer type.
///
/// This matches `INTN` in the specification and pairs
/// with [`UIntN`] as its signed counterpart.
pub type IntN = isize;

/// The false value of a [`Bool`].
///
/// The specification defines the false value as `0`.
pub const EFI_FALSE: Bool = 0;

/// The true value of a [`Bool`].
///
/// The specification defines the true value as `1`.
pub const EFI_TRUE: Bool = 1;

/// The size of one `UEFI` page in bytes.
///
/// Boot services page allocation works in units of
/// 4 KiB pages. This value does not describe the processor
/// page size.
pub const EFI_PAGE_SIZE: UIntN = 4096;

/// The number of low bits in an `UEFI` page offset.
///
/// The shift between byte addresses and page counts
/// is 12, because a page contains 4096 bytes.
pub const EFI_PAGE_SHIFT: u32 = 12;

/// The mask of the low bits of an `UEFI` page offset.
///
/// The mask selects bits `0` through `11` of a byte
/// address that lie within a page.
pub const EFI_PAGE_MASK: UIntN = EFI_PAGE_SIZE - 1;

/// Converts a byte size to a number of `UEFI` pages.
///
/// The result is the smallest number of pages that can
/// hold the requested size, rounding up when the size
/// is not a multiple of [`EFI_PAGE_SIZE`].
///
/// The caller provides a size that fits in [`UIntN`].
/// No overflow check is required at the page granularity
/// because the shift and mask operations are defined
/// for every value of [`UIntN`].
#[must_use]
pub const fn size_to_pages(size: UIntN) -> UIntN {
    (size >> EFI_PAGE_SHIFT) + if size & EFI_PAGE_MASK != 0 { 1 } else { 0 }
}

/// Converts a number of `UEFI` pages to a byte size.
///
/// The result is the page count multiplied by
/// [`EFI_PAGE_SIZE`].
///
/// Returns `None` when the multiplication would overflow
/// [`UIntN`]. Callers that construct a page request from
/// an unchecked size must handle the overflow case.
#[must_use]
pub const fn pages_to_size(pages: UIntN) -> Option<UIntN> {
    pages.checked_mul(EFI_PAGE_SIZE)
}
