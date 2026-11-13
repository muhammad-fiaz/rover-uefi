//! # Extended Display Identification Data (EDID) Protocols and Parser
//!
//! Defined in UEFI Specification 2.10 Section 12.9.
//!
//! Contains EDID Discovered, EDID Active, and EDID Override protocol definitions
//! as well as safe VESA EDID 1.3/1.4 byte structure parsing (manufacturer ID,
//! timing descriptors, preferred resolution, and display capabilities).

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiHandle, EfiStatus};

pub use super::protocol::{EfiEdidActiveProtocol, EfiEdidDiscoveredProtocol};

/// EFI EDID Discovered Protocol GUID: `{1c0c34f6-d380-41fa-a049-8ad06c1a66aa}`.
pub const EFI_EDID_DISCOVERED_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1C0C_34F6,
    0xD380,
    0x41FA,
    [0xA0, 0x49, 0x8A, 0xD0, 0x6C, 0x1A, 0x66, 0xAA],
);

/// EFI EDID Active Protocol GUID: `{bd8c1056-9f36-44ec-92a8-a6337f817986}`.
pub const EFI_EDID_ACTIVE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBD8C_1056,
    0x9F36,
    0x44EC,
    [0x92, 0xA8, 0xA6, 0x33, 0x7F, 0x81, 0x79, 0x86],
);

/// EFI EDID Override Protocol GUID: `{48ecb431-fb72-45c0-a922-f458fe040bd5}`.
pub const EFI_EDID_OVERRIDE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x48EC_B431,
    0xFB72,
    0x45C0,
    [0xA9, 0x22, 0xF4, 0x58, 0xFE, 0x04, 0x0B, 0xD5],
);

/// Flag indicating that the platform does not wish to override EDID for the child handle.
pub const EFI_EDID_OVERRIDE_DONT_OVERRIDE: u32 = 0x01;
/// Flag indicating that hot-plug display detection should be enabled.
pub const EFI_EDID_OVERRIDE_ENABLE_HOT_PLUG: u32 = 0x02;

/// Retrieves override policy and replacement EDID buffer.
pub type EfiEdidOverrideGetEdid = unsafe extern "efiapi" fn(
    this: *mut EfiEdidOverrideProtocol,
    child_handle: *mut EfiHandle,
    attributes: *mut u32,
    edid_size: *mut UIntN,
    edid: *mut *mut u8,
) -> EfiStatus;

/// EFI EDID Override Protocol interface (`EFI_EDID_OVERRIDE_PROTOCOL`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiEdidOverrideProtocol {
    /// Retrieves EDID override data for a video output child handle.
    pub get_edid: EfiEdidOverrideGetEdid,
}

/// Standard 8-byte VESA EDID Header.
pub const EDID_HEADER: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

/// Parsed VESA EDID Basic Display Information.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdidInfo {
    /// Three-letter PNP manufacturer ID (e.g. "DEL", "SAM", "ACR").
    pub manufacturer_id: [u8; 3],
    /// Product code.
    pub product_code: u16,
    /// Serial number (32-bit).
    pub serial_number: u32,
    /// Week of manufacture.
    pub manufacture_week: u8,
    /// Year of manufacture (e.g. 2024).
    pub manufacture_year: u16,
    /// EDID Version (usually 1).
    pub version: u8,
    /// EDID Revision (e.g. 3 or 4).
    pub revision: u8,
    /// Preferred pixel clock in tens of kHz.
    pub preferred_pixel_clock_10khz: u16,
    /// Preferred horizontal active resolution in pixels.
    pub preferred_width: u16,
    /// Preferred vertical active resolution in lines.
    pub preferred_height: u16,
}

/// Parses a 128-byte raw EDID block into structured display information.
#[must_use]
pub fn parse_edid_block(raw: &[u8]) -> Option<EdidInfo> {
    if raw.len() < 128 {
        return None;
    }

    // Verify 8-byte header
    if raw[0..8] != EDID_HEADER {
        return None;
    }

    // Validate 1-byte checksum (sum of all 128 bytes must be 0 mod 256)
    let checksum = raw[0..128].iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    if checksum != 0 {
        return None;
    }

    // Manufacturer ID is encoded in two big-endian bytes (5 bits per letter, ASCII - 1 + 'A')
    let mfg_bytes = u16::from_be_bytes([raw[8], raw[9]]);
    let c1 = (((mfg_bytes >> 10) & 0x1F) as u8).wrapping_add(b'@');
    let c2 = (((mfg_bytes >> 5) & 0x1F) as u8).wrapping_add(b'@');
    let c3 = ((mfg_bytes & 0x1F) as u8).wrapping_add(b'@');

    let product_code = u16::from_le_bytes([raw[10], raw[11]]);
    let serial_number = u32::from_le_bytes([raw[12], raw[13], raw[14], raw[15]]);
    let manufacture_week = raw[16];
    let manufacture_year = 1990u16.saturating_add(u16::from(raw[17]));
    let version = raw[18];
    let revision = raw[19];

    // Detailed Timing Descriptor 1 starts at offset 54 (0x36)
    let dtd = &raw[54..72];
    let pixel_clock = u16::from_le_bytes([dtd[0], dtd[1]]);

    let (preferred_width, preferred_height) = if pixel_clock > 0 {
        let h_active = u16::from(dtd[2]) | (u16::from(dtd[4] >> 4) << 8);
        let v_active = u16::from(dtd[5]) | (u16::from(dtd[7] >> 4) << 8);
        (h_active, v_active)
    } else {
        (0, 0)
    };

    Some(EdidInfo {
        manufacturer_id: [c1, c2, c3],
        product_code,
        serial_number,
        manufacture_week,
        manufacture_year,
        version,
        revision,
        preferred_pixel_clock_10khz: pixel_clock,
        preferred_width,
        preferred_height,
    })
}
