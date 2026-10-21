//! Managed network tests.

use core::mem::{offset_of, size_of};

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::network::guid::MANAGED_NETWORK_GUID;
use rover_uefi::protocols::network::managed::protocol::EfiManagedNetworkProtocol;

#[test]
fn guid_bytes() {
    assert_eq!(
        MANAGED_NETWORK_GUID.to_bytes(),
        [
            0x91, 0x3A, 0xB3, 0x7A, 0xE5, 0xAC, 0x26, 0x43, 0xB5, 0x72, 0xE7, 0xEE, 0x33, 0xD3,
            0x9F, 0x16
        ]
    );
    assert_eq!(
        EfiGuid::from_bytes(MANAGED_NETWORK_GUID.to_bytes()),
        MANAGED_NETWORK_GUID
    );
}

#[test]
fn protocol_has_eight_entries() {
    assert_eq!(
        size_of::<EfiManagedNetworkProtocol>(),
        8 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiManagedNetworkProtocol, poll),
        7 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        offset_of!(EfiManagedNetworkProtocol, configure),
        size_of::<usize>()
    );
}
