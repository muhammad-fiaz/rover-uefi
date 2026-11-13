//! # IPMI Sensor Data Record (SDR)
//!
//! Structures and parsers for IPMI Sensor Data Records, including Full Sensor Records,
//! Compact Sensor Records, and sensor reading conversion formulas.

/// SDR Record Types.
pub mod sdr_record_type {
    /// Full Sensor Record.
    pub const FULL_SENSOR: u8 = 0x01;
    /// Compact Sensor Record.
    pub const COMPACT_SENSOR: u8 = 0x02;
    /// Event-Only Sensor Record.
    pub const EVENT_ONLY: u8 = 0x03;
    /// Entity Association Record.
    pub const ENTITY_ASSOCIATION: u8 = 0x08;
    /// Device-relative Entity Association Record.
    pub const DEVICE_RELATIVE_ENTITY_ASSOC: u8 = 0x09;
    /// Generic Device Locator Record.
    pub const GENERIC_DEVICE_LOCATOR: u8 = 0x10;
    /// FRU Device Locator Record.
    pub const FRU_DEVICE_LOCATOR: u8 = 0x11;
    /// Management Controller Device Locator Record.
    pub const MGMT_CONTROLLER_DEVICE_LOCATOR: u8 = 0x12;
}

/// Common header for any IPMI SDR record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SdrHeader {
    /// Record ID.
    pub record_id: u16,
    /// SDR Version (usually 0x51 for v1.5).
    pub sdr_version: u8,
    /// Record Type.
    pub record_type: u8,
    /// Record Length (bytes following this 5-byte header).
    pub record_length: u8,
}

/// Parsed Full Sensor Record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SdrFullSensor<'a> {
    /// Common SDR header.
    pub header: SdrHeader,
    /// Sensor owner ID (I2C address or software ID).
    pub sensor_owner_id: u8,
    /// Sensor owner LUN.
    pub sensor_owner_lun: u8,
    /// Unique sensor number.
    pub sensor_number: u8,
    /// Entity ID.
    pub entity_id: u8,
    /// Entity instance.
    pub entity_instance: u8,
    /// Sensor type code.
    pub sensor_type: u8,
    /// Event / reading type code.
    pub event_reading_type: u8,
    /// Sensor units 1 (analog/digital format, rate).
    pub sensor_units1: u8,
    /// Base unit type code (e.g. degrees C, Volts, RPM).
    pub base_unit: u8,
    /// Linearization (0 = Linear, 1 = ln, 2 = log10, 3 = log2, etc.).
    pub linearization: u8,
    /// Conversion M factor (signed 10-bit integer).
    pub m: i16,
    /// Conversion B factor (signed 10-bit integer).
    pub b: i16,
    /// Conversion R exponent (signed 4-bit integer, power of 10).
    pub r_exp: i8,
    /// Conversion B exponent (signed 4-bit integer, power of 10).
    pub b_exp: i8,
    /// Sensor Name string if valid ASCII.
    pub id_string: Option<&'a str>,
}

/// Parses a 5-byte SDR record header.
#[must_use]
pub fn parse_sdr_header(slice: &[u8]) -> Option<SdrHeader> {
    if slice.len() < 5 {
        return None;
    }
    let record_id = u16::from_le_bytes([slice[0], slice[1]]);
    let sdr_version = slice[2];
    let record_type = slice[3];
    let record_length = slice[4];

    Some(SdrHeader {
        record_id,
        sdr_version,
        record_type,
        record_length,
    })
}

/// Parses a Full Sensor Record from an SDR byte slice.
#[must_use]
pub fn parse_full_sensor(slice: &[u8]) -> Option<SdrFullSensor<'_>> {
    let header = parse_sdr_header(slice)?;
    if header.record_type != sdr_record_type::FULL_SENSOR {
        return None;
    }
    let total_len = 5 + usize::from(header.record_length);
    if total_len > slice.len() || total_len < 48 {
        return None;
    }

    let sensor_owner_id = slice[5];
    let sensor_owner_lun = slice[6] & 0x03;
    let sensor_number = slice[7];
    let entity_id = slice[8];
    let entity_instance = slice[9];
    let sensor_type = slice[12];
    let event_reading_type = slice[13];
    let sensor_units1 = slice[20];
    let base_unit = slice[21];
    let linearization = slice[23] & 0x7F;

    // Decoding M (10-bit 2's complement)
    let m_low = u16::from(slice[24]);
    let m_high = u16::from(slice[25] & 0xC0);
    let mut m_raw = (m_high << 2) | m_low;
    if (m_raw & 0x0200) != 0 {
        m_raw |= 0xFC00;
    }
    let m = m_raw.cast_signed().clamp(-512, 511);

    // Decoding B (10-bit 2's complement)
    let b_low = u16::from(slice[27]);
    let b_high = u16::from(slice[28] & 0xC0);
    let mut b_raw = (b_high << 2) | b_low;
    if (b_raw & 0x0200) != 0 {
        b_raw |= 0xFC00;
    }
    let b = b_raw.cast_signed().clamp(-512, 511);

    // Decoding R_exp and B_exp (4-bit 2's complement each)
    let exp_byte = slice[29];
    let mut r_exp_raw = (exp_byte >> 4) & 0x0F;
    if (r_exp_raw & 0x08) != 0 {
        r_exp_raw |= 0xF0;
    }
    let r_exp = r_exp_raw.cast_signed();

    let mut b_exp_raw = exp_byte & 0x0F;
    if (b_exp_raw & 0x08) != 0 {
        b_exp_raw |= 0xF0;
    }
    let b_exp = b_exp_raw.cast_signed();

    // Sensor string
    let id_type_len = slice[47];
    let id_len = usize::from(id_type_len & 0x1F);
    let id_start = 48;
    let id_end = (id_start + id_len).min(total_len);
    let id_string = if id_end > id_start {
        core::str::from_utf8(&slice[id_start..id_end]).ok()
    } else {
        None
    };

    Some(SdrFullSensor {
        header,
        sensor_owner_id,
        sensor_owner_lun,
        sensor_number,
        entity_id,
        entity_instance,
        sensor_type,
        event_reading_type,
        sensor_units1,
        base_unit,
        linearization,
        m,
        b,
        r_exp,
        b_exp,
        id_string,
    })
}

/// Helper to compute integer powers of 10 without floating-point runtime requirements.
fn pow10(exp: i8) -> f64 {
    match exp {
        -8 => 1e-8,
        -7 => 1e-7,
        -6 => 1e-6,
        -5 => 1e-5,
        -4 => 1e-4,
        -3 => 1e-3,
        -2 => 1e-2,
        -1 => 1e-1,
        0 => 1.0,
        1 => 10.0,
        2 => 100.0,
        3 => 1_000.0,
        4 => 10_000.0,
        5 => 100_000.0,
        6 => 1_000_000.0,
        7 => 10_000_000.0,
        _ => {
            if exp < 0 {
                0.0
            } else {
                1e8
            }
        }
    }
}

/// Converts a raw 8-bit sensor reading byte into a physical floating value.
///
/// Uses the IPMI linear conversion formula:
/// `(M * raw + (B * 10^b_exp)) * 10^r_exp`.
#[must_use]
pub fn convert_sensor_reading(raw: u8, m: i16, b: i16, r_exp: i8, b_exp: i8) -> f64 {
    let y = f64::from(raw);
    let m_val = f64::from(m);
    let b_val = f64::from(b);

    let k1_factor = pow10(b_exp);
    let k2_factor = pow10(r_exp);

    (m_val * y + (b_val * k1_factor)) * k2_factor
}
