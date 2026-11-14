// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Integration tests for PS/2 mouse controller and packet decoder.

use rover_uefi::abi::types::{EFI_FALSE, EFI_TRUE};
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::console::ps2_mouse::{
    MOUSE_CMD_ENABLE_STREAMING, MOUSE_CMD_GET_DEVICE_ID, MOUSE_CMD_RESET, MOUSE_CMD_SET_RESOLUTION,
    MOUSE_CMD_SET_SAMPLE_RATE, MOUSE_ID_5_BUTTON, MOUSE_ID_INTELLIMOUSE, MOUSE_ID_STANDARD,
    MOUSE_RESP_ACK, MOUSE_RESP_BAT1, MOUSE_RESP_BAT2, MouseResolution, MouseSampleRate,
    PS2_MOUSE_FLAG_LEFT_BUTTON, PS2_MOUSE_FLAG_RIGHT_BUTTON, PS2_MOUSE_FLAG_SYNC,
    PS2_MOUSE_FLAG_X_OVERFLOW, PS2_MOUSE_FLAG_X_SIGN, PS2_MOUSE_FLAG_Y_SIGN, Ps2MouseDecoder,
    Ps2MouseError,
};

#[test]
fn test_ps2_mouse_command_constants() {
    assert_eq!(MOUSE_CMD_SET_RESOLUTION, 0xE8);
    assert_eq!(MOUSE_CMD_GET_DEVICE_ID, 0xF2);
    assert_eq!(MOUSE_CMD_SET_SAMPLE_RATE, 0xF3);
    assert_eq!(MOUSE_CMD_ENABLE_STREAMING, 0xF4);
    assert_eq!(MOUSE_CMD_RESET, 0xFF);

    assert_eq!(MOUSE_RESP_BAT1, 0xAA);
    assert_eq!(MOUSE_RESP_BAT2, 0x00);
    assert_eq!(MOUSE_RESP_ACK, 0xFA);

    assert_eq!(MOUSE_ID_STANDARD, 0x00);
    assert_eq!(MOUSE_ID_INTELLIMOUSE, 0x03);
    assert_eq!(MOUSE_ID_5_BUTTON, 0x04);
}

#[test]
fn test_ps2_mouse_sample_rates_and_resolutions() {
    assert_eq!(MouseSampleRate::Rate10.command_byte(), 10);
    assert_eq!(MouseSampleRate::Rate100.command_byte(), 100);
    assert_eq!(MouseSampleRate::Rate200.command_byte(), 200);

    assert_eq!(MouseResolution::Res1CountPerMm.counts_per_mm(), 1);
    assert_eq!(MouseResolution::Res4CountPerMm.counts_per_mm(), 4);
    assert_eq!(MouseResolution::Res8CountPerMm.counts_per_mm(), 8);
}

#[test]
fn test_ps2_mouse_standard_packet_decoding() {
    let mut decoder = Ps2MouseDecoder::new();

    // Standard 3-byte packet: Left button clicked, dX = +10, dY = +5
    let b0 = PS2_MOUSE_FLAG_SYNC | PS2_MOUSE_FLAG_LEFT_BUTTON;
    let b1 = 10u8;
    let b2 = 5u8;

    assert_eq!(decoder.feed_byte(b0), Ok(None));
    assert_eq!(decoder.feed_byte(b1), Ok(None));
    let state = decoder.feed_byte(b2).unwrap().unwrap();

    assert_eq!(state.relative_movement_x, 10);
    // Y is inverted for screen coordinates (positive Y is down)
    assert_eq!(state.relative_movement_y, -5);
    assert_eq!(state.relative_movement_z, 0);
    assert_eq!(state.left_button, EFI_TRUE);
    assert_eq!(state.right_button, EFI_FALSE);

    // Read state via get_state()
    let retrieved = decoder.get_state().unwrap();
    assert_eq!(retrieved.relative_movement_x, 10);
    assert_eq!(retrieved.relative_movement_y, -5);

    // Subsequent get_state() without changes should return NOT_READY
    assert_eq!(decoder.get_state(), Err(EfiStatus::NOT_READY));
}

#[test]
fn test_ps2_mouse_negative_displacement() {
    let mut decoder = Ps2MouseDecoder::new();

    // Negative movement: dX = -10 (0xF6), dY = -20 (0xEC)
    // Sign bits set in byte 0
    let b0 = PS2_MOUSE_FLAG_SYNC
        | PS2_MOUSE_FLAG_X_SIGN
        | PS2_MOUSE_FLAG_Y_SIGN
        | PS2_MOUSE_FLAG_RIGHT_BUTTON;
    let b1 = 0xF6u8; // 246 unsigned -> -10 signed
    let b2 = 0xECu8; // 236 unsigned -> -20 signed

    decoder.feed_byte(b0).unwrap();
    decoder.feed_byte(b1).unwrap();
    let state = decoder.feed_byte(b2).unwrap().unwrap();

    assert_eq!(state.relative_movement_x, -10);
    // Inverted Y: -(-20) = +20
    assert_eq!(state.relative_movement_y, 20);
    assert_eq!(state.left_button, EFI_FALSE);
    assert_eq!(state.right_button, EFI_TRUE);
}

#[test]
fn test_ps2_mouse_sync_and_overflow_errors() {
    let mut decoder = Ps2MouseDecoder::new();

    // Byte 0 missing sync bit 3 -> OutOfSync error
    let invalid_b0 = 0x01; // Bit 3 is 0
    assert_eq!(decoder.feed_byte(invalid_b0), Err(Ps2MouseError::OutOfSync));

    // Byte 0 with X overflow flag -> DisplacementOverflow error
    let overflow_b0 = PS2_MOUSE_FLAG_SYNC | PS2_MOUSE_FLAG_X_OVERFLOW;
    decoder.feed_byte(overflow_b0).unwrap();
    decoder.feed_byte(0x50).unwrap();
    assert_eq!(
        decoder.feed_byte(0x50),
        Err(Ps2MouseError::DisplacementOverflow)
    );
}

#[test]
fn test_ps2_mouse_intellimouse_wheel() {
    let mut decoder = Ps2MouseDecoder::new();
    decoder.set_intellimouse_mode();

    // IntelliMouse 4-byte packet: dX = 2, dY = 3, dZ = +1 (wheel scroll up)
    let b0 = PS2_MOUSE_FLAG_SYNC;
    let b1 = 2u8;
    let b2 = 3u8;
    let b3 = 0x01u8; // Z = +1

    assert_eq!(decoder.feed_byte(b0), Ok(None));
    assert_eq!(decoder.feed_byte(b1), Ok(None));
    assert_eq!(decoder.feed_byte(b2), Ok(None));
    let state = decoder.feed_byte(b3).unwrap().unwrap();

    assert_eq!(state.relative_movement_x, 2);
    assert_eq!(state.relative_movement_y, -3);
    assert_eq!(state.relative_movement_z, 1);

    // Negative wheel scroll: dZ = -1 (low 4-bit 2's complement: 0x0F)
    decoder.feed_byte(PS2_MOUSE_FLAG_SYNC).unwrap();
    decoder.feed_byte(0).unwrap();
    decoder.feed_byte(0).unwrap();
    let state2 = decoder.feed_byte(0x0F).unwrap().unwrap();

    // Accumulated Z: 1 + (-1) = 0
    assert_eq!(state2.relative_movement_z, 0);
}
