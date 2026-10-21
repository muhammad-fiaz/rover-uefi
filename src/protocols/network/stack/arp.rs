//! # Address resolution
//!
//! Building and parsing of `ARP` messages for IPv4 over Ethernet and
//! a bounded neighbour cache.
//!
//! The message layout follows `RFC 826`. The parser is the one in
//! [`crate::protocols::network::packets`]; this module adds the
//! builders and the cache.

use crate::protocols::network::packets::{
    ARP_FRAME_LEN, ARP_HTYPE_ETHERNET, ARP_OPER_REPLY, ARP_OPER_REQUEST, ARP_PTYPE_IPV4, ArpPacket,
};
use crate::protocols::network::stack::StackError;
use crate::protocols::network::types::{Ipv4Address, MacAddress};

/// The number of entries an [`ArpCache`] holds.
pub const ARP_CACHE_CAPACITY: usize = 32;

/// The default lifetime of a cache entry in milliseconds.
pub const ARP_DEFAULT_LIFETIME_MILLIS: u64 = 60_000;

/// Builds an `ARP` request in `out`.
///
/// The request asks which link-layer address owns `target_ip`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// 28-byte `ARP` body.
pub fn build_request(
    out: &mut [u8],
    sender_mac: MacAddress,
    sender_ip: Ipv4Address,
    target_ip: Ipv4Address,
) -> Result<usize, StackError> {
    build_message(
        out,
        ARP_OPER_REQUEST,
        sender_mac,
        sender_ip,
        MacAddress([0; 6]),
        target_ip,
    )
}

/// Builds an `ARP` reply in `out`.
///
/// The reply tells the requester that `sender_ip` is reachable through
/// `sender_mac`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// 28-byte `ARP` body.
pub fn build_reply(
    out: &mut [u8],
    sender_mac: MacAddress,
    sender_ip: Ipv4Address,
    target_mac: MacAddress,
    target_ip: Ipv4Address,
) -> Result<usize, StackError> {
    build_message(
        out,
        ARP_OPER_REPLY,
        sender_mac,
        sender_ip,
        target_mac,
        target_ip,
    )
}

/// Builds an `ARP` message with the given operation.
fn build_message(
    out: &mut [u8],
    oper: u16,
    sender_mac: MacAddress,
    sender_ip: Ipv4Address,
    target_mac: MacAddress,
    target_ip: Ipv4Address,
) -> Result<usize, StackError> {
    if out.len() < ARP_FRAME_LEN {
        return Err(StackError::BufferTooSmall);
    }
    let body = out
        .get_mut(..ARP_FRAME_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    body[0..2].copy_from_slice(&ARP_HTYPE_ETHERNET.to_be_bytes());
    body[2..4].copy_from_slice(&ARP_PTYPE_IPV4.to_be_bytes());
    body[4] = 6;
    body[5] = 4;
    body[6..8].copy_from_slice(&oper.to_be_bytes());
    body[8..14].copy_from_slice(&sender_mac.0);
    body[14..18].copy_from_slice(&sender_ip.0);
    body[18..24].copy_from_slice(&target_mac.0);
    body[24..28].copy_from_slice(&target_ip.0);
    Ok(ARP_FRAME_LEN)
}

/// Parses an `ARP` message.
///
/// # Errors
///
/// Returns `None` when the message is truncated or does not describe
/// IPv4 over Ethernet.
#[must_use]
pub fn parse_arp(data: &[u8]) -> Option<ArpPacket<'_>> {
    crate::protocols::network::packets::parse_arp(data)
}

/// A single neighbour cache entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArpCacheEntry {
    /// The protocol address that was resolved.
    pub protocol_address: Ipv4Address,
    /// The link-layer address that was learned.
    pub hardware_address: MacAddress,
    /// The timestamp at which the entry was inserted.
    pub inserted_at_millis: u64,
}

/// A bounded neighbour cache.
///
/// The cache never allocates. When it is full the oldest entry is
/// replaced so that a busy network keeps resolving while the table
/// stays inside its fixed capacity.
#[derive(Clone, Debug)]
pub struct ArpCache {
    entries: [Option<ArpCacheEntry>; ARP_CACHE_CAPACITY],
    len: usize,
}

impl Default for ArpCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ArpCache {
    /// Creates an empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: [None; ARP_CACHE_CAPACITY],
            len: 0,
        }
    }

    /// Returns the number of entries the cache can hold.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        ARP_CACHE_CAPACITY
    }

    /// Returns the number of entries currently stored.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the cache is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserts or refreshes an entry.
    ///
    /// An existing entry for the same protocol address is updated in
    /// place. Otherwise a free slot is used, and when the cache is
    /// full the oldest entry is evicted.
    ///
    /// Returns `true` when the entry was stored.
    pub fn insert(
        &mut self,
        protocol_address: Ipv4Address,
        hardware_address: MacAddress,
        now_millis: u64,
    ) -> bool {
        let mut oldest: Option<usize> = None;
        let mut oldest_time = u64::MAX;
        for (index, slot) in self.entries.iter().enumerate() {
            match slot {
                Some(entry) if entry.protocol_address == protocol_address => {
                    self.entries[index] = Some(ArpCacheEntry {
                        protocol_address,
                        hardware_address,
                        inserted_at_millis: now_millis,
                    });
                    return true;
                }
                Some(entry) => {
                    if entry.inserted_at_millis <= oldest_time {
                        oldest_time = entry.inserted_at_millis;
                        oldest = Some(index);
                    }
                }
                None => {
                    self.entries[index] = Some(ArpCacheEntry {
                        protocol_address,
                        hardware_address,
                        inserted_at_millis: now_millis,
                    });
                    self.len += 1;
                    return true;
                }
            }
        }
        let Some(index) = oldest else {
            return false;
        };
        self.entries[index] = Some(ArpCacheEntry {
            protocol_address,
            hardware_address,
            inserted_at_millis: now_millis,
        });
        true
    }

    /// Looks up the link-layer address for `protocol_address`.
    ///
    /// # Errors
    ///
    /// Returns `None` when the address is not cached.
    #[must_use]
    pub fn lookup(&self, protocol_address: &Ipv4Address) -> Option<MacAddress> {
        self.entries
            .iter()
            .flatten()
            .find(|entry| entry.protocol_address == *protocol_address)
            .map(|entry| entry.hardware_address)
    }

    /// Returns the entry for `protocol_address`.
    ///
    /// # Errors
    ///
    /// Returns `None` when the address is not cached.
    #[must_use]
    pub fn entry(&self, protocol_address: &Ipv4Address) -> Option<&ArpCacheEntry> {
        self.entries
            .iter()
            .flatten()
            .find(|entry| entry.protocol_address == *protocol_address)
    }

    /// Removes every entry that is older than `max_age_millis`.
    ///
    /// The age is measured against `now_millis`. Returns the number of
    /// entries that were removed.
    pub fn age_out(&mut self, now_millis: u64, max_age_millis: u64) -> usize {
        let mut removed = 0;
        for slot in &mut self.entries {
            if let Some(entry) = slot {
                let age = now_millis.saturating_sub(entry.inserted_at_millis);
                if age > max_age_millis {
                    *slot = None;
                    removed += 1;
                }
            }
        }
        self.len = self.len.saturating_sub(removed);
        removed
    }

    /// Removes every entry.
    pub fn clear(&mut self) {
        self.entries.fill(None);
        self.len = 0;
    }

    /// Returns an iterator over the stored entries.
    pub fn iter(&self) -> impl Iterator<Item = &ArpCacheEntry> {
        self.entries.iter().flatten()
    }
}
