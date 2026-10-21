//! # FACS
//!
//! Firmware ACPI Control Structure parsing.
//!
//! The table has no 36-byte system description
//! header. Fields begin at the table start.

use crate::protocols::acpi::checksum::acpi_checksum_ok;

/// Parsed FACS data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FacsInfo {
    /// The hardware signature.
    pub hardware_signature: u32,
    /// The firmware waking vector.
    pub firmware_waking_vector: u32,
    /// The global lock.
    pub global_lock: u32,
    /// The flags.
    pub flags: u32,
    /// The extended firmware waking vector.
    pub x_firmware_waking_vector: u64,
    /// The version.
    pub version: u8,
    /// The OSPM flags.
    pub ospm_flags: u32,
}

/// Parses a FACS buffer.
///
/// Returns `None` when the buffer is shorter
/// than 40 bytes, the signature or length is
/// invalid, or the checksum fails.
#[must_use]
pub fn parse_facs(buffer: &[u8]) -> Option<FacsInfo> {
    if buffer.len() < 40 {
        return None;
    }
    let mut signature = [0_u8; 4];
    signature.copy_from_slice(buffer.get(0..4)?);
    if signature != *b"FACS" {
        return None;
    }
    let mut length_raw = [0_u8; 4];
    length_raw.copy_from_slice(buffer.get(4..8)?);
    let length = u32::from_le_bytes(length_raw);
    let table_length = usize::try_from(length).ok()?;
    if table_length < 40 || table_length > buffer.len() {
        return None;
    }
    if !acpi_checksum_ok(buffer.get(..table_length)?) {
        return None;
    }
    let mut hardware_signature_raw = [0_u8; 4];
    hardware_signature_raw.copy_from_slice(buffer.get(8..12)?);
    let mut firmware_waking_vector_raw = [0_u8; 4];
    firmware_waking_vector_raw.copy_from_slice(buffer.get(12..16)?);
    let mut global_lock_raw = [0_u8; 4];
    global_lock_raw.copy_from_slice(buffer.get(16..20)?);
    let mut flags_raw = [0_u8; 4];
    flags_raw.copy_from_slice(buffer.get(20..24)?);
    let mut x_firmware_waking_vector_raw = [0_u8; 8];
    x_firmware_waking_vector_raw.copy_from_slice(buffer.get(24..32)?);
    let mut ospm_flags_raw = [0_u8; 4];
    ospm_flags_raw.copy_from_slice(buffer.get(36..40)?);
    Some(FacsInfo {
        hardware_signature: u32::from_le_bytes(hardware_signature_raw),
        firmware_waking_vector: u32::from_le_bytes(firmware_waking_vector_raw),
        global_lock: u32::from_le_bytes(global_lock_raw),
        flags: u32::from_le_bytes(flags_raw),
        x_firmware_waking_vector: u64::from_le_bytes(x_firmware_waking_vector_raw),
        version: *buffer.get(32)?,
        ospm_flags: u32::from_le_bytes(ospm_flags_raw),
    })
}
