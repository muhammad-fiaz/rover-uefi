//! TCG event log tests.

use rover_uefi::protocols::security::tcg2::event_log::{
    agile_event_iter, is_valid_legacy_log, legacy_event_iter, parse_agile_event, parse_legacy_event,
};

fn build_legacy(pcr: u32, event_type: u32, payload: &[u8]) -> AllocVec {
    AllocVec::from_parts(pcr, event_type, payload)
}

struct AllocVec {
    data: Vec<u8>,
}

impl AllocVec {
    fn from_parts(pcr: u32, event_type: u32, payload: &[u8]) -> Self {
        let mut data = Vec::new();
        data.extend_from_slice(&pcr.to_le_bytes());
        data.extend_from_slice(&event_type.to_le_bytes());
        data.extend_from_slice(&[0xAA_u8; 20]);
        data.extend_from_slice(&(u32::try_from(payload.len()).unwrap_or(u32::MAX)).to_le_bytes());
        data.extend_from_slice(payload);
        Self { data }
    }
}

#[test]
fn legacy_parses_valid_event() {
    let log = build_legacy(7, 0x8000_0001, b"hello");
    let Some(event) = parse_legacy_event(&log.data) else {
        panic!("expected legacy event");
    };
    assert_eq!(event.pcr_index, 7);
    assert_eq!(event.event_type, 0x8000_0001);
    assert_eq!(event.digest.len(), 20);
    assert_eq!(event.event, b"hello");
    assert_eq!(event.total_size, 37);
}

#[test]
fn legacy_rejects_truncated_and_invalid() {
    assert!(parse_legacy_event(&[]).is_none());
    assert!(parse_legacy_event(&[0_u8; 10]).is_none());
    let mut bad_pcr = build_legacy(24, 1, b"x");
    assert!(parse_legacy_event(&bad_pcr.data).is_none());
    bad_pcr.data.truncate(20);
    assert!(parse_legacy_event(&bad_pcr.data).is_none());
    let overflow = build_legacy(0, 1, &[0xFF; 4]);
    let mut truncated = overflow.data;
    truncated.truncate(34);
    assert!(parse_legacy_event(&truncated).is_none());
}

#[test]
fn legacy_iterates_two_events() {
    let first = build_legacy(0, 4, b"AB");
    let second = build_legacy(1, 5, b"C");
    let mut combined = first.data;
    combined.extend_from_slice(&second.data);
    let mut iter = legacy_event_iter(&combined);
    let Some(first_event) = iter.next() else {
        panic!("expected first");
    };
    assert_eq!(first_event.event, b"AB");
    let Some(second_event) = iter.next() else {
        panic!("expected second");
    };
    assert_eq!(second_event.pcr_index, 1);
    assert_eq!(second_event.event, b"C");
    assert!(iter.next().is_none());
    assert!(is_valid_legacy_log(&combined));
    assert!(!is_valid_legacy_log(&combined[..20]));
    assert!(!is_valid_legacy_log(&[]));
}

#[test]
fn agile_parses_single_digest() {
    let mut data = Vec::new();
    data.extend_from_slice(&2_u32.to_le_bytes());
    data.extend_from_slice(&0x8000_0003_u32.to_le_bytes());
    data.extend_from_slice(&1_u32.to_le_bytes());
    data.extend_from_slice(&0x000B_u16.to_le_bytes());
    data.extend_from_slice(&[0x11_u8; 32]);
    data.extend_from_slice(&5_u32.to_le_bytes());
    data.extend_from_slice(b"event");
    let Some(event) = parse_agile_event(&data) else {
        panic!("expected agile event");
    };
    assert_eq!(event.pcr_index, 2);
    assert_eq!(event.digest_count, 1);
    assert_eq!(event.event, b"event");
    assert_eq!(event.total_size, 12 + 2 + 32 + 4 + 5);
}

#[test]
fn agile_parses_two_digests_and_rejects_unknown() {
    let mut data = Vec::new();
    data.extend_from_slice(&0_u32.to_le_bytes());
    data.extend_from_slice(&1_u32.to_le_bytes());
    data.extend_from_slice(&2_u32.to_le_bytes());
    data.extend_from_slice(&0x0004_u16.to_le_bytes());
    data.extend_from_slice(&[0x22_u8; 20]);
    data.extend_from_slice(&0x000D_u16.to_le_bytes());
    data.extend_from_slice(&[0x33_u8; 64]);
    data.extend_from_slice(&0_u32.to_le_bytes());
    let Some(event) = parse_agile_event(&data) else {
        panic!("expected two digests");
    };
    assert_eq!(event.digest_count, 2);
    let mut unknown = Vec::new();
    unknown.extend_from_slice(&0_u32.to_le_bytes());
    unknown.extend_from_slice(&1_u32.to_le_bytes());
    unknown.extend_from_slice(&1_u32.to_le_bytes());
    unknown.extend_from_slice(&0x9999_u16.to_le_bytes());
    unknown.extend_from_slice(&0_u32.to_le_bytes());
    assert!(parse_agile_event(&unknown).is_none());
    let mut iter = agile_event_iter(&data);
    assert_eq!(iter.next().map(|item| item.digest_count), Some(2));
    assert!(iter.next().is_none());
}

#[test]
fn agile_rejects_empty_and_bad_count() {
    assert!(parse_agile_event(&[]).is_none());
    let mut data = Vec::new();
    data.extend_from_slice(&0_u32.to_le_bytes());
    data.extend_from_slice(&1_u32.to_le_bytes());
    data.extend_from_slice(&0_u32.to_le_bytes());
    data.extend_from_slice(&0_u32.to_le_bytes());
    assert!(parse_agile_event(&data).is_none());
    data[8] = 255;
    assert!(parse_agile_event(&data).is_none());
}
