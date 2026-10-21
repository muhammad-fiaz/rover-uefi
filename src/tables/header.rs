//! # Table header
//!
//! The header that precedes every `UEFI` system
//! table.
//!
//! The header identifies the table type by
//! signature, carries the specification revision,
//! advertises the table size and holds a checksum.
//! Because the header is the first field of the
//! system, boot services and runtime services
//! tables, its layout is fixed by the specification
//! and must remain exact.

/// The signature of the system table.
///
/// The bytes spell `IBI SYST` in order.
pub const SYSTEM_TABLE_SIGNATURE: u64 = u64::from_le_bytes(*b"IBI SYST");

/// The signature of the boot services table.
///
/// The bytes spell `BOOTSERV` in order.
pub const BOOT_SERVICES_SIGNATURE: u64 = u64::from_le_bytes(*b"BOOTSERV");

/// The signature of the runtime services table.
///
/// The bytes spell `RUNTSERV` in order.
pub const RUNTIME_SERVICES_SIGNATURE: u64 = u64::from_le_bytes(*b"RUNTSERV");

/// The specification revision for `UEFI` 2.0.
pub const EFI_2_00_SYSTEM_TABLE_REVISION: u32 = 2 << 16;
/// The specification revision for `UEFI` 2.1.
pub const EFI_2_10_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 10;
/// The specification revision for `UEFI` 2.2.
pub const EFI_2_20_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 20;
/// The specification revision for `UEFI` 2.3.
pub const EFI_2_30_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 30;
/// The specification revision for `UEFI` 2.3.1.
pub const EFI_2_31_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 31;
/// The specification revision for `UEFI` 2.4.
pub const EFI_2_40_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 40;
/// The specification revision for `UEFI` 2.5.
pub const EFI_2_50_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 50;
/// The specification revision for `UEFI` 2.6.
pub const EFI_2_60_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 60;
/// The specification revision for `UEFI` 2.7.
pub const EFI_2_70_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 70;
/// The specification revision for `UEFI` 2.8.
pub const EFI_2_80_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 80;
/// The specification revision for `UEFI` 2.9.
pub const EFI_2_90_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 90;
/// The specification revision for `UEFI` 2.10.
pub const EFI_2_100_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 100;
/// The specification revision for `UEFI` 2.11.
pub const EFI_2_110_SYSTEM_TABLE_REVISION: u32 = (2 << 16) + 110;

/// The `UEFI` 2.11 specification revision.
///
/// Firmware reports revisions using the decimal minor version in the
/// low 16 bits, so 2.11 is encoded as `(2 << 16) | 110`.
pub const EFI_SPECIFICATION_VERSION: u32 = EFI_2_110_SYSTEM_TABLE_REVISION;

/// A `UEFI` table header.
///
/// The struct is `#[repr(C)]` so field order, size
/// and alignment match the firmware definition:
/// a 64-bit signature, a 32-bit revision, a 32-bit
/// header size, a 32-bit checksum and a 32-bit
/// reserved field. The total size is 24 bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiTableHeader {
    /// The 64-bit signature that identifies the
    /// table type.
    pub signature: u64,
    /// The revision of the specification.
    ///
    /// The upper 16 bits hold the major revision
    /// and the lower 16 bits hold the minor
    /// revision.
    pub revision: u32,
    /// The size of the entire table in bytes,
    /// including this header.
    pub header_size: u32,
    /// The 32-bit checksum of the table.
    ///
    /// The checksum is computed with this field
    /// set to zero over `header_size` bytes.
    pub crc32: u32,
    /// A reserved field that must be zero.
    pub reserved: u32,
}

impl EfiTableHeader {
    /// Creates a header from its fields.
    #[must_use]
    pub const fn new(
        signature: u64,
        revision: u32,
        header_size: u32,
        crc32: u32,
        reserved: u32,
    ) -> Self {
        Self {
            signature,
            revision,
            header_size,
            crc32,
            reserved,
        }
    }

    /// Returns the major specification revision.
    #[must_use]
    pub const fn major_revision(&self) -> u16 {
        (self.revision >> 16) as u16
    }

    /// Returns the minor specification revision.
    #[must_use]
    pub const fn minor_revision(&self) -> u16 {
        let bytes = self.revision.to_le_bytes();
        u16::from_le_bytes([bytes[0], bytes[1]])
    }

    /// Returns whether the header advertises at least `revision`.
    #[must_use]
    pub const fn supports_revision(&self, revision: u32) -> bool {
        self.revision >= revision
    }

    /// Returns whether the signature matches `expected`.
    #[must_use]
    pub const fn has_signature(&self, expected: u64) -> bool {
        self.signature == expected
    }

    /// Verifies the CRC-32 checksum of a complete table image.
    ///
    /// `image` must begin at the header and contain exactly
    /// `header.header_size` bytes. The comparison uses the same
    /// CRC algorithm as the firmware `CalculateCrc32` service.
    ///
    /// Returns `false` when the image is shorter than the declared
    /// header size or when the stored checksum does not match.
    #[must_use]
    pub fn checksum_valid(&self, image: &[u8]) -> bool {
        let size = self.header_size as usize;
        if size < core::mem::size_of::<Self>() {
            return false;
        }
        let Some(covered) = image.get(..size) else {
            return false;
        };
        // The checksum field is 20 bytes into the header; compute over a
        // scratch copy so the caller's bytes are not mutated.
        let mut state = 0xFFFF_FFFF_u32;
        state = crate::base::crc::crc32_update(state, &covered[..20]);
        state = crate::base::crc::crc32_update(state, &[0_u8; 4]);
        state = crate::base::crc::crc32_update(state, &covered[24..]);
        (state ^ 0xFFFF_FFFF) == self.crc32
    }

    /// Computes the checksum value for a table image.
    ///
    /// `image` must begin at the header and contain `header_size`
    /// bytes. The stored checksum field is ignored while computing.
    ///
    /// Returns `None` when `image` is shorter than `header_size`.
    #[must_use]
    pub fn compute_checksum(&self, image: &[u8]) -> Option<u32> {
        let size = self.header_size as usize;
        if size < core::mem::size_of::<Self>() {
            return None;
        }
        let covered = image.get(..size)?;
        let mut state = 0xFFFF_FFFF_u32;
        state = crate::base::crc::crc32_update(state, &covered[..20]);
        state = crate::base::crc::crc32_update(state, &[0_u8; 4]);
        state = crate::base::crc::crc32_update(state, &covered[24..]);
        Some(state ^ 0xFFFF_FFFF)
    }

    /// Returns whether the header metadata is self-consistent.
    ///
    /// Checks that the declared size covers the header, the signature
    /// is not zero and the reserved field is zero. This does not verify
    /// the checksum; call [`EfiTableHeader::checksum_valid`] for that.
    #[must_use]
    pub const fn is_consistent(&self) -> bool {
        self.header_size as usize >= core::mem::size_of::<Self>()
            && self.signature != 0
            && self.reserved == 0
    }
}

impl core::fmt::Debug for EfiTableHeader {
    /// Formats the header as its fields.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("EfiTableHeader")
            .field("signature", &self.signature)
            .field("revision", &self.revision)
            .field("header_size", &self.header_size)
            .field("crc32", &self.crc32)
            .field("reserved", &self.reserved)
            .finish()
    }
}
