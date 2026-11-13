//! Integration tests for SMM Memory Attribute Protocol.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation
)]

use rover_uefi::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};
use rover_uefi::protocols::firmware::memory_attribute::{
    SMM_MEMORY_ATTRIBUTE_PROTOCOL_GUID, SmmMemoryAttribute, SmmMemoryAttributeProtocol,
    memory_attribute as memory_attr_flags,
};

#[test]
fn test_smm_memory_attribute_guid_and_constants() {
    assert_eq!(
        SMM_MEMORY_ATTRIBUTE_PROTOCOL_GUID,
        EfiGuid::new(
            0x69B7_92EA,
            0x39CE,
            0x402D,
            [0xA2, 0xA6, 0xF7, 0x21, 0xDE, 0x35, 0x1D, 0xFE]
        )
    );
    assert_eq!(memory_attr_flags::RO, 0x0002_0000);
    assert_eq!(memory_attr_flags::XP, 0x0000_4000);
    assert_eq!(memory_attr_flags::RP, 0x0000_2000);
}

static mut MOCK_ATTRS: u64 = memory_attr_flags::WB;

unsafe extern "efiapi" fn mock_get_attrs(
    _this: *const SmmMemoryAttributeProtocol,
    _base: EfiPhysicalAddress,
    _length: u64,
    attrs: *mut u64,
) -> EfiStatus {
    unsafe {
        *attrs = MOCK_ATTRS;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_attrs(
    _this: *const SmmMemoryAttributeProtocol,
    _base: EfiPhysicalAddress,
    _length: u64,
    attrs: u64,
) -> EfiStatus {
    unsafe {
        MOCK_ATTRS |= attrs;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clear_attrs(
    _this: *const SmmMemoryAttributeProtocol,
    _base: EfiPhysicalAddress,
    _length: u64,
    attrs: u64,
) -> EfiStatus {
    unsafe {
        MOCK_ATTRS &= !attrs;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_smm_memory_attribute_client() {
    let proto = SmmMemoryAttributeProtocol {
        get_memory_attributes: Some(mock_get_attrs),
        set_memory_attributes: Some(mock_set_attrs),
        clear_memory_attributes: Some(mock_clear_attrs),
    };

    let client = SmmMemoryAttribute::new(&proto);
    let base = EfiPhysicalAddress::from_u64(0x7F00_0000);

    let initial = client.get_attributes(base, 0x1000).unwrap();
    assert_eq!(initial, memory_attr_flags::WB);

    client
        .set_attributes(base, 0x1000, memory_attr_flags::RO | memory_attr_flags::XP)
        .unwrap();

    let updated = client.get_attributes(base, 0x1000).unwrap();
    assert_eq!(
        updated,
        memory_attr_flags::WB | memory_attr_flags::RO | memory_attr_flags::XP
    );

    client
        .clear_attributes(base, 0x1000, memory_attr_flags::RO)
        .unwrap();

    let final_attrs = client.get_attributes(base, 0x1000).unwrap();
    assert_eq!(final_attrs, memory_attr_flags::WB | memory_attr_flags::XP);
}
