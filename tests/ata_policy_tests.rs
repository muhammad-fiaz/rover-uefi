//! Integration tests for ATA ATAPI Policy protocol.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::storage::ata_passthru::{
    ATA_ATAPI_POLICY_PROTOCOL_GUID, ATA_ATAPI_POLICY_VERSION, AtaAtapiPolicy,
    EfiAtaAtapiPolicyProtocol, devsleep as ata_devsleep, puis as ata_puis,
};

#[test]
fn test_ata_policy_guid_and_constants() {
    assert_eq!(
        ATA_ATAPI_POLICY_PROTOCOL_GUID,
        EfiGuid::new(
            0xE59C_D769,
            0x5083,
            0x4F26,
            [0x90, 0x94, 0x6C, 0x91, 0x9F, 0x91, 0x6C, 0x4E]
        )
    );
    assert_eq!(ATA_ATAPI_POLICY_VERSION, 0x0001_0000);
    assert_eq!(ata_puis::DISABLE, 0);
    assert_eq!(ata_puis::ENABLE, 1);
    assert_eq!(ata_puis::KEEP_HARDWARE, 2);
    assert_eq!(ata_devsleep::DISABLE, 0);
    assert_eq!(ata_devsleep::ENABLE, 1);
}

#[test]
fn test_ata_policy_client() {
    let proto = EfiAtaAtapiPolicyProtocol {
        version: ATA_ATAPI_POLICY_VERSION,
        puis_enable: ata_puis::ENABLE,
        device_sleep_enable: ata_devsleep::ENABLE,
        aggressive_device_sleep_enable: ata_devsleep::DISABLE,
        reserved: 0,
    };

    let client = AtaAtapiPolicy::new(&proto);
    assert_eq!(client.version(), ATA_ATAPI_POLICY_VERSION);
    assert!(client.is_puis_enabled());
    assert!(client.is_device_sleep_enabled());
    assert!(!client.is_aggressive_device_sleep_enabled());
}
