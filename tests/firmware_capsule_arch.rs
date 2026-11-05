// SPDX-License-Identifier: Apache-2.0
//! Integration tests for Capsule architectural protocol.

use rover_uefi::protocols::firmware::EFI_CAPSULE_ARCH_PROTOCOL_GUID;

#[test]
fn test_capsule_arch_guid() {
    assert_eq!(
        EFI_CAPSULE_ARCH_PROTOCOL_GUID.to_u128_le(),
        0x5457_eede_8005_d990_4819_2cbc_5053_697e
    );
}
