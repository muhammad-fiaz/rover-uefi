//! Integration tests for ATA/ATAPI Policy Protocol.
#![allow(clippy::expect_used)]

use rover_uefi::protocols::driver::ata_policy::{
    ATA_ATAPI_POLICY_PROTOCOL_GUID, ATA_ATAPI_POLICY_VERSION, AtaAtapiPolicy,
    AtaAtapiPolicyProtocol,
};

#[test]
fn test_ata_policy_constants() {
    assert_eq!(
        ATA_ATAPI_POLICY_PROTOCOL_GUID.to_string(),
        "e59cd769-5083-4f26-9094-6c919f916c4e"
    );
    assert_eq!(ATA_ATAPI_POLICY_VERSION, 0x0001_0000);
}

#[test]
fn test_ata_policy_wrapper() {
    let mut raw_proto = AtaAtapiPolicyProtocol::default();
    let mut policy =
        unsafe { AtaAtapiPolicy::from_raw(&raw mut raw_proto) }.expect("valid ata policy");

    assert_eq!(policy.version(), ATA_ATAPI_POLICY_VERSION);
    assert!(!policy.is_puis_enabled());
    assert!(!policy.is_device_sleep_enabled());
    assert!(!policy.is_aggressive_device_sleep_enabled());

    policy.set_puis(true);
    assert!(policy.is_puis_enabled());

    policy.set_device_sleep(true);
    assert!(policy.is_device_sleep_enabled());

    policy.set_aggressive_device_sleep(true);
    assert!(policy.is_aggressive_device_sleep_enabled());

    policy.set_puis(false);
    assert!(!policy.is_puis_enabled());
}
