//! Tests for the extended SMBIOS typed decoders.

use rover_uefi::protocols::smbios::typed::{
    parse_additional_information, parse_bios_language, parse_cooling_device, parse_current_probe,
    parse_firmware_inventory_information, parse_group_associations, parse_hardware_security,
    parse_host_interface, parse_ipmi_device_information, parse_management_device,
    parse_management_device_component, parse_management_device_threshold_data,
    parse_memory_channel, parse_memory_error32, parse_memory_error64, parse_oem_strings,
    parse_onboard_devices, parse_onboard_devices_extended, parse_out_of_band_remote_access,
    parse_pointing_device, parse_portable_battery, parse_processor_additional,
    parse_string_property, parse_system_boot, parse_system_configuration, parse_system_event_log,
    parse_system_power_controls, parse_system_power_supply, parse_system_reset,
    parse_temperature_probe, parse_tpm_device, parse_voltage_probe,
};

/// Builds a formatted section with a four-byte header followed by `body`.
fn section(body: &[u8]) -> Vec<u8> {
    let mut buffer = vec![0x00, 0x20, 0x00, 0x00];
    buffer.extend_from_slice(body);
    buffer
}

#[test]
fn onboard_devices_iterates_entries() {
    let formatted = section(&[0x83, 0x01, 0x05, 0x02]);
    let Some(devices) = parse_onboard_devices(&formatted) else {
        panic!("onboard devices did not parse")
    };
    assert_eq!(devices.count(), 2);
    let Some(first) = devices.get(0) else {
        panic!("first device missing")
    };
    assert_eq!(first.device_type, 0x83);
    assert_eq!(first.description_idx, 0x01);
    let Some(second) = devices.get(1) else {
        panic!("second device missing")
    };
    assert_eq!(second.device_type, 0x05);
    assert_eq!(devices.get(2), None);
}

#[test]
fn onboard_devices_ignores_trailing_partial_entry() {
    let formatted = section(&[0x83, 0x01, 0x05]);
    let Some(devices) = parse_onboard_devices(&formatted) else {
        panic!("onboard devices did not parse")
    };
    assert_eq!(devices.count(), 1);
}

#[test]
fn simple_count_records() {
    let oem = section(&[3]);
    assert_eq!(parse_oem_strings(&oem).map(|value| value.count), Some(3));
    assert_eq!(parse_oem_strings(&[]), None);

    let configuration = section(&[5]);
    assert_eq!(
        parse_system_configuration(&configuration).map(|value| value.count),
        Some(5)
    );

    let language = section(&[2, 0x01]);
    let Some(language) = parse_bios_language(&language) else {
        panic!("BIOS language did not parse")
    };
    assert_eq!(language.installed_languages, 2);
    assert_eq!(language.flags, 1);
    assert_eq!(language.current_language_idx, None);
    let mut full = vec![2, 0x01];
    full.resize(0x11, 0);
    full.push(0x03);
    let Some(language) = parse_bios_language(&section(&full)) else {
        panic!("BIOS language did not parse")
    };
    assert_eq!(language.current_language_idx, Some(3));
    assert_eq!(parse_bios_language(&section(&[2])), None);
}

#[test]
fn group_associations_iterates_items() {
    let mut body = vec![0x07];
    body.extend_from_slice(&[0x04, 0x34, 0x12]);
    body.extend_from_slice(&[0x11, 0x78, 0x56]);
    let formatted = section(&body);
    let Some(groups) = parse_group_associations(&formatted) else {
        panic!("group associations did not parse")
    };
    assert_eq!(groups.name_idx, 0x07);
    assert_eq!(groups.count(), 2);
    let Some(first) = groups.get(0) else {
        panic!("first item missing")
    };
    assert_eq!(first.structure_type, 0x04);
    assert_eq!(first.handle, 0x1234);
    let Some(second) = groups.get(1) else {
        panic!("second item missing")
    };
    assert_eq!(second.structure_type, 0x11);
    assert_eq!(second.handle, 0x5678);
    assert_eq!(groups.get(2), None);
}

#[test]
fn system_event_log_decodes_fields() {
    let mut body = Vec::new();
    body.extend_from_slice(&0x0100_u16.to_le_bytes());
    body.extend_from_slice(&0x0020_u16.to_le_bytes());
    body.extend_from_slice(&0x0080_u16.to_le_bytes());
    body.push(0x01);
    body.push(0x03);
    body.extend_from_slice(&0xDEAD_BEEF_u32.to_le_bytes());
    body.extend_from_slice(&0x1234_5678_u32.to_le_bytes());
    body.push(0x00);
    body.push(0x02);
    body.push(0x14);
    let formatted = section(&body);
    let Some(log) = parse_system_event_log(&formatted) else {
        panic!("event log did not parse")
    };
    assert_eq!(log.area_length, 0x0100);
    assert_eq!(log.header_start_offset, 0x0020);
    assert_eq!(log.data_start_offset, 0x0080);
    assert_eq!(log.access_method, 0x01);
    assert_eq!(log.status, 0x03);
    assert_eq!(log.change_token, 0xDEAD_BEEF);
    assert_eq!(log.access_address, 0x1234_5678);
    assert_eq!(log.header_format, 0x00);
    assert_eq!(log.descriptor_count, 0x02);
    assert_eq!(log.descriptor_length, 0x14);
}

#[test]
fn memory_error_records_decode_32_and_64_bit_addresses() {
    let mut body = Vec::new();
    body.push(0x03);
    body.push(0x02);
    body.push(0x01);
    body.extend_from_slice(&0xAAAA_BBBB_u32.to_le_bytes());
    body.extend_from_slice(&0x1111_2222_u32.to_le_bytes());
    body.extend_from_slice(&0x3333_4444_u32.to_le_bytes());
    body.extend_from_slice(&0x0000_0010_u32.to_le_bytes());
    let formatted = section(&body);
    let Some(error) = parse_memory_error32(&formatted) else {
        panic!("32-bit memory error did not parse")
    };
    assert_eq!(error.error_type, 0x03);
    assert_eq!(error.vendor_syndrome, 0xAAAA_BBBB);
    assert_eq!(error.array_error_address, 0x1111_2222);
    assert_eq!(error.device_error_address, 0x3333_4444);
    assert_eq!(error.resolution, 0x0000_0010);
    assert_eq!(parse_memory_error32(&section(&[0, 0])), None);

    let mut body = Vec::new();
    body.push(0x04);
    body.push(0x05);
    body.push(0x06);
    body.extend_from_slice(&0xCAFE_F00D_u32.to_le_bytes());
    body.extend_from_slice(&0x0123_4567_89AB_CDEF_u64.to_le_bytes());
    body.extend_from_slice(&0xFEDC_BA98_7654_3210_u64.to_le_bytes());
    body.extend_from_slice(&0x0000_0040_u32.to_le_bytes());
    let formatted = section(&body);
    let Some(error) = parse_memory_error64(&formatted) else {
        panic!("64-bit memory error did not parse")
    };
    assert_eq!(error.error_type, 0x04);
    assert_eq!(error.vendor_syndrome, 0xCAFE_F00D);
    assert_eq!(error.array_error_address, 0x0123_4567_89AB_CDEF);
    assert_eq!(error.device_error_address, 0xFEDC_BA98_7654_3210);
    assert_eq!(error.resolution, 0x0000_0040);
}

#[test]
fn pointing_device_decodes_fields() {
    let formatted = section(&[0x03, 0x06, 0x05]);
    let Some(device) = parse_pointing_device(&formatted) else {
        panic!("pointing device did not parse")
    };
    assert_eq!(device.device_type, 0x03);
    assert_eq!(device.interface, 0x06);
    assert_eq!(device.buttons, 0x05);
    assert_eq!(parse_pointing_device(&section(&[0x03])), None);
}

#[test]
fn portable_battery_parses_optional_oem_field() {
    let mut body = vec![1, 2, 3, 4, 5, 6];
    body.extend_from_slice(&0x0BB8_u16.to_le_bytes());
    body.extend_from_slice(&0x2DC6_u16.to_le_bytes());
    body.push(7);
    body.push(8);
    body.extend_from_slice(&0x0102_u16.to_le_bytes());
    body.extend_from_slice(&0x0304_u16.to_le_bytes());
    body.push(9);
    body.push(10);
    let without_oem = section(&body);
    let Some(battery) = parse_portable_battery(&without_oem) else {
        panic!("battery did not parse")
    };
    assert_eq!(battery.location_idx, 1);
    assert_eq!(battery.chemistry, 6);
    assert_eq!(battery.design_capacity, 0x0BB8);
    assert_eq!(battery.design_voltage, 0x2DC6);
    assert_eq!(battery.sbds_serial, 0x0102);
    assert_eq!(battery.sbds_manufacture_date, 0x0304);
    assert_eq!(battery.capacity_multiplier, 10);
    assert_eq!(battery.oem_specific, None);

    body.extend_from_slice(&0x1234_5678_u32.to_le_bytes());
    let with_oem = section(&body);
    let Some(battery) = parse_portable_battery(&with_oem) else {
        panic!("battery did not parse")
    };
    assert_eq!(battery.oem_specific, Some(0x1234_5678));
}

#[test]
fn reset_security_and_power_controls() {
    let mut body = Vec::new();
    body.push(0x0F);
    body.extend_from_slice(&3_u16.to_le_bytes());
    body.extend_from_slice(&5_u16.to_le_bytes());
    body.extend_from_slice(&30_u16.to_le_bytes());
    body.extend_from_slice(&60_u16.to_le_bytes());
    let formatted = section(&body);
    let Some(reset) = parse_system_reset(&formatted) else {
        panic!("system reset did not parse")
    };
    assert_eq!(reset.capabilities, 0x0F);
    assert_eq!(reset.reset_count, 3);
    assert_eq!(reset.reset_limit, 5);
    assert_eq!(reset.timer_interval, 30);
    assert_eq!(reset.timeout, 60);

    let Some(security) = parse_hardware_security(&section(&[0x02])) else {
        panic!("hardware security did not parse")
    };
    assert_eq!(security.settings, 0x02);

    let Some(power) = parse_system_power_controls(&section(&[12, 31, 23, 59, 58])) else {
        panic!("power controls did not parse")
    };
    assert_eq!(power.month, 12);
    assert_eq!(power.day, 31);
    assert_eq!(power.hour, 23);
    assert_eq!(power.minute, 59);
    assert_eq!(power.second, 58);
    assert_eq!(parse_system_power_controls(&section(&[12, 31])), None);
}

fn probe_section() -> Vec<u8> {
    let mut body = Vec::new();
    body.push(0x09);
    body.push(0x63);
    body.extend_from_slice(&0x0100_u16.to_le_bytes());
    body.extend_from_slice(&0x0001_u16.to_le_bytes());
    body.extend_from_slice(&0x0002_u16.to_le_bytes());
    body.extend_from_slice(&0x0003_u16.to_le_bytes());
    body.extend_from_slice(&0x0004_u16.to_le_bytes());
    body.extend_from_slice(&0x1122_3344_u32.to_le_bytes());
    section(&body)
}

#[test]
fn probe_records_share_layout() {
    let formatted = probe_section();
    let Some(voltage) = parse_voltage_probe(&formatted) else {
        panic!("voltage probe did not parse")
    };
    let Some(temperature) = parse_temperature_probe(&formatted) else {
        panic!("temperature probe did not parse")
    };
    let Some(current) = parse_current_probe(&formatted) else {
        panic!("current probe did not parse")
    };
    assert_eq!(voltage, temperature);
    assert_eq!(voltage, current);
    assert_eq!(voltage.description_idx, 0x09);
    assert_eq!(voltage.location_status, 0x63);
    assert_eq!(voltage.maximum, 0x0100);
    assert_eq!(voltage.minimum, 0x0001);
    assert_eq!(voltage.resolution, 0x0002);
    assert_eq!(voltage.tolerance, 0x0003);
    assert_eq!(voltage.accuracy, 0x0004);
    assert_eq!(voltage.oem_specific, 0x1122_3344);
    assert_eq!(parse_voltage_probe(&section(&[0; 5])), None);
}

#[test]
fn cooling_device_optional_description() {
    let mut body = Vec::new();
    body.extend_from_slice(&0x0042_u16.to_le_bytes());
    body.push(0x67);
    body.push(0x01);
    body.extend_from_slice(&0xAABB_CCDD_u32.to_le_bytes());
    body.extend_from_slice(&0x0BB8_u16.to_le_bytes());
    let without = section(&body);
    let Some(cooling) = parse_cooling_device(&without) else {
        panic!("cooling device did not parse")
    };
    assert_eq!(cooling.temperature_probe_handle, 0x0042);
    assert_eq!(cooling.device_type_status, 0x67);
    assert_eq!(cooling.cooling_unit_group, 0x01);
    assert_eq!(cooling.oem_specific, 0xAABB_CCDD);
    assert_eq!(cooling.nominal_speed, 0x0BB8);
    assert_eq!(cooling.description_idx, None);

    body.push(0x0C);
    let with = section(&body);
    let Some(cooling) = parse_cooling_device(&with) else {
        panic!("cooling device did not parse")
    };
    assert_eq!(cooling.description_idx, Some(0x0C));
}

#[test]
fn remote_access_and_system_boot() {
    let Some(remote) = parse_out_of_band_remote_access(&section(&[0x09, 0x03])) else {
        panic!("remote access did not parse")
    };
    assert_eq!(remote.manufacturer_idx, 0x09);
    assert_eq!(remote.connections, 0x03);

    let boot_section = section(&[0, 0, 0, 0, 0, 0, 0x01]);
    let Some(boot) = parse_system_boot(&boot_section) else {
        panic!("system boot did not parse")
    };
    assert_eq!(boot.reserved, [0; 6]);
    assert_eq!(boot.boot_status, &[0x01]);
    let boot_section = section(&[1, 2, 3, 4, 5, 6]);
    let Some(boot) = parse_system_boot(&boot_section) else {
        panic!("system boot did not parse")
    };
    assert_eq!(boot.reserved, [1, 2, 3, 4, 5, 6]);
    assert_eq!(boot.boot_status.len(), 0);
    assert_eq!(parse_system_boot(&section(&[0; 5])), None);
}

#[test]
fn management_device_records() {
    let mut body = Vec::new();
    body.push(0x07);
    body.push(0x08);
    body.extend_from_slice(&0xDEAD_BEEF_u32.to_le_bytes());
    body.push(0x02);
    let formatted = section(&body);
    let Some(device) = parse_management_device(&formatted) else {
        panic!("management device did not parse")
    };
    assert_eq!(device.description_idx, 0x07);
    assert_eq!(device.device_type, 0x08);
    assert_eq!(device.address, 0xDEAD_BEEF);
    assert_eq!(device.address_type, 0x02);

    let mut body = Vec::new();
    body.push(0x0A);
    body.extend_from_slice(&0x0001_u16.to_le_bytes());
    body.extend_from_slice(&0x0002_u16.to_le_bytes());
    body.extend_from_slice(&0x0003_u16.to_le_bytes());
    let Some(component) = parse_management_device_component(&section(&body)) else {
        panic!("component did not parse")
    };
    assert_eq!(component.description_idx, 0x0A);
    assert_eq!(component.management_device_handle, 1);
    assert_eq!(component.component_handle, 2);
    assert_eq!(component.threshold_handle, 3);

    let mut body = Vec::new();
    for value in [10_u16, 20, 30, 40, 50, 60] {
        body.extend_from_slice(&value.to_le_bytes());
    }
    let Some(threshold) = parse_management_device_threshold_data(&section(&body)) else {
        panic!("threshold did not parse")
    };
    assert_eq!(threshold.lower_noncritical, 10);
    assert_eq!(threshold.upper_noncritical, 20);
    assert_eq!(threshold.lower_critical, 30);
    assert_eq!(threshold.upper_critical, 40);
    assert_eq!(threshold.lower_nonrecoverable, 50);
    assert_eq!(threshold.upper_nonrecoverable, 60);
    assert_eq!(
        parse_management_device_threshold_data(&section(&[0; 8])),
        None
    );
}

#[test]
fn memory_channel_decodes_handles() {
    let mut body = Vec::new();
    body.push(0x03);
    body.push(0x20);
    body.push(0x02);
    body.extend_from_slice(&0x1111_u16.to_le_bytes());
    body.extend_from_slice(&0x2222_u16.to_le_bytes());
    let channel_section = section(&body);
    let Some(channel) = parse_memory_channel(&channel_section) else {
        panic!("memory channel did not parse")
    };
    assert_eq!(channel.channel_type, 0x03);
    assert_eq!(channel.maximum_load, 0x20);
    assert_eq!(channel.device_count, 0x02);
    assert_eq!(channel.handle(0), Some(0x1111));
    assert_eq!(channel.handle(1), Some(0x2222));
    assert_eq!(channel.handle(2), None);
}

#[test]
fn system_power_supply_decodes_fields() {
    let mut body = Vec::new();
    for value in [1_u8, 2, 3, 4, 5, 6, 7, 8] {
        body.push(value);
    }
    body.extend_from_slice(&500_u16.to_le_bytes());
    body.extend_from_slice(&0x000F_u16.to_le_bytes());
    body.extend_from_slice(&0x0101_u16.to_le_bytes());
    body.extend_from_slice(&0x0202_u16.to_le_bytes());
    body.extend_from_slice(&0x0303_u16.to_le_bytes());
    let Some(supply) = parse_system_power_supply(&section(&body)) else {
        panic!("power supply did not parse")
    };
    assert_eq!(supply.power_unit_group, 1);
    assert_eq!(supply.location_idx, 2);
    assert_eq!(supply.device_name_idx, 3);
    assert_eq!(supply.manufacturer_idx, 4);
    assert_eq!(supply.serial_number_idx, 5);
    assert_eq!(supply.asset_tag_idx, 6);
    assert_eq!(supply.model_part_number_idx, 7);
    assert_eq!(supply.revision_level_idx, 8);
    assert_eq!(supply.maximum_power, 500);
    assert_eq!(supply.characteristics, 0x000F);
    assert_eq!(supply.input_voltage_probe_handle, 0x0101);
    assert_eq!(supply.cooling_device_handle, 0x0202);
    assert_eq!(supply.input_current_probe_handle, 0x0303);
    assert_eq!(parse_system_power_supply(&section(&[0; 10])), None);
}

#[test]
fn additional_information_iterates_length_prefixed_entries() {
    let formatted = section(&[2, 4, b'O', b'E', b'M', 3, b'X', b'Y']);
    let Some(info) = parse_additional_information(&formatted) else {
        panic!("additional information did not parse")
    };
    assert_eq!(info.count, 2);
    let entries: Vec<&[u8]> = info.iter().map(|entry| entry.value).collect();
    assert_eq!(entries, vec![&b"OEM"[..], &b"XY"[..]]);

    let truncated = section(&[1, 8, b'A']);
    let Some(info) = parse_additional_information(&truncated) else {
        panic!("additional information did not parse")
    };
    assert_eq!(info.iter().count(), 0);
}

#[test]
fn onboard_devices_extended_decodes_fields() {
    let Some(extended) = parse_onboard_devices_extended(&section(&[0x11, 0x05, 0x02])) else {
        panic!("onboard extended did not parse")
    };
    assert_eq!(extended.reference_designation_idx, 0x11);
    assert_eq!(extended.device_type, 0x05);
    assert_eq!(extended.device_type_instance, 0x02);
    assert_eq!(
        parse_onboard_devices_extended(&section(&[0x11, 0x05])),
        None
    );
}

#[test]
fn host_interface_splits_data_and_protocols() {
    let mut body = Vec::new();
    body.push(0x40);
    body.push(0x04);
    body.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD]);
    body.push(0x01);
    body.extend_from_slice(&[0x10, 0x20, 0x30]);
    let host_section = section(&body);
    let Some(host) = parse_host_interface(&host_section) else {
        panic!("host interface did not parse")
    };
    assert_eq!(host.interface_type, 0x40);
    assert_eq!(host.interface_data, &[0xAA, 0xBB, 0xCC, 0xDD]);
    assert_eq!(host.protocol_records, &[0x01, 0x10, 0x20, 0x30]);
    assert_eq!(parse_host_interface(&section(&[0x40, 0x08, 0x00])), None);
}

#[test]
fn processor_additional_decodes_fields() {
    let processor_section = section(&[0x34, 0x12, 0x03, 0x01, 0x02]);
    let Some(processor) = parse_processor_additional(&processor_section) else {
        panic!("processor additional did not parse")
    };
    assert_eq!(processor.referenced_handle, 0x1234);
    assert_eq!(processor.processor_type, 0x03);
    assert_eq!(processor.data, &[0x01, 0x02]);
    assert_eq!(parse_processor_additional(&section(&[0x34, 0x12])), None);
}

#[test]
fn firmware_inventory_decodes_fields() {
    let mut body = Vec::new();
    for value in [1_u8, 2, 3, 4, 5, 6, 7, 8] {
        body.push(value);
    }
    body.extend_from_slice(&0x0000_0001_0000_0000_u64.to_le_bytes());
    body.extend_from_slice(&0x00FF_u16.to_le_bytes());
    body.push(0x02);
    body.push(0x03);
    let Some(inventory) = parse_firmware_inventory_information(&section(&body)) else {
        panic!("firmware inventory did not parse")
    };
    assert_eq!(inventory.component_name_idx, 1);
    assert_eq!(inventory.version_idx, 2);
    assert_eq!(inventory.version_format, 3);
    assert_eq!(inventory.firmware_id_idx, 4);
    assert_eq!(inventory.firmware_id_format, 5);
    assert_eq!(inventory.release_date_idx, 6);
    assert_eq!(inventory.manufacturer_idx, 7);
    assert_eq!(inventory.lowest_supported_version_idx, 8);
    assert_eq!(inventory.image_size, 0x0000_0001_0000_0000);
    assert_eq!(inventory.characteristics, 0x00FF);
    assert_eq!(inventory.state, 0x02);
    assert_eq!(inventory.associated_component_count, 0x03);
}

#[test]
fn string_property_decodes_fields() {
    let mut body = Vec::new();
    body.extend_from_slice(&0x0005_u16.to_le_bytes());
    body.push(0x07);
    body.extend_from_slice(&0x0042_u16.to_le_bytes());
    let Some(property) = parse_string_property(&section(&body)) else {
        panic!("string property did not parse")
    };
    assert_eq!(property.property_id, 5);
    assert_eq!(property.value_idx, 7);
    assert_eq!(property.parent_handle, 0x0042);
    assert_eq!(parse_string_property(&section(&[0x05, 0x00])), None);
}

#[test]
fn ipmi_device_information_decodes_fields() {
    let mut body = vec![0x01, 0x20, 0x42, 0x00];
    body.extend_from_slice(&0x0000_0CA2_u64.to_le_bytes());
    body.push(0x03);
    body.push(0x0B);
    let Some(ipmi) = parse_ipmi_device_information(&section(&body)) else {
        panic!("IPMI device information did not parse")
    };
    assert_eq!(ipmi.interface_type, 0x01);
    assert_eq!(ipmi.specification_revision, 0x20);
    assert_eq!(ipmi.i2c_slave_address, 0x42);
    assert_eq!(ipmi.nv_storage_device_address, 0x00);
    assert_eq!(ipmi.base_address, 0x0000_0CA2);
    assert_eq!(ipmi.base_address_modifier, 0x03);
    assert_eq!(ipmi.interrupt_number, 0x0B);
    assert_eq!(parse_ipmi_device_information(&section(&[0; 13])), None);
}

#[test]
fn tpm_device_decodes_fields() {
    let mut body = Vec::new();
    body.extend_from_slice(b"IFX ");
    body.push(2);
    body.push(0);
    body.extend_from_slice(&0x0001_0002_u32.to_le_bytes());
    body.extend_from_slice(&0x0003_0004_u32.to_le_bytes());
    body.push(0x07);
    body.extend_from_slice(&0x1122_3344_5566_7788_u64.to_le_bytes());
    body.extend_from_slice(&0xAABB_CCDD_u32.to_le_bytes());
    let Some(tpm) = parse_tpm_device(&section(&body)) else {
        panic!("TPM device did not parse")
    };
    assert_eq!(&tpm.vendor_id, b"IFX ");
    assert_eq!(tpm.major_spec_version, 2);
    assert_eq!(tpm.minor_spec_version, 0);
    assert_eq!(tpm.firmware_version_1, 0x0001_0002);
    assert_eq!(tpm.firmware_version_2, 0x0003_0004);
    assert_eq!(tpm.description_idx, 0x07);
    assert_eq!(tpm.characteristics, 0x1122_3344_5566_7788);
    assert_eq!(tpm.oem_defined, 0xAABB_CCDD);
    assert_eq!(parse_tpm_device(&section(&[0; 26])), None);
}
