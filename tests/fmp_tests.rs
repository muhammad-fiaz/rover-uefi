//! Integration tests for Firmware Management Protocol (FMP).

use core::ffi::c_void;
use rover_uefi::abi::types::{Char16, UIntN};
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::{
    EFI_FIRMWARE_MANAGEMENT_PROTOCOL_GUID, EfiFirmwareImageDescriptor,
    EfiFirmwareManagementProtocol, EfiFirmwareManagementUpdateImageProgress, FirmwareManagement,
    FmpCapsuleHeader, is_reserved_flags, parse_fmp_capsule_header,
};

static DUMMY_IMAGE_DATA: [u8; 32] = [0xEE; 32];

unsafe extern "efiapi" fn mock_fmp_get_image_info(
    _this: *mut EfiFirmwareManagementProtocol,
    image_info_size: *mut UIntN,
    image_info: *mut EfiFirmwareImageDescriptor,
    descriptor_version: *mut u32,
    descriptor_count: *mut u8,
    descriptor_size: *mut UIntN,
    package_version: *mut u32,
    package_version_name: *mut *mut Char16,
) -> EfiStatus {
    if image_info_size.is_null()
        || image_info.is_null()
        || descriptor_version.is_null()
        || descriptor_count.is_null()
        || descriptor_size.is_null()
        || package_version.is_null()
        || package_version_name.is_null()
    {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *descriptor_version = 3;
        *descriptor_count = 1;
        *descriptor_size = core::mem::size_of::<EfiFirmwareImageDescriptor>() as UIntN;
        *package_version = 0x0102_0304;

        *image_info = EfiFirmwareImageDescriptor {
            image_index: 1,
            image_type_id: EFI_FIRMWARE_MANAGEMENT_PROTOCOL_GUID,
            image_id: 0x1000,
            image_id_name: core::ptr::null_mut(),
            version: 0x0200_0000,
            version_name: core::ptr::null_mut(),
            size: 32,
            attributes_supported: 0x0001,
            attributes_setting: 0x0001,
            compatibilities: 0,
            lowest_supported_image_version: 0x0100_0000,
            last_attempt_version: 0x0200_0000,
            last_attempt_status: 0,
            hardware_instance: 0,
            dependencies: core::ptr::null(),
        };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_fmp_get_image(
    _this: *mut EfiFirmwareManagementProtocol,
    image_index: u8,
    image: *mut c_void,
    image_size: *mut UIntN,
) -> EfiStatus {
    if image.is_null() || image_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if image_index == 1 {
        unsafe {
            if *image_size >= 32 {
                let slice = core::slice::from_raw_parts_mut(image.cast::<u8>(), 32);
                slice.copy_from_slice(&DUMMY_IMAGE_DATA);
                *image_size = 32;
                EfiStatus::SUCCESS
            } else {
                *image_size = 32;
                EfiStatus::BUFFER_TOO_SMALL
            }
        }
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_fmp_set_image(
    _this: *mut EfiFirmwareManagementProtocol,
    image_index: u8,
    image: *const c_void,
    image_size: UIntN,
    _vendor_code: *const c_void,
    progress: Option<EfiFirmwareManagementUpdateImageProgress>,
    _abort_reason: *mut *mut Char16,
) -> EfiStatus {
    if image.is_null() || image_size == 0 {
        return EfiStatus::INVALID_PARAMETER;
    }
    if image_index == 1 {
        if let Some(cb) = progress {
            unsafe {
                let _ = cb(100);
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_fmp_check_image(
    _this: *mut EfiFirmwareManagementProtocol,
    image_index: u8,
    image: *const c_void,
    image_size: UIntN,
    image_updatable: *mut u32,
) -> EfiStatus {
    if image.is_null() || image_size == 0 || image_updatable.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if image_index == 1 {
        unsafe {
            *image_updatable = 0x01; // IMAGE_UPDATABLE_VALID
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_fmp_get_package_info(
    _this: *mut EfiFirmwareManagementProtocol,
    package_version: *mut u32,
    _package_version_name: *mut *mut Char16,
    _package_version_name_max_len: *mut u32,
    attributes_supported: *mut u64,
    attributes_setting: *mut u64,
) -> EfiStatus {
    if package_version.is_null() || attributes_supported.is_null() || attributes_setting.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *package_version = 0x0100_0000;
        *attributes_supported = 0x0001;
        *attributes_setting = 0x0001;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_fmp_set_package_info(
    _this: *mut EfiFirmwareManagementProtocol,
    _image: *const c_void,
    _image_size: UIntN,
    _vendor_code: *const c_void,
    package_version: u32,
    _package_version_name: *const Char16,
) -> EfiStatus {
    if package_version > 0 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

#[test]
fn test_fmp_guid_and_capsule_parsing() {
    assert_eq!(
        EFI_FIRMWARE_MANAGEMENT_PROTOCOL_GUID,
        EfiGuid::new(
            0x86C7_7A67,
            0x0B97,
            0x4633,
            [0xA1, 0x87, 0x49, 0x10, 0x4D, 0x06, 0x85, 0xC7],
        )
    );

    assert!(is_reserved_flags(0));
    assert!(!is_reserved_flags(1));

    // Valid capsule header bytes (48 bytes)
    let mut capsule_bytes = [0u8; 48];
    // CapsuleGuid (16 bytes)
    capsule_bytes[..16].copy_from_slice(&EFI_FIRMWARE_MANAGEMENT_PROTOCOL_GUID.to_bytes());
    // HeaderSize (4 bytes LE) = 48
    capsule_bytes[16..20].copy_from_slice(&48u32.to_le_bytes());
    // Flags (4 bytes LE) = 0
    capsule_bytes[20..24].copy_from_slice(&0u32.to_le_bytes());
    // ImageType GUID (16 bytes)
    capsule_bytes[24..40].copy_from_slice(&[0x11; 16]);
    // ImageVersion (4 bytes LE) = 0x01020304
    capsule_bytes[40..44].copy_from_slice(&0x0102_0304u32.to_le_bytes());
    // ImageSize (4 bytes LE) = 1024
    capsule_bytes[44..48].copy_from_slice(&1024u32.to_le_bytes());

    let header = parse_fmp_capsule_header(&capsule_bytes).unwrap_or(FmpCapsuleHeader {
        header_size: 0,
        flags: 0,
        image_type: EfiGuid::ZERO,
        image_version: 0,
        image_size: 0,
    });
    assert_eq!(header.header_size, 48);
    assert_eq!(header.image_version, 0x0102_0304);
    assert_eq!(header.image_size, 1024);
}

#[test]
fn test_firmware_management_client_wrapper() {
    let mut mock_proto = EfiFirmwareManagementProtocol {
        get_image_info: mock_fmp_get_image_info,
        get_image: mock_fmp_get_image,
        set_image: mock_fmp_set_image,
        check_image: mock_fmp_check_image,
        get_package_info: mock_fmp_get_package_info,
        set_package_info: mock_fmp_set_package_info,
    };

    let mut client = FirmwareManagement::new(&mut mock_proto);

    let mut descriptors = [EfiFirmwareImageDescriptor {
        image_index: 0,
        image_type_id: EfiGuid::ZERO,
        image_id: 0,
        image_id_name: core::ptr::null_mut(),
        version: 0,
        version_name: core::ptr::null_mut(),
        size: 0,
        attributes_supported: 0,
        attributes_setting: 0,
        compatibilities: 0,
        lowest_supported_image_version: 0,
        last_attempt_version: 0,
        last_attempt_status: 0,
        hardware_instance: 0,
        dependencies: core::ptr::null(),
    }];

    let (desc_ver, desc_count, desc_size, pkg_ver) = client
        .get_image_info(&mut descriptors)
        .unwrap_or((0, 0, 0, 0));
    assert_eq!(desc_ver, 3);
    assert_eq!(desc_count, 1);
    assert_eq!(
        desc_size,
        core::mem::size_of::<EfiFirmwareImageDescriptor>()
    );
    assert_eq!(pkg_ver, 0x0102_0304);
    assert_eq!(descriptors[0].image_index, 1);
    assert_eq!(descriptors[0].size, 32);

    let mut img_buf = [0u8; 32];
    let bytes_read = client.get_image(1, &mut img_buf).unwrap_or(0);
    assert_eq!(bytes_read, 32);
    assert_eq!(&img_buf, &DUMMY_IMAGE_DATA);

    assert_eq!(client.get_image(2, &mut img_buf), Err(EfiStatus::NOT_FOUND));

    let new_img = [0x55u8; 32];
    let updatable = client.check_image(1, &new_img).unwrap_or(0);
    assert_eq!(updatable, 0x01);
    assert_eq!(client.check_image(2, &new_img), Err(EfiStatus::NOT_FOUND));

    assert!(client.set_image(1, &new_img, None, None).is_ok());
    assert_eq!(
        client.set_image(2, &new_img, None, None),
        Err(EfiStatus::NOT_FOUND)
    );

    let (pkg_v, attr_supp, attr_set) = client.get_package_info().unwrap_or((0, 0, 0));
    assert_eq!(pkg_v, 0x0100_0000);
    assert_eq!(attr_supp, 0x0001);
    assert_eq!(attr_set, 0x0001);

    let name = [0x0041u16, 0x0000u16];
    assert!(client.set_package_info(&new_img, None, 2, &name).is_ok());
}
