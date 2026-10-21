// SPDX-License-Identifier: GPL-3.0-only
//! Unit tests for VLAN config and ARP protocols.

use core::mem::size_of;
use rover_uefi::protocols::network::{
    ARP_PROTOCOL_GUID, ARP_SERVICE_BINDING_PROTOCOL_GUID, EfiArpConfigData, EfiArpProtocol,
    EfiVlanConfigProtocol, EfiVlanFindData, VLAN_CONFIG_PROTOCOL_GUID,
};

#[test]
fn network_extra_guids() {
    let vlan = VLAN_CONFIG_PROTOCOL_GUID;
    assert_eq!(vlan.data1, 0x9e23_d768);
    assert_eq!(vlan.data2, 0xd2f3);
    assert_eq!(vlan.data3, 0x4366);
    assert_eq!(vlan.data4, [0x9f, 0xc3, 0x3a, 0x7a, 0xba, 0x86, 0x43, 0x74]);

    let arp_sb = ARP_SERVICE_BINDING_PROTOCOL_GUID;
    assert_eq!(arp_sb.data1, 0xf44c_00ee);
    assert_eq!(arp_sb.data2, 0x1f2c);
    assert_eq!(arp_sb.data3, 0x4a00);
    assert_eq!(
        arp_sb.data4,
        [0xaa, 0x09, 0x1c, 0x9f, 0x3e, 0x08, 0x00, 0xa3]
    );

    let arp = ARP_PROTOCOL_GUID;
    assert_eq!(arp.data1, 0xf4b4_27bb);
    assert_eq!(arp.data2, 0xba21);
    assert_eq!(arp.data3, 0x4f16);
    assert_eq!(arp.data4, [0xbc, 0x4e, 0x43, 0xe4, 0x16, 0xab, 0x61, 0x9c]);
}

#[test]
fn vlan_and_arp_structures_layout() {
    assert_eq!(size_of::<EfiVlanFindData>(), 4);

    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(size_of::<EfiVlanConfigProtocol>(), 24);
        assert_eq!(size_of::<EfiArpConfigData>(), 32);
        assert_eq!(size_of::<EfiArpProtocol>(), 56);
    }

    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(size_of::<EfiVlanConfigProtocol>(), 12);
        assert_eq!(size_of::<EfiArpConfigData>(), 24);
        assert_eq!(size_of::<EfiArpProtocol>(), 28);
    }
}
