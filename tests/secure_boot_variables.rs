//! Secure Boot variable tests.

use rover_uefi::protocols::security::secure_boot::variables::{
    VARIABLE_AUDIT_MODE, VARIABLE_DB, VARIABLE_DB_DEFAULT, VARIABLE_DBR, VARIABLE_DBT,
    VARIABLE_DBX, VARIABLE_DEPLOYED_MODE, VARIABLE_KEK, VARIABLE_KEK_DEFAULT, VARIABLE_PK,
    VARIABLE_PK_DEFAULT, VARIABLE_SECURE_BOOT, VARIABLE_SETUP_MODE, VARIABLE_SIGNATURE_SUPPORT,
    VARIABLE_VENDOR_KEYS, is_null_terminated, variable_name_byte_len,
};
use rover_uefi::protocols::security::types::{GLOBAL_VARIABLE_GUID, IMAGE_SECURITY_DATABASE_GUID};

#[test]
fn mode_variable_names() {
    assert_eq!(VARIABLE_SECURE_BOOT.len(), 11);
    assert_eq!(VARIABLE_SECURE_BOOT[10], 0);
    assert_eq!(VARIABLE_SETUP_MODE.len(), 10);
    assert_eq!(VARIABLE_AUDIT_MODE.len(), 10);
    assert_eq!(VARIABLE_DEPLOYED_MODE.len(), 13);
    assert_eq!(variable_name_byte_len(VARIABLE_SECURE_BOOT), Some(22));
    assert!(is_null_terminated(VARIABLE_SECURE_BOOT));
    assert!(!is_null_terminated(&VARIABLE_SECURE_BOOT[..10]));
}

#[test]
fn database_variable_names() {
    assert_eq!(VARIABLE_PK, &[0x50, 0x4B, 0x00]);
    assert_eq!(VARIABLE_KEK, &[0x4B, 0x45, 0x4B, 0x00]);
    assert_eq!(VARIABLE_DB, &[0x64, 0x62, 0x00]);
    assert_eq!(VARIABLE_DBX, &[0x64, 0x62, 0x78, 0x00]);
    assert_eq!(VARIABLE_DBT, &[0x64, 0x62, 0x74, 0x00]);
    assert_eq!(VARIABLE_DBR, &[0x64, 0x62, 0x72, 0x00]);
    assert_eq!(VARIABLE_VENDOR_KEYS.len(), 11);
    assert_eq!(VARIABLE_SIGNATURE_SUPPORT.len(), 17);
}

#[test]
fn default_variable_names() {
    assert_eq!(VARIABLE_PK_DEFAULT.len(), 10);
    assert_eq!(VARIABLE_KEK_DEFAULT.len(), 11);
    assert_eq!(VARIABLE_DB_DEFAULT.len(), 10);
    assert!(is_null_terminated(VARIABLE_DB_DEFAULT));
}

#[test]
fn variable_guids_are_correct() {
    assert_eq!(
        GLOBAL_VARIABLE_GUID.to_bytes(),
        [
            0x61, 0xDF, 0xE4, 0x8B, 0xCA, 0x93, 0xD2, 0x11, 0xAA, 0x0D, 0x00, 0xE0, 0x98, 0x03,
            0x2B, 0x8C
        ]
    );
    assert_eq!(
        IMAGE_SECURITY_DATABASE_GUID.to_bytes(),
        [
            0xCB, 0xB2, 0x19, 0xD7, 0x3A, 0x3D, 0x96, 0x45, 0xA3, 0xBC, 0xDA, 0xD0, 0x0E, 0x67,
            0x65, 0x6F
        ]
    );
}

#[test]
fn names_convert_to_unicode() {
    let name: Vec<u16> = VARIABLE_PK.to_vec();
    assert_eq!(name, vec![0x50, 0x4B, 0x00]);
    assert_eq!(variable_name_byte_len(&name), Some(6));
    assert_eq!(variable_name_byte_len(&[]), Some(0));
}
