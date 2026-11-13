//! Integration tests for HTTP Utilities Protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::network::http::{
    EFI_HTTP_UTILITIES_PROTOCOL_GUID, EfiHttpHeader, EfiHttpUtilitiesProtocol, HttpUtilities,
    format_header_line, parse_header_line, parse_raw_headers,
};

#[test]
fn test_http_utilities_guid() {
    assert_eq!(
        EFI_HTTP_UTILITIES_PROTOCOL_GUID,
        EfiGuid::new(
            0x3E35_C163,
            0x4074,
            0x45DD,
            [0x43, 0x1E, 0x23, 0x98, 0x9D, 0xD8, 0x6B, 0x32],
        )
    );
}

#[test]
fn test_parse_header_line() {
    let line = b"Content-Type: application/json";
    let parsed = parse_header_line(line);
    assert_eq!(parsed, Some(("Content-Type", "application/json")));

    let line_spaces = b"   Host :   rover-firmware.local   ";
    let parsed_spaces = parse_header_line(line_spaces);
    assert_eq!(parsed_spaces, Some(("Host", "rover-firmware.local")));

    let invalid = b"NoColonHeader";
    assert_eq!(parse_header_line(invalid), None);

    let empty_name = b": value_only";
    assert_eq!(parse_header_line(empty_name), None);
}

#[test]
fn test_format_header_line() {
    let mut buf = [0u8; 64];
    let res = format_header_line("User-Agent", "Rover-UEFI", &mut buf);
    assert!(res.is_ok());
    if let Ok(len) = res {
        assert_eq!(&buf[..len], b"User-Agent: Rover-UEFI\r\n");
    }

    let mut small_buf = [0u8; 10];
    let small_res = format_header_line("User-Agent", "Rover-UEFI", &mut small_buf);
    assert_eq!(small_res, Err(EfiStatus::BUFFER_TOO_SMALL));
}

#[test]
fn test_parse_raw_headers() {
    let raw = b"Host: uefi.org\r\nAccept: */*\r\nConnection: close\r\n\r\nBodyStartsHere";
    let mut entries = [("", ""); 8];
    let res = parse_raw_headers(raw, &mut entries);
    assert!(res.is_ok());
    if let Ok(count) = res {
        assert_eq!(count, 3);
        assert_eq!(entries[0], ("Host", "uefi.org"));
        assert_eq!(entries[1], ("Accept", "*/*"));
        assert_eq!(entries[2], ("Connection", "close"));
    }

    // Test buffer too small
    let mut tiny_entries = [("", ""); 2];
    let tiny_res = parse_raw_headers(raw, &mut tiny_entries);
    assert_eq!(tiny_res, Err(EfiStatus::BUFFER_TOO_SMALL));
}

static mut MOCK_BUILD_CALLED: bool = false;
static mut MOCK_PARSE_CALLED: bool = false;

unsafe extern "efiapi" fn mock_build(
    _this: *mut EfiHttpUtilitiesProtocol,
    _seed_message_size: UIntN,
    _seed_message: *const c_void,
    _delete_count: UIntN,
    _delete_list: *const *const u8,
    _append_count: UIntN,
    _append_list: *const *const EfiHttpHeader,
    new_message_size: *mut UIntN,
    new_message: *mut *mut c_void,
) -> EfiStatus {
    unsafe {
        MOCK_BUILD_CALLED = true;
        if !new_message_size.is_null() {
            *new_message_size = 42;
        }
        if !new_message.is_null() {
            *new_message = 0x1234_5678 as *mut c_void;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_parse(
    _this: *mut EfiHttpUtilitiesProtocol,
    _http_message: *const u8,
    _http_message_size: UIntN,
    header_fields: *mut *mut EfiHttpHeader,
    field_count: *mut UIntN,
) -> EfiStatus {
    unsafe {
        MOCK_PARSE_CALLED = true;
        if !field_count.is_null() {
            *field_count = 3;
        }
        if !header_fields.is_null() {
            *header_fields = 0x8765_4321 as *mut EfiHttpHeader;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_http_utilities_protocol_client_wrapper() {
    let mut raw_proto = EfiHttpUtilitiesProtocol {
        build: mock_build,
        parse: mock_parse,
    };

    let mut client = HttpUtilities::new(&mut raw_proto);
    assert!(!client.raw().is_null());

    let mut new_msg_size = 0usize;
    let mut new_msg = core::ptr::null_mut::<c_void>();

    let build_res = unsafe {
        client.build_raw(
            0,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            core::ptr::null(),
            &mut new_msg_size,
            &mut new_msg,
        )
    };
    assert!(build_res.is_ok());
    assert_eq!(new_msg_size, 42);
    assert_eq!(new_msg as usize, 0x1234_5678);

    let mut header_fields = core::ptr::null_mut::<EfiHttpHeader>();
    let mut field_count = 0usize;
    let msg_str = b"Host: test\r\n\r\n";

    let parse_res = unsafe {
        client.parse_raw(
            msg_str.as_ptr(),
            msg_str.len(),
            &mut header_fields,
            &mut field_count,
        )
    };
    assert!(parse_res.is_ok());
    assert_eq!(field_count, 3);
    assert_eq!(header_fields as usize, 0x8765_4321);
}
