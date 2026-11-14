//! Integration tests for Form Browser 2 and Form Display Engine protocols.

use core::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};

use rover_uefi::abi::types::{Bool, EFI_TRUE, UIntN};
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::form_browser::*;
use rover_uefi::protocols::firmware::hii::EfiHiiHandle;

static LAST_FORM_ID: AtomicU16 = AtomicU16::new(0);
static SCREEN_EXIT_CALLED: AtomicBool = AtomicBool::new(false);

unsafe extern "efiapi" fn mock_send_form(
    _this: *const EfiFormBrowser2Protocol,
    handles: *const EfiHiiHandle,
    handle_count: UIntN,
    _form_set_guid: *const EfiGuid,
    form_id: EfiFormId,
    _screen_dimensions: *const EfiScreenDescriptor,
    action_request: *mut UIntN,
) -> EfiStatus {
    if handles.is_null() || handle_count == 0 {
        return EfiStatus::INVALID_PARAMETER;
    }
    LAST_FORM_ID.store(form_id, Ordering::SeqCst);
    if !action_request.is_null() {
        unsafe {
            *action_request = browser_action_request::SUBMIT;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_browser_callback(
    _this: *const EfiFormBrowser2Protocol,
    results_data_size: *mut UIntN,
    results_data: *mut u16,
    retrieve_data: Bool,
    _variable_guid: *const EfiGuid,
    _variable_name: *const u16,
) -> EfiStatus {
    if retrieve_data == EFI_TRUE {
        unsafe {
            if *results_data_size < 4 {
                *results_data_size = 4;
                return EfiStatus::BUFFER_TOO_SMALL;
            }
            *results_data_size = 4;
            *results_data = 0x0041;
            *results_data.add(1) = 0x0000;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::SUCCESS
    }
}

unsafe extern "efiapi" fn mock_form_display(
    _form_data: *const c_void,
    user_input_data: *mut UserInput,
) -> EfiStatus {
    if !user_input_data.is_null() {
        unsafe {
            (*user_input_data).action = 1 << 0;
            (*user_input_data).default_id = 0x0001;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_exit_display() {
    SCREEN_EXIT_CALLED.store(true, Ordering::SeqCst);
}

unsafe extern "efiapi" fn mock_confirm_data_change() -> UIntN {
    browser_action_request::SUBMIT
}

#[test]
fn test_form_browser_guids_and_constants() {
    assert_eq!(
        EFI_FORM_BROWSER2_PROTOCOL_GUID,
        EfiGuid::new(
            0xB9D4_C360,
            0xBCFB,
            0x4F9B,
            [0x92, 0x98, 0x53, 0xC1, 0x36, 0x98, 0x22, 0x58]
        )
    );

    assert_eq!(
        FORM_DISPLAY_ENGINE_PROTOCOL_GUID,
        EfiGuid::new(
            0x9BBE_29E9,
            0xFDA1,
            0x41EC,
            [0xAD, 0x52, 0x45, 0x22, 0x13, 0x74, 0x2D, 0x2E]
        )
    );

    assert_eq!(
        EFI_HII_PLATFORM_SETUP_FORMSET_GUID,
        EfiGuid::new(
            0x9303_B241,
            0x0CF2,
            0x4931,
            [0x8E, 0x47, 0xAD, 0x7E, 0x30, 0x8F, 0x7E, 0xAC]
        )
    );

    assert_eq!(
        EFI_HII_USER_CREDENTIAL_FORMSET_GUID,
        EfiGuid::new(
            0x337F_48D3,
            0x56F5,
            0x4399,
            [0xCD, 0x83, 0x24, 0x7E, 0x81, 0x2E, 0x80, 0x78]
        )
    );

    assert_eq!(browser_action_request::NONE, 0);
    assert_eq!(browser_action_request::RESET, 1);
    assert_eq!(browser_action_request::SUBMIT, 2);
    assert_eq!(browser_action_request::EXIT, 3);
    assert_eq!(browser_action_request::FORM_SUBMIT_EXIT, 4);
    assert_eq!(browser_action_request::FORM_DISCARD_EXIT, 5);
    assert_eq!(browser_action_request::FORM_APPLY, 6);
    assert_eq!(browser_action_request::FORM_DISCARD, 7);
    assert_eq!(browser_action_request::RECONNECT, 8);
    assert_eq!(browser_action_request::QUESTION_APPLY, 9);

    assert_eq!(display_status::SUCCESS, 0);
    assert_eq!(display_status::SUBMIT_FAIL, 0x8000_0001);
    assert_eq!(display_status::NO_SUBMIT_IF, 0x8000_0002);
    assert_eq!(display_status::FORM_NOT_FOUND, 0x8000_0003);
    assert_eq!(display_status::INCONSISTENT_IF, 0x8000_0006);
    assert_eq!(display_status::WARNING_IF, 0x8000_0007);
}

#[test]
fn test_form_browser2_client_wrapper() {
    let proto = EfiFormBrowser2Protocol {
        send_form: mock_send_form,
        browser_callback: mock_browser_callback,
    };
    let client = unsafe { FormBrowser2::from_raw(&raw const proto) };

    let dummy_handle = EfiHiiHandle(0x1000 as *mut c_void);
    let handles = [dummy_handle];

    let action = match client.send_form(&handles, None, 0x0042, None) {
        Ok(act) => act,
        Err(e) => panic!("send_form failed: {e:?}"),
    };
    assert_eq!(action, browser_action_request::SUBMIT);
    assert_eq!(LAST_FORM_ID.load(Ordering::SeqCst), 0x0042);

    assert_eq!(
        client.send_form(&[], None, 0x0042, None),
        Err(EfiStatus::INVALID_PARAMETER)
    );

    let mut buf = [0u16; 16];
    let num_chars = match client.retrieve_uncommitted_data(None, None, &mut buf) {
        Ok(chars) => chars,
        Err(e) => panic!("retrieve_uncommitted_data failed: {e:?}"),
    };
    assert_eq!(num_chars, 2);
    assert_eq!(buf[0], 0x0041);
    assert_eq!(buf[1], 0x0000);

    let mut set_buf = [0x0042u16, 0x0000u16];
    assert!(
        client
            .set_uncommitted_data(None, None, &mut set_buf)
            .is_ok()
    );
}

#[test]
fn test_form_display_engine_client_wrapper() {
    let proto = EfiFormDisplayEngineProtocol {
        form_display: mock_form_display,
        exit_display: mock_exit_display,
        confirm_data_change: mock_confirm_data_change,
    };
    let client = unsafe { FormDisplayEngine::from_raw(&raw const proto) };

    let dummy_form = 0x2000 as *const c_void;
    let input = match unsafe { client.display_form(dummy_form) } {
        Ok(inp) => inp,
        Err(e) => panic!("display_form failed: {e:?}"),
    };
    assert_eq!(input.action, 1);
    assert_eq!(input.default_id, 1);

    client.exit_display();
    assert!(SCREEN_EXIT_CALLED.load(Ordering::SeqCst));

    let confirm = client.confirm_data_change();
    assert_eq!(confirm, browser_action_request::SUBMIT);
}
