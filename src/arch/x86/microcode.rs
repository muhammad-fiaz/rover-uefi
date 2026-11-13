//! # x86 Processor Microcode Update Structures
//!
//! Provides structures and validation routines for Intel and AMD x86/x86-64 processor
//! microcode update binary patches, headers, processor signatures, and extended signature tables.
//!
//! Based on standard x86 Processor Architecture and Initialization Specifications.

/// CPU Microcode Date in BCD format (year, day, month).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C, packed)]
pub struct CpuMicrocodeDate {
    /// Year in BCD format (e.g. 0x2024).
    pub year: u16,
    /// Day in BCD format (e.g. 0x15).
    pub day: u8,
    /// Month in BCD format (e.g. 0x07).
    pub month: u8,
}

impl CpuMicrocodeDate {
    /// Creates a new BCD-encoded date.
    #[must_use]
    pub const fn new(year_bcd: u16, month_bcd: u8, day_bcd: u8) -> Self {
        Self {
            year: year_bcd,
            day: day_bcd,
            month: month_bcd,
        }
    }

    /// Decodes the BCD year into a binary integer.
    #[must_use]
    pub const fn decoded_year(&self) -> u32 {
        let raw = self.year as u32;
        let d0 = raw & 0x0F;
        let d1 = (raw >> 4) & 0x0F;
        let d2 = (raw >> 8) & 0x0F;
        let d3 = (raw >> 12) & 0x0F;
        d0 + d1 * 10 + d2 * 100 + d3 * 1000
    }

    /// Decodes the BCD month into a binary integer (1..12).
    #[must_use]
    pub const fn decoded_month(&self) -> u8 {
        let raw = self.month;
        (raw & 0x0F) + ((raw >> 4) & 0x0F) * 10
    }

    /// Decodes the BCD day into a binary integer (1..31).
    #[must_use]
    pub const fn decoded_day(&self) -> u8 {
        let raw = self.day;
        (raw & 0x0F) + ((raw >> 4) & 0x0F) * 10
    }
}

/// CPU Microcode Processor Signature.
///
/// Encodes stepping, model, family, processor type, extended model, and extended family
/// matching the bit layout returned by the x86 `CPUID.01H:EAX` instruction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CpuMicrocodeProcessorSignature(pub u32);

impl CpuMicrocodeProcessorSignature {
    /// Creates a processor signature from raw CPUID value.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw 32-bit CPUID value.
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Returns stepping ID (bits 0..3).
    #[must_use]
    pub const fn stepping(self) -> u8 {
        (self.0 & 0x0F) as u8
    }

    /// Returns base model ID (bits 4..7).
    #[must_use]
    pub const fn model(self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    /// Returns base family ID (bits 8..11).
    #[must_use]
    pub const fn family(self) -> u8 {
        ((self.0 >> 8) & 0x0F) as u8
    }

    /// Returns processor type (bits 12..13).
    #[must_use]
    pub const fn processor_type(self) -> u8 {
        ((self.0 >> 12) & 0x03) as u8
    }

    /// Returns extended model ID (bits 16..19).
    #[must_use]
    pub const fn extended_model(self) -> u8 {
        ((self.0 >> 16) & 0x0F) as u8
    }

    /// Returns extended family ID (bits 20..27).
    #[must_use]
    pub const fn extended_family(self) -> u8 {
        ((self.0 >> 20) & 0xFF) as u8
    }

    /// Computes the complete effective family ID.
    #[must_use]
    pub const fn effective_family(self) -> u32 {
        let base = self.family() as u32;
        if base == 0x0F {
            base + (self.extended_family() as u32)
        } else {
            base
        }
    }

    /// Computes the complete effective model ID.
    #[must_use]
    pub const fn effective_model(self) -> u32 {
        let base = self.model() as u32;
        let fam = self.family();
        if fam == 0x06 || fam == 0x0F {
            (self.extended_model() as u32) << 4 | base
        } else {
            base
        }
    }
}

/// Standard x86 CPU Microcode Update Binary Header (48 bytes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct CpuMicrocodeHeader {
    /// Version number of the microcode update header (typically 1).
    pub header_version: u32,
    /// Update revision number.
    pub update_revision: u32,
    /// Creation date in BCD format.
    pub date: CpuMicrocodeDate,
    /// Target processor signature (matches CPUID 1 EAX).
    pub processor_signature: CpuMicrocodeProcessorSignature,
    /// 32-bit checksum of update data and header (sum of all DWORDs must be 0).
    pub checksum: u32,
    /// Loader program revision (typically 1).
    pub loader_revision: u32,
    /// Platform flags bitmask matching MSR `IA32_PLATFORM_ID`.
    pub processor_flags: u32,
    /// Encrypted data size in bytes (if 0, data size is default 2000 bytes).
    pub data_size: u32,
    /// Total microcode update size in bytes (if 0, total size is default 2048 bytes).
    pub total_size: u32,
    /// Reserved fields (12 bytes).
    pub reserved: [u8; 12],
}

impl CpuMicrocodeHeader {
    /// Default data payload size in bytes when `data_size == 0`.
    pub const DEFAULT_DATA_SIZE: usize = 2000;
    /// Default total microcode size in bytes when `total_size == 0`.
    pub const DEFAULT_TOTAL_SIZE: usize = 2048;
    /// Size of the microcode update header itself in bytes.
    pub const HEADER_SIZE: usize = 48;

    /// Safely reads the `header_version` field.
    #[must_use]
    pub const fn header_version(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.header_version).read_unaligned() }
    }

    /// Safely reads the `update_revision` field.
    #[must_use]
    pub const fn update_revision(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.update_revision).read_unaligned() }
    }

    /// Safely reads the `checksum` field.
    #[must_use]
    pub const fn checksum(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.checksum).read_unaligned() }
    }

    /// Safely reads the `loader_revision` field.
    #[must_use]
    pub const fn loader_revision(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.loader_revision).read_unaligned() }
    }

    /// Safely reads the `processor_flags` field.
    #[must_use]
    pub const fn processor_flags(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.processor_flags).read_unaligned() }
    }

    /// Safely reads the `processor_signature` field.
    #[must_use]
    pub const fn processor_signature(&self) -> CpuMicrocodeProcessorSignature {
        unsafe { core::ptr::addr_of!(self.processor_signature).read_unaligned() }
    }

    /// Returns the data payload size in bytes.
    #[must_use]
    pub const fn data_size_bytes(&self) -> usize {
        let size = unsafe { core::ptr::addr_of!(self.data_size).read_unaligned() };
        if size == 0 {
            Self::DEFAULT_DATA_SIZE
        } else {
            size as usize
        }
    }

    /// Returns the total microcode update patch size in bytes.
    #[must_use]
    pub const fn total_size_bytes(&self) -> usize {
        let size = unsafe { core::ptr::addr_of!(self.total_size).read_unaligned() };
        if size == 0 {
            Self::DEFAULT_TOTAL_SIZE
        } else {
            size as usize
        }
    }

    /// Checks if this patch targets the given processor signature and platform flags.
    #[must_use]
    pub fn matches_processor(&self, cpuid_signature: u32, platform_id_flag: u32) -> bool {
        let sig = self.processor_signature().raw();
        let flags = self.processor_flags();
        sig == cpuid_signature && ((flags & platform_id_flag) != 0 || flags == 0)
    }

    /// Verifies the 32-bit checksum of the entire microcode buffer.
    ///
    /// Summation of all 32-bit DWORDs across the total microcode size must equal 0 modulo 2^32.
    #[must_use]
    pub fn verify_checksum(&self, buffer: &[u8]) -> bool {
        let total = self.total_size_bytes();
        if buffer.len() < total || total % 4 != 0 {
            return false;
        }

        let mut sum: u32 = 0;
        let mut offset = 0;
        while offset < total {
            let chunk = [
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ];
            let dword = u32::from_le_bytes(chunk);
            sum = sum.wrapping_add(dword);
            offset += 4;
        }

        sum == 0
    }
}

/// Extended Processor Signature Table Header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct CpuMicrocodeExtendedTableHeader {
    /// Number of extended signature entries that follow.
    pub extended_signature_count: u32,
    /// Checksum of the extended table.
    pub extended_checksum: u32,
    /// Reserved fields.
    pub reserved: [u8; 12],
}

impl CpuMicrocodeExtendedTableHeader {
    /// Safely reads the entry count.
    #[must_use]
    pub const fn count(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.extended_signature_count).read_unaligned() }
    }

    /// Safely reads the checksum.
    #[must_use]
    pub const fn checksum(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.extended_checksum).read_unaligned() }
    }
}

/// Extended Processor Signature Entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct CpuMicrocodeExtendedTableEntry {
    /// Alternate processor signature supported by this patch.
    pub processor_signature: CpuMicrocodeProcessorSignature,
    /// Alternate processor platform flags.
    pub processor_flag: u32,
    /// Checksum modifier for this entry.
    pub checksum: u32,
}

impl CpuMicrocodeExtendedTableEntry {
    /// Safely reads the processor signature.
    #[must_use]
    pub const fn processor_signature(&self) -> CpuMicrocodeProcessorSignature {
        unsafe { core::ptr::addr_of!(self.processor_signature).read_unaligned() }
    }

    /// Safely reads the processor flag.
    #[must_use]
    pub const fn processor_flag(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.processor_flag).read_unaligned() }
    }

    /// Safely reads the checksum.
    #[must_use]
    pub const fn checksum(&self) -> u32 {
        unsafe { core::ptr::addr_of!(self.checksum).read_unaligned() }
    }
}

/// Calculates the 32-bit checksum value required to make the summation of `dwords` equal 0.
#[must_use]
pub fn calculate_microcode_checksum(dwords: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    for &dw in dwords {
        sum = sum.wrapping_add(dw);
    }
    0u32.wrapping_sub(sum)
}
