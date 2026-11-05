// SPDX-License-Identifier: Apache-2.0
//! Integration tests for core architectural protocol GUIDs.

use rover_uefi::protocols::core::{
    MONOTONIC_COUNTER_ARCH_PROTOCOL_GUID, RESET_ARCH_PROTOCOL_GUID, VARIABLE_ARCH_PROTOCOL_GUID,
    VARIABLE_WRITE_ARCH_PROTOCOL_GUID,
};

#[test]
fn test_core_arch_guids() {
    assert_eq!(
        MONOTONIC_COUNTER_ARCH_PROTOCOL_GUID.to_u128_le(),
        0x2aff_6fb5_a072_f199_4b30_bddc_1da9_7072
    );
    assert_eq!(
        RESET_ARCH_PROTOCOL_GUID.to_u128_le(),
        0x4dc1_3f27_9000_389a_11d4_46cc_27cf_ac88
    );
    assert_eq!(
        VARIABLE_ARCH_PROTOCOL_GUID.to_u128_le(),
        0x8188_3cc7_8000_f1bc_11d4_8481_1e56_68e2
    );
    assert_eq!(
        VARIABLE_WRITE_ARCH_PROTOCOL_GUID.to_u128_le(),
        0x5324_dd31_ba7d_70b5_4e44_6362_6441_f818
    );
}
