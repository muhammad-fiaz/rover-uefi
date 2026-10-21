//! Network enumeration tests.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::network::guid::{
    IP4_GUID, IP6_GUID, MANAGED_NETWORK_GUID, SIMPLE_NETWORK_GUID,
};

#[test]
fn enumeration_guids_are_distinct_and_nonzero() {
    let guids = [
        SIMPLE_NETWORK_GUID,
        MANAGED_NETWORK_GUID,
        IP4_GUID,
        IP6_GUID,
    ];
    for (index, guid) in guids.iter().enumerate() {
        assert!(!guid.is_zero());
        assert_eq!(EfiGuid::from_bytes(guid.to_bytes()), *guid);
        for (other_index, other) in guids.iter().enumerate() {
            if index != other_index {
                assert_ne!(guid, other);
            }
        }
    }
}
