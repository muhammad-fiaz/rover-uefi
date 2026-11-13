//! Integration tests for Source-Level Debug Agent transfer protocol.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::cast_possible_truncation)]

use rover_uefi::protocols::debug::agent::*;

#[test]
fn test_debug_agent_guid_and_constants() {
    assert_eq!(
        DEBUG_AGENT_GUID.to_string(),
        "865a5a9b-b85d-474c-8455-65d1be844be2"
    );

    assert_eq!(DEBUG_STARTING_SYMBOL_ATTACH, 0xFA);
    assert_eq!(DEBUG_STARTING_SYMBOL_NORMAL, 0xFE);
    assert_eq!(DEBUG_STARTING_SYMBOL_COMPRESS, 0xFC);
    assert_eq!(DEBUG_DATA_UPPER_LIMIT, 255);

    assert_eq!(DEBUG_COMMAND_RESET, 0x00);
    assert_eq!(DEBUG_COMMAND_GO, 0x01);
    assert_eq!(DEBUG_COMMAND_BREAK_CAUSE, 0x02);
    assert_eq!(DEBUG_COMMAND_OK, 0x80);
    assert_eq!(DEBUG_COMMAND_RESEND, 0x81);
}

#[test]
fn test_debug_packet_encode_decode_roundtrip() {
    let payload = b"Hello Target Debugger!";
    let mut buffer = [0_u8; 64];

    let bytes_written = encode_debug_packet(
        DEBUG_STARTING_SYMBOL_NORMAL,
        DEBUG_COMMAND_PRINT_MESSAGE,
        42,
        payload,
        &mut buffer,
    )
    .expect("packet should encode");

    assert_eq!(bytes_written, 6 + payload.len());

    let (hdr, dec_payload) =
        decode_debug_packet(&buffer[0..bytes_written]).expect("packet should decode");

    assert_eq!(hdr.start_symbol, DEBUG_STARTING_SYMBOL_NORMAL);
    assert_eq!(hdr.command, DEBUG_COMMAND_PRINT_MESSAGE);
    assert_eq!(hdr.length, bytes_written as u8);
    assert_eq!(hdr.sequence_no, 42);
    assert_eq!(dec_payload, payload);
}

#[test]
fn test_debug_packet_crc_corruption() {
    let payload = [0xAA, 0xBB, 0xCC, 0xDD];
    let mut buffer = [0_u8; 32];

    let bytes_written = encode_debug_packet(
        DEBUG_STARTING_SYMBOL_NORMAL,
        DEBUG_COMMAND_READ_MEMORY,
        1,
        &payload,
        &mut buffer,
    )
    .expect("packet should encode");

    // Corrupt one byte of payload
    buffer[7] ^= 0xFF;

    let res = decode_debug_packet(&buffer[0..bytes_written]);
    assert_eq!(res.unwrap_err(), DebugPacketError::CrcMismatch);
}
