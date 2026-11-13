// SPDX-License-Identifier: Apache-2.0
//! Unit tests for VLAN config and ARP protocols.

use core::mem::size_of;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::network::{
    ARP_PROTOCOL_GUID, ARP_SERVICE_BINDING_PROTOCOL_GUID, EFI_VLAN_CONFIG_PROTOCOL_GUID,
    EfiArpConfigData, EfiArpProtocol, EfiVlanConfigProtocol, EfiVlanFindData,
    VLAN_CONFIG_PROTOCOL_GUID, VlanConfig,
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

static mut DUMMY_FIND_DATA: EfiVlanFindData = EfiVlanFindData {
    vlan_id: 100,
    priority: 3,
};

unsafe extern "efiapi" fn mock_vlan_set(
    _this: *mut EfiVlanConfigProtocol,
    vlan_id: u16,
    priority: u8,
) -> EfiStatus {
    if vlan_id > 4094 || priority > 7 {
        EfiStatus::INVALID_PARAMETER
    } else {
        EfiStatus::SUCCESS
    }
}

unsafe extern "efiapi" fn mock_vlan_find(
    _this: *mut EfiVlanConfigProtocol,
    vlan_id: *const u16,
    number_of_vlan: *mut u16,
    entries: *mut *mut EfiVlanFindData,
) -> EfiStatus {
    if number_of_vlan.is_null() || entries.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if !vlan_id.is_null() && unsafe { *vlan_id } != 100 {
        return EfiStatus::NOT_FOUND;
    }
    unsafe {
        *number_of_vlan = 1;
        *entries = core::ptr::addr_of_mut!(DUMMY_FIND_DATA);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_vlan_remove(
    _this: *mut EfiVlanConfigProtocol,
    vlan_id: u16,
) -> EfiStatus {
    if vlan_id == 100 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_vlan_config_client() {
    assert_eq!(EFI_VLAN_CONFIG_PROTOCOL_GUID, VLAN_CONFIG_PROTOCOL_GUID);

    let mut mock_proto = EfiVlanConfigProtocol {
        set: mock_vlan_set,
        find: mock_vlan_find,
        remove: mock_vlan_remove,
    };

    let mut vlan = VlanConfig::new(&mut mock_proto);

    assert!(vlan.set(100, 3).is_ok());
    assert!(vlan.set(5000, 0).is_err());

    let (entries, count) = vlan.find(Some(100)).unwrap_or((core::ptr::null_mut(), 0));
    assert_eq!(count, 1);
    assert!(!entries.is_null());
    assert_eq!(unsafe { (*entries).vlan_id }, 100);
    assert_eq!(unsafe { (*entries).priority }, 3);

    assert_eq!(vlan.find(Some(200)).err(), Some(EfiStatus::NOT_FOUND));

    assert!(vlan.remove(100).is_ok());
    assert_eq!(vlan.remove(200).err(), Some(EfiStatus::NOT_FOUND));
}
