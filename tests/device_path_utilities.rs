//! Tests for the device path length and instance helpers.
use rover_uefi::protocols::device_path::utilities::{
    MAX_DEVICE_PATH_SIZE, checked_node_end, device_path_instances,
};

/// A media node of six bytes followed by an end node.
fn instance(end_subtype: u8) -> [u8; 10] {
    [
        0x03,
        0x01,
        0x06,
        0x00,
        0xAA,
        0xBB, // node of six bytes
        0x7F,
        end_subtype,
        0x04,
        0x00, // end node
    ]
}

#[test]
fn node_end_is_checked() {
    assert_eq!(checked_node_end(0, 4), Some(4));
    assert_eq!(checked_node_end(10, 6), Some(16));
    assert_eq!(checked_node_end(0, 3), None);
    assert_eq!(checked_node_end(usize::MAX, 4), None);
    assert_eq!(checked_node_end(usize::MAX - 3, 4), None);
    assert_eq!(checked_node_end(usize::MAX - 4, 4), Some(usize::MAX));
}

#[test]
fn maximum_path_size_is_the_uefi_limit() {
    assert_eq!(MAX_DEVICE_PATH_SIZE, 65_535);
}

#[test]
fn instances_are_split_at_end_instance_nodes() {
    let mut path = [0_u8; 20];
    let first = instance(0x01);
    let second = instance(0xFF);
    path[..10].copy_from_slice(&first);
    path[10..].copy_from_slice(&second);

    let instances: Vec<&[u8]> = device_path_instances(&path).collect();
    assert_eq!(instances.len(), 2);
    assert_eq!(instances[0], &first);
    assert_eq!(instances[1], &second);
}

#[test]
fn empty_path_yields_no_instance() {
    assert_eq!(device_path_instances(&[]).count(), 0);
}

#[test]
fn truncated_header_stops_iteration() {
    assert_eq!(device_path_instances(&[0x03, 0x01, 0x06]).count(), 0);
}

#[test]
fn invalid_node_length_stops_iteration() {
    // A node that claims to be shorter than its header is malformed.
    let malformed = [0x03_u8, 0x01, 0x03, 0x00, 0xAA, 0xBB];
    assert_eq!(device_path_instances(&malformed).count(), 0);
}

#[test]
fn node_longer_than_the_buffer_stops_iteration() {
    let overflowing = [0x03_u8, 0x01, 0xFF, 0xFF, 0xAA, 0xBB];
    assert_eq!(device_path_instances(&overflowing).count(), 0);
}

#[test]
fn unterminated_path_yields_the_partial_instance() {
    // A complete node without an end node is still returned, so callers
    // can reject the malformed path themselves.
    let path = [0x03_u8, 0x01, 0x06, 0x00, 0xAA, 0xBB];
    let instances: Vec<&[u8]> = device_path_instances(&path).collect();
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0], &path);
}
