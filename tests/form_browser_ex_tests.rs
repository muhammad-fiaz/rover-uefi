//! Integration tests for Form Browser Extension and Form Browser Extension 2 protocols.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use rover_uefi::abi::types::Bool;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::console::input::EfiInputKey;
use rover_uefi::protocols::firmware::form_browser_ex::{
    BROWSER_ACTION_DEFAULT, BROWSER_ACTION_DISCARD, BROWSER_ACTION_EXIT, BROWSER_ACTION_GOTO,
    BROWSER_ACTION_RESET, BROWSER_ACTION_SUBMIT, BROWSER_ACTION_UNREGISTER,
    BROWSER_EXTENSION2_VERSION_1, BROWSER_EXTENSION2_VERSION_1_1, BrowserSettingScope,
    EfiFormBrowserExtension2Protocol, EfiFormBrowserExtensionProtocol,
    FORM_BROWSER_EXTENSION_PROTOCOL_GUID, FORM_BROWSER_EXTENSION2_PROTOCOL_GUID,
    FormBrowserExtension, FormBrowserExtension2,
};

static SCOPE_RECORDED: Mutex<Option<BrowserSettingScope>> = Mutex::new(None);
static HOTKEY_ACTION: AtomicU32 = AtomicU32::new(0);
static EXEC_ACTION: AtomicU32 = AtomicU32::new(0);
static EXIT_HANDLER_CALLED: AtomicBool = AtomicBool::new(false);

unsafe extern "efiapi" fn mock_set_scope(scope: BrowserSettingScope) -> EfiStatus {
    if let Ok(mut lock) = SCOPE_RECORDED.lock() {
        *lock = Some(scope);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_hot_key(
    _key: *const EfiInputKey,
    action: u32,
    _default_id: u16,
    _help: *const u16,
) -> EfiStatus {
    HOTKEY_ACTION.store(action, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_register_exit_handler(
    handler: Option<unsafe extern "efiapi" fn()>,
) -> EfiStatus {
    if let Some(h) = handler {
        unsafe {
            h();
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_is_modified() -> Bool {
    1
}

unsafe extern "efiapi" fn mock_execute_action(action: u32, _default_id: u16) -> EfiStatus {
    EXEC_ACTION.store(action, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_is_reset_required() -> Bool {
    0
}

unsafe extern "efiapi" fn sample_exit_cb() {
    EXIT_HANDLER_CALLED.store(true, Ordering::SeqCst);
}

#[test]
fn test_form_browser_ex_constants_and_guids() {
    assert_eq!(
        FORM_BROWSER_EXTENSION_PROTOCOL_GUID,
        EfiGuid::new(
            0x1F73_B18D,
            0x4630,
            0x43C1,
            [0xA1, 0xDE, 0x6F, 0x80, 0x85, 0x5D, 0x7D, 0xA4],
        )
    );
    assert_eq!(
        FORM_BROWSER_EXTENSION2_PROTOCOL_GUID,
        EfiGuid::new(
            0xA770_C357,
            0xB693,
            0x4E6D,
            [0xA6, 0xCF, 0xD2, 0x1C, 0x72, 0x8E, 0x55, 0x0B],
        )
    );
    assert_eq!(BROWSER_EXTENSION2_VERSION_1, 0x0001_0000);
    assert_eq!(BROWSER_EXTENSION2_VERSION_1_1, 0x0001_0001);
    assert_eq!(BROWSER_ACTION_UNREGISTER, 0);
    assert_eq!(BROWSER_ACTION_DISCARD, 1);
    assert_eq!(BROWSER_ACTION_DEFAULT, 2);
    assert_eq!(BROWSER_ACTION_SUBMIT, 4);
    assert_eq!(BROWSER_ACTION_RESET, 8);
    assert_eq!(BROWSER_ACTION_EXIT, 16);
    assert_eq!(BROWSER_ACTION_GOTO, 32);
}

#[test]
fn test_form_browser_extension_wrapper() {
    let mock_proto = EfiFormBrowserExtensionProtocol {
        set_scope: mock_set_scope,
        register_hot_key: mock_register_hot_key,
        register_exit_handler: mock_register_exit_handler,
    };

    let fb = unsafe { FormBrowserExtension::from_raw(&raw const mock_proto) };
    assert!(fb.set_scope(BrowserSettingScope::FormLevel).is_ok());
    let recorded_scope = match SCOPE_RECORDED.lock() {
        Ok(guard) => *guard,
        Err(_) => None,
    };
    assert_eq!(recorded_scope, Some(BrowserSettingScope::FormLevel));

    let key = EfiInputKey {
        scan_code: 0x01,
        unicode_char: 0,
    };
    assert!(
        fb.register_hot_key(&key, BROWSER_ACTION_DISCARD, 0, None)
            .is_ok()
    );
    assert_eq!(HOTKEY_ACTION.load(Ordering::SeqCst), BROWSER_ACTION_DISCARD);

    assert!(fb.register_exit_handler(Some(sample_exit_cb)).is_ok());
    assert!(EXIT_HANDLER_CALLED.load(Ordering::SeqCst));
}

#[test]
fn test_form_browser_extension2_wrapper() {
    let mock_proto = EfiFormBrowserExtension2Protocol {
        version: BROWSER_EXTENSION2_VERSION_1_1,
        set_scope: mock_set_scope,
        register_hot_key: mock_register_hot_key,
        register_exit_handler: mock_register_exit_handler,
        is_browser_data_modified: mock_is_modified,
        execute_action: mock_execute_action,
        is_reset_required: mock_is_reset_required,
    };

    let fb2 = unsafe { FormBrowserExtension2::from_raw(&raw const mock_proto) };
    assert_eq!(fb2.version(), BROWSER_EXTENSION2_VERSION_1_1);
    assert!(fb2.is_browser_data_modified());
    assert!(!fb2.is_reset_required());
    assert!(fb2.execute_action(BROWSER_ACTION_RESET, 0).is_ok());
    assert_eq!(EXEC_ACTION.load(Ordering::SeqCst), BROWSER_ACTION_RESET);
}
