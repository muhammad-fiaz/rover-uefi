//! # Network stack
//!
//! A modular, hardware-independent protocol stack.
//!
//! The stack is layered the same way the corresponding `IETF` RFCs
//! are: link layer (`ethernet`, `arp`), internet layer (`ipv4`,
//! `ipv6`, `icmp`, `igmp`, `ndp`), transport layer (`udp`, `tcp`) and
//! application layer (`dhcp`, `dns_client`). Routing lives in `route`.
//!
//! Every module is free of dynamic memory. Output buffers are
//! supplied by the caller, and tables use fixed-size arrays with
//! explicit capacity limits. Nothing in this module touches firmware
//! interfaces directly: the [`Link`] and [`Timer`] traits provide the
//! hardware hooks a driver must supply.
//!
//! ```
//! use rover_uefi::protocols::network::stack::ethernet;
//! use rover_uefi::protocols::network::types::MacAddress;
//!
//! let mut frame = [0_u8; 64];
//! let dst = MacAddress([0xFF; 6]);
//! let src = MacAddress([0x02, 0, 0, 0, 0, 1]);
//! let written = ethernet::build_frame(&mut frame, dst, src, 0x0800, &[1, 2, 3, 4]).ok();
//! assert!(written.is_some());
//! ```

pub mod arp;
pub mod dhcp;
pub mod dns_client;
pub mod ethernet;
pub mod icmp;
pub mod igmp;
pub mod ipv4;
pub mod ipv6;
pub mod ndp;
pub mod route;
pub mod tcp;
pub mod udp;

use crate::protocols::network::types::{Ipv4Address, Ipv6Address, MacAddress};

pub use arp::{ArpCache, ArpCacheEntry};
pub use dhcp::{DhcpClient, DhcpEvent, DhcpLease, DhcpOffer, DhcpState};
pub use dns_client::{DnsAnswerIter, DnsResponse};
pub use ethernet::{EthernetFrame, build_frame, build_frame_vlan, parse_frame};
pub use icmp::IcmpMessage;
pub use igmp::{GroupRecord, IgmpMessage, IgmpQuery};
pub use ipv4::{Ipv4Params, Ipv4Reassembler, ReassemblyStatus};
pub use ipv6::{ExtensionChain, Ipv6Params};
pub use ndp::{
    NdpOption, NdpOptions, NeighborAdvertisement, NeighborAdvertisementFlags, NeighborSolicitation,
    PrefixInformation, Redirect, RouterAdvertisement, RouterSolicitation,
};
pub use route::{RouteEntry, RoutingTable};
pub use tcp::{Endpoint, TcpConfig, TcpConnection, TcpFlags, TcpState};
pub use udp::UdpParams;

/// The largest frame the stack is able to build or parse.
pub const MAX_FRAME_LEN: usize = 2048;

/// Errors reported by the network stack.
///
/// The stack never panics. Every failure is reported as one of these
/// variants so that callers can react without unwinding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackError {
    /// The supplied output buffer cannot hold the requested encoding.
    BufferTooSmall,
    /// The supplied input is truncated or malformed.
    Malformed,
    /// The operation is not valid for the current protocol state.
    InvalidState,
    /// A bounded table has no free slot.
    TableFull,
    /// The requested feature is not implemented by this stack.
    Unsupported,
}

impl core::fmt::Display for StackError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Self::BufferTooSmall => "the output buffer is too small",
            Self::Malformed => "the input is truncated or malformed",
            Self::InvalidState => "the operation is invalid in the current state",
            Self::TableFull => "the table is full",
            Self::Unsupported => "the feature is not supported",
        };
        formatter.write_str(text)
    }
}

impl core::error::Error for StackError {}

/// An IP address of either family.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IpAddress {
    /// An IPv4 address.
    V4(Ipv4Address),
    /// An IPv6 address.
    V6(Ipv6Address),
}

impl IpAddress {
    /// Returns the IPv4 address, when the address is of that family.
    #[must_use]
    pub const fn as_v4(&self) -> Option<Ipv4Address> {
        match self {
            Self::V4(address) => Some(*address),
            Self::V6(_) => None,
        }
    }

    /// Returns the IPv6 address, when the address is of that family.
    #[must_use]
    pub const fn as_v6(&self) -> Option<Ipv6Address> {
        match self {
            Self::V4(_) => None,
            Self::V6(address) => Some(*address),
        }
    }
}

/// A link-layer interface that the stack can transmit over.
///
/// Implementations adapt a concrete network controller to the stack.
/// The stack never assumes a particular medium: frames are handed over
/// as opaque byte strings.
pub trait Link {
    /// Returns the link-layer address of the interface.
    fn hardware_address(&self) -> MacAddress;

    /// Returns the maximum transmission unit in bytes.
    fn mtu(&self) -> usize;

    /// Transmits one frame.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when the frame cannot be
    /// represented by the medium and [`StackError::Unsupported`] when
    /// the link is not ready.
    fn send(&mut self, frame: &[u8]) -> Result<(), StackError>;
}

/// A monotonic millisecond clock.
///
/// The stack only ever compares timestamps, so any monotonically
/// increasing counter is acceptable.
pub trait Timer {
    /// Returns the elapsed time in milliseconds since an arbitrary
    /// epoch.
    fn now_millis(&self) -> u64;
}
