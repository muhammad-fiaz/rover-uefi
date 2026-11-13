//! # IPMI System Event Log (SEL)
//!
//! Structures and parsers for the IPMI System Event Log records.

/// Standard length of an IPMI SEL record in bytes.
pub const SEL_RECORD_LENGTH: usize = 16;

/// SEL Record Types.
pub mod sel_record_type {
    /// Standard System Event Record.
    pub const SYSTEM_EVENT: u8 = 0x02;
    /// OEM Timestamped Record start (0xC0..0xDF).
    pub const OEM_TIMESTAMPED_MIN: u8 = 0xC0;
    /// OEM Timestamped Record end.
    pub const OEM_TIMESTAMPED_MAX: u8 = 0xDF;
    /// OEM Non-Timestamped Record start (0xE0..0xFF).
    pub const OEM_NON_TIMESTAMPED_MIN: u8 = 0xE0;
}

/// IPMI Sensor Types for SEL entries.
pub mod sensor_type {
    /// Temperature sensor.
    pub const TEMPERATURE: u8 = 0x01;
    /// Voltage sensor.
    pub const VOLTAGE: u8 = 0x02;
    /// Current sensor.
    pub const CURRENT: u8 = 0x03;
    /// Fan / cooling device sensor.
    pub const FAN: u8 = 0x04;
    /// Physical security (chassis intrusion).
    pub const PHYSICAL_SECURITY: u8 = 0x05;
    /// Platform security sensor.
    pub const PLATFORM_SECURITY: u8 = 0x06;
    /// Processor status.
    pub const PROCESSOR: u8 = 0x07;
    /// Power supply status.
    pub const POWER_SUPPLY: u8 = 0x08;
    /// Power unit.
    pub const POWER_UNIT: u8 = 0x09;
    /// Memory error status.
    pub const MEMORY: u8 = 0x0C;
    /// Drive slot / bay.
    pub const DRIVE_SLOT: u8 = 0x0D;
    /// System firmware progress / error.
    pub const SYSTEM_FIRMWARE: u8 = 0x0F;
    /// Event logging disabled.
    pub const EVENT_LOGGING: u8 = 0x10;
    /// Watchdog timer.
    pub const WATCHDOG: u8 = 0x11;
    /// System Event.
    pub const SYSTEM_EVENT: u8 = 0x12;
    /// Critical Interrupt.
    pub const CRITICAL_INTERRUPT: u8 = 0x13;
    /// Button / Switch.
    pub const BUTTON: u8 = 0x14;
    /// Module / Board.
    pub const MODULE_BOARD: u8 = 0x15;
    /// OS Boot / Stop.
    pub const OS_BOOT: u8 = 0x1F;
    /// OS Critical Stop.
    pub const OS_STOP: u8 = 0x20;
    /// Version Change.
    pub const VERSION_CHANGE: u8 = 0x2B;
}

/// Parsed standard IPMI System Event Record (Record Type 0x02).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SelSystemEventRecord {
    /// Unique SEL Record ID.
    pub record_id: u16,
    /// Record Type (0x02 for system event).
    pub record_type: u8,
    /// Timestamp (seconds since 1970-01-01 00:00:00 UTC).
    pub timestamp: u32,
    /// Generator ID (e.g. 0x0020 for BIOS / SMI).
    pub generator_id: u16,
    /// Event message format version (0x04 for IPMI 2.0).
    pub event_format_rev: u8,
    /// Sensor type code.
    pub sensor_type: u8,
    /// Sensor number.
    pub sensor_number: u8,
    /// Event direction: false = assertion, true = deassertion.
    pub is_deassertion: bool,
    /// Event / reading type code.
    pub event_type_code: u8,
    /// Event data byte 1.
    pub event_data1: u8,
    /// Event data byte 2.
    pub event_data2: u8,
    /// Event data byte 3.
    pub event_data3: u8,
}

/// Parses a 16-byte IPMI SEL System Event record from a byte slice.
#[must_use]
pub fn parse_sel_record(slice: &[u8]) -> Option<SelSystemEventRecord> {
    if slice.len() < SEL_RECORD_LENGTH {
        return None;
    }

    let record_id = u16::from_le_bytes([slice[0], slice[1]]);
    let record_type = slice[2];
    if record_type != sel_record_type::SYSTEM_EVENT {
        return None;
    }

    let timestamp = u32::from_le_bytes([slice[3], slice[4], slice[5], slice[6]]);
    let generator_id = u16::from_le_bytes([slice[7], slice[8]]);
    let event_format_rev = slice[9];
    let sensor_type = slice[10];
    let sensor_number = slice[11];

    let dir_and_type = slice[12];
    let is_deassertion = (dir_and_type & 0x80) != 0;
    let event_type_code = dir_and_type & 0x7F;

    let event_data1 = slice[13];
    let event_data2 = slice[14];
    let event_data3 = slice[15];

    Some(SelSystemEventRecord {
        record_id,
        record_type,
        timestamp,
        generator_id,
        event_format_rev,
        sensor_type,
        sensor_number,
        is_deassertion,
        event_type_code,
        event_data1,
        event_data2,
        event_data3,
    })
}

/// An iterator over contiguous 16-byte SEL records in a memory buffer.
pub struct SelIter<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl Iterator for SelIter<'_> {
    type Item = SelSystemEventRecord;

    fn next(&mut self) -> Option<Self::Item> {
        while self.offset + SEL_RECORD_LENGTH <= self.buffer.len() {
            let chunk = &self.buffer[self.offset..self.offset + SEL_RECORD_LENGTH];
            self.offset += SEL_RECORD_LENGTH;
            if let Some(record) = parse_sel_record(chunk) {
                return Some(record);
            }
        }
        None
    }
}

/// Returns an iterator over valid System Event records in a buffer.
#[must_use]
pub const fn sel_iter(buffer: &[u8]) -> SelIter<'_> {
    SelIter { buffer, offset: 0 }
}
