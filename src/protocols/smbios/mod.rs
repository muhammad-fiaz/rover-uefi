//! # SMBIOS
//!
//! SMBIOS table parsing from byte slices.

pub mod discovery;
pub mod entry_point;
pub mod guid;
pub mod header;
pub mod iterator;
pub mod records;
pub mod strings;
pub mod structure;
pub mod typed;
pub mod types;
pub mod validation;

pub use guid::{SMBIOS_2X_GUID, SMBIOS_3X_GUID};
pub use typed::{
    decode_cache_size_kb, decode_memory_device_size_kb, parse_additional_information,
    parse_baseboard_info, parse_bios_info, parse_bios_language, parse_cache_info,
    parse_cooling_device, parse_current_probe, parse_enclosure_info,
    parse_firmware_inventory_information, parse_group_associations, parse_hardware_security,
    parse_host_interface, parse_ipmi_device_information, parse_management_device,
    parse_management_device_component, parse_management_device_threshold_data,
    parse_memory_channel, parse_memory_device, parse_memory_device_mapped, parse_memory_error32,
    parse_memory_error64, parse_oem_strings, parse_onboard_devices, parse_onboard_devices_extended,
    parse_out_of_band_remote_access, parse_physical_memory_array, parse_pointing_device,
    parse_port_connector, parse_portable_battery, parse_processor_additional, parse_processor_info,
    parse_string_property, parse_system_boot, parse_system_configuration, parse_system_event_log,
    parse_system_info, parse_system_power_controls, parse_system_power_supply, parse_system_reset,
    parse_system_slots, parse_temperature_probe, parse_tpm_device, parse_voltage_probe,
};
