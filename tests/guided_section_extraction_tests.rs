//! Integration tests for `GUIDed` Section Extraction Protocol.

use core::ffi::c_void;

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::firmware::guided_section_extraction::*;

static DUMMY_OUTPUT: [u8; 16] = [0xAA; 16];

unsafe extern "efiapi" fn mock_extract_section(
    _this: *const EfiGuidedSectionExtractionProtocol,
    _input_section: *const c_void,
    output_buffer: *mut *mut c_void,
    output_size: *mut UIntN,
    authentication_status: *mut u32,
) -> EfiStatus {
    if !output_buffer.is_null() {
        unsafe {
            *output_buffer = DUMMY_OUTPUT.as_ptr().cast_mut().cast::<c_void>();
        }
    }
    if !output_size.is_null() {
        unsafe {
            *output_size = DUMMY_OUTPUT.len();
        }
    }
    if !authentication_status.is_null() {
        unsafe {
            *authentication_status =
                EFI_AUTH_STATUS_IMAGE_SIGNED | EFI_AUTH_STATUS_PLATFORM_OVERRIDE;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_guided_section_constants() {
    assert_eq!(EFI_GUIDED_SECTION_PROCESSING_REQUIRED, 0x0001);
    assert_eq!(EFI_GUIDED_SECTION_AUTH_STATUS_VALID, 0x0002);
    assert_eq!(EFI_AUTH_STATUS_PLATFORM_OVERRIDE, 0x0001);
    assert_eq!(EFI_AUTH_STATUS_IMAGE_SIGNED, 0x0002);
    assert_eq!(EFI_AUTH_STATUS_NOT_TESTED, 0x0004);
    assert_eq!(EFI_AUTH_STATUS_TEST_FAILED, 0x0008);
    assert_eq!(EFI_AUTH_STATUS_ALL, 0x000F);
}

#[test]
fn test_guided_section_client_wrapper() {
    let mock_proto = EfiGuidedSectionExtractionProtocol {
        extract_section: mock_extract_section,
    };
    let client = unsafe { GuidedSectionExtraction::from_raw(&raw const mock_proto) };

    let dummy_input: [u8; 8] = [0x55; 8];
    let result = client.extract_section(&dummy_input);

    assert!(result.is_ok());
    if let Ok(extracted) = result {
        assert_eq!(
            extracted.buffer,
            DUMMY_OUTPUT.as_ptr().cast_mut().cast::<c_void>()
        );
        assert_eq!(extracted.size, 16);
        assert_eq!(
            extracted.authentication_status,
            EFI_AUTH_STATUS_IMAGE_SIGNED | EFI_AUTH_STATUS_PLATFORM_OVERRIDE
        );
    }
}
