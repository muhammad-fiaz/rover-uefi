// SPDX-License-Identifier: Apache-2.0
//! Integration tests for MM/SMM Base protocol.

use core::ffi::c_void;

use rover_uefi::abi::types::Bool;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::firmware::{
    EfiMmBaseProtocol, MM_BASE_PROTOCOL_GUID, MmBaseProtocol, SMM_BASE2_PROTOCOL_GUID,
};

static mut MOCK_IN_MMRAM: Bool = 1;
static mut MOCK_MMST_PTR: usize = 0xDEAD_BEEF;

unsafe extern "efiapi" fn mock_in_mm(
    _this: *const EfiMmBaseProtocol,
    in_mmram: *mut Bool,
) -> EfiStatus {
    if in_mmram.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *in_mmram = MOCK_IN_MMRAM;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_mmst(
    _this: *const EfiMmBaseProtocol,
    mmst: *mut *mut c_void,
) -> EfiStatus {
    if mmst.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *mmst = MOCK_MMST_PTR as *mut c_void;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_mm_base_protocol() {
    assert_eq!(MM_BASE_PROTOCOL_GUID, SMM_BASE2_PROTOCOL_GUID);
    assert_eq!(
        MM_BASE_PROTOCOL_GUID.to_u128_le(),
        0x91c1_50f1_a810_d49d_47fd_f6e0_f4cc_bfb7
    );

    let raw = EfiMmBaseProtocol {
        in_mm: mock_in_mm,
        get_mmst_location: mock_get_mmst,
    };
    let mm_base = MmBaseProtocol::new(&raw);

    let in_mm = mm_base.in_mm();
    let Ok(is_in_mm) = in_mm else {
        panic!("expected success");
    };
    assert!(is_in_mm);

    let mmst = mm_base.get_mmst_location();
    let Ok(mmst_ptr) = mmst else {
        panic!("expected success");
    };
    assert_eq!(mmst_ptr as usize, 0xDEAD_BEEF);
}
