// SPDX-License-Identifier: Apache-2.0
//! Unit tests for DHCP, MTFTP, and IP Config protocols.

use rover_uefi::protocols::network::{
    EFI_DHCP4_PROTOCOL_GUID, EFI_DHCP6_PROTOCOL_GUID, EFI_IP4_CONFIG2_PROTOCOL_GUID,
    EFI_IP6_CONFIG_PROTOCOL_GUID, EFI_MTFTP4_PROTOCOL_GUID, EFI_MTFTP6_PROTOCOL_GUID,
    EfiDhcp4State, EfiDhcp6State, EfiIp4Config2DataType, EfiIp4Config2Policy, EfiIp6ConfigPolicy,
};

#[test]
fn network_protocols_guids_and_states() {
    assert!(!EFI_DHCP4_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DHCP6_PROTOCOL_GUID.is_zero());
    assert!(!EFI_MTFTP4_PROTOCOL_GUID.is_zero());
    assert!(!EFI_MTFTP6_PROTOCOL_GUID.is_zero());
    assert!(!EFI_IP4_CONFIG2_PROTOCOL_GUID.is_zero());
    assert!(!EFI_IP6_CONFIG_PROTOCOL_GUID.is_zero());

    assert_eq!(EfiDhcp4State::Dhcp4Stopped as u32, 0);
    assert_eq!(EfiDhcp4State::Dhcp4Bound as u32, 4);

    assert_eq!(EfiDhcp6State::Dhcp6Stopped as u32, 0);
    assert_eq!(EfiDhcp6State::Dhcp6Bound as u32, 5);

    assert_eq!(EfiIp4Config2Policy::Static as u32, 0);
    assert_eq!(EfiIp4Config2Policy::Dhcp as u32, 1);

    assert_eq!(EfiIp4Config2DataType::InterfaceInfo as u32, 0);
    assert_eq!(EfiIp4Config2DataType::ManualAddress as u32, 2);

    assert_eq!(EfiIp6ConfigPolicy::Manual as u32, 0);
    assert_eq!(EfiIp6ConfigPolicy::Automatic as u32, 1);
}
