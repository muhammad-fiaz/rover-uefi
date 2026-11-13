//! Unit and integration tests for Platform Specific Reset Filter and Reset Handler.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use core::ffi::c_void;
use core::sync::atomic::{AtomicU32, Ordering};
use rover_uefi::base::{EfiStatus, ResetType};
use rover_uefi::protocols::driver::{
    EfiPlatformSpecificResetFilterProtocol, EfiPlatformSpecificResetHandlerProtocol,
    PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID, PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID,
    PlatformSpecificResetFilter, PlatformSpecificResetHandler,
};

static FILTER_CALL_COUNT: AtomicU32 = AtomicU32::new(0);
static HANDLER_CALL_COUNT: AtomicU32 = AtomicU32::new(0);

unsafe extern "efiapi" fn mock_reset_filter(
    reset_type: ResetType,
    reset_status: EfiStatus,
    data_size: usize,
    _reset_data: *mut c_void,
) {
    assert_eq!(reset_type, ResetType::PLATFORM_SPECIFIC);
    assert_eq!(reset_status, EfiStatus::SUCCESS);
    assert_eq!(data_size, 4);
    FILTER_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
}

unsafe extern "efiapi" fn mock_reset_handler(
    reset_type: ResetType,
    reset_status: EfiStatus,
    data_size: usize,
    _reset_data: *mut c_void,
) {
    assert_eq!(reset_type, ResetType::COLD);
    assert_eq!(reset_status, EfiStatus::SUCCESS);
    assert_eq!(data_size, 0);
    HANDLER_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
}

#[test]
fn test_platform_specific_reset_guids() {
    assert_eq!(
        PlatformSpecificResetFilter::GUID,
        PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID
    );
    assert_eq!(
        PlatformSpecificResetHandler::GUID,
        PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID
    );
}

#[test]
fn test_platform_specific_reset_filter_and_handler_invocation() {
    let raw_filter = EfiPlatformSpecificResetFilterProtocol {
        reset_filter: mock_reset_filter,
    };
    let filter = unsafe { PlatformSpecificResetFilter::from_raw(&raw const raw_filter) };

    let mut custom_data = [0xDE, 0xAD, 0xBE, 0xEF];
    filter.invoke_filter(
        ResetType::PLATFORM_SPECIFIC,
        EfiStatus::SUCCESS,
        Some(&mut custom_data),
    );
    assert_eq!(FILTER_CALL_COUNT.load(Ordering::SeqCst), 1);

    let raw_handler = EfiPlatformSpecificResetHandlerProtocol {
        reset_handler: mock_reset_handler,
    };
    let handler = unsafe { PlatformSpecificResetHandler::from_raw(&raw const raw_handler) };

    handler.invoke_handler(ResetType::COLD, EfiStatus::SUCCESS, None);
    assert_eq!(HANDLER_CALL_COUNT.load(Ordering::SeqCst), 1);
}
