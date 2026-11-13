//! # IPMI FRU Information Storage
//!
//! Parses IPMI Field Replaceable Unit (FRU) Information structures, including
//! the FRU Common Header, Board Info Area, and Product Info Area.
//!
//! Defined by the IPMI Platform Management FRU Information Storage Definition.

/// FRU Area offsets multiplier (offsets in common header are in 8-byte multiples).
pub const FRU_AREA_OFFSET_MULTIPLIER: usize = 8;

/// Parsed FRU Common Header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FruCommonHeader {
    /// Format version (typically 1).
    pub format_version: u8,
    /// Offset to Internal Use Area in bytes (0 if not present).
    pub internal_use_offset: usize,
    /// Offset to Chassis Info Area in bytes (0 if not present).
    pub chassis_info_offset: usize,
    /// Offset to Board Info Area in bytes (0 if not present).
    pub board_info_offset: usize,
    /// Offset to Product Info Area in bytes (0 if not present).
    pub product_info_offset: usize,
    /// Offset to multi-record area in bytes (0 if not present).
    pub multirecord_offset: usize,
}

/// A parsed FRU Board Information Area view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FruBoardInfo<'a> {
    /// Format version (typically 1).
    pub format_version: u8,
    /// Language code.
    pub language_code: u8,
    /// Manufacturing date / time: minutes since 1996-01-01 00:00:00.
    pub mfg_date_minutes: u32,
    /// Board Manufacturer text if valid ASCII.
    pub manufacturer: Option<&'a str>,
    /// Board Product Name text if valid ASCII.
    pub product_name: Option<&'a str>,
    /// Board Serial Number text if valid ASCII.
    pub serial_number: Option<&'a str>,
    /// Board Part Number text if valid ASCII.
    pub part_number: Option<&'a str>,
    /// FRU File ID text if valid ASCII.
    pub fru_file_id: Option<&'a str>,
}

/// A parsed FRU Product Information Area view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FruProductInfo<'a> {
    /// Format version (typically 1).
    pub format_version: u8,
    /// Language code.
    pub language_code: u8,
    /// Manufacturer Name text if valid ASCII.
    pub manufacturer: Option<&'a str>,
    /// Product Name text if valid ASCII.
    pub product_name: Option<&'a str>,
    /// Product Part / Model Number text if valid ASCII.
    pub model_number: Option<&'a str>,
    /// Product Version text if valid ASCII.
    pub version: Option<&'a str>,
    /// Product Serial Number text if valid ASCII.
    pub serial_number: Option<&'a str>,
    /// Asset Tag text if valid ASCII.
    pub asset_tag: Option<&'a str>,
    /// FRU File ID text if valid ASCII.
    pub fru_file_id: Option<&'a str>,
}

/// Computes the zero-checksum for an 8-byte aligned FRU block.
///
/// Returns true if sum of all bytes modulo 256 is 0.
#[must_use]
pub fn fru_checksum_ok(bytes: &[u8]) -> bool {
    let sum = bytes.iter().fold(0_u8, |acc, &b| acc.wrapping_add(b));
    sum == 0
}

/// Parses the 8-byte FRU Common Header.
#[must_use]
pub fn parse_fru_common_header(buffer: &[u8]) -> Option<FruCommonHeader> {
    if buffer.len() < 8 {
        return None;
    }
    let header_slice = &buffer[..8];
    if !fru_checksum_ok(header_slice) {
        return None;
    }

    let format_version = header_slice[0] & 0x0F;
    let internal_use_offset = usize::from(header_slice[1]) * FRU_AREA_OFFSET_MULTIPLIER;
    let chassis_info_offset = usize::from(header_slice[2]) * FRU_AREA_OFFSET_MULTIPLIER;
    let board_info_offset = usize::from(header_slice[3]) * FRU_AREA_OFFSET_MULTIPLIER;
    let product_info_offset = usize::from(header_slice[4]) * FRU_AREA_OFFSET_MULTIPLIER;
    let multirecord_offset = usize::from(header_slice[5]) * FRU_AREA_OFFSET_MULTIPLIER;

    Some(FruCommonHeader {
        format_version,
        internal_use_offset,
        chassis_info_offset,
        board_info_offset,
        product_info_offset,
        multirecord_offset,
    })
}

/// Helper to decode an IPMI type/length byte and read the string slice.
/// Returns (decoded string, next offset).
fn read_type_length_str(slice: &[u8], offset: usize) -> (Option<&str>, usize) {
    if offset >= slice.len() {
        return (None, offset);
    }
    let tl = slice[offset];
    if tl == 0xC1 {
        // Sentinel byte marking end of fields.
        return (None, offset + 1);
    }
    let type_code = (tl >> 6) & 0x03;
    let length = usize::from(tl & 0x3F);
    let str_start = offset + 1;
    let str_end = str_start + length;

    if str_end > slice.len() {
        return (None, slice.len());
    }

    // Type 3: 8-bit ASCII / Latin-1. Type 2: 6-bit packed ASCII. Type 0: binary.
    let parsed_str = if type_code == 3 || type_code == 2 {
        core::str::from_utf8(&slice[str_start..str_end]).ok()
    } else {
        None
    };

    (parsed_str, str_end)
}

/// Parses the FRU Board Information Area.
#[must_use]
pub fn parse_fru_board_info(buffer: &[u8], offset: usize) -> Option<FruBoardInfo<'_>> {
    if offset >= buffer.len() {
        return None;
    }
    let slice = buffer.get(offset..)?;
    if slice.len() < 8 {
        return None;
    }

    let format_version = slice[0] & 0x0F;
    let area_len = usize::from(slice[1]) * FRU_AREA_OFFSET_MULTIPLIER;
    if area_len < 8 || area_len > slice.len() {
        return None;
    }
    let area_bytes = &slice[..area_len];
    if !fru_checksum_ok(area_bytes) {
        return None;
    }

    let language_code = area_bytes[2];
    let mfg_date = u32::from_le_bytes([area_bytes[3], area_bytes[4], area_bytes[5], 0]);

    let mut cursor = 6;
    let (mfg, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (prod, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (serial, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (part, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (fru_file_id, _) = read_type_length_str(area_bytes, cursor);

    Some(FruBoardInfo {
        format_version,
        language_code,
        mfg_date_minutes: mfg_date,
        manufacturer: mfg,
        product_name: prod,
        serial_number: serial,
        part_number: part,
        fru_file_id,
    })
}

/// Parses the FRU Product Information Area.
#[must_use]
pub fn parse_fru_product_info(buffer: &[u8], offset: usize) -> Option<FruProductInfo<'_>> {
    if offset >= buffer.len() {
        return None;
    }
    let slice = buffer.get(offset..)?;
    if slice.len() < 8 {
        return None;
    }

    let format_version = slice[0] & 0x0F;
    let area_len = usize::from(slice[1]) * FRU_AREA_OFFSET_MULTIPLIER;
    if area_len < 8 || area_len > slice.len() {
        return None;
    }
    let area_bytes = &slice[..area_len];
    if !fru_checksum_ok(area_bytes) {
        return None;
    }

    let language_code = area_bytes[2];
    let mut cursor = 3;

    let (mfg, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (prod, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (model, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (version, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (serial, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (asset, next) = read_type_length_str(area_bytes, cursor);
    cursor = next;
    let (fru_file_id, _) = read_type_length_str(area_bytes, cursor);

    Some(FruProductInfo {
        format_version,
        language_code,
        manufacturer: mfg,
        product_name: prod,
        model_number: model,
        version,
        serial_number: serial,
        asset_tag: asset,
        fru_file_id,
    })
}
