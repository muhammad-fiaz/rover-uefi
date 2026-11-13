//! # TPM2
//!
//! ACPI Trusted Platform Module 2.0 (TPM2) table parsing.

use crate::protocols::acpi::header::parse_header;
use crate::protocols::acpi::types::SIG_TPM2;

/// TPM2 platform class.
pub mod platform_class {
    /// Client PC / Mobile TPM.
    pub const CLIENT: u16 = 0;
    /// Server platform TPM.
    pub const SERVER: u16 = 1;
}

/// TPM2 start method.
pub mod start_method {
    /// ACPI start method.
    pub const ACPI: u32 = 1;
    /// Trusted Identity Support / Hardware start method.
    pub const TIS: u32 = 2;
    /// Memory mapped I/O Command Response Buffer (CRB).
    pub const CRB: u32 = 6;
    /// Command Response Buffer with ACPI start method.
    pub const CRB_WITH_ACPI: u32 = 7;
    /// Command Response Buffer with ARM SMC/HVC start method.
    pub const CRB_WITH_SMC: u32 = 8;
}

/// Parsed TPM2 table info.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tpm2Info {
    /// Platform class (0 = Client, 1 = Server).
    pub platform_class: u16,
    /// Physical address of the TPM2 Control Area.
    pub control_area_address: u64,
    /// Start method identifier.
    pub start_method: u32,
    /// Start method specific parameters (e.g. SMC function ID).
    pub start_method_parameters: [u8; 12],
    /// Log area minimum length in bytes (if table length >= 76).
    pub log_area_minimum_length: Option<u32>,
    /// Physical address of the TCPA/TPM2 event log (if table length >= 76).
    pub log_area_start_address: Option<u64>,
}

/// Parses an ACPI TPM2 table buffer.
///
/// Returns `None` if the buffer is shorter than the 52-byte minimum
/// TPM2 structure or if the ACPI signature/header is invalid.
#[must_use]
pub fn parse_tpm2(buffer: &[u8]) -> Option<Tpm2Info> {
    let header = parse_header(buffer)?;
    if header.signature != SIG_TPM2 {
        return None;
    }

    let table_len = usize::try_from(header.length).ok()?;
    if table_len < 52 || buffer.len() < 52 {
        return None;
    }

    let platform_class = u16::from_le_bytes([*buffer.get(36)?, *buffer.get(37)?]);

    let mut ctrl_addr_raw = [0_u8; 8];
    ctrl_addr_raw.copy_from_slice(buffer.get(40..48)?);
    let control_area_address = u64::from_le_bytes(ctrl_addr_raw);

    let mut start_method_raw = [0_u8; 4];
    start_method_raw.copy_from_slice(buffer.get(48..52)?);
    let start_method = u32::from_le_bytes(start_method_raw);

    let mut start_method_parameters = [0_u8; 12];
    if table_len >= 64 && buffer.len() >= 64 {
        start_method_parameters.copy_from_slice(buffer.get(52..64)?);
    }

    let (log_area_min_len, log_area_start_addr) = if table_len >= 76 && buffer.len() >= 76 {
        let mut len_raw = [0_u8; 4];
        len_raw.copy_from_slice(buffer.get(64..68)?);
        let mut addr_raw = [0_u8; 8];
        addr_raw.copy_from_slice(buffer.get(68..76)?);
        (
            Some(u32::from_le_bytes(len_raw)),
            Some(u64::from_le_bytes(addr_raw)),
        )
    } else {
        (None, None)
    };

    Some(Tpm2Info {
        platform_class,
        control_area_address,
        start_method,
        start_method_parameters,
        log_area_minimum_length: log_area_min_len,
        log_area_start_address: log_area_start_addr,
    })
}
