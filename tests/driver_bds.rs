// SPDX-License-Identifier: Apache-2.0
//! Integration tests for BDS architectural protocol.

use rover_uefi::protocols::driver::{
    BdsArchProtocol, EFI_BDS_ARCH_PROTOCOL_GUID, EfiBdsArchProtocol,
};

static mut BDS_CALLED: bool = false;

unsafe extern "efiapi" fn mock_bds_entry(_this: *mut EfiBdsArchProtocol) {
    unsafe {
        BDS_CALLED = true;
    }
}

#[test]
fn test_bds_arch_protocol() {
    let raw = EfiBdsArchProtocol {
        entry: mock_bds_entry,
    };
    let bds = BdsArchProtocol::new(&raw);

    assert_eq!(
        EFI_BDS_ARCH_PROTOCOL_GUID.to_u128_le(),
        0x4dc1_3f27_9000_389a_11d4_46cc_665e_3ff6
    );

    unsafe {
        bds.entry();
    }
    assert!(unsafe { BDS_CALLED });
}
