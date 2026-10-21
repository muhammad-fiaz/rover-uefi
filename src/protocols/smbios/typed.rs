//! # SMBIOS typed records
//!
//! Typed decoders for every standard SMBIOS structure that has a
//! fixed field layout.
//!
//! Each decoder operates on the formatted section
//! slice including the four byte structure header
//! with type, length and handle. Offsets are counted
//! from the structure start, so the first field
//! after the header is at offset `0x04`. All
//! decoders check bounds and return `None` for
//! truncated or malformed input without panicking.
//!
//! String indices are stored as raw bytes for the
//! caller to resolve with the structure string set.
//! Optional trailing fields are `None` when the
//! formatted length does not cover them.

use crate::base::bytes::read_u8;

/// The minimum BIOS formatted length in bytes.
pub const MIN_BIOS_FORMATTED_LEN: usize = 23;

/// The minimum system formatted length in bytes.
pub const MIN_SYSTEM_FORMATTED_LEN: usize = 27;

/// The minimum baseboard formatted length in bytes.
pub const MIN_BASEBOARD_FORMATTED_LEN: usize = 15;

/// The minimum enclosure formatted length in bytes.
pub const MIN_ENCLOSURE_FORMATTED_LEN: usize = 14;

/// The minimum processor formatted length in bytes.
pub const MIN_PROCESSOR_FORMATTED_LEN: usize = 31;

/// The minimum cache formatted length in bytes.
pub const MIN_CACHE_FORMATTED_LEN: usize = 17;

/// The minimum port connector formatted length in bytes.
pub const MIN_PORT_CONNECTOR_FORMATTED_LEN: usize = 8;

/// The minimum system slot formatted length in bytes.
pub const MIN_SYSTEM_SLOTS_FORMATTED_LEN: usize = 13;

/// The minimum physical memory array formatted length in bytes.
pub const MIN_PHYSICAL_MEMORY_ARRAY_FORMATTED_LEN: usize = 12;

/// The minimum memory device formatted length in bytes.
pub const MIN_MEMORY_DEVICE_FORMATTED_LEN: usize = 27;

/// The minimum memory array mapped formatted length in bytes.
pub const MIN_MEMORY_ARRAY_MAPPED_FORMATTED_LEN: usize = 15;

/// The minimum memory device mapped formatted length in bytes.
pub const MIN_MEMORY_DEVICE_MAPPED_FORMATTED_LEN: usize = 18;

/// Reads a little endian `u16` at an offset.
///
/// Returns `None` if the two bytes are out of range.
#[must_use]
pub fn u16_le(buffer: &[u8], offset: usize) -> Option<u16> {
    crate::base::bytes::read_u16_le(buffer, offset)
}

/// Reads a little endian `u32` at an offset.
///
/// Returns `None` if the four bytes are out of range.
#[must_use]
pub fn u32_le(buffer: &[u8], offset: usize) -> Option<u32> {
    crate::base::bytes::read_u32_le(buffer, offset)
}

/// Reads a little endian `u64` at an offset.
///
/// Returns `None` if the eight bytes are out of range.
#[must_use]
pub fn u64_le(buffer: &[u8], offset: usize) -> Option<u64> {
    crate::base::bytes::read_u64_le(buffer, offset)
}

/// Decodes a cache size word to kibibytes.
///
/// Bit 15 selects the unit: zero means the low 15
/// bits are kibibytes and one means mebibytes.
#[must_use]
pub fn decode_cache_size_kb(raw: u16) -> u32 {
    let amount = u32::from(raw & 0x7FFF);
    if raw & 0x8000 != 0 {
        amount.saturating_mul(1024)
    } else {
        amount
    }
}

/// Decodes a memory device size word to kibibytes.
///
/// Bit 15 selects the unit with the opposite polarity
/// to cache sizes: one means kibibytes and zero means
/// mebibytes. The value `0x7FFF` is unknown and
/// returns `None`. The value `0xFFFF` means not
/// installed and returns `Some(0)`.
#[must_use]
pub fn decode_memory_device_size_kb(raw: u16) -> Option<u32> {
    if raw == 0x7FFF {
        return None;
    }
    if raw == 0xFFFF {
        return Some(0);
    }
    let amount = u32::from(raw & 0x7FFF);
    if raw & 0x8000 != 0 {
        Some(amount)
    } else {
        amount.checked_mul(1024)
    }
}

/// The BIOS information record for Type 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BiosInfo {
    /// The vendor string index at offset `0x04`.
    pub vendor_idx: u8,
    /// The version string index at offset `0x05`.
    pub version_idx: u8,
    /// The starting address segment at offset `0x06`.
    pub starting_address: u16,
    /// The release date string index at offset `0x08`.
    pub release_date_idx: u8,
    /// The ROM size size byte at offset `0x09`.
    pub rom_size: u8,
    /// The characteristics word at offset `0x0A`.
    pub characteristics: u64,
    /// The first extension byte at offset `0x12`.
    pub characteristic_extension_1: u8,
    /// The second extension byte at offset `0x13`.
    pub characteristic_extension_2: u8,
    /// The major BIOS version at offset `0x14`.
    pub major_version: u8,
    /// The minor BIOS version at offset `0x15`.
    pub minor_version: u8,
    /// The embedded controller revision at offset `0x16`.
    pub embedded_controller_revision: u8,
}

/// Parses BIOS information from a formatted slice.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 0 length or any field is truncated.
#[must_use]
pub fn parse_bios_info(formatted: &[u8]) -> Option<BiosInfo> {
    if formatted.len() < MIN_BIOS_FORMATTED_LEN {
        return None;
    }
    Some(BiosInfo {
        vendor_idx: formatted.get(0x04).copied()?,
        version_idx: formatted.get(0x05).copied()?,
        starting_address: u16_le(formatted, 0x06)?,
        release_date_idx: formatted.get(0x08).copied()?,
        rom_size: formatted.get(0x09).copied()?,
        characteristics: u64_le(formatted, 0x0A)?,
        characteristic_extension_1: formatted.get(0x12).copied()?,
        characteristic_extension_2: formatted.get(0x13).copied()?,
        major_version: formatted.get(0x14).copied()?,
        minor_version: formatted.get(0x15).copied()?,
        embedded_controller_revision: formatted.get(0x16).copied()?,
    })
}

/// The system information record for Type 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemInfo {
    /// The manufacturer string index at offset `0x04`.
    pub manufacturer_idx: u8,
    /// The product name string index at offset `0x05`.
    pub product_name_idx: u8,
    /// The version string index at offset `0x06`.
    pub version_idx: u8,
    /// The serial number string index at offset `0x07`.
    pub serial_number_idx: u8,
    /// The UUID bytes at offset `0x08`.
    pub uuid: [u8; 16],
    /// The wake-up type at offset `0x18`.
    pub wakeup_type: u8,
    /// The SKU number string index at offset `0x19`.
    pub sku_number_idx: u8,
    /// The family string index at offset `0x1A`.
    pub family_idx: u8,
}

/// Parses system information from a formatted slice.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 1 length or the UUID is truncated.
#[must_use]
pub fn parse_system_info(formatted: &[u8]) -> Option<SystemInfo> {
    if formatted.len() < MIN_SYSTEM_FORMATTED_LEN {
        return None;
    }
    let mut uuid = [0_u8; 16];
    uuid.copy_from_slice(formatted.get(0x08..0x18)?);
    Some(SystemInfo {
        manufacturer_idx: formatted.get(0x04).copied()?,
        product_name_idx: formatted.get(0x05).copied()?,
        version_idx: formatted.get(0x06).copied()?,
        serial_number_idx: formatted.get(0x07).copied()?,
        uuid,
        wakeup_type: formatted.get(0x18).copied()?,
        sku_number_idx: formatted.get(0x19).copied()?,
        family_idx: formatted.get(0x1A).copied()?,
    })
}

/// The baseboard information record for Type 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BaseboardInfo {
    /// The manufacturer string index at offset `0x04`.
    pub manufacturer_idx: u8,
    /// The product string index at offset `0x05`.
    pub product_idx: u8,
    /// The version string index at offset `0x06`.
    pub version_idx: u8,
    /// The serial number string index at offset `0x07`.
    pub serial_number_idx: u8,
    /// The asset tag string index at offset `0x08`.
    pub asset_tag_idx: u8,
    /// The feature flags at offset `0x09`.
    pub feature_flags: u8,
    /// The location string index at offset `0x0A`.
    pub location_in_chassis_idx: u8,
    /// The chassis handle at offset `0x0B`.
    pub chassis_handle: u16,
    /// The board type at offset `0x0D`.
    pub board_type: u8,
    /// The object handle count at offset `0x0E`.
    pub object_handle_count: u8,
}

/// Parses baseboard information from a formatted slice.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 2 length.
#[must_use]
pub fn parse_baseboard_info(formatted: &[u8]) -> Option<BaseboardInfo> {
    if formatted.len() < MIN_BASEBOARD_FORMATTED_LEN {
        return None;
    }
    Some(BaseboardInfo {
        manufacturer_idx: formatted.get(0x04).copied()?,
        product_idx: formatted.get(0x05).copied()?,
        version_idx: formatted.get(0x06).copied()?,
        serial_number_idx: formatted.get(0x07).copied()?,
        asset_tag_idx: formatted.get(0x08).copied()?,
        feature_flags: formatted.get(0x09).copied()?,
        location_in_chassis_idx: formatted.get(0x0A).copied()?,
        chassis_handle: u16_le(formatted, 0x0B)?,
        board_type: formatted.get(0x0D).copied()?,
        object_handle_count: formatted.get(0x0E).copied()?,
    })
}

/// The enclosure information record for Type 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnclosureInfo {
    /// The manufacturer string index at offset `0x04`.
    pub manufacturer_idx: u8,
    /// The enclosure type at offset `0x05`.
    pub enclosure_type: u8,
    /// The version string index at offset `0x06`.
    pub version_idx: u8,
    /// The serial number string index at offset `0x07`.
    pub serial_number_idx: u8,
    /// The asset tag string index at offset `0x08`.
    pub asset_tag_idx: u8,
    /// The boot-up state at offset `0x09`.
    pub bootup_state: u8,
    /// The power supply state at offset `0x0A`.
    pub power_supply_state: u8,
    /// The thermal state at offset `0x0B`.
    pub thermal_state: u8,
    /// The security status at offset `0x0C`.
    pub security_status: u8,
    /// The OEM defined count at offset `0x0D`.
    pub oem_defined_count: u8,
    /// The height after the OEM defined entries.
    pub height: Option<u16>,
}

/// Parses enclosure information from a formatted slice.
///
/// The height is read after the OEM defined entries
/// which are four bytes each. Returns `None` if the
/// slice is shorter than the minimum Type 3 length
/// or the OEM defined area overruns the slice.
#[must_use]
pub fn parse_enclosure_info(formatted: &[u8]) -> Option<EnclosureInfo> {
    if formatted.len() < MIN_ENCLOSURE_FORMATTED_LEN {
        return None;
    }
    let oem_defined_count = formatted.get(0x0D).copied()?;
    let oem_bytes = usize::from(oem_defined_count).checked_mul(4)?;
    let oem_end = 0x0E_usize.checked_add(oem_bytes)?;
    if oem_end > formatted.len() {
        return None;
    }
    Some(EnclosureInfo {
        manufacturer_idx: formatted.get(0x04).copied()?,
        enclosure_type: formatted.get(0x05).copied()?,
        version_idx: formatted.get(0x06).copied()?,
        serial_number_idx: formatted.get(0x07).copied()?,
        asset_tag_idx: formatted.get(0x08).copied()?,
        bootup_state: formatted.get(0x09).copied()?,
        power_supply_state: formatted.get(0x0A).copied()?,
        thermal_state: formatted.get(0x0B).copied()?,
        security_status: formatted.get(0x0C).copied()?,
        oem_defined_count,
        height: u16_le(formatted, oem_end),
    })
}

/// The processor information record for Type 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProcessorInfo {
    /// The socket designation index at offset `0x04`.
    pub socket_designation_idx: u8,
    /// The processor type at offset `0x05`.
    pub processor_type: u8,
    /// The processor family at offset `0x06`.
    pub processor_family: u8,
    /// The processor ID at offset `0x07`.
    pub processor_id: u64,
    /// The version string index at offset `0x0F`.
    pub version_idx: u8,
    /// The voltage at offset `0x10`.
    pub voltage: u8,
    /// The external clock speed in MHz at offset `0x11`.
    pub external_clock_mhz: u16,
    /// The maximum speed in MHz at offset `0x13`.
    pub max_speed_mhz: u16,
    /// The current speed in MHz at offset `0x15`.
    pub current_speed_mhz: u16,
    /// The status at offset `0x17`.
    pub status: u8,
    /// The processor upgrade at offset `0x18`.
    pub processor_upgrade: u8,
    /// The L1 cache handle at offset `0x19`.
    pub l1_cache_handle: u16,
    /// The L2 cache handle at offset `0x1B`.
    pub l2_cache_handle: u16,
    /// The L3 cache handle at offset `0x1D`.
    pub l3_cache_handle: u16,
    /// The serial number index at offset `0x1F`.
    pub serial_number_idx: Option<u8>,
    /// The asset tag index at offset `0x20`.
    pub asset_tag_idx: Option<u8>,
    /// The part number index at offset `0x21`.
    pub part_number_idx: Option<u8>,
    /// The maximum core count at offset `0x22`.
    pub core_count_max: Option<u8>,
    /// The core enabled count at offset `0x23`.
    pub core_enabled: Option<u8>,
    /// The maximum thread count at offset `0x24`.
    pub thread_count_max: Option<u8>,
    /// The thread enabled count at offset `0x25`.
    pub thread_enabled: Option<u8>,
    /// The characteristics at offset `0x26`.
    pub characteristics: Option<u16>,
    /// The extended processor family at offset `0x28`.
    pub processor_family_2: Option<u16>,
    /// The extended maximum core count at offset `0x2A`.
    pub core_count_max_2: Option<u16>,
    /// The extended core enabled count at offset `0x2C`.
    pub core_enabled_2: Option<u16>,
    /// The extended maximum thread count at offset `0x2E`.
    pub thread_count_max_2: Option<u16>,
    /// The extended thread enabled count at offset `0x30`.
    pub thread_enabled_2: Option<u16>,
    /// The instruction set at offset `0x32`.
    pub instruction_set: Option<u16>,
}

/// Parses processor information from a formatted slice.
///
/// Fields from the serial number onward are optional
/// and are `None` when the formatted length is too
/// short. Returns `None` if the slice is shorter
/// than the minimum Type 4 length or a required
/// field is truncated.
#[must_use]
pub fn parse_processor_info(formatted: &[u8]) -> Option<ProcessorInfo> {
    if formatted.len() < MIN_PROCESSOR_FORMATTED_LEN {
        return None;
    }
    Some(ProcessorInfo {
        socket_designation_idx: formatted.get(0x04).copied()?,
        processor_type: formatted.get(0x05).copied()?,
        processor_family: formatted.get(0x06).copied()?,
        processor_id: u64_le(formatted, 0x07)?,
        version_idx: formatted.get(0x0F).copied()?,
        voltage: formatted.get(0x10).copied()?,
        external_clock_mhz: u16_le(formatted, 0x11)?,
        max_speed_mhz: u16_le(formatted, 0x13)?,
        current_speed_mhz: u16_le(formatted, 0x15)?,
        status: formatted.get(0x17).copied()?,
        processor_upgrade: formatted.get(0x18).copied()?,
        l1_cache_handle: u16_le(formatted, 0x19)?,
        l2_cache_handle: u16_le(formatted, 0x1B)?,
        l3_cache_handle: u16_le(formatted, 0x1D)?,
        serial_number_idx: formatted.get(0x1F).copied(),
        asset_tag_idx: formatted.get(0x20).copied(),
        part_number_idx: formatted.get(0x21).copied(),
        core_count_max: formatted.get(0x22).copied(),
        core_enabled: formatted.get(0x23).copied(),
        thread_count_max: formatted.get(0x24).copied(),
        thread_enabled: formatted.get(0x25).copied(),
        characteristics: u16_le(formatted, 0x26),
        processor_family_2: u16_le(formatted, 0x28),
        core_count_max_2: u16_le(formatted, 0x2A),
        core_enabled_2: u16_le(formatted, 0x2C),
        thread_count_max_2: u16_le(formatted, 0x2E),
        thread_enabled_2: u16_le(formatted, 0x30),
        instruction_set: u16_le(formatted, 0x32),
    })
}

/// The cache information record for Type 7.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CacheInfo {
    /// The socket designation index at offset `0x04`.
    pub socket_designation_idx: u8,
    /// The cache configuration at offset `0x05`.
    pub cache_configuration: u16,
    /// The maximum cache size word at offset `0x07`.
    pub max_size_raw: u16,
    /// The current cache size word at offset `0x09`.
    pub current_size_raw: u16,
    /// The maximum cache size in kibibytes.
    pub max_size_kb: u32,
    /// The current cache size in kibibytes.
    pub current_size_kb: u32,
    /// The supported SRAM type at offset `0x0B`.
    pub supported_sram_type: u8,
    /// The current SRAM type at offset `0x0C`.
    pub current_sram_type: u8,
    /// The cache speed at offset `0x0D`.
    pub speed: u8,
    /// The error correction type at offset `0x0E`.
    pub error_correction_type: u8,
    /// The system cache type at offset `0x0F`.
    pub system_cache_type: u8,
    /// The associativity at offset `0x10`.
    pub associativity: u8,
}

/// Parses cache information from a formatted slice.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 7 length.
#[must_use]
pub fn parse_cache_info(formatted: &[u8]) -> Option<CacheInfo> {
    if formatted.len() < MIN_CACHE_FORMATTED_LEN {
        return None;
    }
    let max_size_raw = u16_le(formatted, 0x07)?;
    let current_size_raw = u16_le(formatted, 0x09)?;
    Some(CacheInfo {
        socket_designation_idx: formatted.get(0x04).copied()?,
        cache_configuration: u16_le(formatted, 0x05)?,
        max_size_raw,
        current_size_raw,
        max_size_kb: decode_cache_size_kb(max_size_raw),
        current_size_kb: decode_cache_size_kb(current_size_raw),
        supported_sram_type: formatted.get(0x0B).copied()?,
        current_sram_type: formatted.get(0x0C).copied()?,
        speed: formatted.get(0x0D).copied()?,
        error_correction_type: formatted.get(0x0E).copied()?,
        system_cache_type: formatted.get(0x0F).copied()?,
        associativity: formatted.get(0x10).copied()?,
    })
}

/// The port connector record for Type 8.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PortConnector {
    /// The internal designator index at offset `0x04`.
    pub internal_designator_idx: u8,
    /// The external designator index at offset `0x05`.
    pub external_designator_idx: u8,
    /// The port type at offset `0x06`.
    pub port_type: u8,
    /// The port connector type at offset `0x07`.
    pub connector_type: u8,
}

/// Parses a port connector from a formatted slice.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 8 length.
#[must_use]
pub fn parse_port_connector(formatted: &[u8]) -> Option<PortConnector> {
    if formatted.len() < MIN_PORT_CONNECTOR_FORMATTED_LEN {
        return None;
    }
    Some(PortConnector {
        internal_designator_idx: formatted.get(0x04).copied()?,
        external_designator_idx: formatted.get(0x05).copied()?,
        port_type: formatted.get(0x06).copied()?,
        connector_type: formatted.get(0x07).copied()?,
    })
}

/// The system slots record for Type 9.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemSlots {
    /// The slot designation index at offset `0x04`.
    pub designation_idx: u8,
    /// The slot type at offset `0x05`.
    pub slot_type: u8,
    /// The slot data bus width at offset `0x06`.
    pub data_bus_width: u8,
    /// The current usage at offset `0x07`.
    pub current_usage: u8,
    /// The slot length at offset `0x08`.
    pub length: u8,
    /// The slot ID at offset `0x09`.
    pub id: u16,
    /// The first slot characteristic at offset `0x0B`.
    pub characteristics_1: u8,
    /// The second slot characteristic at offset `0x0C`.
    pub characteristics_2: u8,
}

/// Parses system slots from a formatted slice.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 9 length.
#[must_use]
pub fn parse_system_slots(formatted: &[u8]) -> Option<SystemSlots> {
    if formatted.len() < MIN_SYSTEM_SLOTS_FORMATTED_LEN {
        return None;
    }
    Some(SystemSlots {
        designation_idx: formatted.get(0x04).copied()?,
        slot_type: formatted.get(0x05).copied()?,
        data_bus_width: formatted.get(0x06).copied()?,
        current_usage: formatted.get(0x07).copied()?,
        length: formatted.get(0x08).copied()?,
        id: u16_le(formatted, 0x09)?,
        characteristics_1: formatted.get(0x0B).copied()?,
        characteristics_2: formatted.get(0x0C).copied()?,
    })
}

/// The physical memory array record for Type 16.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PhysicalMemoryArray {
    /// The location use at offset `0x04`.
    pub use_kind: u8,
    /// The error correction type at offset `0x05`.
    pub error_correction_type: u8,
    /// The maximum capacity at offset `0x06`.
    pub maximum_capacity: u32,
    /// The maximum device count at offset `0x0A`.
    pub maximum_number_of_devices: u16,
}

/// Parses a physical memory array from a formatted slice.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 16 length.
#[must_use]
pub fn parse_physical_memory_array(formatted: &[u8]) -> Option<PhysicalMemoryArray> {
    if formatted.len() < MIN_PHYSICAL_MEMORY_ARRAY_FORMATTED_LEN {
        return None;
    }
    Some(PhysicalMemoryArray {
        use_kind: formatted.get(0x04).copied()?,
        error_correction_type: formatted.get(0x05).copied()?,
        maximum_capacity: u32_le(formatted, 0x06)?,
        maximum_number_of_devices: u16_le(formatted, 0x0A)?,
    })
}

/// The memory device record for Type 17.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryDevice {
    /// The physical memory array handle at offset `0x04`.
    pub physical_memory_array_handle: u16,
    /// The memory error handle at offset `0x06`.
    pub memory_error_info_handle: u16,
    /// The total width at offset `0x08`.
    pub total_width: u16,
    /// The data width at offset `0x0A`.
    pub data_width: u16,
    /// The size word at offset `0x0C`.
    pub size_raw: u16,
    /// The size in kibibytes when known.
    pub size_kb: Option<u32>,
    /// The form factor at offset `0x0E`.
    pub form_factor: u8,
    /// The device locator index at offset `0x0F`.
    pub device_locator_idx: u8,
    /// The bank locator index at offset `0x10`.
    pub bank_locator_idx: u8,
    /// The memory type at offset `0x11`.
    pub memory_type: u8,
    /// The type detail at offset `0x12`.
    pub type_detail: u16,
    /// The speed in megatransfers at offset `0x14`.
    pub speed_mtps: u16,
    /// The manufacturer index at offset `0x16`.
    pub manufacturer_idx: u8,
    /// The serial number index at offset `0x17`.
    pub serial_number_idx: u8,
    /// The asset tag index at offset `0x18`.
    pub asset_tag_idx: u8,
    /// The part number index at offset `0x19`.
    pub part_number_idx: u8,
    /// The attributes at offset `0x1A`.
    pub attributes: u8,
    /// The extended size at offset `0x1B`.
    pub extended_size: Option<u32>,
    /// The configured clock speed at offset `0x1F`.
    pub configured_clock_speed_mtps: Option<u16>,
    /// The minimum voltage in millivolts at offset `0x21`.
    pub minimum_voltage_mv: Option<u16>,
    /// The maximum voltage in millivolts at offset `0x23`.
    pub maximum_voltage_mv: Option<u16>,
    /// The configured voltage in millivolts at offset `0x25`.
    pub configured_voltage_mv: Option<u16>,
}

/// Parses a memory device from a formatted slice.
///
/// Fields after attributes are optional and are
/// `None` when the formatted length is too short.
/// Returns `None` if the slice is shorter than the
/// minimum Type 17 length.
#[must_use]
pub fn parse_memory_device(formatted: &[u8]) -> Option<MemoryDevice> {
    if formatted.len() < MIN_MEMORY_DEVICE_FORMATTED_LEN {
        return None;
    }
    let size_raw = u16_le(formatted, 0x0C)?;
    Some(MemoryDevice {
        physical_memory_array_handle: u16_le(formatted, 0x04)?,
        memory_error_info_handle: u16_le(formatted, 0x06)?,
        total_width: u16_le(formatted, 0x08)?,
        data_width: u16_le(formatted, 0x0A)?,
        size_raw,
        size_kb: decode_memory_device_size_kb(size_raw),
        form_factor: formatted.get(0x0E).copied()?,
        device_locator_idx: formatted.get(0x0F).copied()?,
        bank_locator_idx: formatted.get(0x10).copied()?,
        memory_type: formatted.get(0x11).copied()?,
        type_detail: u16_le(formatted, 0x12)?,
        speed_mtps: u16_le(formatted, 0x14)?,
        manufacturer_idx: formatted.get(0x16).copied()?,
        serial_number_idx: formatted.get(0x17).copied()?,
        asset_tag_idx: formatted.get(0x18).copied()?,
        part_number_idx: formatted.get(0x19).copied()?,
        attributes: formatted.get(0x1A).copied()?,
        extended_size: u32_le(formatted, 0x1B),
        configured_clock_speed_mtps: u16_le(formatted, 0x1F),
        minimum_voltage_mv: u16_le(formatted, 0x21),
        maximum_voltage_mv: u16_le(formatted, 0x23),
        configured_voltage_mv: u16_le(formatted, 0x25),
    })
}

/// The memory array mapped record for Type 19.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryArrayMapped {
    /// The starting address at offset `0x04`.
    pub starting_address: u32,
    /// The ending address at offset `0x08`.
    pub ending_address: u32,
    /// The physical memory array handle at offset `0x0C`.
    pub physical_memory_array_handle: u16,
    /// The partition width at offset `0x0E`.
    pub partition_width: u8,
}

/// Parses a memory array mapped record.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 19 length.
#[must_use]
pub fn parse_memory_array_mapped(formatted: &[u8]) -> Option<MemoryArrayMapped> {
    if formatted.len() < MIN_MEMORY_ARRAY_MAPPED_FORMATTED_LEN {
        return None;
    }
    Some(MemoryArrayMapped {
        starting_address: u32_le(formatted, 0x04)?,
        ending_address: u32_le(formatted, 0x08)?,
        physical_memory_array_handle: u16_le(formatted, 0x0C)?,
        partition_width: formatted.get(0x0E).copied()?,
    })
}

/// The memory device mapped record for Type 20.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryDeviceMapped {
    /// The mapped address handle at offset `0x04`.
    pub memory_array_mapped_address_handle: u16,
    /// The starting address at offset `0x06`.
    pub starting_address: u32,
    /// The ending address at offset `0x0A`.
    pub ending_address: u32,
    /// The partition row position at offset `0x0E`.
    pub partition_row_position: u16,
    /// The interleave position at offset `0x10`.
    pub interleave_position: u8,
    /// The maximum interleave at offset `0x11`.
    pub maximum_interleave: u8,
}

/// Parses a memory device mapped record.
///
/// Returns `None` if the slice is shorter than the
/// minimum Type 20 length.
#[must_use]
pub fn parse_memory_device_mapped(formatted: &[u8]) -> Option<MemoryDeviceMapped> {
    if formatted.len() < MIN_MEMORY_DEVICE_MAPPED_FORMATTED_LEN {
        return None;
    }
    Some(MemoryDeviceMapped {
        memory_array_mapped_address_handle: u16_le(formatted, 0x04)?,
        starting_address: u32_le(formatted, 0x06)?,
        ending_address: u32_le(formatted, 0x0A)?,
        partition_row_position: u16_le(formatted, 0x0E)?,
        interleave_position: formatted.get(0x10).copied()?,
        maximum_interleave: formatted.get(0x11).copied()?,
    })
}

/// An on-board device entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OnboardDevice {
    /// The device type and status byte.
    pub device_type: u8,
    /// The description string index.
    pub description_idx: u8,
}

/// The on-board devices record for Type 10.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OnboardDevices<'a> {
    /// The raw device entries, two bytes each.
    pub entries: &'a [u8],
}

impl OnboardDevices<'_> {
    /// Returns the number of device entries.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.entries.len() / 2
    }

    /// Returns the device at `index`.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<OnboardDevice> {
        let offset = index.checked_mul(2)?;
        Some(OnboardDevice {
            device_type: read_u8(self.entries, offset)?,
            description_idx: read_u8(self.entries, offset + 1)?,
        })
    }
}

/// Parses the on-board devices record.
#[must_use]
pub fn parse_onboard_devices(formatted: &[u8]) -> Option<OnboardDevices<'_>> {
    let entries = formatted.get(4..)?;
    Some(OnboardDevices {
        entries: &entries[..entries.len() - entries.len() % 2],
    })
}

/// The OEM strings record for Type 11.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OemStrings {
    /// The number of OEM strings.
    pub count: u8,
}

/// Parses the OEM strings record.
#[must_use]
pub fn parse_oem_strings(formatted: &[u8]) -> Option<OemStrings> {
    Some(OemStrings {
        count: read_u8(formatted, 4)?,
    })
}

/// The system configuration options record for Type 12.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemConfiguration {
    /// The number of configuration options.
    pub count: u8,
}

/// Parses the system configuration options record.
#[must_use]
pub fn parse_system_configuration(formatted: &[u8]) -> Option<SystemConfiguration> {
    Some(SystemConfiguration {
        count: read_u8(formatted, 4)?,
    })
}

/// The BIOS language information record for Type 13.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BiosLanguage {
    /// The number of installed languages.
    pub installed_languages: u8,
    /// The language flags.
    pub flags: u8,
    /// The current language string index at offset `0x15` when present.
    pub current_language_idx: Option<u8>,
}

/// Parses the BIOS language information record.
#[must_use]
pub fn parse_bios_language(formatted: &[u8]) -> Option<BiosLanguage> {
    Some(BiosLanguage {
        installed_languages: read_u8(formatted, 4)?,
        flags: read_u8(formatted, 5)?,
        current_language_idx: read_u8(formatted, 0x15),
    })
}

/// A group association item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GroupAssociationItem {
    /// The referenced structure type.
    pub structure_type: u8,
    /// The referenced structure handle.
    pub handle: u16,
}

/// The group associations record for Type 14.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GroupAssociations<'a> {
    /// The group name string index.
    pub name_idx: u8,
    /// The raw item entries, three bytes each.
    pub items: &'a [u8],
}

impl GroupAssociations<'_> {
    /// Returns the number of associated items.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.items.len() / 3
    }

    /// Returns the item at `index`.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<GroupAssociationItem> {
        let offset = index.checked_mul(3)?;
        Some(GroupAssociationItem {
            structure_type: read_u8(self.items, offset)?,
            handle: u16_le(self.items, offset + 1)?,
        })
    }
}

/// Parses the group associations record.
#[must_use]
pub fn parse_group_associations(formatted: &[u8]) -> Option<GroupAssociations<'_>> {
    let items = formatted.get(5..)?;
    Some(GroupAssociations {
        name_idx: read_u8(formatted, 4)?,
        items: &items[..items.len() - items.len() % 3],
    })
}

/// The system event log record for Type 15.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemEventLog {
    /// The log area length.
    pub area_length: u16,
    /// The log header start offset.
    pub header_start_offset: u16,
    /// The log data start offset.
    pub data_start_offset: u16,
    /// The access method.
    pub access_method: u8,
    /// The log status.
    pub status: u8,
    /// The log change token.
    pub change_token: u32,
    /// The access method address.
    pub access_address: u32,
    /// The log header format.
    pub header_format: u8,
    /// The number of supported log type descriptors.
    pub descriptor_count: u8,
    /// The length of each log type descriptor.
    pub descriptor_length: u8,
}

/// Parses the system event log record.
#[must_use]
pub fn parse_system_event_log(formatted: &[u8]) -> Option<SystemEventLog> {
    Some(SystemEventLog {
        area_length: u16_le(formatted, 4)?,
        header_start_offset: u16_le(formatted, 6)?,
        data_start_offset: u16_le(formatted, 8)?,
        access_method: read_u8(formatted, 10)?,
        status: read_u8(formatted, 11)?,
        change_token: u32_le(formatted, 12)?,
        access_address: u32_le(formatted, 16)?,
        header_format: read_u8(formatted, 20)?,
        descriptor_count: read_u8(formatted, 21)?,
        descriptor_length: read_u8(formatted, 22)?,
    })
}

/// The 32-bit memory error information record for Type 18.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryError32 {
    /// The error type.
    pub error_type: u8,
    /// The error granularity.
    pub granularity: u8,
    /// The error operation.
    pub operation: u8,
    /// The vendor syndrome.
    pub vendor_syndrome: u32,
    /// The memory array error address.
    pub array_error_address: u32,
    /// The device error address.
    pub device_error_address: u32,
    /// The error resolution.
    pub resolution: u32,
}

/// Parses the 32-bit memory error information record.
#[must_use]
pub fn parse_memory_error32(formatted: &[u8]) -> Option<MemoryError32> {
    Some(MemoryError32 {
        error_type: read_u8(formatted, 4)?,
        granularity: read_u8(formatted, 5)?,
        operation: read_u8(formatted, 6)?,
        vendor_syndrome: u32_le(formatted, 7)?,
        array_error_address: u32_le(formatted, 11)?,
        device_error_address: u32_le(formatted, 15)?,
        resolution: u32_le(formatted, 19)?,
    })
}

/// The built-in pointing device record for Type 21.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PointingDevice {
    /// The pointing device type.
    pub device_type: u8,
    /// The interface type.
    pub interface: u8,
    /// The number of buttons.
    pub buttons: u8,
}

/// Parses the built-in pointing device record.
#[must_use]
pub fn parse_pointing_device(formatted: &[u8]) -> Option<PointingDevice> {
    Some(PointingDevice {
        device_type: read_u8(formatted, 4)?,
        interface: read_u8(formatted, 5)?,
        buttons: read_u8(formatted, 6)?,
    })
}

/// The portable battery record for Type 22.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PortableBattery {
    /// The location string index.
    pub location_idx: u8,
    /// The manufacturer string index.
    pub manufacturer_idx: u8,
    /// The manufacture date string index.
    pub manufacture_date_idx: u8,
    /// The serial number string index.
    pub serial_number_idx: u8,
    /// The device name string index.
    pub device_name_idx: u8,
    /// The device chemistry.
    pub chemistry: u8,
    /// The design capacity in milliwatt-hours.
    pub design_capacity: u16,
    /// The design voltage in millivolts.
    pub design_voltage: u16,
    /// The smart battery data specification version string index.
    pub sbds_version_idx: u8,
    /// The maximum error in battery data.
    pub maximum_error: u8,
    /// The smart battery serial number.
    pub sbds_serial: u16,
    /// The smart battery manufacture date.
    pub sbds_manufacture_date: u16,
    /// The smart battery chemistry string index.
    pub sbds_chemistry_idx: u8,
    /// The design capacity multiplier.
    pub capacity_multiplier: u8,
    /// The OEM-specific value when present.
    pub oem_specific: Option<u32>,
}

/// Parses the portable battery record.
#[must_use]
pub fn parse_portable_battery(formatted: &[u8]) -> Option<PortableBattery> {
    Some(PortableBattery {
        location_idx: read_u8(formatted, 4)?,
        manufacturer_idx: read_u8(formatted, 5)?,
        manufacture_date_idx: read_u8(formatted, 6)?,
        serial_number_idx: read_u8(formatted, 7)?,
        device_name_idx: read_u8(formatted, 8)?,
        chemistry: read_u8(formatted, 9)?,
        design_capacity: u16_le(formatted, 10)?,
        design_voltage: u16_le(formatted, 12)?,
        sbds_version_idx: read_u8(formatted, 14)?,
        maximum_error: read_u8(formatted, 15)?,
        sbds_serial: u16_le(formatted, 16)?,
        sbds_manufacture_date: u16_le(formatted, 18)?,
        sbds_chemistry_idx: read_u8(formatted, 20)?,
        capacity_multiplier: read_u8(formatted, 21)?,
        oem_specific: u32_le(formatted, 22),
    })
}

/// The system reset record for Type 23.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemReset {
    /// The reset capabilities.
    pub capabilities: u8,
    /// The reset count.
    pub reset_count: u16,
    /// The reset limit.
    pub reset_limit: u16,
    /// The timer interval in minutes.
    pub timer_interval: u16,
    /// The timeout in minutes.
    pub timeout: u16,
}

/// Parses the system reset record.
#[must_use]
pub fn parse_system_reset(formatted: &[u8]) -> Option<SystemReset> {
    Some(SystemReset {
        capabilities: read_u8(formatted, 4)?,
        reset_count: u16_le(formatted, 5)?,
        reset_limit: u16_le(formatted, 7)?,
        timer_interval: u16_le(formatted, 9)?,
        timeout: u16_le(formatted, 11)?,
    })
}

/// The hardware security record for Type 24.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HardwareSecurity {
    /// The hardware security settings.
    pub settings: u8,
}

/// Parses the hardware security record.
#[must_use]
pub fn parse_hardware_security(formatted: &[u8]) -> Option<HardwareSecurity> {
    Some(HardwareSecurity {
        settings: read_u8(formatted, 4)?,
    })
}

/// The system power controls record for Type 25.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemPowerControls {
    /// The next scheduled power-on month.
    pub month: u8,
    /// The next scheduled power-on day.
    pub day: u8,
    /// The next scheduled power-on hour.
    pub hour: u8,
    /// The next scheduled power-on minute.
    pub minute: u8,
    /// The next scheduled power-on second.
    pub second: u8,
}

/// Parses the system power controls record.
#[must_use]
pub fn parse_system_power_controls(formatted: &[u8]) -> Option<SystemPowerControls> {
    Some(SystemPowerControls {
        month: read_u8(formatted, 4)?,
        day: read_u8(formatted, 5)?,
        hour: read_u8(formatted, 6)?,
        minute: read_u8(formatted, 7)?,
        second: read_u8(formatted, 8)?,
    })
}

/// A voltage, temperature, or current probe record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProbeRecord {
    /// The description string index.
    pub description_idx: u8,
    /// The location and status byte.
    pub location_status: u8,
    /// The maximum value.
    pub maximum: i16,
    /// The minimum value.
    pub minimum: i16,
    /// The resolution.
    pub resolution: i16,
    /// The tolerance.
    pub tolerance: i16,
    /// The accuracy.
    pub accuracy: i16,
    /// The OEM-defined value.
    pub oem_specific: u32,
}

fn parse_probe(formatted: &[u8]) -> Option<ProbeRecord> {
    Some(ProbeRecord {
        description_idx: read_u8(formatted, 4)?,
        location_status: read_u8(formatted, 5)?,
        maximum: crate::base::bytes::read_i16_le(formatted, 6)?,
        minimum: crate::base::bytes::read_i16_le(formatted, 8)?,
        resolution: crate::base::bytes::read_i16_le(formatted, 10)?,
        tolerance: crate::base::bytes::read_i16_le(formatted, 12)?,
        accuracy: crate::base::bytes::read_i16_le(formatted, 14)?,
        oem_specific: u32_le(formatted, 16)?,
    })
}

/// Parses the voltage probe record for Type 26.
#[must_use]
pub fn parse_voltage_probe(formatted: &[u8]) -> Option<ProbeRecord> {
    parse_probe(formatted)
}

/// Parses the temperature probe record for Type 28.
#[must_use]
pub fn parse_temperature_probe(formatted: &[u8]) -> Option<ProbeRecord> {
    parse_probe(formatted)
}

/// Parses the electrical current probe record for Type 29.
#[must_use]
pub fn parse_current_probe(formatted: &[u8]) -> Option<ProbeRecord> {
    parse_probe(formatted)
}

/// The cooling device record for Type 27.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CoolingDevice {
    /// The temperature probe handle.
    pub temperature_probe_handle: u16,
    /// The device type and status byte.
    pub device_type_status: u8,
    /// The cooling unit group.
    pub cooling_unit_group: u8,
    /// The OEM-defined value.
    pub oem_specific: u32,
    /// The nominal speed in revolutions per minute.
    pub nominal_speed: u16,
    /// The description string index when present.
    pub description_idx: Option<u8>,
}

/// Parses the cooling device record.
#[must_use]
pub fn parse_cooling_device(formatted: &[u8]) -> Option<CoolingDevice> {
    Some(CoolingDevice {
        temperature_probe_handle: u16_le(formatted, 4)?,
        device_type_status: read_u8(formatted, 6)?,
        cooling_unit_group: read_u8(formatted, 7)?,
        oem_specific: u32_le(formatted, 8)?,
        nominal_speed: u16_le(formatted, 12)?,
        description_idx: read_u8(formatted, 14),
    })
}

/// The out-of-band remote access record for Type 30.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OutOfBandRemoteAccess {
    /// The manufacturer name string index.
    pub manufacturer_idx: u8,
    /// The connections flags.
    pub connections: u8,
}

/// Parses the out-of-band remote access record.
#[must_use]
pub fn parse_out_of_band_remote_access(formatted: &[u8]) -> Option<OutOfBandRemoteAccess> {
    Some(OutOfBandRemoteAccess {
        manufacturer_idx: read_u8(formatted, 4)?,
        connections: read_u8(formatted, 5)?,
    })
}

/// The system boot information record for Type 32.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemBoot<'a> {
    /// The reserved bytes at offset `0x04`.
    pub reserved: [u8; 6],
    /// The boot status bytes.
    pub boot_status: &'a [u8],
}

/// Parses the system boot information record.
#[must_use]
pub fn parse_system_boot(formatted: &[u8]) -> Option<SystemBoot<'_>> {
    let mut reserved = [0_u8; 6];
    reserved.copy_from_slice(formatted.get(4..10)?);
    Some(SystemBoot {
        reserved,
        boot_status: formatted.get(10..)?,
    })
}

/// The 64-bit memory error information record for Type 33.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryError64 {
    /// The error type.
    pub error_type: u8,
    /// The error granularity.
    pub granularity: u8,
    /// The error operation.
    pub operation: u8,
    /// The vendor syndrome.
    pub vendor_syndrome: u32,
    /// The memory array error address.
    pub array_error_address: u64,
    /// The device error address.
    pub device_error_address: u64,
    /// The error resolution.
    pub resolution: u32,
}

/// Parses the 64-bit memory error information record.
#[must_use]
pub fn parse_memory_error64(formatted: &[u8]) -> Option<MemoryError64> {
    Some(MemoryError64 {
        error_type: read_u8(formatted, 4)?,
        granularity: read_u8(formatted, 5)?,
        operation: read_u8(formatted, 6)?,
        vendor_syndrome: u32_le(formatted, 7)?,
        array_error_address: u64_le(formatted, 11)?,
        device_error_address: u64_le(formatted, 19)?,
        resolution: u32_le(formatted, 27)?,
    })
}

/// The management device record for Type 34.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ManagementDevice {
    /// The description string index.
    pub description_idx: u8,
    /// The device type.
    pub device_type: u8,
    /// The device address.
    pub address: u32,
    /// The address type.
    pub address_type: u8,
}

/// Parses the management device record.
#[must_use]
pub fn parse_management_device(formatted: &[u8]) -> Option<ManagementDevice> {
    Some(ManagementDevice {
        description_idx: read_u8(formatted, 4)?,
        device_type: read_u8(formatted, 5)?,
        address: u32_le(formatted, 6)?,
        address_type: read_u8(formatted, 10)?,
    })
}

/// The management device component record for Type 35.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ManagementDeviceComponent {
    /// The description string index.
    pub description_idx: u8,
    /// The management device handle.
    pub management_device_handle: u16,
    /// The component handle.
    pub component_handle: u16,
    /// The threshold data handle.
    pub threshold_handle: u16,
}

/// Parses the management device component record.
#[must_use]
pub fn parse_management_device_component(formatted: &[u8]) -> Option<ManagementDeviceComponent> {
    Some(ManagementDeviceComponent {
        description_idx: read_u8(formatted, 4)?,
        management_device_handle: u16_le(formatted, 5)?,
        component_handle: u16_le(formatted, 7)?,
        threshold_handle: u16_le(formatted, 9)?,
    })
}

/// The management device threshold data record for Type 36.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ManagementDeviceThresholdData {
    /// The lower non-critical threshold.
    pub lower_noncritical: u16,
    /// The upper non-critical threshold.
    pub upper_noncritical: u16,
    /// The lower critical threshold.
    pub lower_critical: u16,
    /// The upper critical threshold.
    pub upper_critical: u16,
    /// The lower non-recoverable threshold.
    pub lower_nonrecoverable: u16,
    /// The upper non-recoverable threshold.
    pub upper_nonrecoverable: u16,
}

/// Parses the management device threshold data record.
#[must_use]
pub fn parse_management_device_threshold_data(
    formatted: &[u8],
) -> Option<ManagementDeviceThresholdData> {
    Some(ManagementDeviceThresholdData {
        lower_noncritical: u16_le(formatted, 4)?,
        upper_noncritical: u16_le(formatted, 6)?,
        lower_critical: u16_le(formatted, 8)?,
        upper_critical: u16_le(formatted, 10)?,
        lower_nonrecoverable: u16_le(formatted, 12)?,
        upper_nonrecoverable: u16_le(formatted, 14)?,
    })
}

/// The memory channel record for Type 37.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryChannel<'a> {
    /// The channel type.
    pub channel_type: u8,
    /// The maximum channel load.
    pub maximum_load: u8,
    /// The memory device count.
    pub device_count: u8,
    /// The raw device handle entries.
    pub device_handles: &'a [u8],
}

impl MemoryChannel<'_> {
    /// Returns the device handle at `index`.
    #[must_use]
    pub fn handle(&self, index: usize) -> Option<u16> {
        u16_le(self.device_handles, index.checked_mul(2)?)
    }
}

/// Parses the memory channel record.
#[must_use]
pub fn parse_memory_channel(formatted: &[u8]) -> Option<MemoryChannel<'_>> {
    let handles = formatted.get(7..)?;
    Some(MemoryChannel {
        channel_type: read_u8(formatted, 4)?,
        maximum_load: read_u8(formatted, 5)?,
        device_count: read_u8(formatted, 6)?,
        device_handles: &handles[..handles.len() - handles.len() % 2],
    })
}

/// The system power supply record for Type 39.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemPowerSupply {
    /// The power unit group.
    pub power_unit_group: u8,
    /// The location string index.
    pub location_idx: u8,
    /// The device name string index.
    pub device_name_idx: u8,
    /// The manufacturer string index.
    pub manufacturer_idx: u8,
    /// The serial number string index.
    pub serial_number_idx: u8,
    /// The asset tag string index.
    pub asset_tag_idx: u8,
    /// The model part number string index.
    pub model_part_number_idx: u8,
    /// The revision level string index.
    pub revision_level_idx: u8,
    /// The maximum power capacity in watts.
    pub maximum_power: u16,
    /// The power supply characteristics.
    pub characteristics: u16,
    /// The input voltage probe handle.
    pub input_voltage_probe_handle: u16,
    /// The cooling device handle.
    pub cooling_device_handle: u16,
    /// The input current probe handle.
    pub input_current_probe_handle: u16,
}

/// Parses the system power supply record.
#[must_use]
pub fn parse_system_power_supply(formatted: &[u8]) -> Option<SystemPowerSupply> {
    Some(SystemPowerSupply {
        power_unit_group: read_u8(formatted, 4)?,
        location_idx: read_u8(formatted, 5)?,
        device_name_idx: read_u8(formatted, 6)?,
        manufacturer_idx: read_u8(formatted, 7)?,
        serial_number_idx: read_u8(formatted, 8)?,
        asset_tag_idx: read_u8(formatted, 9)?,
        model_part_number_idx: read_u8(formatted, 10)?,
        revision_level_idx: read_u8(formatted, 11)?,
        maximum_power: u16_le(formatted, 12)?,
        characteristics: u16_le(formatted, 14)?,
        input_voltage_probe_handle: u16_le(formatted, 16)?,
        cooling_device_handle: u16_le(formatted, 18)?,
        input_current_probe_handle: u16_le(formatted, 20)?,
    })
}

/// A single additional information entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdditionalInformationEntry<'a> {
    /// The entry value string.
    pub value: &'a [u8],
}

/// The additional information record for Type 40.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdditionalInformation<'a> {
    /// The declared entry count.
    pub count: u8,
    /// The raw entry stream.
    pub entries: &'a [u8],
}

impl<'a> AdditionalInformation<'a> {
    /// Iterates the length-prefixed entries.
    #[must_use]
    pub fn iter(&self) -> AdditionalInformationIter<'a> {
        AdditionalInformationIter {
            bytes: self.entries,
            offset: 0,
        }
    }
}

impl<'a> IntoIterator for &AdditionalInformation<'a> {
    type Item = AdditionalInformationEntry<'a>;
    type IntoIter = AdditionalInformationIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterates additional information entries.
#[derive(Clone, Debug)]
pub struct AdditionalInformationIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for AdditionalInformationIter<'a> {
    type Item = AdditionalInformationEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let length = usize::from(*self.bytes.get(self.offset)?);
        if length < 1 {
            return None;
        }
        let end = self.offset.checked_add(length)?;
        let value = self.bytes.get(self.offset + 1..end)?;
        self.offset = end;
        Some(AdditionalInformationEntry { value })
    }
}

/// Parses the additional information record.
#[must_use]
pub fn parse_additional_information(formatted: &[u8]) -> Option<AdditionalInformation<'_>> {
    Some(AdditionalInformation {
        count: read_u8(formatted, 4)?,
        entries: formatted.get(5..)?,
    })
}

/// The onboard devices extended information record for Type 41.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OnboardDevicesExtended {
    /// The reference designation string index.
    pub reference_designation_idx: u8,
    /// The device type.
    pub device_type: u8,
    /// The device type instance.
    pub device_type_instance: u8,
}

/// Parses the onboard devices extended information record.
#[must_use]
pub fn parse_onboard_devices_extended(formatted: &[u8]) -> Option<OnboardDevicesExtended> {
    Some(OnboardDevicesExtended {
        reference_designation_idx: read_u8(formatted, 4)?,
        device_type: read_u8(formatted, 5)?,
        device_type_instance: read_u8(formatted, 6)?,
    })
}

/// The management controller host interface record for Type 42.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HostInterface<'a> {
    /// The interface type.
    pub interface_type: u8,
    /// The interface-specific data.
    pub interface_data: &'a [u8],
    /// The protocol records following the interface data.
    pub protocol_records: &'a [u8],
}

/// Parses the management controller host interface record.
#[must_use]
pub fn parse_host_interface(formatted: &[u8]) -> Option<HostInterface<'_>> {
    let interface_type = read_u8(formatted, 4)?;
    let length = usize::from(read_u8(formatted, 5)?);
    let data = formatted.get(6..6_usize.checked_add(length)?)?;
    let protocol_records = formatted.get(6 + length..)?;
    Some(HostInterface {
        interface_type,
        interface_data: data,
        protocol_records,
    })
}

/// The processor additional information record for Type 44.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProcessorAdditional<'a> {
    /// The referenced processor handle.
    pub referenced_handle: u16,
    /// The processor type.
    pub processor_type: u8,
    /// The processor-specific data.
    pub data: &'a [u8],
}

/// Parses the processor additional information record.
#[must_use]
pub fn parse_processor_additional(formatted: &[u8]) -> Option<ProcessorAdditional<'_>> {
    Some(ProcessorAdditional {
        referenced_handle: u16_le(formatted, 4)?,
        processor_type: read_u8(formatted, 6)?,
        data: formatted.get(7..)?,
    })
}

/// The firmware inventory information record for Type 45.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FirmwareInventoryInformation {
    /// The firmware component name string index.
    pub component_name_idx: u8,
    /// The firmware version string index.
    pub version_idx: u8,
    /// The firmware version format.
    pub version_format: u8,
    /// The firmware identifier string index.
    pub firmware_id_idx: u8,
    /// The firmware identifier format.
    pub firmware_id_format: u8,
    /// The release date string index.
    pub release_date_idx: u8,
    /// The manufacturer string index.
    pub manufacturer_idx: u8,
    /// The lowest supported version string index.
    pub lowest_supported_version_idx: u8,
    /// The image size in bytes.
    pub image_size: u64,
    /// The firmware characteristics.
    pub characteristics: u16,
    /// The firmware state.
    pub state: u8,
    /// The number of associated components.
    pub associated_component_count: u8,
}

/// Parses the firmware inventory information record.
#[must_use]
pub fn parse_firmware_inventory_information(
    formatted: &[u8],
) -> Option<FirmwareInventoryInformation> {
    Some(FirmwareInventoryInformation {
        component_name_idx: read_u8(formatted, 4)?,
        version_idx: read_u8(formatted, 5)?,
        version_format: read_u8(formatted, 6)?,
        firmware_id_idx: read_u8(formatted, 7)?,
        firmware_id_format: read_u8(formatted, 8)?,
        release_date_idx: read_u8(formatted, 9)?,
        manufacturer_idx: read_u8(formatted, 10)?,
        lowest_supported_version_idx: read_u8(formatted, 11)?,
        image_size: u64_le(formatted, 12)?,
        characteristics: u16_le(formatted, 20)?,
        state: read_u8(formatted, 22)?,
        associated_component_count: read_u8(formatted, 23)?,
    })
}

/// The string property record for Type 46.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StringProperty {
    /// The string property identifier.
    pub property_id: u16,
    /// The string property value index.
    pub value_idx: u8,
    /// The parent handle.
    pub parent_handle: u16,
}

/// Parses the string property record.
#[must_use]
pub fn parse_string_property(formatted: &[u8]) -> Option<StringProperty> {
    Some(StringProperty {
        property_id: u16_le(formatted, 4)?,
        value_idx: read_u8(formatted, 6)?,
        parent_handle: u16_le(formatted, 7)?,
    })
}

/// The IPMI device information record for Type 38.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IpmiDeviceInformation {
    /// The interface type at offset `0x04`.
    pub interface_type: u8,
    /// The IPMI specification revision at offset `0x05`.
    pub specification_revision: u8,
    /// The I2C slave address at offset `0x06`.
    pub i2c_slave_address: u8,
    /// The non-volatile storage device address at offset `0x07`.
    pub nv_storage_device_address: u8,
    /// The base address at offset `0x08`.
    pub base_address: u64,
    /// The base address modifier and interrupt information at offset `0x10`.
    pub base_address_modifier: u8,
    /// The interrupt number at offset `0x11`.
    pub interrupt_number: u8,
}

/// Parses the IPMI device information record.
///
/// Returns `None` if the formatted section is shorter than the
/// 18-byte minimum for Type 38.
#[must_use]
pub fn parse_ipmi_device_information(formatted: &[u8]) -> Option<IpmiDeviceInformation> {
    if formatted.len() < 0x12 {
        return None;
    }
    Some(IpmiDeviceInformation {
        interface_type: read_u8(formatted, 0x04)?,
        specification_revision: read_u8(formatted, 0x05)?,
        i2c_slave_address: read_u8(formatted, 0x06)?,
        nv_storage_device_address: read_u8(formatted, 0x07)?,
        base_address: u64_le(formatted, 0x08)?,
        base_address_modifier: read_u8(formatted, 0x10)?,
        interrupt_number: read_u8(formatted, 0x11)?,
    })
}

/// The TPM device record for Type 43.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TpmDevice {
    /// The vendor identifier at offset `0x04`.
    pub vendor_id: [u8; 4],
    /// The major specification version at offset `0x08`.
    pub major_spec_version: u8,
    /// The minor specification version at offset `0x09`.
    pub minor_spec_version: u8,
    /// The first firmware version word at offset `0x0A`.
    pub firmware_version_1: u32,
    /// The second firmware version word at offset `0x0E`.
    pub firmware_version_2: u32,
    /// The description string index at offset `0x12`.
    pub description_idx: u8,
    /// The TPM characteristics at offset `0x13`.
    pub characteristics: u64,
    /// The OEM-defined value at offset `0x1B`.
    pub oem_defined: u32,
}

/// Parses the TPM device record.
///
/// Returns `None` if the formatted section is shorter than the
/// 31-byte minimum for Type 43.
#[must_use]
pub fn parse_tpm_device(formatted: &[u8]) -> Option<TpmDevice> {
    if formatted.len() < 0x1F {
        return None;
    }
    let mut vendor_id = [0_u8; 4];
    vendor_id.copy_from_slice(formatted.get(0x04..0x08)?);
    Some(TpmDevice {
        vendor_id,
        major_spec_version: read_u8(formatted, 0x08)?,
        minor_spec_version: read_u8(formatted, 0x09)?,
        firmware_version_1: u32_le(formatted, 0x0A)?,
        firmware_version_2: u32_le(formatted, 0x0E)?,
        description_idx: read_u8(formatted, 0x12)?,
        characteristics: u64_le(formatted, 0x13)?,
        oem_defined: u32_le(formatted, 0x1B)?,
    })
}
