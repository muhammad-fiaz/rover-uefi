//! UDP tests.

use core::mem::size_of;

use rover_uefi::protocols::network::guid::{UDP4_GUID, UDP6_GUID};
use rover_uefi::protocols::network::types::Ipv4Address;
use rover_uefi::protocols::network::udp::config::UdpConfig;
use rover_uefi::protocols::network::udp::protocol::{EfiUdp4Protocol, EfiUdp6Protocol};

#[test]
fn udp_config() {
    let config = UdpConfig {
        station_address: Ipv4Address([127, 0, 0, 1]),
        station_port: 53,
    };
    assert_eq!(config.station_port, 53);
    assert_eq!(config.station_address.0, [127, 0, 0, 1]);
    let zero = UdpConfig {
        station_address: Ipv4Address([0, 0, 0, 0]),
        station_port: 0,
    };
    assert_eq!(zero.station_port, 0);
    assert_eq!(size_of::<UdpConfig>(), 6);
}

#[test]
fn udp_protocol_sizes() {
    assert_eq!(
        size_of::<EfiUdp4Protocol>(),
        8 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiUdp6Protocol>(),
        7 * core::mem::size_of::<usize>()
    );
    assert_ne!(UDP4_GUID, UDP6_GUID);
    assert_eq!(
        UDP4_GUID.to_bytes(),
        [
            0x29, 0xDF, 0xD9, 0x3A, 0x01, 0x45, 0x8D, 0x47, 0xB1, 0xF8, 0x7F, 0x7F, 0xE7, 0x0E,
            0x50, 0xF3
        ]
    );
}
