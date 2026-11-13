//! Integration tests for Legacy Region 2 Protocol.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::{
    EFI_LEGACY_REGION2_PROTOCOL_GUID, EfiLegacyRegion2Protocol, LEGACY_REGION_END,
    LEGACY_REGION_SIZE, LEGACY_REGION_START, LegacyRegion2, LegacyRegionAttribute,
    LegacyRegionDescriptor,
};

#[test]
fn test_legacy_region2_guid_and_constants() {
    assert_eq!(
        EFI_LEGACY_REGION2_PROTOCOL_GUID,
        EfiGuid::new(
            0x7010_1EAF,
            0x0085,
            0x440C,
            [0xB3, 0x56, 0x8E, 0xE3, 0x6F, 0xEF, 0x24, 0xF0],
        )
    );
    assert_eq!(LEGACY_REGION_START, 0x000C_0000);
    assert_eq!(LEGACY_REGION_END, 0x000F_FFFF);
    assert_eq!(LEGACY_REGION_SIZE, 0x0004_0000);

    assert_eq!(LegacyRegionAttribute::Decoded as u32, 0);
    assert_eq!(LegacyRegionAttribute::NotDecoded as u32, 1);
    assert_eq!(LegacyRegionAttribute::WriteEnabled as u32, 2);
    assert_eq!(LegacyRegionAttribute::WriteDisabled as u32, 3);
    assert_eq!(LegacyRegionAttribute::BootLocked as u32, 4);
    assert_eq!(LegacyRegionAttribute::NotLocked as u32, 5);
}

static mut MOCK_DECODE_CALLED: bool = false;
static mut MOCK_LOCK_CALLED: bool = false;
static mut MOCK_BOOT_LOCK_CALLED: bool = false;
static mut MOCK_UNLOCK_CALLED: bool = false;
static mut MOCK_GET_INFO_CALLED: bool = false;

unsafe extern "efiapi" fn mock_decode(
    _this: *mut EfiLegacyRegion2Protocol,
    start: u32,
    _length: u32,
    granularity: *mut u32,
    on: *const bool,
) -> EfiStatus {
    if !(LEGACY_REGION_START..=LEGACY_REGION_END).contains(&start) {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        MOCK_DECODE_CALLED = true;
        if !granularity.is_null() {
            *granularity = 0x4000; // 16 KiB granularity
        }
        if !on.is_null() && !*on {
            return EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_lock(
    _this: *mut EfiLegacyRegion2Protocol,
    start: u32,
    _length: u32,
    granularity: *mut u32,
) -> EfiStatus {
    if !(LEGACY_REGION_START..=LEGACY_REGION_END).contains(&start) {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        MOCK_LOCK_CALLED = true;
        if !granularity.is_null() {
            *granularity = 0x4000;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_boot_lock(
    _this: *mut EfiLegacyRegion2Protocol,
    start: u32,
    _length: u32,
    granularity: *mut u32,
) -> EfiStatus {
    if !(LEGACY_REGION_START..=LEGACY_REGION_END).contains(&start) {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        MOCK_BOOT_LOCK_CALLED = true;
        if !granularity.is_null() {
            *granularity = 0x4000;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unlock(
    _this: *mut EfiLegacyRegion2Protocol,
    start: u32,
    _length: u32,
    granularity: *mut u32,
) -> EfiStatus {
    if !(LEGACY_REGION_START..=LEGACY_REGION_END).contains(&start) {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        MOCK_UNLOCK_CALLED = true;
        if !granularity.is_null() {
            *granularity = 0x4000;
        }
    }
    EfiStatus::SUCCESS
}

static mut MOCK_DESCRIPTORS: [LegacyRegionDescriptor; 1] = [LegacyRegionDescriptor {
    start: 0x000C_0000,
    length: 0x0001_0000,
    attribute: LegacyRegionAttribute::WriteEnabled,
    granularity: 0x4000,
}];

unsafe extern "efiapi" fn mock_get_info(
    _this: *mut EfiLegacyRegion2Protocol,
    descriptor_count: *mut u32,
    descriptor: *mut *mut LegacyRegionDescriptor,
) -> EfiStatus {
    unsafe {
        MOCK_GET_INFO_CALLED = true;
        if !descriptor_count.is_null() {
            *descriptor_count = 1;
        }
        if !descriptor.is_null() {
            *descriptor =
                core::ptr::addr_of_mut!(MOCK_DESCRIPTORS).cast::<LegacyRegionDescriptor>();
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_legacy_region2_client_wrapper() {
    let mut raw_proto = EfiLegacyRegion2Protocol {
        decode: mock_decode,
        lock: mock_lock,
        boot_lock: mock_boot_lock,
        unlock: mock_unlock,
        get_info: mock_get_info,
    };

    let mut client = LegacyRegion2::new(&mut raw_proto);
    assert!(!client.raw().is_null());

    // Test decode valid and invalid
    let dec_res = client.decode(0x000C_0000, 0x4000, true);
    assert!(dec_res.is_ok());
    if let Ok(granularity) = dec_res {
        assert_eq!(granularity, 0x4000);
    }

    let dec_invalid = client.decode(0x0005_0000, 0x4000, true);
    assert_eq!(dec_invalid, Err(EfiStatus::INVALID_PARAMETER));

    // Test lock
    let lock_res = client.lock(0x000D_0000, 0x4000);
    assert!(lock_res.is_ok());
    if let Ok(granularity) = lock_res {
        assert_eq!(granularity, 0x4000);
    }

    // Test boot_lock
    let boot_res = client.boot_lock(0x000E_0000, 0x4000);
    assert!(boot_res.is_ok());
    if let Ok(granularity) = boot_res {
        assert_eq!(granularity, 0x4000);
    }

    // Test unlock
    let unlock_res = client.unlock(0x000F_0000, 0x4000);
    assert!(unlock_res.is_ok());
    if let Ok(granularity) = unlock_res {
        assert_eq!(granularity, 0x4000);
    }

    // Test get_info_raw
    let mut count = 0u32;
    let mut desc_ptr = core::ptr::null_mut::<LegacyRegionDescriptor>();
    let info_res = unsafe { client.get_info_raw(&mut count, &mut desc_ptr) };
    assert!(info_res.is_ok());
    assert_eq!(count, 1);
    assert!(!desc_ptr.is_null());
    let first = unsafe { &*desc_ptr };
    assert_eq!(first.start, 0x000C_0000);
    assert_eq!(first.length, 0x0001_0000);
    assert_eq!(first.attribute, LegacyRegionAttribute::WriteEnabled);
    assert_eq!(first.granularity, 0x4000);
}
