//! # HII Configuration Keyword Handler Protocol Tests
//!
//! Unit tests for HII configuration keyword handler protocol definitions and client wrapper.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::firmware::hii_keyword::{
    ConfigKeywordHandler, EFI_CONFIG_KEYWORD_HANDLER_PROTOCOL_GUID,
    EfiConfigKeywordHandlerProtocol, KEYWORD_HANDLER_ACCESS_NOT_PERMITTED,
    KEYWORD_HANDLER_INCOMPATIBLE_VALUE_DETECTED, KEYWORD_HANDLER_KEYWORD_NOT_FOUND,
    KEYWORD_HANDLER_MALFORMED_STRING, KEYWORD_HANDLER_NAMESPACE_ID_NOT_FOUND,
    KEYWORD_HANDLER_NO_ERROR, KEYWORD_HANDLER_UNDEFINED_PROCESSING_ERROR,
};

#[test]
fn test_hii_keyword_guid_and_constants() {
    assert_ne!(
        EFI_CONFIG_KEYWORD_HANDLER_PROTOCOL_GUID,
        rover_uefi::base::EfiGuid::ZERO
    );

    assert_eq!(KEYWORD_HANDLER_NO_ERROR, 0x0000_0000);
    assert_eq!(KEYWORD_HANDLER_NAMESPACE_ID_NOT_FOUND, 0x0000_0001);
    assert_eq!(KEYWORD_HANDLER_MALFORMED_STRING, 0x0000_0002);
    assert_eq!(KEYWORD_HANDLER_KEYWORD_NOT_FOUND, 0x0000_0004);
    assert_eq!(KEYWORD_HANDLER_INCOMPATIBLE_VALUE_DETECTED, 0x0000_0008);
    assert_eq!(KEYWORD_HANDLER_ACCESS_NOT_PERMITTED, 0x0000_0010);
    assert_eq!(KEYWORD_HANDLER_UNDEFINED_PROCESSING_ERROR, 0x8000_0000);
}

static SET_DATA_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static GET_DATA_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static mut MOCK_RESULTS: [u16; 8] = [0x0041, 0x0042, 0x0043, 0x0000, 0, 0, 0, 0];

unsafe extern "efiapi" fn mock_set_data(
    _this: *mut EfiConfigKeywordHandlerProtocol,
    _keyword_string: *const u16,
    progress: *mut *mut u16,
    progress_err: *mut u32,
) -> EfiStatus {
    SET_DATA_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !progress_err.is_null() {
        unsafe {
            *progress_err = KEYWORD_HANDLER_NO_ERROR;
        }
    }
    if !progress.is_null() {
        unsafe {
            *progress = core::ptr::null_mut();
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_data(
    _this: *mut EfiConfigKeywordHandlerProtocol,
    _namespace_id: *const u16,
    _keyword_string: *const u16,
    progress: *mut *mut u16,
    progress_err: *mut u32,
    results: *mut *mut u16,
) -> EfiStatus {
    GET_DATA_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !progress_err.is_null() {
        unsafe {
            *progress_err = KEYWORD_HANDLER_NO_ERROR;
        }
    }
    if !progress.is_null() {
        unsafe {
            *progress = core::ptr::null_mut();
        }
    }
    if !results.is_null() {
        unsafe {
            *results = core::ptr::addr_of_mut!(MOCK_RESULTS).cast::<u16>();
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_hii_keyword_client_wrapper() {
    let mut raw = EfiConfigKeywordHandlerProtocol {
        set_data: mock_set_data,
        get_data: mock_get_data,
    };

    let mut client = ConfigKeywordHandler::new(&mut raw);
    assert!(!client.raw().is_null());

    let dummy_ucs2: [u16; 4] = [0x0058, 0x0059, 0x005A, 0x0000];
    let set_res = unsafe { client.set_data_raw(dummy_ucs2.as_ptr()) };
    assert!(set_res.is_ok());
    assert!(SET_DATA_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    let get_res = unsafe { client.get_data_raw(core::ptr::null(), dummy_ucs2.as_ptr()) };
    assert!(get_res.is_ok());
    let results_ptr = match get_res {
        Ok(ptr) => ptr,
        Err(_) => core::ptr::null_mut(),
    };
    assert_eq!(
        results_ptr,
        core::ptr::addr_of_mut!(MOCK_RESULTS).cast::<u16>()
    );
    assert!(GET_DATA_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}
