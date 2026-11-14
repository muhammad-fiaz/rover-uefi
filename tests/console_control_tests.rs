//! Integration tests for Console Control Protocol.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::console::console_control::*;

static CURRENT_MODE: AtomicU32 = AtomicU32::new(EfiConsoleControlScreenMode::Text as u32);
static STDIN_LOCKED: AtomicBool = AtomicBool::new(false);

unsafe extern "efiapi" fn mock_get_mode(
    _this: *const EfiConsoleControlProtocol,
    mode: *mut EfiConsoleControlScreenMode,
    gop_uva: *mut bool,
    std_in_locked: *mut bool,
) -> EfiStatus {
    if !mode.is_null() {
        let raw = CURRENT_MODE.load(Ordering::SeqCst);
        unsafe {
            *mode = if raw == 1 {
                EfiConsoleControlScreenMode::Graphics
            } else {
                EfiConsoleControlScreenMode::Text
            };
        }
    }
    if !gop_uva.is_null() {
        unsafe {
            *gop_uva = true;
        }
    }
    if !std_in_locked.is_null() {
        unsafe {
            *std_in_locked = STDIN_LOCKED.load(Ordering::SeqCst);
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_mode(
    _this: *const EfiConsoleControlProtocol,
    mode: EfiConsoleControlScreenMode,
) -> EfiStatus {
    CURRENT_MODE.store(mode as u32, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_lock_std_in(
    _this: *const EfiConsoleControlProtocol,
    _password: *const u16,
) -> EfiStatus {
    STDIN_LOCKED.store(true, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

#[test]
fn test_console_control_constants_and_guids() {
    assert_eq!(
        EFI_CONSOLE_CONTROL_PROTOCOL_GUID,
        EfiGuid::new(
            0xf42f_7782,
            0x012e,
            0x4c70,
            [0x99, 0xa6, 0x4b, 0xb7, 0x7d, 0xdd, 0xba, 0xbf]
        )
    );
    assert_eq!(ConsoleControl::GUID, EFI_CONSOLE_CONTROL_PROTOCOL_GUID);
}

#[test]
fn test_console_control_client_wrapper() {
    let mock_proto = EfiConsoleControlProtocol {
        get_mode: mock_get_mode,
        set_mode: mock_set_mode,
        lock_std_in: mock_lock_std_in,
    };
    let client = unsafe { ConsoleControl::from_raw(&raw const mock_proto) };

    let info = client.get_mode();
    assert!(info.is_ok());
    if let Ok(m) = info {
        assert_eq!(m.mode, EfiConsoleControlScreenMode::Text);
        assert!(m.gop_uva);
        assert!(!m.std_in_locked);
    }

    assert!(
        client
            .set_mode(EfiConsoleControlScreenMode::Graphics)
            .is_ok()
    );
    assert_eq!(CURRENT_MODE.load(Ordering::SeqCst), 1);

    let info2 = client.get_mode();
    assert!(info2.is_ok());
    if let Ok(m) = info2 {
        assert_eq!(m.mode, EfiConsoleControlScreenMode::Graphics);
    }

    let pass = [u16::from(b'1'), u16::from(b'2'), 0];
    assert!(client.lock_std_in(Some(&pass)).is_ok());
    assert!(STDIN_LOCKED.load(Ordering::SeqCst));
}
