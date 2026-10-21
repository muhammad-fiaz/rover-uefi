//! Tests for the native `IGMP` message encoders and decoders.

use rover_uefi::protocols::network::stack::{StackError, igmp};
use rover_uefi::protocols::network::types::Ipv4Address;

fn group(last: u8) -> Ipv4Address {
    Ipv4Address([224, 0, 0, last])
}

fn sources(values: [u8; 2]) -> [u8; 8] {
    let mut bytes = [0_u8; 8];
    bytes[0] = 10;
    bytes[3] = values[0];
    bytes[4] = 10;
    bytes[7] = values[1];
    bytes
}

#[test]
fn query_roundtrip() {
    let mut buffer = [0_u8; 32];
    let Ok(len) = igmp::build_query(&mut buffer, group(1), 100) else {
        panic!("query did not build");
    };
    assert_eq!(len, igmp::IGMP_HEADER_LEN);
    assert!(igmp::verify_checksum(&buffer[..len]));

    let Some(message) = igmp::IgmpMessage::parse(&buffer[..len]) else {
        panic!("query did not parse");
    };
    assert_eq!(message.message_type(), igmp::IGMP_MEMBERSHIP_QUERY);
    let igmp::IgmpMessage::Query(query) = message else {
        panic!("query variant expected");
    };
    assert_eq!(query.group, group(1));
    assert_eq!(query.max_response_time_millis(), 10_000);
    assert_eq!(query.source_count(), 0);
}

#[test]
fn max_response_time_encoding() {
    let query = igmp::IgmpQuery {
        max_response_code: 0,
        group: Ipv4Address([0; 4]),
        suppress_router_processing: false,
        robustness: 0,
        query_interval_code: 0,
        sources: &[],
    };
    assert_eq!(query.max_response_time_millis(), 0);

    let query = igmp::IgmpQuery {
        max_response_code: 0x7F,
        ..query
    };
    assert_eq!(query.max_response_time_millis(), 12_700);

    // 0xFF is mantissa 0x0F with exponent 7: (0x0F | 0x10) << 10.
    let query = igmp::IgmpQuery {
        max_response_code: 0xFF,
        ..query
    };
    assert_eq!(query.max_response_time_millis(), 3_174_400);
}

#[test]
fn v3_query_fields_and_sources() {
    let mut buffer = [0_u8; 64];
    buffer[0] = igmp::IGMP_MEMBERSHIP_QUERY;
    buffer[1] = 20;
    buffer[4..8].copy_from_slice(&group(2).0);
    buffer[8] = 0x0A;
    buffer[9] = 125;
    buffer[12..16].copy_from_slice(&[10, 0, 0, 5]);
    let _ = igmp::write_checksum(&mut buffer[..16]);

    let Some(message) = igmp::IgmpMessage::parse(&buffer[..16]) else {
        panic!("version 3 query did not parse");
    };
    let igmp::IgmpMessage::Query(query) = message else {
        panic!("query variant expected");
    };
    assert!(query.suppress_router_processing);
    assert_eq!(query.robustness, 2);
    assert_eq!(query.query_interval_code, 125);
    assert_eq!(query.source_count(), 1);
}

#[test]
fn report_and_leave_roundtrip() {
    let mut buffer = [0_u8; 32];
    let Ok(len) = igmp::build_v2_report(&mut buffer, group(3)) else {
        panic!("report did not build");
    };
    assert!(igmp::verify_checksum(&buffer[..len]));
    let Some(message) = igmp::IgmpMessage::parse(&buffer[..len]) else {
        panic!("report did not parse");
    };
    assert_eq!(message, igmp::IgmpMessage::V2Report(group(3)));
    assert_eq!(message.message_type(), igmp::IGMP_V2_MEMBERSHIP_REPORT);

    let Ok(len) = igmp::build_leave_group(&mut buffer, group(4)) else {
        panic!("leave did not build");
    };
    let Some(message) = igmp::IgmpMessage::parse(&buffer[..len]) else {
        panic!("leave did not parse");
    };
    assert_eq!(message, igmp::IgmpMessage::LeaveGroup(group(4)));
    assert_eq!(message.message_type(), igmp::IGMP_V2_LEAVE_GROUP);
}

#[test]
fn v3_report_roundtrip() {
    let one = sources([7, 8]);
    let two = sources([9, 10]);
    let records = [
        igmp::GroupRecord {
            record_type: igmp::RECORD_CHANGE_TO_EXCLUDE,
            multicast_address: group(5),
            sources: &one,
            auxiliary: &[],
        },
        igmp::GroupRecord {
            record_type: igmp::RECORD_ALLOW_NEW_SOURCES,
            multicast_address: group(6),
            sources: &two,
            auxiliary: &[0xDE, 0xAD, 0xBE, 0xEF],
        },
    ];
    let mut buffer = [0_u8; 128];
    let Ok(len) = igmp::build_v3_report(&mut buffer, &records) else {
        panic!("version 3 report did not build");
    };
    assert!(igmp::verify_checksum(&buffer[..len]));

    let Some(message) = igmp::IgmpMessage::parse(&buffer[..len]) else {
        panic!("version 3 report did not parse");
    };
    assert_eq!(message.message_type(), igmp::IGMP_V3_MEMBERSHIP_REPORT);
    let igmp::IgmpMessage::V3Report(report) = message else {
        panic!("report variant expected");
    };
    let mut count = 0;
    for record in report.records {
        assert_eq!(record, records[count]);
        count += 1;
    }
    assert_eq!(count, 2);
}

#[test]
fn truncated_v3_report_is_rejected() {
    let one = sources([1, 2]);
    let records = [igmp::GroupRecord {
        record_type: igmp::RECORD_MODE_IS_INCLUDE,
        multicast_address: group(7),
        sources: &one,
        auxiliary: &[],
    }];
    let mut buffer = [0_u8; 64];
    let Ok(len) = igmp::build_v3_report(&mut buffer, &records) else {
        panic!("version 3 report did not build");
    };
    // Claim two records while only one is present.
    buffer[6..8].copy_from_slice(&2_u16.to_be_bytes());
    assert!(igmp::IgmpMessage::parse(&buffer[..len]).is_none());
}

#[test]
fn unknown_type_is_rejected() {
    let mut buffer = [0_u8; 8];
    buffer[0] = 0x30;
    assert!(igmp::IgmpMessage::parse(&buffer).is_none());
}

#[test]
fn short_message_is_rejected() {
    let buffer = [0_u8; 4];
    assert!(igmp::IgmpMessage::parse(&buffer).is_none());
    assert!(!igmp::verify_checksum(&buffer));
}

#[test]
fn checksum_detects_tampering() {
    let mut buffer = [0_u8; 8];
    let Ok(len) = igmp::build_v2_report(&mut buffer, group(8)) else {
        panic!("report did not build");
    };
    buffer[len - 1] ^= 0x01;
    assert!(!igmp::verify_checksum(&buffer[..len]));
}

#[test]
fn buffer_too_small_is_reported() {
    let mut small = [0_u8; 4];
    assert_eq!(
        igmp::build_v2_report(&mut small, group(1)),
        Err(StackError::BufferTooSmall)
    );
    let records = [igmp::GroupRecord {
        record_type: igmp::RECORD_MODE_IS_INCLUDE,
        multicast_address: group(1),
        sources: &[],
        auxiliary: &[],
    }];
    assert_eq!(
        igmp::build_v3_report(&mut small, &records),
        Err(StackError::BufferTooSmall)
    );
}
