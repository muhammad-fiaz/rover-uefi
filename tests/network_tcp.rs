//! TCP tests.

use core::mem::size_of;

use rover_uefi::protocols::network::guid::{TCP4_GUID, TCP6_GUID};
use rover_uefi::protocols::network::tcp::config::TcpConfig;
use rover_uefi::protocols::network::tcp::protocol::{EfiTcp4Protocol, EfiTcp6Protocol};

#[test]
fn tcp_config() {
    let config = TcpConfig { station_port: 80 };
    assert_eq!(config.station_port, 80);
    let zero = TcpConfig { station_port: 0 };
    assert_eq!(zero.station_port, 0);
    let max = TcpConfig {
        station_port: u16::MAX,
    };
    assert_eq!(max.station_port, 65535);
    assert_eq!(size_of::<TcpConfig>(), 2);
}

#[test]
fn tcp_protocol_sizes_and_guids() {
    assert_eq!(
        size_of::<EfiTcp4Protocol>(),
        10 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        size_of::<EfiTcp6Protocol>(),
        9 * core::mem::size_of::<usize>()
    );
    assert_ne!(TCP4_GUID, TCP6_GUID);
    assert_eq!(
        TCP4_GUID.to_bytes(),
        [
            0xC7, 0x0B, 0x53, 0x65, 0x59, 0xA3, 0x0F, 0x41, 0xB0, 0x10, 0x5A, 0xAD, 0xC7, 0xEC,
            0x2B, 0x62
        ]
    );
}
