//! Unit and integration tests for the UEFI Regular Expression Protocol and native regex engine.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use rover_uefi::abi::types::{Bool, EFI_TRUE, UIntN};
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::core::regex::{
    EFI_REGEX_SYNTAX_TYPE_ECMA_262_GUID, EFI_REGEX_SYNTAX_TYPE_PERL_GUID,
    EFI_REGEX_SYNTAX_TYPE_POSIX_EXTENDED_GUID, EFI_REGULAR_EXPRESSION_PROTOCOL_GUID,
    EfiRegexCapture, EfiRegularExpressionProtocol, RegularExpression, regex_match, regex_match_str,
};

#[test]
fn test_regex_protocol_and_syntax_guids() {
    assert_eq!(
        EFI_REGULAR_EXPRESSION_PROTOCOL_GUID,
        EfiGuid::new(
            0xB3F7_9D9A,
            0x436C,
            0xDC11,
            [0xB0, 0x52, 0xCD, 0x85, 0xDF, 0x52, 0x4C, 0xE6],
        )
    );
    assert_eq!(
        EFI_REGEX_SYNTAX_TYPE_POSIX_EXTENDED_GUID,
        EfiGuid::new(
            0x5F05_B20F,
            0x4A56,
            0xC231,
            [0xFA, 0x0B, 0xA7, 0xB1, 0xF1, 0x10, 0x04, 0x1D],
        )
    );
    assert_eq!(
        EFI_REGEX_SYNTAX_TYPE_PERL_GUID,
        EfiGuid::new(
            0x63E6_0A51,
            0x497D,
            0xD427,
            [0xC4, 0xA5, 0xB8, 0xAB, 0xDC, 0x3A, 0xAE, 0xB6],
        )
    );
    assert_eq!(
        EFI_REGEX_SYNTAX_TYPE_ECMA_262_GUID,
        EfiGuid::new(
            0x9A47_3A4A,
            0x4CEB,
            0xB95A,
            [0x41, 0x5E, 0x5B, 0xA0, 0xBC, 0x63, 0x9B, 0x2E],
        )
    );
}

#[test]
fn test_native_regex_literals_and_wildcards() {
    assert!(regex_match_str("rover", "rover"));
    assert!(!regex_match_str("rover", "rover2"));
    assert!(regex_match_str("rover", "r.v.r"));
    assert!(!regex_match_str("rover", "r.x"));
}

#[test]
fn test_native_regex_anchors() {
    assert!(regex_match_str("rover uefi", "^rover"));
    assert!(!regex_match_str("boot rover uefi", "^rover"));
    assert!(regex_match_str("rover uefi", "uefi$"));
    assert!(!regex_match_str("rover uefi boot", "uefi$"));
    assert!(regex_match_str("firmware", "^firmware$"));
    assert!(!regex_match_str("firmware2", "^firmware$"));
}

#[test]
fn test_native_regex_quantifiers() {
    // Star (*)
    assert!(regex_match_str("caaat", "ca*t"));
    assert!(regex_match_str("ct", "ca*t"));
    assert!(!regex_match_str("cbt", "^ca*t$"));

    // Plus (+)
    assert!(regex_match_str("caaat", "^ca+t$"));
    assert!(!regex_match_str("ct", "^ca+t$"));

    // Question mark (?)
    assert!(regex_match_str("colour", "^colou?r$"));
    assert!(regex_match_str("color", "^colou?r$"));
    assert!(!regex_match_str("colouur", "^colou?r$"));
}

#[test]
fn test_native_regex_character_classes() {
    assert!(regex_match_str("a", "[a-z]"));
    assert!(regex_match_str("5", "[0-9]"));
    assert!(!regex_match_str("A", "^[a-z]$"));
    assert!(regex_match_str("B", "[A-Z]"));

    // Inverted range
    assert!(regex_match_str("a", "[^0-9]"));
    assert!(!regex_match_str("5", "^[^0-9]$"));
}

#[test]
fn test_native_regex_escaped_classes() {
    assert!(regex_match_str("123", r"^\d+$"));
    assert!(!regex_match_str("12a", r"^\d+$"));
    assert!(regex_match_str("hello_world1", r"^\w+$"));
    assert!(!regex_match_str("hello world", r"^\w+$"));
    assert!(regex_match_str("   \t", r"^\s+$"));
}

#[test]
fn test_native_regex_utf16_matching() {
    let input: [u16; 5] = [
        u16::from(b'R'),
        u16::from(b'O'),
        u16::from(b'V'),
        u16::from(b'E'),
        u16::from(b'R'),
    ];
    let pattern: [u16; 7] = [
        u16::from(b'^'),
        u16::from(b'R'),
        u16::from(b'O'),
        u16::from(b'.'),
        u16::from(b'E'),
        u16::from(b'R'),
        u16::from(b'$'),
    ];
    assert!(regex_match(&input, &pattern));
}

// --- Mock Protocol Implementation ---
unsafe extern "efiapi" fn mock_match_string(
    _this: *mut EfiRegularExpressionProtocol,
    string: *const u16,
    pattern: *const u16,
    _syntax_type: *const EfiGuid,
    result: *mut Bool,
    _captures: *mut *mut EfiRegexCapture,
    _captures_count: *mut UIntN,
) -> EfiStatus {
    unsafe {
        if string.is_null() || pattern.is_null() {
            return EfiStatus::INVALID_PARAMETER;
        }
        *result = EFI_TRUE;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_info(
    _this: *mut EfiRegularExpressionProtocol,
    regex_syntax_type_list_size: *mut UIntN,
    regex_syntax_type_list: *mut EfiGuid,
) -> EfiStatus {
    unsafe {
        let count = 3;
        let required_size = count * core::mem::size_of::<EfiGuid>();
        if *regex_syntax_type_list_size < required_size {
            *regex_syntax_type_list_size = required_size;
            return EfiStatus::BUFFER_TOO_SMALL;
        }
        if !regex_syntax_type_list.is_null() {
            *regex_syntax_type_list.add(0) = EFI_REGEX_SYNTAX_TYPE_POSIX_EXTENDED_GUID;
            *regex_syntax_type_list.add(1) = EFI_REGEX_SYNTAX_TYPE_PERL_GUID;
            *regex_syntax_type_list.add(2) = EFI_REGEX_SYNTAX_TYPE_ECMA_262_GUID;
        }
        *regex_syntax_type_list_size = required_size;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_regular_expression_client_wrapper() {
    let raw_protocol = EfiRegularExpressionProtocol {
        match_string: mock_match_string,
        get_info: mock_get_info,
    };

    let client = RegularExpression::new(&raw_protocol);

    let mut guid_buf = [EfiGuid::ZERO; 4];
    let num_types = client.get_supported_syntax_types(&mut guid_buf).unwrap();
    assert_eq!(num_types, 3);
    assert_eq!(guid_buf[0], EFI_REGEX_SYNTAX_TYPE_POSIX_EXTENDED_GUID);
    assert_eq!(guid_buf[1], EFI_REGEX_SYNTAX_TYPE_PERL_GUID);
    assert_eq!(guid_buf[2], EFI_REGEX_SYNTAX_TYPE_ECMA_262_GUID);

    let sample_str: [u16; 4] = [u16::from(b'u'), u16::from(b'e'), u16::from(b'f'), 0];
    let sample_pat: [u16; 4] = [u16::from(b'u'), u16::from(b'e'), u16::from(b'f'), 0];

    let is_matched = unsafe {
        client.matches(
            sample_str.as_ptr(),
            sample_pat.as_ptr(),
            Some(&EFI_REGEX_SYNTAX_TYPE_POSIX_EXTENDED_GUID),
        )
    }
    .unwrap();
    assert!(is_matched);
}
