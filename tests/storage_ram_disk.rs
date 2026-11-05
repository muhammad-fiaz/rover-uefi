// SPDX-License-Identifier: Apache-2.0
//! Unit tests for UEFI 2.6 RAM Disk Protocol.

use core::mem::size_of;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::device_path::EfiDevicePath;
use rover_uefi::protocols::storage::{
    EfiRamDiskProtocol, PERSISTENT_VIRTUAL_CD_GUID, PERSISTENT_VIRTUAL_DISK_GUID,
    RAM_DISK_PROTOCOL_GUID, RamDiskProtocol, VIRTUAL_CD_GUID, VIRTUAL_DISK_GUID,
};

static mut FAKE_DEVICE_PATH: EfiDevicePath = EfiDevicePath {
    device_type: 0x7f,
    subtype: 0xff,
    length: [4, 0],
};

unsafe extern "efiapi" fn mock_register_ramdisk(
    _base: u64,
    size: u64,
    _disk_type: *const EfiGuid,
    _parent: *const EfiDevicePath,
    device_path: *mut *mut EfiDevicePath,
) -> EfiStatus {
    if size == 0 || device_path.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *device_path = core::ptr::addr_of_mut!(FAKE_DEVICE_PATH) };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unregister_ramdisk(device_path: *const EfiDevicePath) -> EfiStatus {
    if device_path.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::SUCCESS
}

#[test]
fn guid_constants() {
    assert_eq!(
        RAM_DISK_PROTOCOL_GUID.to_bytes(),
        [
            0xdf, 0xa0, 0x38, 0xab, 0x73, 0x68, 0xa9, 0x44, 0x87, 0xe6, 0xd4, 0xeb, 0x56, 0x14,
            0x84, 0x49
        ]
    );
    assert_eq!(
        VIRTUAL_DISK_GUID.to_bytes(),
        [
            0x5a, 0x53, 0xab, 0x77, 0xfc, 0x45, 0x4b, 0x62, 0x55, 0x60, 0xf7, 0xb2, 0x81, 0xd1,
            0xf9, 0x6e
        ]
    );
    assert_eq!(
        VIRTUAL_CD_GUID.to_bytes(),
        [
            0x30, 0xbd, 0x5a, 0x3d, 0x75, 0x41, 0xce, 0x87, 0x6d, 0x64, 0xd2, 0xad, 0xe5, 0x23,
            0xc4, 0xbb
        ]
    );
    assert_eq!(
        PERSISTENT_VIRTUAL_DISK_GUID.to_bytes(),
        [
            0xc9, 0x02, 0xea, 0x5c, 0x07, 0x4d, 0xd3, 0x69, 0x26, 0x9f, 0x44, 0x96, 0xfb, 0xe0,
            0x96, 0xf9
        ]
    );
    assert_eq!(
        PERSISTENT_VIRTUAL_CD_GUID.to_bytes(),
        [
            0x88, 0x81, 0x01, 0x08, 0xcd, 0x42, 0x48, 0xbb, 0x10, 0x0f, 0x53, 0x87, 0xd5, 0x3d,
            0xed, 0x3d
        ]
    );
}

#[test]
fn protocol_layout() {
    assert_eq!(size_of::<EfiRamDiskProtocol>(), size_of::<usize>() * 2);
}

#[test]
fn protocol_wrapper() {
    let mut raw = EfiRamDiskProtocol {
        register: mock_register_ramdisk,
        unregister: mock_unregister_ramdisk,
    };

    let Some(mut proto) = (unsafe { RamDiskProtocol::from_raw(&raw mut raw) }) else {
        panic!("expected protocol wrapper");
    };

    // Zero size error
    let (status, dp) = proto.register_ramdisk(0x1000_0000, 0, &VIRTUAL_DISK_GUID, None);
    assert_eq!(status, EfiStatus::INVALID_PARAMETER);
    assert!(dp.is_none());

    // Valid registration
    let (status, dp) = proto.register_ramdisk(0x1000_0000, 0x10_0000, &VIRTUAL_DISK_GUID, None);
    assert_eq!(status, EfiStatus::SUCCESS);
    let Some(dev_path) = dp else {
        panic!("expected device path");
    };

    // Valid unregistration
    assert_eq!(
        unsafe { proto.unregister_ramdisk(dev_path) },
        EfiStatus::SUCCESS
    );

    // Null pointer error
    assert_eq!(
        unsafe { proto.unregister_ramdisk(core::ptr::null()) },
        EfiStatus::INVALID_PARAMETER
    );
}
