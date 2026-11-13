//! Integration tests for TPM 2.0 command frames, builder, and parser.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use rover_uefi::protocols::security::tcg2::command::*;

#[test]
fn test_tpm_command_constants() {
    assert_eq!(TPM_ST_NO_SESSIONS, 0x8001);
    assert_eq!(TPM_ST_SESSIONS, 0x8002);

    assert_eq!(TPM_SU_CLEAR, 0x0000);
    assert_eq!(TPM_SU_STATE, 0x0001);

    assert_eq!(TPM_CC_STARTUP, 0x0000_0144);
    assert_eq!(TPM_CC_SELF_TEST, 0x0000_0143);
    assert_eq!(TPM_CC_GET_RANDOM, 0x0000_017B);

    assert_eq!(TPM_RC_SUCCESS, 0x000);
    assert_eq!(TPM_RC_BAD_TAG, 0x01E);
    assert_eq!(TPM_RC_INITIALIZE, 0x100);
}

#[test]
fn test_build_startup_command() {
    let mut buf = [0_u8; 16];
    let len = build_startup_command(TPM_SU_CLEAR, &mut buf).expect("startup cmd should build");
    assert_eq!(len, 12);

    assert_eq!(&buf[0..2], &TPM_ST_NO_SESSIONS.to_be_bytes());
    assert_eq!(&buf[2..6], &12_u32.to_be_bytes());
    assert_eq!(&buf[6..10], &TPM_CC_STARTUP.to_be_bytes());
    assert_eq!(&buf[10..12], &TPM_SU_CLEAR.to_be_bytes());
}

#[test]
fn test_build_self_test_command() {
    let mut buf = [0_u8; 16];
    let len = build_self_test_command(true, &mut buf).expect("self test cmd should build");
    assert_eq!(len, 11);

    assert_eq!(&buf[0..2], &TPM_ST_NO_SESSIONS.to_be_bytes());
    assert_eq!(&buf[2..6], &11_u32.to_be_bytes());
    assert_eq!(&buf[6..10], &TPM_CC_SELF_TEST.to_be_bytes());
    assert_eq!(buf[10], 1);
}

#[test]
fn test_build_get_random_command() {
    let mut buf = [0_u8; 16];
    let len = build_get_random_command(32, &mut buf).expect("get random cmd should build");
    assert_eq!(len, 12);

    assert_eq!(&buf[0..2], &TPM_ST_NO_SESSIONS.to_be_bytes());
    assert_eq!(&buf[2..6], &12_u32.to_be_bytes());
    assert_eq!(&buf[6..10], &TPM_CC_GET_RANDOM.to_be_bytes());
    assert_eq!(&buf[10..12], &32_u16.to_be_bytes());
}

#[test]
fn test_parse_response_header() {
    let mut success_resp = [0_u8; 10];
    success_resp[0..2].copy_from_slice(&TPM_ST_NO_SESSIONS.to_be_bytes());
    success_resp[2..6].copy_from_slice(&10_u32.to_be_bytes());
    success_resp[6..10].copy_from_slice(&TPM_RC_SUCCESS.to_be_bytes());

    let hdr = parse_response_header(&success_resp).expect("success resp should parse");
    let hdr_tag = hdr.tag;
    let hdr_size = hdr.param_size;
    let hdr_rc = hdr.response_code;
    assert_eq!(hdr_tag, TPM_ST_NO_SESSIONS);
    assert_eq!(hdr_size, 10);
    assert_eq!(hdr_rc, TPM_RC_SUCCESS);

    let mut err_resp = [0_u8; 10];
    err_resp[0..2].copy_from_slice(&TPM_ST_NO_SESSIONS.to_be_bytes());
    err_resp[2..6].copy_from_slice(&10_u32.to_be_bytes());
    err_resp[6..10].copy_from_slice(&TPM_RC_INITIALIZE.to_be_bytes());

    let err = parse_response_header(&err_resp);
    assert_eq!(
        err.unwrap_err(),
        TpmCommandError::TpmError(TPM_RC_INITIALIZE)
    );
}
