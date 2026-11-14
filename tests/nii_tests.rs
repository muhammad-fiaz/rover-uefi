//! Integration tests for Network Interface Identifier (NII / UNDI) protocol.

use rover_uefi::protocols::network::nii::{
    EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_GUID,
    EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_GUID_31,
    EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_REVISION, EfiNetworkInterfaceIdentifierProtocol,
    EfiNetworkInterfaceType, NetworkInterfaceIdentifier,
};

#[test]
fn test_nii_guids_and_revision() {
    assert_ne!(
        EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_GUID,
        EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_GUID_31
    );
    assert_eq!(
        EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_REVISION,
        0x0002_0000
    );
    assert_eq!(EfiNetworkInterfaceType::Undi as u8, 1);
}

#[test]
fn test_nii_protocol_wrapper() {
    let raw_proto = EfiNetworkInterfaceIdentifierProtocol {
        revision: EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_REVISION,
        id: 0x1234_5678,
        image_addr: 0x8000_0000,
        image_size: 65536,
        string_id: *b"UNDI",
        interface_type: EfiNetworkInterfaceType::Undi as u8,
        major_ver: 3,
        minor_ver: 1,
        ipv6_supported: 1,
        if_num: 0,
    };

    let nii = NetworkInterfaceIdentifier::new(&raw_proto);
    assert_eq!(
        nii.revision(),
        EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_REVISION
    );
    assert!(nii.is_started());
    assert!(nii.is_ipv6_supported());
    assert_eq!(nii.string_id(), b"UNDI");
    assert_eq!(nii.version(), (3, 1));
    assert_eq!(nii.interface_num(), 0);
}
