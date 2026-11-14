//! Integration tests for UEFI Boot Options and Load Options.

use rover_uefi::protocols::console::EfiInputKey;
use rover_uefi::tables::boot_option::*;

#[test]
fn test_boot_option_variable_names_and_paths() {
    assert_eq!(EFI_BOOT_ORDER_VARIABLE_NAME, "BootOrder");
    assert_eq!(EFI_BOOT_NEXT_VARIABLE_NAME, "BootNext");
    assert_eq!(EFI_BOOT_CURRENT_VARIABLE_NAME, "BootCurrent");
    assert_eq!(EFI_TIMEOUT_VARIABLE_NAME, "Timeout");
    assert_eq!(
        EFI_REMOVABLE_MEDIA_FILE_NAME_X64,
        "\\EFI\\BOOT\\BOOTX64.EFI"
    );
    assert_eq!(
        EFI_REMOVABLE_MEDIA_FILE_NAME_AARCH64,
        "\\EFI\\BOOT\\BOOTAA64.EFI"
    );
    assert_eq!(
        EFI_REMOVABLE_MEDIA_FILE_NAME_IA32,
        "\\EFI\\BOOT\\BOOTIA32.EFI"
    );
    assert_eq!(EFI_CAPSULE_FILE_DIRECTORY, "\\EFI\\UpdateCapsule\\");
}

#[test]
fn test_format_and_parse_option_names() {
    let mut buf8 = [0u8; 8];
    assert_eq!(format_boot_option_name(0x0001, &mut buf8), "Boot0001");
    assert_eq!(format_boot_option_name(0x001A, &mut buf8), "Boot001A");
    assert_eq!(format_boot_option_name(0xFFFF, &mut buf8), "BootFFFF");

    let mut buf10 = [0u8; 10];
    assert_eq!(format_driver_option_name(0x0002, &mut buf10), "Driver0002");

    let mut buf11 = [0u8; 11];
    assert_eq!(
        format_sysprep_option_name(0x0003, &mut buf11),
        "SysPrep0003"
    );

    let mut buf20 = [0u8; 20];
    assert_eq!(
        format_platform_recovery_option_name(0x0005, &mut buf20),
        "PlatformRecovery0005"
    );

    let mut buf_load = [0u8; 20];
    assert_eq!(
        format_load_option_name(LoadOptionType::Boot, 0x0010, &mut buf_load),
        "Boot0010"
    );
    assert_eq!(
        format_load_option_name(LoadOptionType::PlatformRecovery, 0x0020, &mut buf_load),
        "PlatformRecovery0020"
    );

    let mut buf7 = [0u8; 7];
    assert_eq!(format_key_option_name(0x0004, &mut buf7), "Key0004");

    assert_eq!(parse_boot_option_index("Boot0001"), Some(0x0001));
    assert_eq!(parse_boot_option_index("Boot001A"), Some(0x001A));
    assert_eq!(parse_boot_option_index("BOOT1234"), Some(0x1234));
    assert_eq!(parse_boot_option_index("Boot001Z"), None);
    assert_eq!(parse_boot_option_index("Boot01"), None);
    assert_eq!(parse_boot_option_index("Driver0001"), None);

    assert_eq!(
        is_valid_load_option_variable_name("Boot0001"),
        Some((LoadOptionType::Boot, 0x0001))
    );
    assert_eq!(
        is_valid_load_option_variable_name("Driver00A0"),
        Some((LoadOptionType::Driver, 0x00A0))
    );
    assert_eq!(
        is_valid_load_option_variable_name("SysPrep0002"),
        Some((LoadOptionType::SysPrep, 0x0002))
    );
    assert_eq!(
        is_valid_load_option_variable_name("PlatformRecovery0003"),
        Some((LoadOptionType::PlatformRecovery, 0x0003))
    );
    assert_eq!(is_valid_load_option_variable_name("Invalid0001"), None);

    assert_eq!(is_valid_key_option_variable_name("Key0001"), Some(0x0001));
    assert_eq!(is_valid_key_option_variable_name("KeyFFFF"), Some(0xFFFF));
    assert_eq!(is_valid_key_option_variable_name("Key01"), None);
}

#[test]
fn test_load_option_serialization_and_parsing() {
    let desc: [u16; 8] = [
        u16::from(b'R'),
        u16::from(b'o'),
        u16::from(b'v'),
        u16::from(b'e'),
        u16::from(b'r'),
        u16::from(b'O'),
        u16::from(b'S'),
        0x0020,
    ];
    let file_path_list = [0x04, 0x01, 0x06, 0x00, 0x7F, 0xFF, 0x04, 0x00];
    let optional_data = [0xDE, 0xAD, 0xBE, 0xEF];

    let original = LoadOptionView {
        attributes: LOAD_OPTION_ACTIVE | LOAD_OPTION_CATEGORY_BOOT,
        description: &desc,
        file_path_list: &file_path_list,
        optional_data: &optional_data,
    };

    assert!(original.is_active());
    assert!(!original.is_hidden());
    assert!(!original.is_force_reconnect());
    assert!(!original.is_app());

    let mut buffer = [0u8; 128];
    let size_result = original.serialize(&mut buffer);
    assert!(size_result.is_ok());
    if let Ok(size) = size_result {
        assert_eq!(size, original.serialized_size());
        assert!(validate_load_option(&buffer[..size]));

        let parsed = LoadOptionView::parse(&buffer[..size]);
        assert!(parsed.is_ok());
        if let Ok(view) = parsed {
            assert_eq!(view.attributes, original.attributes);
            assert_eq!(view.description, original.description);
            assert_eq!(view.file_path_list, original.file_path_list);
            assert_eq!(view.optional_data, original.optional_data);
            assert!(view.is_active());
        }
    }

    assert!(!validate_load_option(&[0u8; 4]));
}

#[test]
fn test_boot_order_view_and_manipulation() {
    let raw = [0x01, 0x00, 0x05, 0x00, 0x0A, 0x00];
    let view = BootOrderView::new(&raw);

    assert_eq!(view.count(), 3);
    assert_eq!(view.get(0), Some(1));
    assert_eq!(view.get(1), Some(5));
    assert_eq!(view.get(2), Some(10));
    assert_eq!(view.get(3), None);
    assert_eq!(view.find(5), Some(1));
    assert_eq!(view.find(99), None);

    let collected: Vec<u16> = view.iter().collect();
    assert_eq!(collected, vec![1, 5, 10]);

    let order = [1u16, 5, 10];
    let mut out = [0u16; 8];

    // Add option 3 at position 1
    let new_len = add_option_number_to_order(&order, 3, 1, &mut out);
    assert!(new_len.is_ok());
    if let Ok(len) = new_len {
        assert_eq!(len, 4);
        assert_eq!(&out[..4], &[1, 3, 5, 10]);
    }

    // Attempt to add duplicate option 5
    assert_eq!(
        add_option_number_to_order(&order, 5, 0, &mut out),
        Err(LoadOptionError::AlreadyExists)
    );

    // Remove option 5
    let mut rem_out = [0u16; 8];
    let rem_len = remove_option_number_from_order(&order, 5, &mut rem_out);
    assert!(rem_len.is_ok());
    if let Ok(len) = rem_len {
        assert_eq!(len, 2);
        assert_eq!(&rem_out[..2], &[1, 10]);
    }

    // Find free option number
    let free = find_free_option_number(&[0, 1, 2], Some(3));
    assert_eq!(free, Ok(4));
}

#[test]
fn test_key_option_parsing_and_serialization() {
    let key_data = EfiBootKeyData::new(true, false, true, false, false, false, 1);
    assert!(key_data.shift_pressed());
    assert!(!key_data.control_pressed());
    assert!(key_data.alt_pressed());
    assert_eq!(key_data.input_key_count(), 1);

    let key_data2 =
        EfiBootKeyData::from_modifiers(BOOT_MANAGER_SHIFT_PRESSED | BOOT_MANAGER_ALT_PRESSED, 1);
    assert_eq!(key_data.0, key_data2.0);

    let header = EfiKeyOptionHeader {
        key_data: key_data.0,
        boot_option_crc: 0x1234_5678,
        boot_option: 0x0001,
    };

    let key = EfiInputKey {
        scan_code: 0x0017, // Esc
        unicode_char: 0,
    };

    let mut raw_buffer = [0u8; 64];
    let hdr_size = core::mem::size_of::<EfiKeyOptionHeader>();
    let key_size = core::mem::size_of::<EfiInputKey>();

    unsafe {
        core::ptr::copy_nonoverlapping(
            (&raw const header).cast::<u8>(),
            raw_buffer.as_mut_ptr(),
            hdr_size,
        );
        core::ptr::copy_nonoverlapping(
            (&raw const key).cast::<u8>(),
            raw_buffer.as_mut_ptr().add(hdr_size),
            key_size,
        );
    }

    let parsed = KeyOptionView::parse(&raw_buffer[..hdr_size + key_size]);
    assert!(parsed.is_ok());
    if let Ok(view) = parsed {
        assert_eq!(view.boot_option, 0x0001);
        assert_eq!(view.boot_option_crc, 0x1234_5678);
        assert_eq!(view.keys.len(), 1);
        assert_eq!(view.keys[0].scan_code, 0x0017);

        assert!(view.validate_crc(b"payload", |_| 0x1234_5678));
        assert!(!view.validate_crc(b"payload", |_| 0x9999_9999));

        let mut ser_buf = [0u8; 64];
        let ser_len = view.serialize(&mut ser_buf);
        assert!(ser_len.is_ok());
        if let Ok(l) = ser_len {
            assert_eq!(l, view.serialized_size());
            assert_eq!(&ser_buf[..l], &raw_buffer[..hdr_size + key_size]);
        }
    }
}
