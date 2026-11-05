// SPDX-License-Identifier: Apache-2.0
//! Unit tests for SMBIOS protocol.

use core::ffi::c_void;
use core::sync::atomic::{AtomicU16, Ordering};
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::smbios::header::SmbiosHeader;
use rover_uefi::protocols::smbios::{EFI_SMBIOS_PROTOCOL_GUID, EfiSmbiosProtocol, SmbiosProtocol};

static REMOVED_HANDLE: AtomicU16 = AtomicU16::new(0);

unsafe extern "efiapi" fn mock_remove(_this: *const EfiSmbiosProtocol, handle: u16) -> EfiStatus {
    REMOVED_HANDLE.store(handle, Ordering::SeqCst);
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_add(
    _this: *const EfiSmbiosProtocol,
    _producer: *const c_void,
    _handle: *mut u16,
    _record: *const SmbiosHeader,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_update_string(
    _this: *const EfiSmbiosProtocol,
    _handle: *mut u16,
    _str_num: *mut UIntN,
    _string: *const u8,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_get_next(
    _this: *const EfiSmbiosProtocol,
    _handle: *mut u16,
    _type: *mut u8,
    _record: *mut *mut SmbiosHeader,
    _producer: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn smbios_protocol_and_wrapper() {
    assert!(!EFI_SMBIOS_PROTOCOL_GUID.is_zero());

    let proto = EfiSmbiosProtocol {
        add: dummy_add,
        update_string: dummy_update_string,
        remove: mock_remove,
        get_next: dummy_get_next,
        major_version: 3,
        minor_version: 6,
    };

    let wrapper = SmbiosProtocol::new(&proto);
    assert_eq!(wrapper.version(), (3, 6));

    assert_eq!(wrapper.remove_record(0x100), Ok(()));
    assert_eq!(REMOVED_HANDLE.load(Ordering::SeqCst), 0x100);
}
