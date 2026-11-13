//! Unit and integration tests for UEFI Human Interface Infrastructure (HII).

#![allow(clippy::expect_used, clippy::unwrap_used)]

use core::ffi::c_void;
use rover_uefi::abi::types::{Bool, UIntN};
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::firmware::hii::{
    EFI_HII_CONFIG_ROUTING_PROTOCOL_GUID, EFI_HII_DATABASE_PROTOCOL_GUID,
    EFI_HII_FONT_PROTOCOL_GUID, EFI_HII_IMAGE_DECODER_NAME_JPEG_GUID,
    EFI_HII_IMAGE_DECODER_NAME_PNG_GUID, EFI_HII_IMAGE_DECODER_PROTOCOL_GUID,
    EFI_HII_IMAGE_PROTOCOL_GUID, EFI_HII_POPUP_PROTOCOL_GUID,
    EFI_HII_SET_KEYBOARD_LAYOUT_EVENT_GUID, EFI_HII_STRING_PROTOCOL_GUID, EFI_STRING_ID_NULL,
    EfiHiiDatabaseNotify, EfiHiiDatabaseProtocol, EfiHiiHandle, EfiHiiImageDecoderProtocol,
    EfiHiiKeyboardLayout, EfiHiiPackageListHeader, EfiHiiPopupProtocol, EfiHiiStringProtocol,
    EfiImageOutput, EfiKeyDescriptor, EfiStringId, HiiDatabase, HiiImageDecoder,
    HiiImageDecoderColorType, HiiImageDecoderHeader, HiiPackageType, HiiPopup, HiiPopupSelection,
    HiiPopupStyle, HiiPopupType, HiiString, is_known_package_type, parse_package_header,
};

#[test]
fn test_hii_guids_and_package_types() {
    assert_eq!(
        EFI_HII_DATABASE_PROTOCOL_GUID,
        EfiGuid::new(
            0xEF9F_C172,
            0xA1B2,
            0x4693,
            [0xB3, 0x27, 0x6D, 0x32, 0xFC, 0x41, 0x60, 0x42],
        )
    );
    assert_eq!(
        EFI_HII_STRING_PROTOCOL_GUID,
        EfiGuid::new(
            0x0FD9_6974,
            0x23AA,
            0x4CDC,
            [0xB9, 0xCB, 0x98, 0xD1, 0x77, 0x50, 0x32, 0x2A],
        )
    );
    assert_eq!(
        EFI_HII_CONFIG_ROUTING_PROTOCOL_GUID,
        EfiGuid::new(
            0x587E_72D7,
            0xCC50,
            0x4F79,
            [0x82, 0x09, 0xCA, 0x29, 0x1F, 0xC1, 0xA1, 0x0F],
        )
    );
    assert_eq!(
        EFI_HII_FONT_PROTOCOL_GUID,
        EfiGuid::new(
            0xE9CA_4775,
            0x8657,
            0x47FC,
            [0x97, 0xE7, 0x7E, 0xD6, 0x5A, 0x08, 0x43, 0x24],
        )
    );
    assert_eq!(
        EFI_HII_IMAGE_PROTOCOL_GUID,
        EfiGuid::new(
            0x31A6_406A,
            0x6B56,
            0x4E25,
            [0x89, 0x21, 0x99, 0x2D, 0xC3, 0xFC, 0xFE, 0x16],
        )
    );
    assert_eq!(
        EFI_HII_POPUP_PROTOCOL_GUID,
        EfiGuid::new(
            0x4311_EDC0,
            0x6054,
            0x46D4,
            [0x9E, 0x40, 0x89, 0x3E, 0xA9, 0x52, 0xFC, 0xCC],
        )
    );
    assert_eq!(
        EFI_HII_IMAGE_DECODER_PROTOCOL_GUID,
        EfiGuid::new(
            0x9E66_F251,
            0x727C,
            0x418C,
            [0xBF, 0xD6, 0xC2, 0xB4, 0x25, 0x28, 0x18, 0xEA],
        )
    );
    assert_eq!(
        EFI_HII_IMAGE_DECODER_NAME_JPEG_GUID,
        EfiGuid::new(
            0xEFEF_D093,
            0x0D9B,
            0x46EB,
            [0xA8, 0x56, 0x48, 0x35, 0x07, 0x00, 0xC9, 0x08],
        )
    );
    assert_eq!(
        EFI_HII_IMAGE_DECODER_NAME_PNG_GUID,
        EfiGuid::new(
            0xAF06_0190,
            0x5E3A,
            0x4025,
            [0xAF, 0xBD, 0xE1, 0xF9, 0x05, 0xBF, 0xAA, 0x4C],
        )
    );
    assert_eq!(
        EFI_HII_SET_KEYBOARD_LAYOUT_EVENT_GUID,
        EfiGuid::new(
            0x1498_2A35,
            0x5A29,
            0x498D,
            [0xA2, 0x44, 0x63, 0x98, 0x8C, 0x74, 0x1E, 0x93],
        )
    );

    assert!(is_known_package_type(HiiPackageType::Forms as u8));
    assert!(is_known_package_type(HiiPackageType::Strings as u8));
    assert!(is_known_package_type(HiiPackageType::End as u8));
    assert!(!is_known_package_type(0xAA));
}

#[test]
fn test_hii_package_header_parsing() {
    // 24-bit length = 16 bytes, type = Strings (0x03)
    let mut buffer = [0u8; 16];
    buffer[0] = 16;
    buffer[1] = 0;
    buffer[2] = 0;
    buffer[3] = HiiPackageType::Strings as u8;

    let hdr = parse_package_header(&buffer).unwrap();
    assert_eq!(hdr.package_length, 16);
    assert_eq!(hdr.package_type, 0x03);

    // Buffer smaller than 4 bytes
    assert_eq!(parse_package_header(&[1, 2, 3]), None);

    // Declared length exceeds buffer size
    let short_buf = [100, 0, 0, 0x02];
    assert_eq!(parse_package_header(&short_buf), None);
}

#[test]
fn test_hii_key_descriptor_layout() {
    let key_desc = EfiKeyDescriptor {
        key: 0x0041,
        unicode_char: u16::from(b'a'),
        shifted_unicode_char: u16::from(b'A'),
        alt_gr_unicode_char: 0,
        shifted_alt_gr_unicode_char: 0,
        modifier: 0,
    };
    assert_eq!(key_desc.unicode_char, u16::from(b'a'));
    assert_eq!(key_desc.shifted_unicode_char, u16::from(b'A'));
}

// --- Mock HII Database Protocol ---
unsafe extern "efiapi" fn mock_new_package_list(
    _this: *const EfiHiiDatabaseProtocol,
    _package_list: *const EfiHiiPackageListHeader,
    _driver_handle: EfiHandle,
    handle: *mut EfiHiiHandle,
) -> EfiStatus {
    unsafe {
        *handle = EfiHiiHandle::from_ptr(0x5000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_remove_package_list(
    _this: *const EfiHiiDatabaseProtocol,
    handle: EfiHiiHandle,
) -> EfiStatus {
    if handle == EfiHiiHandle::from_ptr(0x5000 as *mut c_void) {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_update_package_list(
    _this: *const EfiHiiDatabaseProtocol,
    _handle: EfiHiiHandle,
    _package_list: *const EfiHiiPackageListHeader,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_list_package_lists(
    _this: *const EfiHiiDatabaseProtocol,
    _package_type: u8,
    _package_guid: *const EfiGuid,
    _handle_buffer_length: *mut UIntN,
    _handle_buffer: *mut EfiHiiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_export_package_lists(
    _this: *const EfiHiiDatabaseProtocol,
    _handle: EfiHiiHandle,
    _buffer_size: *mut UIntN,
    _buffer: *mut EfiHiiPackageListHeader,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_package_notify(
    _this: *const EfiHiiDatabaseProtocol,
    _package_type: u8,
    _package_guid: *const EfiGuid,
    _package_notify_fn: EfiHiiDatabaseNotify,
    _notify_type: UIntN,
    _notify_handle: *mut EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unregister_package_notify(
    _this: *const EfiHiiDatabaseProtocol,
    _notify_handle: EfiHandle,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_find_keyboard_layouts(
    _this: *const EfiHiiDatabaseProtocol,
    _key_guid_buffer_length: *mut u16,
    _key_guid_buffer: *mut EfiGuid,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_keyboard_layout(
    _this: *const EfiHiiDatabaseProtocol,
    _key_guid: *const EfiGuid,
    _keyboard_layout_length: *mut u16,
    _keyboard_layout: *mut EfiHiiKeyboardLayout,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_keyboard_layout(
    _this: *const EfiHiiDatabaseProtocol,
    _key_guid: *const EfiGuid,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_package_list_handle(
    _this: *const EfiHiiDatabaseProtocol,
    _package_list_handle: EfiHiiHandle,
    driver_handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        *driver_handle = EfiHandle::from_ptr(0x7000 as *mut c_void);
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_hii_database_client_wrapper() {
    let raw_protocol = EfiHiiDatabaseProtocol {
        new_package_list: mock_new_package_list,
        remove_package_list: mock_remove_package_list,
        update_package_list: mock_update_package_list,
        list_package_lists: mock_list_package_lists,
        export_package_lists: mock_export_package_lists,
        register_package_notify: mock_register_package_notify,
        unregister_package_notify: mock_unregister_package_notify,
        find_keyboard_layouts: mock_find_keyboard_layouts,
        get_keyboard_layout: mock_get_keyboard_layout,
        set_keyboard_layout: mock_set_keyboard_layout,
        get_package_list_handle: mock_get_package_list_handle,
    };

    let client = HiiDatabase::new(&raw_protocol);
    let pkg_list_hdr = EfiHiiPackageListHeader {
        package_list_guid: EFI_HII_DATABASE_PROTOCOL_GUID,
        package_length: 32,
    };

    let handle = client
        .new_package_list(&pkg_list_hdr, EfiHandle::NULL)
        .unwrap();
    assert_eq!(handle, EfiHiiHandle::from_ptr(0x5000 as *mut c_void));

    let driver_handle = client.get_driver_handle(handle).unwrap();
    assert_eq!(driver_handle, EfiHandle::from_ptr(0x7000 as *mut c_void));

    assert!(client.remove_package_list(handle).is_ok());
    assert!(client.remove_package_list(EfiHiiHandle::NULL).is_err());
}

// --- Mock HII String Protocol ---
unsafe extern "efiapi" fn mock_new_string(
    _this: *const EfiHiiStringProtocol,
    _package_list: EfiHiiHandle,
    string_id: *mut EfiStringId,
    _language: *const u8,
    _language_name: *const u16,
    _string: *const u16,
    _string_font_info: *const c_void,
) -> EfiStatus {
    unsafe {
        *string_id = 42;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_string(
    _this: *const EfiHiiStringProtocol,
    _language: *const u8,
    _package_list: EfiHiiHandle,
    string_id: EfiStringId,
    string: *mut u16,
    string_size: *mut UIntN,
    _string_font_info: *mut *mut c_void,
) -> EfiStatus {
    if string_id == 42 {
        unsafe {
            let required_size = 5 * core::mem::size_of::<u16>();
            if *string_size < required_size {
                *string_size = required_size;
                return EfiStatus::BUFFER_TOO_SMALL;
            }
            *string.add(0) = u16::from(b'T');
            *string.add(1) = u16::from(b'E');
            *string.add(2) = u16::from(b'S');
            *string.add(3) = u16::from(b'T');
            *string.add(4) = 0;
            *string_size = required_size;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_set_string(
    _this: *const EfiHiiStringProtocol,
    _package_list: EfiHiiHandle,
    _string_id: EfiStringId,
    _language: *const u8,
    _string: *const u16,
    _string_font_info: *const c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_languages(
    _this: *const EfiHiiStringProtocol,
    _package_list: EfiHiiHandle,
    _languages: *mut u8,
    _languages_size: *mut UIntN,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_secondary_languages(
    _this: *const EfiHiiStringProtocol,
    _package_list: EfiHiiHandle,
    _primary_language: *const u8,
    _secondary_languages: *mut u8,
    _secondary_languages_size: *mut UIntN,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_hii_string_client_wrapper() {
    let raw_protocol = EfiHiiStringProtocol {
        new_string: mock_new_string,
        get_string: mock_get_string,
        set_string: mock_set_string,
        get_languages: mock_get_languages,
        get_secondary_languages: mock_get_secondary_languages,
    };

    let client = HiiString::new(&raw_protocol);
    let sample_text: [u16; 5] = [
        u16::from(b'A'),
        u16::from(b'B'),
        u16::from(b'C'),
        u16::from(b'D'),
        0,
    ];

    let id = client
        .new_string(EfiHiiHandle::NULL, b"en-US\0", &sample_text)
        .unwrap();
    assert_eq!(id, 42);

    let mut out_buf = [0u16; 8];
    let num_chars = client
        .get_string(EfiHiiHandle::NULL, id, b"en-US\0", &mut out_buf)
        .unwrap();
    assert_eq!(num_chars, 5);
    assert_eq!(out_buf[0], u16::from(b'T'));
    assert_eq!(out_buf[1], u16::from(b'E'));
    assert_eq!(out_buf[2], u16::from(b'S'));
    assert_eq!(out_buf[3], u16::from(b'T'));

    assert!(
        client
            .get_string(
                EfiHiiHandle::NULL,
                EFI_STRING_ID_NULL,
                b"en-US\0",
                &mut out_buf
            )
            .is_err()
    );
}

// --- Mock HII Popup Protocol ---
unsafe extern "efiapi" fn mock_create_popup(
    _this: *const EfiHiiPopupProtocol,
    _popup_style: HiiPopupStyle,
    _popup_type: HiiPopupType,
    _hii_handle: EfiHiiHandle,
    _message: EfiStringId,
    user_selection: *mut HiiPopupSelection,
) -> EfiStatus {
    unsafe {
        *user_selection = HiiPopupSelection::Yes;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_hii_popup_client_wrapper() {
    let raw_protocol = EfiHiiPopupProtocol {
        revision: 1,
        create_popup: mock_create_popup,
    };

    let client = HiiPopup::new(&raw_protocol);
    let selection = client
        .create_popup(
            HiiPopupStyle::Warning,
            HiiPopupType::YesNo,
            EfiHiiHandle::NULL,
            42,
        )
        .unwrap();
    assert_eq!(selection, HiiPopupSelection::Yes);
}

// --- Mock HII Image Decoder Protocol ---
static TEST_IMAGE_HEADER: HiiImageDecoderHeader = HiiImageDecoderHeader {
    decoder_name: EFI_HII_IMAGE_DECODER_NAME_PNG_GUID,
    image_info_size: 28,
    image_width: 1920,
    image_height: 1080,
    color_type: HiiImageDecoderColorType::Rgba,
    color_depth_in_bits: 32,
};

unsafe extern "efiapi" fn mock_get_image_decoder_name(
    _this: *const EfiHiiImageDecoderProtocol,
    _decoder_name: *mut *mut EfiGuid,
    _number_of_decoder_names: *mut u16,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_image_info(
    _this: *const EfiHiiImageDecoderProtocol,
    _image: *const c_void,
    size_of_image: UIntN,
    image_info: *mut *mut HiiImageDecoderHeader,
) -> EfiStatus {
    if size_of_image >= 8 {
        unsafe {
            *image_info = (&raw const TEST_IMAGE_HEADER).cast_mut();
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

unsafe extern "efiapi" fn mock_decode_image(
    _this: *const EfiHiiImageDecoderProtocol,
    _image: *const c_void,
    _image_raw_data_size: UIntN,
    _bitmap: *mut *mut EfiImageOutput,
    _transparent: Bool,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_hii_image_decoder_client_wrapper() {
    let raw_protocol = EfiHiiImageDecoderProtocol {
        get_image_decoder_name: mock_get_image_decoder_name,
        get_image_info: mock_get_image_info,
        decode_image: mock_decode_image,
    };

    let client = HiiImageDecoder::new(&raw_protocol);
    let fake_png_bytes = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

    let header = client.get_image_info(&fake_png_bytes).unwrap();
    assert_eq!(header.decoder_name, EFI_HII_IMAGE_DECODER_NAME_PNG_GUID);
    assert_eq!(header.image_width, 1920);
    assert_eq!(header.image_height, 1080);
    assert_eq!(header.color_type, HiiImageDecoderColorType::Rgba);
    assert_eq!(header.color_depth_in_bits, 32);

    let short_bytes = [1, 2, 3];
    assert!(client.get_image_info(&short_bytes).is_err());
}
