//! # IPv4
//!
//! Building, checksumming and fragmenting IPv4 datagrams, plus a
//! bounded fragment reassembler.
//!
//! The header layout follows `RFC 791` and the checksum follows
//! `RFC 1071`. The checksum helper is reused from
//! [`crate::protocols::network::packets`].

use crate::protocols::network::packets::{
    IP_PROTOCOL_UDP, internet_checksum, parse_ipv4, verify_checksum,
};
use crate::protocols::network::stack::StackError;
use crate::protocols::network::types::Ipv4Address;

/// Length of the IPv4 header without options in bytes.
pub const IPV4_HEADER_LEN: usize = 20;

/// The largest total length an IPv4 datagram may declare.
pub const IPV4_MAX_TOTAL_LEN: usize = 65_535;

/// The number of bytes of payload that fit in one fragment.
pub const IPV4_FRAGMENT_UNIT: usize = 8;

/// The number of reassembly slots the reassembler holds.
pub const REASSEMBLY_SLOT_COUNT: usize = 4;

/// The size of one reassembly buffer in bytes.
pub const REASSEMBLY_BUFFER_SIZE: usize = 2048;

/// The number of 64-bit words in one reassembly bitmap.
const REASSEMBLY_BITMAP_WORDS: usize = REASSEMBLY_BUFFER_SIZE / 64;

/// Parameters used to build an IPv4 header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ipv4Params {
    /// The source address.
    pub src: Ipv4Address,
    /// The destination address.
    pub dst: Ipv4Address,
    /// The upper-layer protocol number.
    pub protocol: u8,
    /// The time to live in hops.
    pub ttl: u8,
    /// The identification field, shared by all fragments.
    pub identification: u16,
    /// The differentiated services code point.
    pub dscp: u8,
    /// The explicit congestion notification field.
    pub ecn: u8,
    /// Whether the don't-fragment flag is set.
    pub dont_fragment: bool,
    /// Whether the more-fragments flag is set.
    pub more_fragments: bool,
    /// The fragment offset in eight-byte units.
    pub fragment_offset: u16,
}

impl Default for Ipv4Params {
    fn default() -> Self {
        Self {
            src: Ipv4Address([0, 0, 0, 0]),
            dst: Ipv4Address([0, 0, 0, 0]),
            protocol: IP_PROTOCOL_UDP,
            ttl: 64,
            identification: 0,
            dscp: 0,
            ecn: 0,
            dont_fragment: false,
            more_fragments: false,
            fragment_offset: 0,
        }
    }
}

/// Computes the IPv4 header checksum over `header`.
///
/// The checksum field of `header` must be zero when this is called.
///
/// # Errors
///
/// Returns `None` when `header` is shorter than 20 bytes.
#[must_use]
pub fn header_checksum(header: &[u8]) -> Option<u16> {
    if header.len() < IPV4_HEADER_LEN {
        return None;
    }
    Some(internet_checksum(header))
}

/// Returns whether an IPv4 header verifies.
///
/// # Errors
///
/// Returns `false` when `header` is shorter than 20 bytes or when the
/// stored checksum does not match.
#[must_use]
pub fn validate_checksum(header: &[u8]) -> bool {
    header.len() >= IPV4_HEADER_LEN && verify_checksum(header)
}

/// Writes an IPv4 header into `out`.
///
/// The header is filled in with the fields of `params` and the header
/// checksum is computed. `payload_len` is the length of the payload
/// that will follow the header and takes part in the total length
/// field.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// 20 bytes or when the resulting total length does not fit into the
/// 16-bit total length field.
pub fn write_header(
    out: &mut [u8],
    params: &Ipv4Params,
    payload_len: usize,
) -> Result<usize, StackError> {
    let total = IPV4_HEADER_LEN
        .checked_add(payload_len)
        .ok_or(StackError::BufferTooSmall)?;
    let total_u16 = u16::try_from(total).map_err(|_| StackError::BufferTooSmall)?;
    let header = out
        .get_mut(..IPV4_HEADER_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    header[0] = 0x45;
    header[1] = ((params.dscp & 0x3F) << 2) | (params.ecn & 0x03);
    header[2..4].copy_from_slice(&total_u16.to_be_bytes());
    header[4..6].copy_from_slice(&params.identification.to_be_bytes());
    let mut flags: u16 = params.fragment_offset & 0x1FFF;
    if params.dont_fragment {
        flags |= 0x4000;
    }
    if params.more_fragments {
        flags |= 0x2000;
    }
    header[6..8].copy_from_slice(&flags.to_be_bytes());
    header[8] = params.ttl;
    header[9] = params.protocol;
    header[10..12].copy_from_slice(&[0, 0]);
    header[12..16].copy_from_slice(&params.src.0);
    header[16..20].copy_from_slice(&params.dst.0);
    let checksum = internet_checksum(header);
    header[10..12].copy_from_slice(&checksum.to_be_bytes());
    Ok(IPV4_HEADER_LEN)
}

/// Builds a complete IPv4 datagram in `out`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// header and the payload.
pub fn build_datagram(
    out: &mut [u8],
    params: &Ipv4Params,
    payload: &[u8],
) -> Result<usize, StackError> {
    let header_len = write_header(out, params, payload.len())?;
    let body = out
        .get_mut(
            header_len
                ..header_len
                    .checked_add(payload.len())
                    .ok_or(StackError::BufferTooSmall)?,
        )
        .ok_or(StackError::BufferTooSmall)?;
    body.copy_from_slice(payload);
    Ok(header_len + payload.len())
}

/// The description of one fragment of a datagram payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FragmentLayout {
    /// The fragment offset in eight-byte units.
    pub offset_units: u16,
    /// The byte offset of the fragment inside the original payload.
    pub byte_offset: usize,
    /// The length of the fragment payload in bytes.
    pub length: usize,
    /// Whether further fragments follow.
    pub more_fragments: bool,
}

/// Splits a payload of `payload_len` bytes into fragments.
///
/// Every fragment except the last one carries a payload length that is
/// a multiple of eight bytes, as required by `RFC 791`. The layouts
/// are written into `out` and the number of fragments is returned.
///
/// # Errors
///
/// Returns [`StackError::InvalidState`] when `max_payload` is zero or
/// is not a multiple of eight bytes, and [`StackError::TableFull`]
/// when `out` cannot hold every fragment.
pub fn layout_fragments(
    payload_len: usize,
    max_payload: usize,
    out: &mut [FragmentLayout],
) -> Result<usize, StackError> {
    if max_payload == 0 || max_payload % IPV4_FRAGMENT_UNIT != 0 {
        return Err(StackError::InvalidState);
    }
    let mut count = 0_usize;
    let mut offset = 0_usize;
    while offset < payload_len {
        let length = max_payload.min(payload_len - offset);
        let more = offset + length < payload_len;
        let slot = out.get_mut(count).ok_or(StackError::TableFull)?;
        let offset_units = u16::try_from(offset / IPV4_FRAGMENT_UNIT).unwrap_or(u16::MAX);
        *slot = FragmentLayout {
            offset_units,
            byte_offset: offset,
            length,
            more_fragments: more,
        };
        count += 1;
        offset += length;
    }
    Ok(count)
}

/// Writes one fragment of a datagram into `out`.
///
/// `payload` is the complete datagram payload and `layout` selects the
/// part that this fragment carries.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// fragment or when the layout does not describe a part of `payload`.
pub fn write_fragment(
    out: &mut [u8],
    params: &Ipv4Params,
    payload: &[u8],
    layout: &FragmentLayout,
) -> Result<usize, StackError> {
    let end = layout
        .byte_offset
        .checked_add(layout.length)
        .ok_or(StackError::Malformed)?;
    let part = payload
        .get(layout.byte_offset..end)
        .ok_or(StackError::Malformed)?;
    let mut fragment_params = *params;
    fragment_params.more_fragments = layout.more_fragments;
    fragment_params.fragment_offset = layout.offset_units;
    build_datagram(out, &fragment_params, part)
}

/// The outcome of feeding a datagram into the reassembler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReassemblyStatus {
    /// The fragment was accepted and more fragments are expected.
    Incomplete,
    /// The datagram is complete and the payload can be read.
    Complete,
    /// The fragment was rejected because it overlapped earlier data,
    /// was malformed or did not fit into the buffer.
    Rejected,
    /// The datagram was not fragmented and needs no reassembly.
    NotFragmented,
}

/// The identity of a fragmented datagram.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FragmentKey {
    src: Ipv4Address,
    dst: Ipv4Address,
    identification: u16,
    protocol: u8,
}

/// One reassembly slot.
struct ReassemblySlot {
    key: Option<FragmentKey>,
    buffer: [u8; REASSEMBLY_BUFFER_SIZE],
    bitmap: [u64; REASSEMBLY_BITMAP_WORDS],
    received: usize,
    total_len: Option<usize>,
    last_seen: bool,
}

impl ReassemblySlot {
    /// Creates an empty slot.
    fn new() -> Self {
        Self {
            key: None,
            buffer: [0; REASSEMBLY_BUFFER_SIZE],
            bitmap: [0; REASSEMBLY_BITMAP_WORDS],
            received: 0,
            total_len: None,
            last_seen: false,
        }
    }

    /// Returns whether the slot holds no datagram.
    fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    /// Discards the stored datagram.
    fn reset(&mut self) {
        self.key = None;
        self.bitmap = [0; REASSEMBLY_BITMAP_WORDS];
        self.received = 0;
        self.total_len = None;
        self.last_seen = false;
    }
}

/// A bounded IPv4 fragment reassembler.
///
/// Up to [`REASSEMBLY_SLOT_COUNT`] datagrams are reassembled at the
/// same time. Each slot owns a fixed buffer of
/// [`REASSEMBLY_BUFFER_SIZE`] bytes, so no memory is allocated at run
/// time. Fragments that overlap already received data are rejected.
pub struct Ipv4Reassembler {
    slots: [ReassemblySlot; REASSEMBLY_SLOT_COUNT],
}

impl Default for Ipv4Reassembler {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for Ipv4Reassembler {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Ipv4Reassembler")
            .field("slots", &REASSEMBLY_SLOT_COUNT)
            .field("active", &self.active())
            .finish()
    }
}

impl Ipv4Reassembler {
    /// Creates an empty reassembler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            slots: core::array::from_fn(|_| ReassemblySlot::new()),
        }
    }

    /// Returns the number of datagrams currently being reassembled.
    #[must_use]
    pub fn active(&self) -> usize {
        self.slots.iter().filter(|slot| !slot.is_empty()).count()
    }

    /// Discards every partially reassembled datagram.
    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            slot.reset();
        }
    }

    /// Returns the payload of the datagram that was completed last.
    ///
    /// # Errors
    ///
    /// Returns `None` when no datagram has been completed.
    #[must_use]
    pub fn payload(&self) -> Option<&[u8]> {
        self.slots
            .iter()
            .find(|slot| slot.last_seen && slot.total_len == Some(slot.received))
            .map(|slot| slot.buffer.get(..slot.received).unwrap_or(&[]))
    }

    /// Returns the upper-layer protocol of the completed datagram.
    ///
    /// # Errors
    ///
    /// Returns `None` when no datagram has been completed.
    #[must_use]
    pub fn protocol(&self) -> Option<u8> {
        self.slots
            .iter()
            .find(|slot| slot.last_seen && slot.total_len == Some(slot.received))
            .and_then(|slot| slot.key)
            .map(|key| key.protocol)
    }

    /// Feeds a datagram into the reassembler.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::Malformed`] when `datagram` does not
    /// contain a valid IPv4 header.
    pub fn push(&mut self, datagram: &[u8]) -> Result<ReassemblyStatus, StackError> {
        let header = parse_ipv4(datagram).ok_or(StackError::Malformed)?;
        if !header.is_fragmented() {
            return Ok(ReassemblyStatus::NotFragmented);
        }
        let key = FragmentKey {
            src: header.src,
            dst: header.dst,
            identification: header.identification,
            protocol: header.protocol,
        };
        let payload = header.payload;
        let offset = usize::try_from(header.fragment_offset_bytes()).unwrap_or(usize::MAX);
        let end = offset
            .checked_add(payload.len())
            .ok_or(StackError::Malformed)?;
        if end > REASSEMBLY_BUFFER_SIZE {
            return Ok(ReassemblyStatus::Rejected);
        }
        if header.more_fragments && payload.len() % IPV4_FRAGMENT_UNIT != 0 {
            // Every fragment except the last one has to start and end
            // on an eight-byte boundary.
            return Ok(ReassemblyStatus::Rejected);
        }
        let slot_index = self.select_slot(key);
        let slot = self
            .slots
            .get_mut(slot_index)
            .ok_or(StackError::TableFull)?;
        if slot.key != Some(key) {
            slot.reset();
            slot.key = Some(key);
        }
        for index in offset..end {
            if bitmap_test(&slot.bitmap, index) {
                // Overlapping fragments are not accepted.
                return Ok(ReassemblyStatus::Rejected);
            }
        }
        for (position, byte) in payload.iter().enumerate() {
            let index = offset + position;
            if let Some(target) = slot.buffer.get_mut(index) {
                *target = *byte;
            }
            bitmap_set(&mut slot.bitmap, index);
        }
        slot.received = slot.received.saturating_add(payload.len());
        if !header.more_fragments {
            slot.total_len = Some(end);
            slot.last_seen = true;
        }
        if slot.total_len == Some(slot.received) {
            Ok(ReassemblyStatus::Complete)
        } else {
            Ok(ReassemblyStatus::Incomplete)
        }
    }

    /// Chooses the slot that receives a fragment of `key`.
    fn select_slot(&mut self, key: FragmentKey) -> usize {
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.key == Some(key) {
                return index;
            }
        }
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.is_empty() {
                return index;
            }
        }
        // Every slot is busy; evict the one with the least progress.
        let mut victim = 0;
        let mut least = usize::MAX;
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.received < least {
                least = slot.received;
                victim = index;
            }
        }
        victim
    }
}

/// Returns whether the byte at `index` was already received.
fn bitmap_test(bitmap: &[u64; REASSEMBLY_BITMAP_WORDS], index: usize) -> bool {
    let word = index / 64;
    let bit = index % 64;
    match bitmap.get(word) {
        Some(value) => (value >> bit) & 1 == 1,
        None => false,
    }
}

/// Marks the byte at `index` as received.
fn bitmap_set(bitmap: &mut [u64; REASSEMBLY_BITMAP_WORDS], index: usize) {
    let word = index / 64;
    let bit = index % 64;
    if let Some(value) = bitmap.get_mut(word) {
        *value |= 1 << bit;
    }
}
