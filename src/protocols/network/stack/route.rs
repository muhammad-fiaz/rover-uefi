//! # Routing
//!
//! A fixed-capacity IPv4 routing table.
//!
//! Every entry consists of a destination prefix, the length of that
//! prefix, a next hop and the index of the interface the traffic has
//! to leave through. Lookups use the longest prefix match, and a
//! prefix of length zero is the default route.

use crate::protocols::network::stack::StackError;
use crate::protocols::network::types::Ipv4Address;

/// The number of routes the table holds.
pub const ROUTING_TABLE_CAPACITY: usize = 32;

/// The largest prefix length of an IPv4 prefix.
pub const MAX_PREFIX_LENGTH: u8 = 32;

/// One entry of the routing table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteEntry {
    /// The destination prefix.
    pub destination: Ipv4Address,
    /// The length of the prefix in bits.
    pub prefix_length: u8,
    /// The next hop of the route.
    pub next_hop: Ipv4Address,
    /// The index of the interface the route uses.
    pub interface: u32,
    /// The metric of the route, lower is better.
    pub metric: u32,
}

impl RouteEntry {
    /// Creates a default route towards `next_hop`.
    #[must_use]
    pub const fn default_route(next_hop: Ipv4Address, interface: u32) -> Self {
        Self {
            destination: Ipv4Address([0, 0, 0, 0]),
            prefix_length: 0,
            next_hop,
            interface,
            metric: 0,
        }
    }

    /// Returns whether the entry is the default route.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.prefix_length == 0
    }

    /// Returns whether `address` belongs to the destination prefix.
    #[must_use]
    pub fn matches(&self, address: &Ipv4Address) -> bool {
        let mask = prefix_mask(self.prefix_length);
        let target = u32::from_be_bytes(address.0);
        let prefix = u32::from_be_bytes(self.destination.0);
        (target & mask) == (prefix & mask)
    }
}

/// Returns the mask of a prefix of `prefix_length` bits.
#[must_use]
pub fn prefix_mask(prefix_length: u8) -> u32 {
    let bits = u32::from(prefix_length.min(32));
    if bits == 0 {
        return 0;
    }
    u32::MAX << (32 - bits)
}

/// A fixed-capacity routing table.
#[derive(Clone, Debug)]
pub struct RoutingTable {
    entries: [Option<RouteEntry>; ROUTING_TABLE_CAPACITY],
    len: usize,
}

impl Default for RoutingTable {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutingTable {
    /// Creates an empty table.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: [None; ROUTING_TABLE_CAPACITY],
            len: 0,
        }
    }

    /// Returns the number of routes the table can hold.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        ROUTING_TABLE_CAPACITY
    }

    /// Returns the number of stored routes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the table is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Adds a route or replaces the route with the same prefix.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::TableFull`] when the table has no free
    /// slot and [`StackError::Malformed`] when the prefix length is
    /// larger than 32.
    pub fn insert(&mut self, entry: RouteEntry) -> Result<(), StackError> {
        if entry.prefix_length > MAX_PREFIX_LENGTH {
            return Err(StackError::Malformed);
        }
        for slot in &mut self.entries {
            if let Some(existing) = slot
                && existing.prefix_length == entry.prefix_length
                && existing.destination == entry.destination
            {
                *slot = Some(entry);
                return Ok(());
            }
        }
        for slot in &mut self.entries {
            if slot.is_none() {
                *slot = Some(entry);
                self.len += 1;
                return Ok(());
            }
        }
        Err(StackError::TableFull)
    }

    /// Removes the route with the given prefix.
    ///
    /// Returns `true` when a route was removed.
    pub fn remove(&mut self, destination: &Ipv4Address, prefix_length: u8) -> bool {
        for slot in &mut self.entries {
            if let Some(existing) = slot
                && existing.prefix_length == prefix_length
                && existing.destination == *destination
            {
                *slot = None;
                self.len = self.len.saturating_sub(1);
                return true;
            }
        }
        false
    }

    /// Looks up the route that matches `address`.
    ///
    /// The longest matching prefix wins. Routes with the same prefix
    /// length are ordered by their metric.
    ///
    /// # Errors
    ///
    /// Returns `None` when no route matches.
    #[must_use]
    pub fn lookup(&self, address: &Ipv4Address) -> Option<&RouteEntry> {
        let mut best: Option<&RouteEntry> = None;
        for entry in self.entries.iter().flatten() {
            if !entry.matches(address) {
                continue;
            }
            let better = match best {
                None => true,
                Some(current) => {
                    entry.prefix_length > current.prefix_length
                        || (entry.prefix_length == current.prefix_length
                            && entry.metric < current.metric)
                }
            };
            if better {
                best = Some(entry);
            }
        }
        best
    }

    /// Returns the default route.
    ///
    /// # Errors
    ///
    /// Returns `None` when the table holds no default route.
    #[must_use]
    pub fn default_route(&self) -> Option<&RouteEntry> {
        self.entries
            .iter()
            .flatten()
            .filter(|entry| entry.is_default())
            .min_by_key(|entry| entry.metric)
    }

    /// Removes every route.
    pub fn clear(&mut self) {
        self.entries.fill(None);
        self.len = 0;
    }

    /// Returns an iterator over the stored routes.
    pub fn iter(&self) -> impl Iterator<Item = &RouteEntry> {
        self.entries.iter().flatten()
    }
}
