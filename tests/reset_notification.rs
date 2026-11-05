// SPDX-License-Identifier: Apache-2.0
//! Unit tests for UEFI 2.7 Reset Notification Protocol.

use core::mem::size_of;
use rover_uefi::base::{EfiStatus, ResetType};
use rover_uefi::protocols::core::{
    EfiResetNotificationProtocol, RESET_NOTIFICATION_PROTOCOL_GUID, ResetNotificationProtocol,
};
use rover_uefi::tables::EfiResetSystem;

static mut RESET_CALLED: bool = false;

unsafe extern "efiapi" fn mock_reset_fn(
    _reset_type: ResetType,
    _status: EfiStatus,
    _data_size: rover_uefi::abi::UIntN,
    _reset_data: *mut core::ffi::c_void,
) {
    unsafe { RESET_CALLED = true };
}

unsafe extern "efiapi" fn mock_register_reset_notify(
    _this: *mut EfiResetNotificationProtocol,
    _reset_function: EfiResetSystem,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unregister_reset_notify(
    _this: *mut EfiResetNotificationProtocol,
    _reset_function: EfiResetSystem,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn guid_and_layout() {
    assert_eq!(
        RESET_NOTIFICATION_PROTOCOL_GUID.to_bytes(),
        [
            0xe0, 0x4a, 0xa3, 0x9d, 0xf9, 0xea, 0xbf, 0x4b, 0x8e, 0xc3, 0xfd, 0x60, 0x22, 0x6c,
            0x44, 0xbe
        ]
    );
    assert_eq!(
        size_of::<EfiResetNotificationProtocol>(),
        size_of::<usize>() * 2
    );
}

#[test]
fn protocol_wrapper() {
    let mut raw = EfiResetNotificationProtocol {
        register_reset_notify: mock_register_reset_notify,
        unregister_reset_notify: mock_unregister_reset_notify,
    };

    let Some(mut proto) = (unsafe { ResetNotificationProtocol::from_raw(&raw mut raw) }) else {
        panic!("expected protocol wrapper");
    };
    assert_eq!(proto.register(mock_reset_fn), EfiStatus::SUCCESS);
    assert_eq!(proto.unregister(mock_reset_fn), EfiStatus::SUCCESS);
}
