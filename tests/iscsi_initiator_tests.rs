// SPDX-License-Identifier: Apache-2.0
//! Integration tests for iSCSI Initiator Name Protocol.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::network::{
    EFI_ISCSI_INITIATOR_NAME_PROTOCOL_GUID, EfiIScsiInitiatorNameProtocol,
    ISCSI_MAX_INITIATOR_NAME_LEN, IScsiInitiatorName,
};

static mut CURRENT_NAME: [u8; ISCSI_MAX_INITIATOR_NAME_LEN] = {
    let mut buf = [0u8; ISCSI_MAX_INITIATOR_NAME_LEN];
    let init = b"iqn.1998-01.com.rover:initiator-01\0";
    let mut i = 0;
    while i < init.len() {
        buf[i] = init[i];
        i += 1;
    }
    buf
};
static mut CURRENT_NAME_LEN: usize = 35; // 34 chars + null

unsafe extern "efiapi" fn mock_iscsi_get(
    _this: *mut EfiIScsiInitiatorNameProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut c_void,
) -> EfiStatus {
    if buffer_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let required = unsafe { CURRENT_NAME_LEN };
    let available = unsafe { *buffer_size };
    if buffer.is_null() || available < required {
        unsafe {
            *buffer_size = required as UIntN;
        }
        return EfiStatus::BUFFER_TOO_SMALL;
    }
    unsafe {
        let name_src = core::ptr::addr_of!(CURRENT_NAME).cast::<u8>();
        core::ptr::copy_nonoverlapping(name_src, buffer.cast::<u8>(), required);
        *buffer_size = required as UIntN;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_iscsi_set(
    _this: *mut EfiIScsiInitiatorNameProtocol,
    buffer_size: *mut UIntN,
    buffer: *const c_void,
) -> EfiStatus {
    if buffer_size.is_null() || buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let size = unsafe { *buffer_size };
    if size >= ISCSI_MAX_INITIATOR_NAME_LEN {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        let name_dst = core::ptr::addr_of_mut!(CURRENT_NAME).cast::<u8>();
        core::ptr::copy_nonoverlapping(buffer.cast::<u8>(), name_dst, size);
        CURRENT_NAME_LEN = size;
    }
    EfiStatus::SUCCESS
}

#[test]
fn iscsi_initiator_guid_and_constants() {
    assert_eq!(
        EFI_ISCSI_INITIATOR_NAME_PROTOCOL_GUID,
        EfiGuid::new(
            0x5932_4945,
            0xEC44,
            0x4C0D,
            [0xB1, 0xCD, 0x9D, 0xB1, 0x39, 0xDF, 0x07, 0x0C]
        )
    );
    assert_eq!(ISCSI_MAX_INITIATOR_NAME_LEN, 224);
}

#[test]
fn iscsi_initiator_client_wrapper() {
    let mut mock_proto = EfiIScsiInitiatorNameProtocol {
        get: mock_iscsi_get,
        set: mock_iscsi_set,
    };

    let mut client = IScsiInitiatorName::new(&mut mock_proto);
    assert!(!client.raw().is_null());

    let mut buf = [0u8; 128];
    let len = client.get(&mut buf).unwrap_or(0);
    assert_eq!(len, 35);

    let name = client.get_name(&mut buf).unwrap_or("");
    assert_eq!(name, "iqn.1998-01.com.rover:initiator-01");

    // Set new initiator name
    let new_name = "iqn.2026-11.org.rover:target-boot-node";
    assert!(client.set(new_name).is_ok());

    let updated_name = client.get_name(&mut buf).unwrap_or("");
    assert_eq!(updated_name, new_name);

    // Buffer too small check
    let mut tiny_buf = [0u8; 8];
    assert_eq!(client.get(&mut tiny_buf), Err(EfiStatus::BUFFER_TOO_SMALL));

    // Name too long check
    let too_long = "a".repeat(250);
    assert_eq!(client.set(&too_long), Err(EfiStatus::INVALID_PARAMETER));
}
