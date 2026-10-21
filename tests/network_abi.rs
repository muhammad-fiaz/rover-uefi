//! Network ABI tests.

use core::mem::{offset_of, size_of};

use rover_uefi::protocols::network::dns::protocol::{EfiDns4Protocol, EfiDns6Protocol};
use rover_uefi::protocols::network::http::protocol::EfiHttpProtocol;
use rover_uefi::protocols::network::ip_protocol::{EfiIp4Protocol, EfiIp6Protocol};
use rover_uefi::protocols::network::managed::protocol::EfiManagedNetworkProtocol;
use rover_uefi::protocols::network::pxe::protocol::EfiPxeBaseCodeProtocol;
use rover_uefi::protocols::network::simple_network::EfiSimpleNetworkProtocol;
use rover_uefi::protocols::network::simple_network::mode::EfiSimpleNetworkMode;
use rover_uefi::protocols::network::tcp::protocol::{EfiTcp4Protocol, EfiTcp6Protocol};
use rover_uefi::protocols::network::udp::protocol::{EfiUdp4Protocol, EfiUdp6Protocol};

fn pointer_size() -> usize {
    size_of::<usize>()
}

#[test]
fn simple_network_size() {
    assert_eq!(
        size_of::<EfiSimpleNetworkProtocol>(),
        8 + 15 * pointer_size()
    );
    assert_eq!(offset_of!(EfiSimpleNetworkProtocol, revision), 0);
    assert_eq!(offset_of!(EfiSimpleNetworkProtocol, start), 8);
    assert_eq!(
        offset_of!(EfiSimpleNetworkProtocol, transmit),
        8 + 11 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiSimpleNetworkProtocol, wait_for_packet),
        8 + 13 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiSimpleNetworkProtocol, mode),
        8 + 14 * pointer_size()
    );
    assert_eq!(size_of::<EfiSimpleNetworkMode>(), 160);
}

#[test]
fn managed_ip_udp_sizes() {
    assert_eq!(size_of::<EfiManagedNetworkProtocol>(), 8 * pointer_size());
    assert_eq!(size_of::<EfiIp4Protocol>(), 8 * pointer_size());
    assert_eq!(size_of::<EfiIp6Protocol>(), 9 * pointer_size());
    assert_eq!(size_of::<EfiUdp4Protocol>(), 8 * pointer_size());
    assert_eq!(size_of::<EfiUdp6Protocol>(), 7 * pointer_size());
}

#[test]
fn tcp_dns_http_sizes() {
    assert_eq!(size_of::<EfiTcp4Protocol>(), 10 * pointer_size());
    assert_eq!(size_of::<EfiTcp6Protocol>(), 9 * pointer_size());
    assert_eq!(size_of::<EfiDns4Protocol>(), 8 * pointer_size());
    assert_eq!(size_of::<EfiDns6Protocol>(), 8 * pointer_size());
    assert_eq!(size_of::<EfiHttpProtocol>(), 6 * pointer_size());
}

#[test]
fn pxe_size() {
    assert_eq!(size_of::<EfiPxeBaseCodeProtocol>(), 8 + 13 * pointer_size());
    assert_eq!(offset_of!(EfiPxeBaseCodeProtocol, revision), 0);
    assert_eq!(
        offset_of!(EfiPxeBaseCodeProtocol, mode),
        8 + 12 * pointer_size()
    );
}
