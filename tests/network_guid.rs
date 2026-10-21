//! Network GUID tests.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::network::guid::{
    DNS4_GUID, DNS6_GUID, HTTP_GUID, IP4_GUID, IP6_GUID, MANAGED_NETWORK_GUID, PXE_BASE_CODE_GUID,
    SIMPLE_NETWORK_GUID, TCP4_GUID, TCP6_GUID, UDP4_GUID, UDP6_GUID,
};

#[test]
fn guid_bytes() {
    assert_eq!(
        SIMPLE_NETWORK_GUID.to_bytes(),
        [
            0xB9, 0x32, 0x98, 0xA1, 0x25, 0xAC, 0xD3, 0x11, 0x9A, 0x2D, 0x00, 0x90, 0x27, 0x3F,
            0xC1, 0x4D
        ]
    );
    assert_eq!(
        PXE_BASE_CODE_GUID.to_bytes(),
        [
            0x03, 0xE6, 0xC4, 0x03, 0x28, 0xAC, 0xD3, 0x11, 0x9A, 0x2D, 0x00, 0x90, 0x27, 0x3F,
            0xC1, 0x4D
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(PXE_BASE_CODE_GUID.to_bytes()),
        PXE_BASE_CODE_GUID
    );
    assert_eq!(EfiGuid::from_bytes(HTTP_GUID.to_bytes()), HTTP_GUID);
    assert_eq!(EfiGuid::from_bytes(IP4_GUID.to_bytes()), IP4_GUID);
    assert_eq!(EfiGuid::from_bytes(IP6_GUID.to_bytes()), IP6_GUID);
    assert_eq!(EfiGuid::from_bytes(UDP4_GUID.to_bytes()), UDP4_GUID);
    assert_eq!(EfiGuid::from_bytes(UDP6_GUID.to_bytes()), UDP6_GUID);
    assert_eq!(EfiGuid::from_bytes(TCP4_GUID.to_bytes()), TCP4_GUID);
    assert_eq!(EfiGuid::from_bytes(TCP6_GUID.to_bytes()), TCP6_GUID);
    assert_eq!(EfiGuid::from_bytes(DNS4_GUID.to_bytes()), DNS4_GUID);
    assert_eq!(EfiGuid::from_bytes(DNS6_GUID.to_bytes()), DNS6_GUID);
    assert_eq!(
        EfiGuid::from_bytes(MANAGED_NETWORK_GUID.to_bytes()),
        MANAGED_NETWORK_GUID
    );
}

#[test]
fn guid_family_is_unique() {
    let all = [
        SIMPLE_NETWORK_GUID,
        MANAGED_NETWORK_GUID,
        IP4_GUID,
        IP6_GUID,
        UDP4_GUID,
        UDP6_GUID,
        TCP4_GUID,
        TCP6_GUID,
        DNS4_GUID,
        DNS6_GUID,
        HTTP_GUID,
        PXE_BASE_CODE_GUID,
    ];
    for (index, guid) in all.iter().enumerate() {
        assert!(!guid.is_zero());
        for (other_index, other) in all.iter().enumerate() {
            if index < other_index {
                assert_ne!(guid, other);
            }
        }
    }
}
