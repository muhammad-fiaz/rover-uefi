//! # Simple network mode
//!
//! Mode information helpers.

use crate::abi::types::Bool;

/// The maximum multicast filter entries.
pub const MAX_MCAST_FILTER_CNT: usize = 16;

/// The simple network mode structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiSimpleNetworkMode {
    /// The current interface state.
    pub state: u32,
    /// The hardware address size.
    pub hw_address_size: u32,
    /// The media header size.
    pub media_header_size: u32,
    /// The maximum packet size.
    pub max_packet_size: u32,
    /// The non-volatile memory size.
    pub nv_ram_size: u32,
    /// The non-volatile memory access size.
    pub nv_ram_access_size: u32,
    /// The multicast receive filter mask.
    pub receive_filter_mask: u32,
    /// The current receive filter setting.
    pub receive_filter_setting: u32,
    /// The maximum multicast filter count.
    pub max_mcast_filter_count: u32,
    /// The current multicast filter count.
    pub mcast_filter_count: u32,
    /// The multicast filter addresses.
    pub mcast_filter: [[u8; 6]; MAX_MCAST_FILTER_CNT],
    /// The current hardware address.
    pub current_address: [u8; 6],
    /// The broadcast address.
    pub broadcast_address: [u8; 6],
    /// The permanent hardware address.
    pub permanent_address: [u8; 6],
    /// The interface type.
    pub if_type: u8,
    /// Whether the address can change.
    pub mac_address_changeable: Bool,
    /// Whether multiple transmissions are supported.
    pub multiple_tx_supported: Bool,
    /// Whether media presence is supported.
    pub media_present_supported: Bool,
    /// Whether media are present.
    pub media_present: Bool,
}

/// Returns whether the media is present.
#[must_use]
pub fn is_media_present(media_present: u8) -> bool {
    media_present != 0
}
