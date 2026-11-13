//! # FPDT
//!
//! ACPI Firmware Performance Data Table (FPDT) parsing and performance record analysis.
//!
//! Conforms to ACPI Specification Section 5.2.23 and Platform Firmware Performance
//! Data Table architecture. Tracks nanosecond-accurate boot and S3 suspend/resume
//! timestamps across SEC, PEI, DXE, and BDS phases.

use crate::base::EfiGuid;
use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_FPDT;

/// FPDT Root Record Type: Basic Boot Performance Table Pointer.
pub const FPDT_RECORD_TYPE_FIRMWARE_BASIC_BOOT_POINTER: u16 = 0x0000;
/// FPDT Root Record Type: S3 Performance Table Pointer.
pub const FPDT_RECORD_TYPE_S3_PERFORMANCE_TABLE_POINTER: u16 = 0x0001;

/// FPDT Performance Record Type: Basic S3 Resume.
pub const FPDT_RUNTIME_RECORD_TYPE_S3_RESUME: u16 = 0x0000;
/// FPDT Performance Record Type: Basic S3 Suspend.
pub const FPDT_RUNTIME_RECORD_TYPE_S3_SUSPEND: u16 = 0x0001;
/// FPDT Performance Record Type: Basic Boot.
pub const FPDT_RUNTIME_RECORD_TYPE_FIRMWARE_BASIC_BOOT: u16 = 0x0002;

/// Extended Performance Record Type: GUID Event Record.
pub const FPDT_RECORD_TYPE_GUID_EVENT: u16 = 0x1010;
/// Extended Performance Record Type: Dynamic String Event Record.
pub const FPDT_RECORD_TYPE_DYNAMIC_STRING_EVENT: u16 = 0x1011;
/// Extended Performance Record Type: Dual GUID String Event Record.
pub const FPDT_RECORD_TYPE_DUAL_GUID_STRING_EVENT: u16 = 0x1012;
/// Extended Performance Record Type: GUID QWORD Event Record.
pub const FPDT_RECORD_TYPE_GUID_QWORD_EVENT: u16 = 0x1013;
/// Extended Performance Record Type: GUID QWORD String Event Record.
pub const FPDT_RECORD_TYPE_GUID_QWORD_STRING_EVENT: u16 = 0x1014;

/// Signature for Firmware Basic Boot Performance Table (`FBPT`).
pub const SIG_FBPT: [u8; 4] = *b"FBPT";
/// Signature for S3 Performance Table (`S3PT`).
pub const SIG_S3PT: [u8; 4] = *b"S3PT";

/// Common FPDT Performance Record Header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct FpdtRecordHeader {
    /// Performance record type identifier.
    pub record_type: u16,
    /// Total record length in bytes.
    pub length: u8,
    /// Performance record revision.
    pub revision: u8,
}

/// Common Performance Table Header (`FBPT` or `S3PT`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct FpdtTableHeader {
    /// 4-byte table signature (e.g. `FBPT` or `S3PT`).
    pub signature: [u8; 4],
    /// Length of the performance table in bytes.
    pub length: u32,
}

/// Parsed FPDT Root Table pointers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FpdtTableInfo {
    /// Physical 64-bit address of the Firmware Basic Boot Performance Table (`FBPT`).
    pub boot_performance_table_address: Option<u64>,
    /// Physical 64-bit address of the S3 Performance Table (`S3PT`).
    pub s3_performance_table_address: Option<u64>,
}

/// Parsed Basic Boot Performance Record.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FpdtBasicBootRecord {
    /// Timestamp when reset was de-asserted (nanoseconds).
    pub reset_end: u64,
    /// Timestamp when OS loader image was loaded (nanoseconds).
    pub os_loader_load_image_start: u64,
    /// Timestamp when OS loader image was executed (nanoseconds).
    pub os_loader_start_image_start: u64,
    /// Timestamp when `ExitBootServices` was invoked (nanoseconds).
    pub exit_boot_services_entry: u64,
    /// Timestamp when `ExitBootServices` returned (nanoseconds).
    pub exit_boot_services_exit: u64,
}

/// Parsed S3 Resume Performance Record.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FpdtS3ResumeRecord {
    /// Count of S3 resume cycles since full boot.
    pub resume_count: u32,
    /// Nanosecond duration of the most recent full S3 resume cycle.
    pub full_resume: u64,
    /// Average nanosecond duration of S3 resume across all cycles.
    pub average_resume: u64,
}

/// Parsed S3 Suspend Performance Record.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FpdtS3SuspendRecord {
    /// Timestamp when OS wrote to `SLP_TYP` (nanoseconds).
    pub suspend_start: u64,
    /// Timestamp when firmware completed suspend operations (nanoseconds).
    pub suspend_end: u64,
}

/// Parsed Extended GUID Event Record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FpdtGuidEventRecord {
    /// Progress identifier (start/stop markers).
    pub progress_id: u16,
    /// APIC ID of processor clock source.
    pub apic_id: u32,
    /// Nanoseconds elapsed since processor reset.
    pub timestamp: u64,
    /// Referenced module GUID.
    pub guid: EfiGuid,
}

/// Parsed Extended String Event Record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FpdtStringEventRecord {
    /// Progress identifier.
    pub progress_id: u16,
    /// APIC ID of processor clock source.
    pub apic_id: u32,
    /// Nanoseconds elapsed since processor reset.
    pub timestamp: u64,
    /// Description token (up to 24 ASCII characters).
    pub description_length: usize,
    /// ASCII bytes.
    pub description: [u8; 24],
}

/// Discriminated FPDT Performance Record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpdtParsedRecord {
    /// Basic Boot Performance record.
    BasicBoot(FpdtBasicBootRecord),
    /// S3 Resume Performance record.
    S3Resume(FpdtS3ResumeRecord),
    /// S3 Suspend Performance record.
    S3Suspend(FpdtS3SuspendRecord),
    /// Extended GUID Event record.
    GuidEvent(FpdtGuidEventRecord),
    /// Extended String Event record.
    StringEvent(FpdtStringEventRecord),
    /// Unrecognized or raw record.
    Unknown {
        /// Record type.
        record_type: u16,
        /// Record byte length.
        length: u8,
    },
}

/// Parses the root ACPI FPDT table.
///
/// Returns `None` if the table is malformed or invalid.
#[must_use]
pub fn parse_fpdt_root(buffer: &[u8]) -> Option<FpdtTableInfo> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_FPDT {
        return None;
    }
    let table_len = usize::try_from(header.length).ok()?;
    if buffer.len() < table_len || table_len < 36 {
        return None;
    }

    let mut info = FpdtTableInfo::default();
    let mut offset = 36;

    while offset + 4 <= table_len {
        let rec_type = u16::from_le_bytes([buffer[offset], buffer[offset + 1]]);
        let rec_len = buffer[offset + 2] as usize;
        if rec_len < 4 || offset + rec_len > table_len {
            break;
        }

        match rec_type {
            FPDT_RECORD_TYPE_FIRMWARE_BASIC_BOOT_POINTER if rec_len >= 16 => {
                let mut ptr_bytes = [0u8; 8];
                ptr_bytes.copy_from_slice(&buffer[offset + 8..offset + 16]);
                info.boot_performance_table_address = Some(u64::from_le_bytes(ptr_bytes));
            }
            FPDT_RECORD_TYPE_S3_PERFORMANCE_TABLE_POINTER if rec_len >= 16 => {
                let mut ptr_bytes = [0u8; 8];
                ptr_bytes.copy_from_slice(&buffer[offset + 8..offset + 16]);
                info.s3_performance_table_address = Some(u64::from_le_bytes(ptr_bytes));
            }
            _ => {}
        }

        offset += rec_len;
    }

    Some(info)
}

/// Iterator over individual performance records within a performance table (`FBPT` or `S3PT`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FpdtRecordIter<'a> {
    buffer: &'a [u8],
    offset: usize,
    table_length: usize,
}

impl<'a> FpdtRecordIter<'a> {
    /// Creates an iterator over performance records in a table buffer.
    ///
    /// Validates the 8-byte table header (`FBPT` or `S3PT`).
    #[must_use]
    pub fn new(buffer: &'a [u8]) -> Option<Self> {
        if buffer.len() < 8 {
            return None;
        }
        let sig = [buffer[0], buffer[1], buffer[2], buffer[3]];
        if sig != SIG_FBPT && sig != SIG_S3PT {
            return None;
        }
        let table_len = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]) as usize;
        let bounded_len = core::cmp::min(table_len, buffer.len());
        Some(Self {
            buffer,
            offset: 8,
            table_length: bounded_len,
        })
    }
}

impl Iterator for FpdtRecordIter<'_> {
    type Item = FpdtParsedRecord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + 4 > self.table_length {
            return None;
        }

        let rec_type = u16::from_le_bytes([self.buffer[self.offset], self.buffer[self.offset + 1]]);
        let rec_len = self.buffer[self.offset + 2] as usize;
        if rec_len < 4 || self.offset + rec_len > self.table_length {
            return None;
        }

        let slice = &self.buffer[self.offset..self.offset + rec_len];
        self.offset += rec_len;

        let parsed = match rec_type {
            FPDT_RUNTIME_RECORD_TYPE_FIRMWARE_BASIC_BOOT if rec_len >= 48 => {
                let mut re = [0u8; 8];
                let mut os_load = [0u8; 8];
                let mut os_start = [0u8; 8];
                let mut ebs_entry = [0u8; 8];
                let mut ebs_exit = [0u8; 8];

                re.copy_from_slice(&slice[8..16]);
                os_load.copy_from_slice(&slice[16..24]);
                os_start.copy_from_slice(&slice[24..32]);
                ebs_entry.copy_from_slice(&slice[32..40]);
                ebs_exit.copy_from_slice(&slice[40..48]);

                FpdtParsedRecord::BasicBoot(FpdtBasicBootRecord {
                    reset_end: u64::from_le_bytes(re),
                    os_loader_load_image_start: u64::from_le_bytes(os_load),
                    os_loader_start_image_start: u64::from_le_bytes(os_start),
                    exit_boot_services_entry: u64::from_le_bytes(ebs_entry),
                    exit_boot_services_exit: u64::from_le_bytes(ebs_exit),
                })
            }
            FPDT_RUNTIME_RECORD_TYPE_S3_RESUME if rec_len >= 24 => {
                let mut count = [0u8; 4];
                let mut full = [0u8; 8];
                let mut avg = [0u8; 8];

                count.copy_from_slice(&slice[4..8]);
                full.copy_from_slice(&slice[8..16]);
                avg.copy_from_slice(&slice[16..24]);

                FpdtParsedRecord::S3Resume(FpdtS3ResumeRecord {
                    resume_count: u32::from_le_bytes(count),
                    full_resume: u64::from_le_bytes(full),
                    average_resume: u64::from_le_bytes(avg),
                })
            }
            FPDT_RUNTIME_RECORD_TYPE_S3_SUSPEND if rec_len >= 20 => {
                let mut start = [0u8; 8];
                let mut end = [0u8; 8];

                start.copy_from_slice(&slice[4..12]);
                end.copy_from_slice(&slice[12..20]);

                FpdtParsedRecord::S3Suspend(FpdtS3SuspendRecord {
                    suspend_start: u64::from_le_bytes(start),
                    suspend_end: u64::from_le_bytes(end),
                })
            }
            FPDT_RECORD_TYPE_GUID_EVENT if rec_len >= 34 => {
                let mut prog = [0u8; 2];
                let mut apic = [0u8; 4];
                let mut ts = [0u8; 8];
                let mut guid_bytes = [0u8; 16];

                prog.copy_from_slice(&slice[4..6]);
                apic.copy_from_slice(&slice[6..10]);
                ts.copy_from_slice(&slice[10..18]);
                guid_bytes.copy_from_slice(&slice[18..34]);

                FpdtParsedRecord::GuidEvent(FpdtGuidEventRecord {
                    progress_id: u16::from_le_bytes(prog),
                    apic_id: u32::from_le_bytes(apic),
                    timestamp: u64::from_le_bytes(ts),
                    guid: EfiGuid::from_bytes(guid_bytes),
                })
            }
            FPDT_RECORD_TYPE_DYNAMIC_STRING_EVENT if rec_len >= 18 => {
                let mut prog = [0u8; 2];
                let mut apic = [0u8; 4];
                let mut ts = [0u8; 8];

                prog.copy_from_slice(&slice[4..6]);
                apic.copy_from_slice(&slice[6..10]);
                ts.copy_from_slice(&slice[10..18]);

                let string_slice = &slice[18..];
                let mut desc = [0u8; 24];
                let copy_len = core::cmp::min(string_slice.len(), 24);
                desc[..copy_len].copy_from_slice(&string_slice[..copy_len]);

                FpdtParsedRecord::StringEvent(FpdtStringEventRecord {
                    progress_id: u16::from_le_bytes(prog),
                    apic_id: u32::from_le_bytes(apic),
                    timestamp: u64::from_le_bytes(ts),
                    description_length: copy_len,
                    description: desc,
                })
            }
            _ => FpdtParsedRecord::Unknown {
                record_type: rec_type,
                length: u8::try_from(rec_len).unwrap_or(u8::MAX),
            },
        };

        Some(parsed)
    }
}
