//! Print2 and `Print2S` protocol integration tests.

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::console::print2::{
    EFI_PRINT2_PROTOCOL_GUID, EFI_PRINT2S_PROTOCOL_GUID, EfiPrint2Protocol, EfiPrint2SProtocol,
    Print2, Print2S, format_int64_ascii, format_int64_unicode, print_flags,
};

#[test]
fn test_print2_protocol_guids() {
    let expected_print2 = EfiGuid::new(
        0xF059_76EF,
        0x83F1,
        0x4F3D,
        [0x86, 0x19, 0xF7, 0x59, 0x5D, 0x41, 0xE5, 0x38],
    );
    assert_eq!(EFI_PRINT2_PROTOCOL_GUID, expected_print2);

    let expected_s = EfiGuid::new(
        0x0CC2_52D2,
        0xC106,
        0x4661,
        [0xB5, 0xBD, 0x31, 0x47, 0xA4, 0xF8, 0x1F, 0x92],
    );
    assert_eq!(EFI_PRINT2S_PROTOCOL_GUID, expected_s);
}

#[test]
fn test_print_flags() {
    assert_eq!(print_flags::PREFIX_ZERO, 0x01);
    assert_eq!(print_flags::COMMA_TYPE, 0x02);
    assert_eq!(print_flags::RADIX_HEX, 0x04);
    assert_eq!(print_flags::LEFT_JUSTIFY, 0x08);
    assert_eq!(print_flags::MAXIMUM_VALUE_CHARACTERS, 38);
}

#[test]
fn test_format_int64_ascii_basic() {
    let mut buf = [0u8; 64];

    // Positive integer
    let Ok(len) = format_int64_ascii(&mut buf, 12345, 0, 0) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 5);
    assert_eq!(&buf[..6], b"12345\0");

    // Zero
    let Ok(len) = format_int64_ascii(&mut buf, 0, 0, 0) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 1);
    assert_eq!(&buf[..2], b"0\0");

    // Negative integer
    let Ok(len) = format_int64_ascii(&mut buf, -9876, 0, 0) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 5);
    assert_eq!(&buf[..6], b"-9876\0");
}

#[test]
fn test_format_int64_ascii_hex() {
    let mut buf = [0u8; 64];

    // Hex 0
    let Ok(len) = format_int64_ascii(&mut buf, 0, print_flags::RADIX_HEX, 0) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 1);
    assert_eq!(&buf[..2], b"0\0");

    // Hex positive
    let Ok(len) = format_int64_ascii(&mut buf, 0x001A_2B3C, print_flags::RADIX_HEX, 0) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 6);
    assert_eq!(&buf[..7], b"1A2B3C\0");

    // Hex with zero padding
    let Ok(len) = format_int64_ascii(
        &mut buf,
        0xFE,
        print_flags::RADIX_HEX | print_flags::PREFIX_ZERO,
        6,
    ) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 6);
    assert_eq!(&buf[..7], b"0000FE\0");
}

#[test]
fn test_format_int64_ascii_comma() {
    let mut buf = [0u8; 64];

    // Comma 1,234,567
    let Ok(len) = format_int64_ascii(&mut buf, 1_234_567, print_flags::COMMA_TYPE, 0) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 9);
    assert_eq!(&buf[..10], b"1,234,567\0");

    // Negative with comma
    let Ok(len) = format_int64_ascii(&mut buf, -1_000_000, print_flags::COMMA_TYPE, 0) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 10);
    assert_eq!(&buf[..11], b"-1,000,000\0");
}

#[test]
fn test_format_int64_ascii_zero_padding() {
    let mut buf = [0u8; 64];

    let Ok(len) = format_int64_ascii(&mut buf, 42, print_flags::PREFIX_ZERO, 6) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 6);
    assert_eq!(&buf[..7], b"000042\0");

    let Ok(len) = format_int64_ascii(&mut buf, -42, print_flags::PREFIX_ZERO, 6) else {
        panic!("formatting failed");
    };
    assert_eq!(len, 6);
    assert_eq!(&buf[..7], b"-00042\0");
}

#[test]
fn test_format_int64_ascii_errors() {
    let mut buf = [0u8; 64];

    // Incompatible flags: HEX + COMMA
    let res = format_int64_ascii(
        &mut buf,
        100,
        print_flags::RADIX_HEX | print_flags::COMMA_TYPE,
        0,
    );
    assert_eq!(res, Err(EfiStatus::INVALID_PARAMETER));

    // Buffer too small
    let mut small_buf = [0u8; 3];
    let res = format_int64_ascii(&mut small_buf, 12345, 0, 0);
    assert_eq!(res, Err(EfiStatus::BUFFER_TOO_SMALL));
}

#[test]
fn test_format_int64_unicode() {
    let mut u_buf = [0u16; 32];

    let Ok(len) = format_int64_unicode(&mut u_buf, -12345, 0, 0) else {
        panic!("unicode formatting failed");
    };
    assert_eq!(len, 6);

    let expected: [u16; 7] = [
        '-' as u16, '1' as u16, '2' as u16, '3' as u16, '4' as u16, '5' as u16, 0,
    ];
    assert_eq!(&u_buf[..7], &expected);

    // Buffer too small
    let mut small_u = [0u16; 3];
    let res = format_int64_unicode(&mut small_u, 12345, 0, 0);
    assert_eq!(res, Err(EfiStatus::BUFFER_TOO_SMALL));
}

// Dummy callbacks for mock protocol instances
unsafe extern "efiapi" fn dummy_unicode_bs_print(
    _start_of_buffer: *mut u16,
    _buffer_size: UIntN,
    _format_string: *const u16,
    _marker: *const core::ffi::c_void,
) -> UIntN {
    0
}

unsafe extern "C" fn dummy_unicode_s_print(
    _start_of_buffer: *mut u16,
    _buffer_size: UIntN,
    _format_string: *const u16,
    _: ...
) -> UIntN {
    0
}

unsafe extern "efiapi" fn dummy_unicode_bs_print_ascii(
    _start_of_buffer: *mut u16,
    _buffer_size: UIntN,
    _format_string: *const u8,
    _marker: *const core::ffi::c_void,
) -> UIntN {
    0
}

unsafe extern "C" fn dummy_unicode_s_print_ascii(
    _start_of_buffer: *mut u16,
    _buffer_size: UIntN,
    _format_string: *const u8,
    _: ...
) -> UIntN {
    0
}

unsafe extern "efiapi" fn dummy_unicode_val_to_str(
    buffer: *mut u16,
    flags: UIntN,
    value: i64,
    width: UIntN,
) -> UIntN {
    let slice = unsafe { core::slice::from_raw_parts_mut(buffer, 38) };
    match format_int64_unicode(slice, value, flags, width) {
        Ok(l) => l as UIntN,
        Err(_) => 0,
    }
}

unsafe extern "efiapi" fn dummy_unicode_val_to_str_s(
    buffer: *mut u16,
    buffer_size: UIntN,
    flags: UIntN,
    value: i64,
    width: UIntN,
) -> EfiStatus {
    let num_chars = buffer_size / 2;
    if num_chars == 0 {
        return EfiStatus::BUFFER_TOO_SMALL;
    }
    let slice = unsafe { core::slice::from_raw_parts_mut(buffer, num_chars) };
    match format_int64_unicode(slice, value, flags, width) {
        Ok(_) => EfiStatus::SUCCESS,
        Err(status) => status,
    }
}

unsafe extern "efiapi" fn dummy_ascii_bs_print(
    _start_of_buffer: *mut u8,
    _buffer_size: UIntN,
    _format_string: *const u8,
    _marker: *const core::ffi::c_void,
) -> UIntN {
    0
}

unsafe extern "C" fn dummy_ascii_s_print(
    _start_of_buffer: *mut u8,
    _buffer_size: UIntN,
    _format_string: *const u8,
    _: ...
) -> UIntN {
    0
}

unsafe extern "efiapi" fn dummy_ascii_bs_print_unicode(
    _start_of_buffer: *mut u8,
    _buffer_size: UIntN,
    _format_string: *const u16,
    _marker: *const core::ffi::c_void,
) -> UIntN {
    0
}

unsafe extern "C" fn dummy_ascii_s_print_unicode(
    _start_of_buffer: *mut u8,
    _buffer_size: UIntN,
    _format_string: *const u16,
    _: ...
) -> UIntN {
    0
}

unsafe extern "efiapi" fn dummy_ascii_val_to_str(
    buffer: *mut u8,
    flags: UIntN,
    value: i64,
    width: UIntN,
) -> UIntN {
    let slice = unsafe { core::slice::from_raw_parts_mut(buffer, 38) };
    match format_int64_ascii(slice, value, flags, width) {
        Ok(l) => l as UIntN,
        Err(_) => 0,
    }
}

unsafe extern "efiapi" fn dummy_ascii_val_to_str_s(
    buffer: *mut u8,
    buffer_size: UIntN,
    flags: UIntN,
    value: i64,
    width: UIntN,
) -> EfiStatus {
    if buffer_size == 0 {
        return EfiStatus::BUFFER_TOO_SMALL;
    }
    let slice = unsafe { core::slice::from_raw_parts_mut(buffer, buffer_size) };
    match format_int64_ascii(slice, value, flags, width) {
        Ok(_) => EfiStatus::SUCCESS,
        Err(status) => status,
    }
}

#[test]
fn test_print2_protocol_wrapper() {
    let mock_proto = EfiPrint2Protocol {
        unicode_bs_print: dummy_unicode_bs_print,
        unicode_s_print: dummy_unicode_s_print,
        unicode_bs_print_ascii_format: dummy_unicode_bs_print_ascii,
        unicode_s_print_ascii_format: dummy_unicode_s_print_ascii,
        unicode_value_to_string: dummy_unicode_val_to_str,
        ascii_bs_print: dummy_ascii_bs_print,
        ascii_s_print: dummy_ascii_s_print,
        ascii_bs_print_unicode_format: dummy_ascii_bs_print_unicode,
        ascii_s_print_unicode_format: dummy_ascii_s_print_unicode,
        ascii_value_to_string: dummy_ascii_val_to_str,
    };

    let wrapper = unsafe { Print2::from_raw(&raw const mock_proto) };
    let mut ascii_buf = [0u8; 32];
    let len = unsafe { wrapper.value_to_ascii(ascii_buf.as_mut_ptr(), 0, 789, 0) };
    assert_eq!(len, 3);
    assert_eq!(&ascii_buf[..4], b"789\0");

    let mut unicode_buf = [0u16; 32];
    let len_u = unsafe { wrapper.value_to_unicode(unicode_buf.as_mut_ptr(), 0, 789, 0) };
    assert_eq!(len_u, 3);
    assert_eq!(unicode_buf[0], '7' as u16);
    assert_eq!(unicode_buf[1], '8' as u16);
    assert_eq!(unicode_buf[2], '9' as u16);
    assert_eq!(unicode_buf[3], 0);
}

#[test]
fn test_print2s_protocol_wrapper() {
    let mock_proto = EfiPrint2SProtocol {
        unicode_bs_print: dummy_unicode_bs_print,
        unicode_s_print: dummy_unicode_s_print,
        unicode_bs_print_ascii_format: dummy_unicode_bs_print_ascii,
        unicode_s_print_ascii_format: dummy_unicode_s_print_ascii,
        unicode_value_to_string_s: dummy_unicode_val_to_str_s,
        ascii_bs_print: dummy_ascii_bs_print,
        ascii_s_print: dummy_ascii_s_print,
        ascii_bs_print_unicode_format: dummy_ascii_bs_print_unicode,
        ascii_s_print_unicode_format: dummy_ascii_s_print_unicode,
        ascii_value_to_string_s: dummy_ascii_val_to_str_s,
    };

    let wrapper = unsafe { Print2S::from_raw(&raw const mock_proto) };

    let mut ascii_buf = [0u8; 32];
    let res = wrapper.value_to_ascii(&mut ascii_buf, 456, 0, 0);
    assert_eq!(res, Ok(()));
    assert_eq!(&ascii_buf[..4], b"456\0");

    let mut unicode_buf = [0u16; 32];
    let res_u = wrapper.value_to_unicode(&mut unicode_buf, 456, 0, 0);
    assert_eq!(res_u, Ok(()));
    assert_eq!(unicode_buf[0], '4' as u16);
    assert_eq!(unicode_buf[1], '5' as u16);
    assert_eq!(unicode_buf[2], '6' as u16);
    assert_eq!(unicode_buf[3], 0);
}
