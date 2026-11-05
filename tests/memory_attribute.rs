// SPDX-License-Identifier: Apache-2.0
//! Unit tests for UEFI 2.10 Memory Attribute Protocol.

use core::mem::size_of;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::core::{
    EFI_MEMORY_ATTRIBUTE_MASK, EFI_MEMORY_RO, EFI_MEMORY_RP, EFI_MEMORY_XP,
    EfiMemoryAttributeProtocol, MEMORY_ATTRIBUTE_PROTOCOL_GUID, MemoryAttributeProtocol,
};

unsafe extern "efiapi" fn mock_get_memory_attributes(
    _this: *mut EfiMemoryAttributeProtocol,
    base: u64,
    _length: u64,
    attributes: *mut u64,
) -> EfiStatus {
    if attributes.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if base == 0x1000 {
        unsafe { *attributes = EFI_MEMORY_RO | EFI_MEMORY_XP };
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_set_memory_attributes(
    _this: *mut EfiMemoryAttributeProtocol,
    _base: u64,
    _length: u64,
    _attributes: u64,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clear_memory_attributes(
    _this: *mut EfiMemoryAttributeProtocol,
    _base: u64,
    _length: u64,
    _attributes: u64,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn guid_and_constants() {
    assert_eq!(
        MEMORY_ATTRIBUTE_PROTOCOL_GUID.to_bytes(),
        [
            0xf6, 0x0c, 0x56, 0xf4, 0xec, 0x40, 0x4a, 0x4b, 0xa1, 0x92, 0xbf, 0x1d, 0x57, 0xd0,
            0xb1, 0x89
        ]
    );
    assert_eq!(EFI_MEMORY_RP, 0x0000_2000);
    assert_eq!(EFI_MEMORY_XP, 0x0000_4000);
    assert_eq!(EFI_MEMORY_RO, 0x0002_0000);
    assert_eq!(
        EFI_MEMORY_ATTRIBUTE_MASK,
        EFI_MEMORY_RP | EFI_MEMORY_XP | EFI_MEMORY_RO
    );
}

#[test]
fn protocol_layout() {
    assert_eq!(
        size_of::<EfiMemoryAttributeProtocol>(),
        size_of::<usize>() * 3
    );
}

#[test]
fn protocol_wrapper() {
    let mut raw = EfiMemoryAttributeProtocol {
        get_memory_attributes: mock_get_memory_attributes,
        set_memory_attributes: mock_set_memory_attributes,
        clear_memory_attributes: mock_clear_memory_attributes,
    };

    let Some(mut proto) = (unsafe { MemoryAttributeProtocol::from_raw(&raw mut raw) }) else {
        panic!("expected protocol wrapper");
    };

    // Zero length error
    assert_eq!(
        proto.get_attributes(0x1000, 0),
        (EfiStatus::INVALID_PARAMETER, 0)
    );
    assert_eq!(
        proto.set_attributes(0x1000, 0, EFI_MEMORY_RO),
        EfiStatus::INVALID_PARAMETER
    );

    // Invalid attribute bits error
    assert_eq!(
        proto.set_attributes(0x1000, 0x1000, 0x8000_0000),
        EfiStatus::INVALID_PARAMETER
    );

    // Valid get
    let (status, attrs) = proto.get_attributes(0x1000, 0x1000);
    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(attrs, EFI_MEMORY_RO | EFI_MEMORY_XP);

    // Valid set & clear
    assert_eq!(
        proto.set_attributes(0x1000, 0x1000, EFI_MEMORY_RO),
        EfiStatus::SUCCESS
    );
    assert_eq!(
        proto.clear_attributes(0x1000, 0x1000, EFI_MEMORY_RO),
        EfiStatus::SUCCESS
    );
}
