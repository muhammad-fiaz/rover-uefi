//! Integration tests for PS/2 Policy Protocol.
#![allow(clippy::expect_used)]

use rover_uefi::base::{EfiHandle, EfiStatus};
use rover_uefi::protocols::driver::ps2_policy::{
    KEYBOARD_LIGHT_CAPSLOCK, KEYBOARD_LIGHT_NUMLOCK, KEYBOARD_LIGHT_SCROLLLOCK,
    PS2_POLICY_PROTOCOL_GUID, Ps2Policy, Ps2PolicyProtocol,
};

static mut INIT_CALLED: bool = false;

unsafe extern "efiapi" fn mock_ps2_init_hardware(_handle: EfiHandle) -> EfiStatus {
    unsafe {
        INIT_CALLED = true;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_ps2_policy_constants() {
    assert_eq!(
        PS2_POLICY_PROTOCOL_GUID.to_string(),
        "4df19259-dc71-4d46-bef1-357bb578c418"
    );
    assert_eq!(KEYBOARD_LIGHT_SCROLLLOCK, 0x01);
    assert_eq!(KEYBOARD_LIGHT_NUMLOCK, 0x02);
    assert_eq!(KEYBOARD_LIGHT_CAPSLOCK, 0x04);
}

#[test]
fn test_ps2_policy_wrapper() {
    let mut raw_proto = Ps2PolicyProtocol {
        keyboard_light: KEYBOARD_LIGHT_NUMLOCK | KEYBOARD_LIGHT_CAPSLOCK,
        ps2_init_hardware: Some(mock_ps2_init_hardware),
    };

    let mut policy = unsafe { Ps2Policy::from_raw(&raw mut raw_proto) }.expect("valid ps2 policy");

    assert_eq!(
        policy.keyboard_light(),
        KEYBOARD_LIGHT_NUMLOCK | KEYBOARD_LIGHT_CAPSLOCK
    );
    assert!(policy.is_numlock_enabled());
    assert!(policy.is_capslock_enabled());
    assert!(!policy.is_scrolllock_enabled());

    policy.set_keyboard_light(KEYBOARD_LIGHT_SCROLLLOCK);
    assert!(policy.is_scrolllock_enabled());
    assert!(!policy.is_numlock_enabled());

    let init_res = policy.init_hardware(EfiHandle::NULL);
    assert!(init_res.is_ok());
    assert!(unsafe { INIT_CALLED });
}
