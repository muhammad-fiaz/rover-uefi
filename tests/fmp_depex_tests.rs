//! Integration tests for Firmware Management Protocol dependency expression evaluator.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::firmware::fmp_depex::*;

const TEST_IMAGE_TYPE: EfiGuid = EfiGuid::new(
    0x1234_5678,
    0xABCD,
    0xEF01,
    [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
);

#[test]
fn test_fmp_depex_opcodes_and_constants() {
    assert_eq!(EFI_FMP_DEP_PUSH_GUID, 0x00);
    assert_eq!(EFI_FMP_DEP_PUSH_VERSION, 0x01);
    assert_eq!(EFI_FMP_DEP_VERSION_STR, 0x02);
    assert_eq!(EFI_FMP_DEP_AND, 0x03);
    assert_eq!(EFI_FMP_DEP_OR, 0x04);
    assert_eq!(EFI_FMP_DEP_NOT, 0x05);
    assert_eq!(EFI_FMP_DEP_TRUE, 0x06);
    assert_eq!(EFI_FMP_DEP_FALSE, 0x07);
    assert_eq!(EFI_FMP_DEP_EQ, 0x08);
    assert_eq!(EFI_FMP_DEP_GT, 0x09);
    assert_eq!(EFI_FMP_DEP_GTE, 0x0A);
    assert_eq!(EFI_FMP_DEP_LT, 0x0B);
    assert_eq!(EFI_FMP_DEP_LTE, 0x0C);
    assert_eq!(EFI_FMP_DEP_END, 0x0D);
    assert_eq!(EFI_FMP_DEP_DECLARE_LENGTH, 0x0E);

    assert_eq!(LAST_ATTEMPT_STATUS_FMP_RESERVED_MIN, 0x1000);
    assert_eq!(LAST_ATTEMPT_STATUS_FMP_RESERVED_MAX, 0x1FFF);
}

#[test]
fn test_fmp_depex_validation() {
    // True END
    let valid_depex = [EFI_FMP_DEP_TRUE, EFI_FMP_DEP_END];
    let len = validate_fmp_depex(&valid_depex).expect("simple depex should validate");
    assert_eq!(len, 2);

    // Declared length + True + False + OR + END
    let mut declared_depex = vec![EFI_FMP_DEP_DECLARE_LENGTH];
    declared_depex.extend_from_slice(&8_u32.to_le_bytes()); // 5 bytes prefix + 3 bytes body = 8
    declared_depex.push(EFI_FMP_DEP_TRUE);
    declared_depex.push(EFI_FMP_DEP_FALSE);
    declared_depex.push(EFI_FMP_DEP_OR);
    declared_depex.push(EFI_FMP_DEP_END);

    let len2 = validate_fmp_depex(&declared_depex).expect("declared length depex should validate");
    assert_eq!(len2, declared_depex.len());

    // Missing END opcode
    let invalid_depex = [EFI_FMP_DEP_TRUE];
    assert_eq!(
        validate_fmp_depex(&invalid_depex).unwrap_err(),
        FmpDepexError::MissingEndOpcode
    );
}

#[test]
fn test_fmp_depex_evaluation_boolean_logic() {
    // TRUE AND FALSE => false
    let expr1 = [
        EFI_FMP_DEP_TRUE,
        EFI_FMP_DEP_FALSE,
        EFI_FMP_DEP_AND,
        EFI_FMP_DEP_END,
    ];
    let res1 = evaluate_fmp_depex(&expr1, |_| None).expect("should evaluate");
    assert!(!res1);

    // TRUE OR FALSE => true
    let expr2 = [
        EFI_FMP_DEP_TRUE,
        EFI_FMP_DEP_FALSE,
        EFI_FMP_DEP_OR,
        EFI_FMP_DEP_END,
    ];
    let res2 = evaluate_fmp_depex(&expr2, |_| None).expect("should evaluate");
    assert!(res2);

    // FALSE NOT => true
    let expr3 = [EFI_FMP_DEP_FALSE, EFI_FMP_DEP_NOT, EFI_FMP_DEP_END];
    let res3 = evaluate_fmp_depex(&expr3, |_| None).expect("should evaluate");
    assert!(res3);
}

#[test]
fn test_fmp_depex_evaluation_version_checks() {
    // Version 200 >= 100 => true
    let mut expr = Vec::new();
    expr.push(EFI_FMP_DEP_PUSH_VERSION);
    expr.extend_from_slice(&200_u32.to_le_bytes());
    expr.push(EFI_FMP_DEP_PUSH_VERSION);
    expr.extend_from_slice(&100_u32.to_le_bytes());
    expr.push(EFI_FMP_DEP_GTE);
    expr.push(EFI_FMP_DEP_END);

    let res = evaluate_fmp_depex(&expr, |_| None).expect("version comparison should evaluate");
    assert!(res);

    // Resolve ImageTypeId -> version 150; Compare: 150 < 300 => true
    let mut expr2 = Vec::new();
    expr2.push(EFI_FMP_DEP_PUSH_GUID);
    expr2.extend_from_slice(&TEST_IMAGE_TYPE.to_bytes());
    expr2.push(EFI_FMP_DEP_PUSH_VERSION);
    expr2.extend_from_slice(&300_u32.to_le_bytes());
    expr2.push(EFI_FMP_DEP_LT);
    expr2.push(EFI_FMP_DEP_END);

    let resolver = |guid: &EfiGuid| {
        if *guid == TEST_IMAGE_TYPE {
            Some(150)
        } else {
            None
        }
    };

    let res2 =
        evaluate_fmp_depex(&expr2, resolver).expect("image version comparison should evaluate");
    assert!(res2);

    // Unknown image should return ImageNotFound
    let bad_guid = EfiGuid::new(0, 0, 0, [0; 8]);
    let mut expr3 = Vec::new();
    expr3.push(EFI_FMP_DEP_PUSH_GUID);
    expr3.extend_from_slice(&bad_guid.to_bytes());
    expr3.push(EFI_FMP_DEP_END);
    assert_eq!(
        evaluate_fmp_depex(&expr3, resolver).unwrap_err(),
        FmpDepexError::ImageNotFound
    );
}
