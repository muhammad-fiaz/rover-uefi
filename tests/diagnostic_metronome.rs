// SPDX-License-Identifier: Apache-2.0
//! Integration tests for Metronome architectural protocol.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::diagnostic::{
    EfiMetronomeArchProtocol, METRONOME_ARCH_PROTOCOL_GUID, MetronomeArch,
};

static mut LAST_TICK_NUMBER: u32 = 0;

unsafe extern "efiapi" fn mock_wait_for_tick(
    _this: *mut EfiMetronomeArchProtocol,
    tick_number: u32,
) -> EfiStatus {
    unsafe {
        LAST_TICK_NUMBER = tick_number;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_metronome_arch_protocol() {
    assert_eq!(
        METRONOME_ARCH_PROTOCOL_GUID.to_u128_le(),
        0x8188_3cc7_8000_e7bc_11d4_6f42_26ba_ccb2
    );

    let raw = EfiMetronomeArchProtocol {
        wait_for_tick: mock_wait_for_tick,
        tick_period: 100, // 10 microseconds (100 * 100ns)
    };
    let metronome = MetronomeArch::new(&raw);

    assert_eq!(metronome.tick_period(), 100);

    let result = metronome.wait_for_tick(50);
    assert!(result.is_ok());
    assert_eq!(unsafe { LAST_TICK_NUMBER }, 50);
}
