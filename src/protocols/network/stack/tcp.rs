//! # TCP
//!
//! The transmission control protocol: sequence arithmetic, header and
//! option handling and a client-side connection state machine.
//!
//! The state machine follows `RFC 793`, the window scale option
//! follows `RFC 7323` and the retransmission timer follows
//! `RFC 6298`. The implementation is client oriented but implements the
//! passive close paths as well, so that a peer that closes first is
//! handled correctly.
//!
//! Every buffer is fixed size, so the connection never allocates.

use crate::base::bytes::read_u32_be;

use crate::protocols::network::packets::{
    IP_PROTOCOL_TCP, TCP_FLAG_ACK, TCP_FLAG_FIN, TCP_FLAG_PSH, TCP_FLAG_RST, TCP_FLAG_SYN,
    TCP_FLAG_URG, TCP_MIN_HEADER_LEN, TcpHeader, ipv4_segment_checksum, ipv6_segment_checksum,
    parse_tcp,
};
use crate::protocols::network::stack::{IpAddress, StackError};

/// The explicit congestion notification echo flag of a TCP header.
pub const TCP_FLAG_ECE: u8 = 0x40;

/// The congestion window reduced flag of a TCP header.
pub const TCP_FLAG_CWR: u8 = 0x80;

// ---------------------------------------------------------------------------
// Sequence arithmetic
// ---------------------------------------------------------------------------

/// Adds `offset` to `seq` with 32-bit wraparound.
#[must_use]
pub const fn seq_add(seq: u32, offset: u32) -> u32 {
    seq.wrapping_add(offset)
}

/// Returns whether `a` precedes `b` in the serial number space.
///
/// The comparison is modular, as described in `RFC 1982`. The result
/// is only defined while the two numbers are less than half of the
/// sequence space apart.
#[must_use]
pub const fn seq_lt(a: u32, b: u32) -> bool {
    a.wrapping_sub(b) & 0x8000_0000 != 0
}

/// Returns whether `a` follows `b` in the serial number space.
#[must_use]
pub const fn seq_gt(a: u32, b: u32) -> bool {
    seq_lt(b, a)
}

/// Returns whether `a` does not follow `b` in the serial number space.
#[must_use]
pub const fn seq_ge(a: u32, b: u32) -> bool {
    a == b || seq_lt(b, a)
}

/// Returns the distance from `from` to `to` in the serial number space.
#[must_use]
pub const fn seq_distance(from: u32, to: u32) -> u32 {
    to.wrapping_sub(from)
}

// ---------------------------------------------------------------------------
// Flags
// ---------------------------------------------------------------------------

/// The control flags of a TCP segment.
///
/// The bits are the ones defined by `RFC 793` plus the two congestion
/// control bits of `RFC 3168`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcpFlags(pub u8);

impl TcpFlags {
    /// No flag at all.
    pub const NONE: Self = Self(0);
    /// The finish flag.
    pub const FIN: Self = Self(TCP_FLAG_FIN);
    /// The synchronize flag.
    pub const SYN: Self = Self(TCP_FLAG_SYN);
    /// The reset flag.
    pub const RST: Self = Self(TCP_FLAG_RST);
    /// The push flag.
    pub const PSH: Self = Self(TCP_FLAG_PSH);
    /// The acknowledgment flag.
    pub const ACK: Self = Self(TCP_FLAG_ACK);
    /// The urgent flag.
    pub const URG: Self = Self(TCP_FLAG_URG);
    /// The explicit congestion notification echo flag.
    pub const ECE: Self = Self(TCP_FLAG_ECE);
    /// The congestion window reduced flag.
    pub const CWR: Self = Self(TCP_FLAG_CWR);
    /// Returns the raw flag bits.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Returns whether every bit of `other` is set.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns whether any bit of `other` is set.
    #[must_use]
    pub const fn is_set(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    /// Returns the union of two flag sets.
    #[must_use]
    pub const fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl core::ops::BitOr for TcpFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

// ---------------------------------------------------------------------------
// Options
// ---------------------------------------------------------------------------

/// The option kinds defined for TCP.
pub mod option {
    /// The end of the option list.
    pub const EOL: u8 = 0;
    /// A no-operation byte.
    pub const NOP: u8 = 1;
    /// The maximum segment size option.
    pub const MSS: u8 = 2;
    /// The window scale option.
    pub const WINDOW_SCALE: u8 = 3;
    /// The selective acknowledgment permitted option.
    pub const SACK_PERMITTED: u8 = 4;
    /// The selective acknowledgment option.
    pub const SACK: u8 = 5;
    /// The timestamp option.
    pub const TIMESTAMP: u8 = 8;
}

/// The largest options area the stack builds.
pub const TCP_MAX_OPTIONS_LEN: usize = 40;

/// The maximum window scale shift.
pub const TCP_MAX_WINDOW_SCALE: u8 = 14;

/// Writes the maximum segment size option into `out`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// four bytes.
pub fn write_mss(out: &mut [u8], mss: u16) -> Result<usize, StackError> {
    let option = out.get_mut(..4).ok_or(StackError::BufferTooSmall)?;
    option[0] = option::MSS;
    option[1] = 4;
    option[2..4].copy_from_slice(&mss.to_be_bytes());
    Ok(4)
}

/// Writes the window scale option into `out`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// four bytes.
pub fn write_window_scale(out: &mut [u8], shift: u8) -> Result<usize, StackError> {
    let option = out.get_mut(..4).ok_or(StackError::BufferTooSmall)?;
    option[0] = option::WINDOW_SCALE;
    option[1] = 3;
    option[2] = shift.min(TCP_MAX_WINDOW_SCALE);
    option[3] = option::NOP;
    Ok(4)
}

/// Writes the selective acknowledgment permitted option into `out`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// four bytes.
pub fn write_sack_permitted(out: &mut [u8]) -> Result<usize, StackError> {
    let option = out.get_mut(..4).ok_or(StackError::BufferTooSmall)?;
    option[0] = option::SACK_PERMITTED;
    option[1] = 2;
    option[2] = option::NOP;
    option[3] = option::NOP;
    Ok(4)
}

/// Writes the timestamp option into `out`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// twelve bytes.
pub fn write_timestamp(out: &mut [u8], ts_value: u32, ts_echo: u32) -> Result<usize, StackError> {
    let option = out.get_mut(..12).ok_or(StackError::BufferTooSmall)?;
    option[0] = option::TIMESTAMP;
    option[1] = 10;
    option[2..6].copy_from_slice(&ts_value.to_be_bytes());
    option[6..10].copy_from_slice(&ts_echo.to_be_bytes());
    option[10] = 0;
    option[11] = 0;
    Ok(12)
}

/// Writes a selective acknowledgment option with two blocks into
/// `out`.
///
/// The option is padded with no-operation bytes so that the option
/// list can continue behind it.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// twenty bytes.
pub fn write_sack_blocks(
    out: &mut [u8],
    first_left: u32,
    first_right: u32,
    second_left: u32,
    second_right: u32,
) -> Result<usize, StackError> {
    let option = out.get_mut(..20).ok_or(StackError::BufferTooSmall)?;
    option[0] = option::SACK;
    option[1] = 18;
    option[2..6].copy_from_slice(&first_left.to_be_bytes());
    option[6..10].copy_from_slice(&first_right.to_be_bytes());
    option[10..14].copy_from_slice(&second_left.to_be_bytes());
    option[14..18].copy_from_slice(&second_right.to_be_bytes());
    option[18] = option::NOP;
    option[19] = option::NOP;
    Ok(20)
}

/// A single option of a TCP segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcpOption<'a> {
    /// The option kind.
    pub kind: u8,
    /// The option value.
    pub value: &'a [u8],
}

/// An iterator over the options of a TCP segment.
#[derive(Clone, Debug)]
pub struct TcpOptionIter<'a> {
    raw: &'a [u8],
    offset: usize,
    done: bool,
}

impl<'a> Iterator for TcpOptionIter<'a> {
    type Item = TcpOption<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let kind = *self.raw.get(self.offset)?;
        if kind == option::EOL {
            self.done = true;
            return None;
        }
        if kind == option::NOP {
            self.offset = self.offset.checked_add(1)?;
            return Some(TcpOption { kind, value: &[] });
        }
        let length = usize::from(*self.raw.get(self.offset.checked_add(1)?)?);
        if length < 2 {
            self.done = true;
            return None;
        }
        let start = self.offset.checked_add(2)?;
        let end = start.checked_add(length.checked_sub(2)?)?;
        let value = self.raw.get(start..end)?;
        self.offset = end;
        Some(TcpOption { kind, value })
    }
}

/// A selective acknowledgement block.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SackBlock {
    /// The first sequence number of the block.
    pub left: u32,
    /// The sequence number just after the block.
    pub right: u32,
}

/// An iterator over the blocks of a selective acknowledgement option.
#[derive(Clone, Debug)]
pub struct SackBlocks<'a> {
    value: &'a [u8],
    offset: usize,
}

impl Iterator for SackBlocks<'_> {
    type Item = SackBlock;

    fn next(&mut self) -> Option<Self::Item> {
        let left = read_u32_be(self.value, self.offset)?;
        let right = read_u32_be(self.value, self.offset.checked_add(4)?)?;
        self.offset = self.offset.checked_add(8)?;
        Some(SackBlock { left, right })
    }
}

impl<'a> IntoIterator for &SackBlocks<'a> {
    type Item = SackBlock;
    type IntoIter = SackBlocks<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.clone()
    }
}

impl SackBlocks<'_> {
    /// Returns an iterator over the blocks.
    #[must_use]
    pub fn iter(&self) -> SackBlocks<'_> {
        self.clone()
    }
}

/// The options of a TCP segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcpOptions<'a> {
    raw: &'a [u8],
}

impl<'a> TcpOptions<'a> {
    /// Wraps the raw options area.
    #[must_use]
    pub const fn new(raw: &'a [u8]) -> Self {
        Self { raw }
    }

    /// Returns the raw options area.
    #[must_use]
    pub const fn raw(&self) -> &'a [u8] {
        self.raw
    }

    /// Returns an iterator over the options.
    #[must_use]
    pub fn iter(&self) -> TcpOptionIter<'a> {
        TcpOptionIter {
            raw: self.raw,
            offset: 0,
            done: false,
        }
    }

    /// Returns the maximum segment size option, if present.
    #[must_use]
    pub fn mss(&self) -> Option<u16> {
        for item in self {
            if item.kind == option::MSS && item.value.len() == 2 {
                let high = u16::from(item.value.first().copied()?);
                let low = u16::from(item.value.get(1).copied()?);
                return Some((high << 8) | low);
            }
        }
        None
    }

    /// Returns the window scale option, if present.
    #[must_use]
    pub fn window_scale(&self) -> Option<u8> {
        for item in self {
            if item.kind == option::WINDOW_SCALE && item.value.len() == 1 {
                return item.value.first().copied();
            }
        }
        None
    }

    /// Returns whether the selective acknowledgment option is present.
    #[must_use]
    pub fn sack_permitted(&self) -> bool {
        self.iter().any(|item| item.kind == option::SACK_PERMITTED)
    }

    /// Returns the timestamp option, if present.
    ///
    /// The first value is the timestamp value of the sender and the
    /// second one is the echoed timestamp of the peer.
    #[must_use]
    pub fn timestamps(&self) -> Option<(u32, u32)> {
        for item in self {
            if item.kind == option::TIMESTAMP && item.value.len() == 8 {
                let value = read_u32_be(item.value, 0)?;
                let echo = read_u32_be(item.value, 4)?;
                return Some((value, echo));
            }
        }
        None
    }

    /// Returns an iterator over the selective acknowledgment blocks.
    #[must_use]
    pub fn sack_blocks(&self) -> SackBlocks<'a> {
        let mut value: &[u8] = &[];
        for item in self {
            if item.kind == option::SACK {
                value = item.value;
                break;
            }
        }
        SackBlocks { value, offset: 0 }
    }
}

impl<'a> IntoIterator for &TcpOptions<'a> {
    type Item = TcpOption<'a>;
    type IntoIter = TcpOptionIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// ---------------------------------------------------------------------------
// Segments
// ---------------------------------------------------------------------------

/// A TCP segment that is about to be transmitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcpSegment<'a> {
    /// The flags to set.
    pub flags: TcpFlags,
    /// The sequence number.
    pub seq: u32,
    /// The acknowledgment number.
    pub ack: u32,
    /// The advertised receive window.
    pub window: u16,
    /// The urgent pointer.
    pub urgent_pointer: u16,
    /// The payload bytes.
    pub payload: &'a [u8],
    /// The options bytes.
    pub options: &'a [u8],
}

impl Default for TcpSegment<'_> {
    fn default() -> Self {
        Self {
            flags: TcpFlags::NONE,
            seq: 0,
            ack: 0,
            window: 0,
            urgent_pointer: 0,
            payload: &[],
            options: &[],
        }
    }
}

/// Writes a TCP segment into `out`.
///
/// The checksum is computed over the pseudo-header of the address
/// family of `src` and `dst`. The options area is padded to a
/// multiple of four bytes.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// segment and [`StackError::InvalidState`] when the address families
/// of `src` and `dst` differ.
pub fn write_segment(
    out: &mut [u8],
    src_port: u16,
    dst_port: u16,
    segment: &TcpSegment<'_>,
    src: &IpAddress,
    dst: &IpAddress,
) -> Result<usize, StackError> {
    let options_len = segment.options.len();
    let padded = options_len.div_ceil(4) * 4;
    let header_len = TCP_MIN_HEADER_LEN + padded;
    let total = header_len
        .checked_add(segment.payload.len())
        .ok_or(StackError::BufferTooSmall)?;
    let data_offset = u8::try_from(header_len / 4).map_err(|_| StackError::BufferTooSmall)?;
    let buffer = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    buffer[0..2].copy_from_slice(&src_port.to_be_bytes());
    buffer[2..4].copy_from_slice(&dst_port.to_be_bytes());
    buffer[4..8].copy_from_slice(&segment.seq.to_be_bytes());
    buffer[8..12].copy_from_slice(&segment.ack.to_be_bytes());
    buffer[12] = data_offset << 4;
    buffer[13] = segment.flags.bits();
    buffer[14..16].copy_from_slice(&segment.window.to_be_bytes());
    buffer[16..18].copy_from_slice(&[0, 0]);
    buffer[18..20].copy_from_slice(&segment.urgent_pointer.to_be_bytes());
    if padded > 0 {
        let area = buffer
            .get_mut(TCP_MIN_HEADER_LEN..header_len)
            .ok_or(StackError::BufferTooSmall)?;
        let written = area
            .get_mut(..options_len)
            .ok_or(StackError::BufferTooSmall)?;
        written.copy_from_slice(segment.options);
        for byte in area
            .get_mut(options_len..)
            .ok_or(StackError::BufferTooSmall)?
        {
            *byte = 0;
        }
    }
    let body = buffer
        .get_mut(header_len..total)
        .ok_or(StackError::BufferTooSmall)?;
    body.copy_from_slice(segment.payload);
    let checksum = match (src, dst) {
        (IpAddress::V4(source), IpAddress::V4(destination)) => {
            ipv4_segment_checksum(source, destination, IP_PROTOCOL_TCP, buffer)
        }
        (IpAddress::V6(source), IpAddress::V6(destination)) => {
            ipv6_segment_checksum(source, destination, IP_PROTOCOL_TCP, buffer)
        }
        _ => return Err(StackError::InvalidState),
    };
    let field = out.get_mut(16..18).ok_or(StackError::BufferTooSmall)?;
    field.copy_from_slice(&checksum.to_be_bytes());
    Ok(total)
}

/// Parses a TCP segment.
///
/// # Errors
///
/// Returns `None` when the segment is truncated or declares an
/// impossible data offset.
#[must_use]
pub fn parse_segment(data: &[u8]) -> Option<TcpHeader<'_>> {
    parse_tcp(data)
}

/// Returns the options area of a parsed segment.
#[must_use]
pub fn segment_options<'a>(segment: &'a TcpHeader<'a>) -> &'a [u8] {
    segment
        .segment
        .get(TCP_MIN_HEADER_LEN..segment.header_len)
        .unwrap_or(&[])
}

// ---------------------------------------------------------------------------
// Timers
// ---------------------------------------------------------------------------

/// The maximum segment lifetime in milliseconds.
pub const MSL_MILLIS: u64 = 30_000;

/// The time-wait duration, twice the maximum segment lifetime.
pub const TIME_WAIT_MILLIS: u64 = 2 * MSL_MILLIS;

/// The estimator of the retransmission timeout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RtoEstimator {
    srtt: Option<u32>,
    rttvar: u32,
    rto: u32,
}

impl Default for RtoEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl RtoEstimator {
    /// The retransmission timeout used before any sample was taken.
    pub const INITIAL_RTO_MILLIS: u32 = 1000;
    /// The smallest retransmission timeout.
    pub const MIN_RTO_MILLIS: u32 = 200;
    /// The largest retransmission timeout.
    pub const MAX_RTO_MILLIS: u32 = 60_000;
    /// The clock granularity of the timer.
    pub const GRANULARITY_MILLIS: u32 = 1;

    /// Creates an estimator without any sample.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            srtt: None,
            rttvar: 0,
            rto: Self::INITIAL_RTO_MILLIS,
        }
    }

    /// Feeds a round trip time sample into the estimator.
    pub fn sample(&mut self, rtt_millis: u32) {
        match self.srtt {
            None => {
                self.srtt = Some(rtt_millis);
                self.rttvar = rtt_millis / 2;
            }
            Some(srtt) => {
                let difference = srtt.abs_diff(rtt_millis);
                self.rttvar = (3 * self.rttvar + difference) / 4;
                self.srtt = Some((7 * srtt + rtt_millis) / 8);
            }
        }
        self.recompute();
    }

    /// Returns the current retransmission timeout.
    #[must_use]
    pub const fn rto_millis(&self) -> u32 {
        self.rto
    }

    /// Returns the smoothed round trip time, if a sample was taken.
    #[must_use]
    pub const fn srtt_millis(&self) -> Option<u32> {
        self.srtt
    }

    /// Forgets every sample.
    pub fn reset(&mut self) {
        self.srtt = None;
        self.rttvar = 0;
        self.rto = Self::INITIAL_RTO_MILLIS;
    }

    /// Recomputes the timeout from the smoothed values.
    fn recompute(&mut self) {
        let srtt = self.srtt.unwrap_or(Self::INITIAL_RTO_MILLIS);
        let variance = self.rttvar.saturating_mul(4).max(Self::GRANULARITY_MILLIS);
        let rto = srtt.saturating_add(variance);
        self.rto = rto.clamp(Self::MIN_RTO_MILLIS, Self::MAX_RTO_MILLIS);
    }
}

/// A retransmission timer with exponential backoff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetransmitTimer {
    rto_millis: u32,
    attempts: u32,
    deadline_millis: u64,
    running: bool,
}

impl Default for RetransmitTimer {
    fn default() -> Self {
        Self::new(RtoEstimator::INITIAL_RTO_MILLIS)
    }
}

impl RetransmitTimer {
    /// Creates a timer with the given timeout.
    #[must_use]
    pub const fn new(rto_millis: u32) -> Self {
        Self {
            rto_millis,
            attempts: 0,
            deadline_millis: 0,
            running: false,
        }
    }

    /// Starts the timer at `now_millis`.
    pub fn start(&mut self, now_millis: u64) {
        self.running = true;
        self.deadline_millis = now_millis.saturating_add(u64::from(self.rto_millis));
    }

    /// Restarts the timer at `now_millis` without resetting the
    /// attempt counter.
    pub fn restart(&mut self, now_millis: u64) {
        self.start(now_millis);
    }

    /// Stops the timer.
    pub fn stop(&mut self) {
        self.running = false;
        self.deadline_millis = 0;
    }

    /// Returns whether the timer is running.
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.running
    }

    /// Returns the number of retransmissions so far.
    #[must_use]
    pub const fn attempts(&self) -> u32 {
        self.attempts
    }

    /// Returns the current timeout in milliseconds.
    #[must_use]
    pub const fn rto_millis(&self) -> u32 {
        self.rto_millis
    }

    /// Sets the timeout in milliseconds.
    pub fn set_rto_millis(&mut self, rto_millis: u32) {
        self.rto_millis =
            rto_millis.clamp(RtoEstimator::MIN_RTO_MILLIS, RtoEstimator::MAX_RTO_MILLIS);
    }

    /// Returns the moment at which the timer expires.
    #[must_use]
    pub const fn deadline_millis(&self) -> u64 {
        self.deadline_millis
    }

    /// Returns whether the timer has expired at `now_millis`.
    #[must_use]
    pub fn expired(&self, now_millis: u64) -> bool {
        self.running && now_millis >= self.deadline_millis
    }

    /// Doubles the timeout and counts one more attempt.
    pub fn backoff(&mut self) {
        self.attempts = self.attempts.saturating_add(1);
        self.rto_millis = self
            .rto_millis
            .saturating_mul(2)
            .min(RtoEstimator::MAX_RTO_MILLIS);
    }
}

// ---------------------------------------------------------------------------
// Connection
// ---------------------------------------------------------------------------

/// The states of the TCP state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcpState {
    /// The connection is closed.
    Closed,
    /// A connection request has been sent.
    SynSent,
    /// A connection request has been received or answered.
    SynReceived,
    /// The connection is established.
    Established,
    /// The local side closed first and the close is not acknowledged.
    FinWait1,
    /// The local close has been acknowledged.
    FinWait2,
    /// The remote side closed first.
    CloseWait,
    /// Both sides closed at the same time.
    Closing,
    /// The local close waits for the last acknowledgment.
    LastAck,
    /// The connection waits for twice the maximum segment lifetime.
    TimeWait,
}

/// The reason a connection was aborted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcpError {
    /// The peer sent a reset segment.
    ConnectionReset,
    /// The peer sent an invalid segment.
    ProtocolError,
    /// The local side aborted the connection.
    Aborted,
    /// The retransmission limit was reached.
    Timeout,
}

/// A TCP endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Endpoint {
    /// The IP address of the endpoint.
    pub address: IpAddress,
    /// The port of the endpoint.
    pub port: u16,
}

/// The configuration of a TCP connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcpConfig {
    /// The maximum segment size advertised in the synchronize segment.
    pub local_mss: u16,
    /// The size of the receive window in bytes.
    pub receive_window: u16,
    /// The window scale shift advertised in the synchronize segment.
    pub window_scale: u8,
    /// Whether timestamp options are used.
    pub timestamps: bool,
    /// The retransmission timeout used before any sample was taken.
    pub initial_rto_millis: u32,
    /// The number of retransmissions before the connection is aborted.
    pub max_retries: u32,
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            local_mss: 1460,
            receive_window: 2048,
            window_scale: 0,
            timestamps: false,
            initial_rto_millis: RtoEstimator::INITIAL_RTO_MILLIS,
            max_retries: 5,
        }
    }
}

/// The size of the send buffer in bytes.
pub const TCP_SEND_BUFFER_SIZE: usize = 4096;

/// The size of the receive buffer in bytes.
pub const TCP_RECV_BUFFER_SIZE: usize = 2048;

/// The result of processing an inbound segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcpOutcome {
    /// The segment was accepted.
    Accepted,
    /// The segment was accepted and a response was queued.
    Response,
    /// The segment carried only data that was already received.
    Duplicate,
    /// The segment was outside the receive window.
    OutOfWindow,
    /// The segment reset the connection.
    Reset,
    /// The segment was invalid.
    Invalid,
}

/// A segment that waits for transmission.
#[derive(Clone, Copy, Debug)]
struct QueuedSegment {
    flags: TcpFlags,
    seq: u32,
    ack: u32,
    window: u16,
    payload_offset: usize,
    payload_len: usize,
    options_len: usize,
    sequence_length: u32,
    next_state: Option<TcpState>,
}

/// A client-side TCP connection.
///
/// The connection owns the send and the receive buffer, tracks the
/// sequence numbers and drives the state machine. Segments are handed
/// to the caller by [`TcpConnection::poll`] and serialized by
/// [`TcpConnection::encode`].
pub struct TcpConnection {
    state: TcpState,
    local: Endpoint,
    remote: Endpoint,
    config: TcpConfig,
    iss: u32,
    irs: u32,
    snd_una: u32,
    snd_nxt: u32,
    snd_wnd: u32,
    rcv_nxt: u32,
    peer_mss: u16,
    peer_window_scale: u8,
    local_window_scale: u8,
    send_buf: [u8; TCP_SEND_BUFFER_SIZE],
    send_base: u32,
    send_end: usize,
    send_sent: usize,
    send_acked: usize,
    recv_buf: [u8; TCP_RECV_BUFFER_SIZE],
    recv_end: usize,
    recv_start: usize,
    options: [u8; TCP_MAX_OPTIONS_LEN],
    options_len: usize,
    queued: Option<QueuedSegment>,
    rto: RtoEstimator,
    retransmit: RetransmitTimer,
    time_wait_deadline: Option<u64>,
    pending_fin: bool,
    error: Option<TcpError>,
    ts_value: u32,
    ts_recent: u32,
    last_send_time: Option<u64>,
    last_send_seq: u32,
}

impl core::fmt::Debug for TcpConnection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("TcpConnection")
            .field("state", &self.state)
            .field("local", &self.local)
            .field("remote", &self.remote)
            .field("config", &self.config)
            .field("iss", &self.iss)
            .field("irs", &self.irs)
            .field("snd_una", &self.snd_una)
            .field("snd_nxt", &self.snd_nxt)
            .field("snd_wnd", &self.snd_wnd)
            .field("rcv_nxt", &self.rcv_nxt)
            .field("peer_mss", &self.peer_mss)
            .field("peer_window_scale", &self.peer_window_scale)
            .field("local_window_scale", &self.local_window_scale)
            .field("send_buf", &self.send_buf.get(..8).unwrap_or(&[]))
            .field("send_base", &self.send_base)
            .field("send_end", &self.send_end)
            .field("send_sent", &self.send_sent)
            .field("send_acked", &self.send_acked)
            .field("recv_buf", &self.recv_buf.get(..8).unwrap_or(&[]))
            .field("recv_end", &self.recv_end)
            .field("recv_start", &self.recv_start)
            .field("options", &self.options.get(..8).unwrap_or(&[]))
            .field("options_len", &self.options_len)
            .field("queued", &self.queued)
            .field("rto", &self.rto)
            .field("retransmit", &self.retransmit)
            .field("time_wait_deadline", &self.time_wait_deadline)
            .field("pending_fin", &self.pending_fin)
            .field("error", &self.error)
            .field("ts_value", &self.ts_value)
            .field("ts_recent", &self.ts_recent)
            .field("last_send_time", &self.last_send_time)
            .field("last_send_seq", &self.last_send_seq)
            .finish()
    }
}

impl TcpConnection {
    /// Creates a connection that is ready to open.
    ///
    /// `initial_seq` is the initial send sequence number. The
    /// connection starts in the closed state; [`TcpConnection::open`]
    /// queues the synchronize segment.
    #[must_use]
    pub fn new_client(
        local: Endpoint,
        remote: Endpoint,
        initial_seq: u32,
        config: TcpConfig,
    ) -> Self {
        let mut timer = RetransmitTimer::new(config.initial_rto_millis);
        timer.set_rto_millis(config.initial_rto_millis);
        Self {
            state: TcpState::Closed,
            local,
            remote,
            config,
            iss: initial_seq,
            irs: 0,
            snd_una: initial_seq,
            snd_nxt: initial_seq,
            snd_wnd: 0,
            rcv_nxt: 0,
            peer_mss: 536,
            peer_window_scale: 0,
            local_window_scale: config.window_scale.min(TCP_MAX_WINDOW_SCALE),
            send_buf: [0; TCP_SEND_BUFFER_SIZE],
            send_base: seq_add(initial_seq, 1),
            send_end: 0,
            send_sent: 0,
            send_acked: 0,
            recv_buf: [0; TCP_RECV_BUFFER_SIZE],
            recv_end: 0,
            recv_start: 0,
            options: [0; TCP_MAX_OPTIONS_LEN],
            options_len: 0,
            queued: None,
            rto: RtoEstimator::new(),
            retransmit: timer,
            time_wait_deadline: None,
            pending_fin: false,
            error: None,
            ts_value: 0,
            ts_recent: 0,
            last_send_time: None,
            last_send_seq: initial_seq,
        }
    }

    /// Returns the current state of the connection.
    #[must_use]
    pub const fn state(&self) -> TcpState {
        self.state
    }

    /// Returns the reason the connection was aborted.
    #[must_use]
    pub const fn error(&self) -> Option<TcpError> {
        self.error
    }

    /// Returns the local endpoint.
    #[must_use]
    pub const fn local(&self) -> Endpoint {
        self.local
    }

    /// Returns the remote endpoint.
    #[must_use]
    pub const fn remote(&self) -> Endpoint {
        self.remote
    }

    /// Returns the initial send sequence number.
    #[must_use]
    pub const fn initial_send_sequence(&self) -> u32 {
        self.iss
    }

    /// Returns the initial receive sequence number.
    #[must_use]
    pub const fn initial_receive_sequence(&self) -> u32 {
        self.irs
    }

    /// Returns the first sequence number that is not acknowledged.
    #[must_use]
    pub const fn send_unacknowledged(&self) -> u32 {
        self.snd_una
    }

    /// Returns the sequence number of the next byte to send.
    #[must_use]
    pub const fn send_next(&self) -> u32 {
        self.snd_nxt
    }

    /// Returns the next sequence number that is expected.
    #[must_use]
    pub const fn receive_next(&self) -> u32 {
        self.rcv_nxt
    }

    /// Returns the window the peer advertised.
    #[must_use]
    pub const fn peer_window(&self) -> u32 {
        self.snd_wnd
    }

    /// Returns the window this side advertises.
    #[must_use]
    pub fn advertised_window(&self) -> u16 {
        let free = TCP_RECV_BUFFER_SIZE.saturating_sub(self.recv_end);
        if free == 0 {
            return 0;
        }
        let scaled = free >> self.local_window_scale;
        u16::try_from(scaled).unwrap_or(u16::MAX).max(1)
    }

    /// Returns the maximum segment size that is used for outgoing
    /// data.
    #[must_use]
    pub fn effective_mss(&self) -> u16 {
        self.peer_mss.min(self.config.local_mss).max(1)
    }

    /// Returns the number of bytes that wait for transmission.
    #[must_use]
    pub const fn queued_data(&self) -> usize {
        self.send_end - self.send_sent
    }

    /// Returns the number of bytes that were received but not read.
    #[must_use]
    pub const fn available_data(&self) -> usize {
        self.recv_end - self.recv_start
    }

    /// Returns the bytes that were received but not read.
    #[must_use]
    pub fn received_data(&self) -> &[u8] {
        self.recv_buf
            .get(self.recv_start..self.recv_end)
            .unwrap_or(&[])
    }

    /// Copies received bytes out of the receive buffer.
    ///
    /// Returns the number of bytes that were copied.
    pub fn drain(&mut self, out: &mut [u8]) -> usize {
        let available = self.available_data();
        let count = available.min(out.len());
        if count == 0 {
            return 0;
        }
        let source = self
            .recv_buf
            .get(self.recv_start..self.recv_start + count)
            .unwrap_or(&[]);
        let target = out.get_mut(..count).unwrap_or(&mut []);
        target.copy_from_slice(source);
        self.recv_start += count;
        self.compact_receive();
        count
    }

    /// Moves the unread bytes to the front of the receive buffer.
    fn compact_receive(&mut self) {
        let remaining = self.recv_end - self.recv_start;
        if remaining > 0 {
            self.recv_buf.copy_within(self.recv_start..self.recv_end, 0);
        }
        self.recv_end = remaining;
        self.recv_start = 0;
    }

    /// Queues data for transmission.
    ///
    /// Returns the number of bytes that were accepted. Bytes that do
    /// not fit into the send buffer are dropped.
    pub fn send_data(&mut self, data: &[u8]) -> usize {
        if self.send_end == TCP_SEND_BUFFER_SIZE && self.send_acked > 0 {
            self.compact_send();
        }
        let space = TCP_SEND_BUFFER_SIZE.saturating_sub(self.send_end);
        let count = data.len().min(space);
        if count == 0 {
            return 0;
        }
        let source = data.get(..count).unwrap_or(&[]);
        if let Some(target) = self.send_buf.get_mut(self.send_end..self.send_end + count) {
            target.copy_from_slice(source);
        }
        self.send_end += count;
        count
    }

    /// Moves the unacknowledged bytes to the front of the send buffer.
    fn compact_send(&mut self) {
        let remaining = self.send_end - self.send_acked;
        if remaining > 0 {
            self.send_buf.copy_within(self.send_acked..self.send_end, 0);
        }
        let acked = u32::try_from(self.send_acked).unwrap_or(u32::MAX);
        self.send_base = seq_add(self.send_base, acked);
        self.send_end = remaining;
        self.send_sent -= self.send_acked;
        self.send_acked = 0;
    }

    /// Starts the connection by queueing a synchronize segment.
    ///
    /// Returns `false` when the connection is not closed.
    pub fn open(&mut self) -> bool {
        if self.state != TcpState::Closed {
            return false;
        }
        self.state = TcpState::SynSent;
        self.snd_una = self.iss;
        self.snd_nxt = seq_add(self.iss, 1);
        let window = self.advertised_window();
        self.queue_segment(TcpFlags::SYN, self.iss, 0, window, None);
        true
    }

    /// Requests an orderly close of the connection.
    ///
    /// Returns `false` when the connection cannot be closed from its
    /// current state.
    pub fn close(&mut self) -> bool {
        match self.state {
            TcpState::Established | TcpState::CloseWait => {
                self.pending_fin = true;
                true
            }
            _ => false,
        }
    }

    /// Aborts the connection immediately.
    pub fn abort(&mut self) {
        self.abort_with(TcpError::Aborted);
    }

    /// Processes an inbound segment.
    ///
    /// `now_millis` is the current time and is used to measure the
    /// round trip time of the acknowledged data.
    pub fn on_segment(&mut self, segment: &TcpHeader<'_>, now_millis: u64) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        if flags.is_set(TcpFlags::RST) {
            if self.state == TcpState::Closed {
                return TcpOutcome::Reset;
            }
            self.abort_with(TcpError::ConnectionReset);
            return TcpOutcome::Reset;
        }
        match self.state {
            TcpState::Closed => {
                self.queue_reset(segment);
                TcpOutcome::Reset
            }
            TcpState::SynSent => self.on_syn_sent(segment),
            TcpState::SynReceived => self.on_syn_received(segment),
            TcpState::Established => self.on_established(segment, now_millis),
            TcpState::FinWait1 => self.on_fin_wait1(segment, now_millis),
            TcpState::FinWait2 => self.on_fin_wait2(segment, now_millis),
            TcpState::CloseWait => self.on_close_wait(segment, now_millis),
            TcpState::Closing => self.on_closing(segment, now_millis),
            TcpState::LastAck => self.on_last_ack(segment, now_millis),
            TcpState::TimeWait => self.on_time_wait(segment),
        }
    }

    /// Returns the next segment that has to be transmitted.
    ///
    /// The call also advances the timers. `now_millis` is the current
    /// time.
    #[must_use]
    pub fn poll(&mut self, now_millis: u64) -> Option<TcpSegment<'_>> {
        self.expire_time_wait(now_millis);
        if let Some(queued) = self.take_queued(now_millis) {
            return Some(self.describe(queued));
        }
        if self.retransmit.expired(now_millis) {
            if self.retransmit.attempts() >= self.config.max_retries {
                self.abort_with(TcpError::Timeout);
                return None;
            }
            let queued = self.retransmit_segment();
            self.retransmit.backoff();
            self.retransmit.restart(now_millis);
            return Some(self.describe(queued));
        }
        let queued = self.build_data_segment();
        if let Some(segment) = queued {
            if let Some(next) = segment.next_state {
                self.state = next;
            }
            self.note_sent(&segment, now_millis);
            return Some(self.describe(segment));
        }
        None
    }

    /// Serializes a segment into `out` with the correct checksum.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when `out` cannot hold
    /// the segment.
    pub fn encode(&self, out: &mut [u8], segment: &TcpSegment<'_>) -> Result<usize, StackError> {
        write_segment(
            out,
            self.local.port,
            self.remote.port,
            segment,
            &self.local.address,
            &self.remote.address,
        )
    }

    // -- internals ----------------------------------------------------------

    /// Aborts the connection with the given reason.
    fn abort_with(&mut self, error: TcpError) {
        self.state = TcpState::Closed;
        self.error = Some(error);
        self.retransmit.stop();
        self.time_wait_deadline = None;
        self.queued = None;
    }

    /// Moves out of the time-wait state when its timer expired.
    fn expire_time_wait(&mut self, now_millis: u64) {
        if self.state != TcpState::TimeWait {
            return;
        }
        if let Some(deadline) = self.time_wait_deadline
            && now_millis >= deadline
        {
            self.state = TcpState::Closed;
            self.time_wait_deadline = None;
        }
    }

    /// Reads the options of an inbound segment.
    fn peer_options<'a>(segment: &'a TcpHeader<'a>) -> TcpOptions<'a> {
        TcpOptions::new(segment_options(segment))
    }

    /// Stores the options the peer announced.
    fn learn_peer_options(&mut self, segment: &TcpHeader<'_>) {
        let options = Self::peer_options(segment);
        if let Some(mss) = options.mss() {
            self.peer_mss = mss.max(1);
        }
        if let Some(shift) = options.window_scale() {
            self.peer_window_scale = shift.min(TCP_MAX_WINDOW_SCALE);
        }
        if let Some((value, _)) = options.timestamps() {
            self.ts_recent = value;
        }
    }

    /// Builds the options area of an outgoing segment.
    fn build_options(&mut self, with_mss: bool) -> usize {
        self.options = [0; TCP_MAX_OPTIONS_LEN];
        let mut length = 0_usize;
        if with_mss {
            if let Some(slot) = self.options.get_mut(length..length + 4) {
                let _ = write_mss(slot, self.config.local_mss);
                length += 4;
            }
            if self.local_window_scale > 0
                && let Some(slot) = self.options.get_mut(length..length + 4)
            {
                let _ = write_window_scale(slot, self.local_window_scale);
                length += 4;
            }
        }
        if self.config.timestamps
            && let Some(slot) = self.options.get_mut(length..length + 12)
        {
            self.ts_value = self.ts_value.wrapping_add(1);
            let _ = write_timestamp(slot, self.ts_value, self.ts_recent);
            length += 12;
        }
        self.options_len = length;
        length
    }

    /// Queues a segment for transmission.
    fn queue_segment(
        &mut self,
        flags: TcpFlags,
        seq: u32,
        ack: u32,
        window: u16,
        next_state: Option<TcpState>,
    ) {
        let with_mss = flags.is_set(TcpFlags::SYN);
        let sequence_length = u32::from(flags.is_set(TcpFlags::SYN) || flags.is_set(TcpFlags::FIN));
        let options_len = self.build_options(with_mss);
        self.queued = Some(QueuedSegment {
            flags,
            seq,
            ack,
            window,
            payload_offset: 0,
            payload_len: 0,
            options_len,
            sequence_length,
            next_state,
        });
    }

    /// Queues a plain acknowledgment.
    fn queue_ack(&mut self) {
        if let Some(queued) = self.queued
            && queued.payload_len > 0
        {
            // A data segment must not be dropped.
            return;
        }
        let window = self.advertised_window();
        self.queue_segment(TcpFlags::ACK, self.snd_nxt, self.rcv_nxt, window, None);
    }

    /// Queues a reset for an unexpected segment.
    fn queue_reset(&mut self, segment: &TcpHeader<'_>) {
        let payload_len = u32::try_from(segment.payload.len()).unwrap_or(u32::MAX);
        if segment.flags & TCP_FLAG_ACK != 0 {
            self.queue_segment(TcpFlags::RST | TcpFlags::ACK, segment.ack, 0, 0, None);
        } else {
            self.queue_segment(TcpFlags::RST, 0, seq_add(segment.seq, payload_len), 0, None);
        }
    }

    /// Takes the queued segment and applies its state change.
    fn take_queued(&mut self, now_millis: u64) -> Option<QueuedSegment> {
        let queued = self.queued.take()?;
        if let Some(next) = queued.next_state {
            self.state = next;
        }
        self.note_sent(&queued, now_millis);
        Some(queued)
    }

    /// Arms the retransmission timer for a segment that was sent.
    fn note_sent(&mut self, queued: &QueuedSegment, now_millis: u64) {
        if queued.sequence_length == 0 {
            return;
        }
        self.retransmit.set_rto_millis(self.rto.rto_millis());
        self.retransmit.restart(now_millis);
        self.last_send_time = Some(now_millis);
        self.last_send_seq = queued.seq;
    }

    /// Turns a queued segment into a transmittable description.
    fn describe(&self, queued: QueuedSegment) -> TcpSegment<'_> {
        let payload = self
            .send_buf
            .get(queued.payload_offset..queued.payload_offset + queued.payload_len)
            .unwrap_or(&[]);
        let options = self.options.get(..queued.options_len).unwrap_or(&[]);
        TcpSegment {
            flags: queued.flags,
            seq: queued.seq,
            ack: queued.ack,
            window: queued.window,
            urgent_pointer: 0,
            payload,
            options,
        }
    }

    /// Builds the segment that repeats the outstanding data.
    fn retransmit_segment(&self) -> QueuedSegment {
        let window = self.advertised_window();
        if self.state == TcpState::SynSent || self.state == TcpState::SynReceived {
            return QueuedSegment {
                flags: TcpFlags::SYN,
                seq: self.iss,
                ack: self.rcv_nxt,
                window,
                payload_offset: 0,
                payload_len: 0,
                options_len: self.options_len,
                sequence_length: 1,
                next_state: None,
            };
        }
        let outstanding = self.send_sent - self.send_acked;
        if outstanding > 0 {
            let length = outstanding.min(usize::from(self.effective_mss()));
            let offset = u32::try_from(self.send_acked).unwrap_or(u32::MAX);
            return QueuedSegment {
                flags: TcpFlags::ACK,
                seq: seq_add(self.send_base, offset),
                ack: self.rcv_nxt,
                window,
                payload_offset: self.send_acked,
                payload_len: length,
                options_len: self.options_len,
                sequence_length: u32::try_from(length).unwrap_or(u32::MAX),
                next_state: None,
            };
        }
        QueuedSegment {
            flags: TcpFlags::FIN | TcpFlags::ACK,
            seq: seq_add(self.snd_nxt, u32::MAX),
            ack: self.rcv_nxt,
            window,
            payload_offset: 0,
            payload_len: 0,
            options_len: self.options_len,
            sequence_length: 1,
            next_state: None,
        }
    }

    /// Builds the next data segment, if any data may be sent.
    fn build_data_segment(&mut self) -> Option<QueuedSegment> {
        if self.state != TcpState::Established {
            return None;
        }
        if self.send_end == self.send_sent {
            if self.pending_fin {
                return Some(self.build_fin_segment(TcpState::FinWait1));
            }
            return None;
        }
        let in_flight = u32::try_from(self.send_sent - self.send_acked).unwrap_or(u32::MAX);
        if self.snd_wnd <= in_flight {
            // The window of the peer is closed.
            return None;
        }
        let available = self.snd_wnd - in_flight;
        let mss = u32::from(self.effective_mss());
        let limit = available.min(mss);
        let pending = u32::try_from(self.send_end - self.send_sent).unwrap_or(u32::MAX);
        let length = limit.min(pending);
        if length == 0 {
            return None;
        }
        let offset = self.send_sent;
        let count = usize::try_from(length).unwrap_or(usize::MAX);
        let seq = seq_add(self.send_base, u32::try_from(offset).unwrap_or(u32::MAX));
        self.send_sent += count;
        self.snd_nxt = seq_add(
            self.send_base,
            u32::try_from(self.send_sent).unwrap_or(u32::MAX),
        );
        let window = self.advertised_window();
        let options_len = self.build_options(false);
        Some(QueuedSegment {
            flags: TcpFlags::ACK | TcpFlags::PSH,
            seq,
            ack: self.rcv_nxt,
            window,
            payload_offset: offset,
            payload_len: count,
            options_len,
            sequence_length: length,
            next_state: None,
        })
    }

    /// Builds the segment that carries the finish flag.
    fn build_fin_segment(&mut self, next_state: TcpState) -> QueuedSegment {
        let seq = self.snd_nxt;
        self.snd_nxt = seq_add(self.snd_nxt, 1);
        self.pending_fin = false;
        let window = self.advertised_window();
        let options_len = self.build_options(false);
        QueuedSegment {
            flags: TcpFlags::FIN | TcpFlags::ACK,
            seq,
            ack: self.rcv_nxt,
            window,
            payload_offset: 0,
            payload_len: 0,
            options_len,
            sequence_length: 1,
            next_state: Some(next_state),
        }
    }

    /// Handles a segment in the synchronize sent state.
    fn on_syn_sent(&mut self, segment: &TcpHeader<'_>) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        if !flags.is_set(TcpFlags::ACK) {
            if flags.is_set(TcpFlags::SYN) {
                // The peer opened the connection at the same time.
                self.irs = segment.seq;
                self.rcv_nxt = seq_add(segment.seq, 1);
                self.learn_peer_options(segment);
                let window = self.advertised_window();
                self.queue_segment(
                    TcpFlags::SYN | TcpFlags::ACK,
                    self.snd_nxt,
                    self.rcv_nxt,
                    window,
                    Some(TcpState::SynReceived),
                );
                return TcpOutcome::Response;
            }
            return TcpOutcome::Invalid;
        }
        if segment.ack != self.snd_nxt {
            return TcpOutcome::Invalid;
        }
        if !flags.is_set(TcpFlags::SYN) {
            return TcpOutcome::Invalid;
        }
        self.irs = segment.seq;
        self.rcv_nxt = seq_add(segment.seq, 1);
        self.snd_una = segment.ack;
        self.retransmit.stop();
        self.learn_peer_options(segment);
        // The window of the synchronize segment is never scaled.
        self.snd_wnd = u32::from(segment.window);
        self.state = TcpState::SynReceived;
        let window = self.advertised_window();
        self.queue_segment(
            TcpFlags::ACK,
            self.snd_nxt,
            self.rcv_nxt,
            window,
            Some(TcpState::Established),
        );
        TcpOutcome::Response
    }

    /// Handles a segment in the synchronize received state.
    fn on_syn_received(&mut self, segment: &TcpHeader<'_>) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        if flags.is_set(TcpFlags::SYN) {
            self.queue_reset(segment);
            self.abort_with(TcpError::ProtocolError);
            return TcpOutcome::Invalid;
        }
        if !flags.is_set(TcpFlags::ACK) {
            return TcpOutcome::Invalid;
        }
        self.snd_una = segment.ack;
        self.update_window(segment);
        self.state = TcpState::Established;
        self.on_data(segment, true)
    }

    /// Handles a segment in the established state.
    fn on_established(&mut self, segment: &TcpHeader<'_>, now_millis: u64) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        if flags.is_set(TcpFlags::SYN) {
            self.queue_reset(segment);
            self.abort_with(TcpError::ProtocolError);
            return TcpOutcome::Invalid;
        }
        self.update_window(segment);
        if flags.is_set(TcpFlags::ACK) {
            if seq_gt(segment.ack, self.snd_nxt) {
                self.queue_ack();
                return TcpOutcome::Invalid;
            }
            if seq_gt(self.snd_una, segment.ack) {
                self.snd_una = segment.ack;
                self.discard_acknowledged();
                self.sample_round_trip(segment.ack, now_millis);
                if self.snd_una == self.snd_nxt {
                    self.retransmit.stop();
                }
            }
        }
        let outcome = self.on_data(segment, true);
        if flags.is_set(TcpFlags::FIN) {
            self.rcv_nxt = seq_add(self.rcv_nxt, 1);
            self.queue_ack();
            self.state = TcpState::CloseWait;
            return TcpOutcome::Response;
        }
        outcome
    }

    /// Handles a segment in the first finish wait state.
    fn on_fin_wait1(&mut self, segment: &TcpHeader<'_>, now_millis: u64) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        self.update_window(segment);
        if flags.is_set(TcpFlags::ACK) && seq_gt(self.snd_una, segment.ack) {
            self.snd_una = segment.ack;
            self.discard_acknowledged();
            self.sample_round_trip(segment.ack, now_millis);
        }
        let fin_acked = flags.is_set(TcpFlags::ACK) && segment.ack == self.snd_nxt;
        let outcome = self.on_data(segment, true);
        if flags.is_set(TcpFlags::FIN) {
            self.rcv_nxt = seq_add(self.rcv_nxt, 1);
            self.queue_ack();
            if fin_acked {
                self.enter_time_wait(now_millis);
            } else {
                self.state = TcpState::Closing;
            }
            return TcpOutcome::Response;
        }
        if fin_acked {
            self.state = TcpState::FinWait2;
            self.retransmit.stop();
            return TcpOutcome::Accepted;
        }
        outcome
    }

    /// Handles a segment in the second finish wait state.
    fn on_fin_wait2(&mut self, segment: &TcpHeader<'_>, now_millis: u64) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        self.update_window(segment);
        if flags.is_set(TcpFlags::ACK) && seq_gt(self.snd_una, segment.ack) {
            self.snd_una = segment.ack;
            self.discard_acknowledged();
            self.sample_round_trip(segment.ack, now_millis);
        }
        let outcome = self.on_data(segment, true);
        if flags.is_set(TcpFlags::FIN) {
            self.rcv_nxt = seq_add(self.rcv_nxt, 1);
            self.queue_ack();
            self.enter_time_wait(now_millis);
            return TcpOutcome::Response;
        }
        outcome
    }

    /// Handles a segment in the close wait state.
    fn on_close_wait(&mut self, segment: &TcpHeader<'_>, now_millis: u64) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        self.update_window(segment);
        if flags.is_set(TcpFlags::ACK) && seq_gt(self.snd_una, segment.ack) {
            self.snd_una = segment.ack;
            self.discard_acknowledged();
            self.sample_round_trip(segment.ack, now_millis);
        }
        self.on_data(segment, true)
    }

    /// Handles a segment in the closing state.
    fn on_closing(&mut self, segment: &TcpHeader<'_>, now_millis: u64) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        self.update_window(segment);
        if flags.is_set(TcpFlags::ACK) && seq_gt(self.snd_una, segment.ack) {
            self.snd_una = segment.ack;
            self.discard_acknowledged();
            self.sample_round_trip(segment.ack, now_millis);
        }
        if flags.is_set(TcpFlags::ACK) && segment.ack == self.snd_nxt {
            self.enter_time_wait(now_millis);
            return TcpOutcome::Accepted;
        }
        self.on_data(segment, true)
    }

    /// Handles a segment in the last acknowledgment state.
    fn on_last_ack(&mut self, segment: &TcpHeader<'_>, now_millis: u64) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        self.update_window(segment);
        if flags.is_set(TcpFlags::ACK) {
            if seq_gt(self.snd_una, segment.ack) {
                self.snd_una = segment.ack;
                self.discard_acknowledged();
                self.sample_round_trip(segment.ack, now_millis);
            }
            if segment.ack == self.snd_nxt {
                self.state = TcpState::Closed;
                self.retransmit.stop();
                return TcpOutcome::Accepted;
            }
        }
        self.on_data(segment, true)
    }

    /// Handles a segment in the time-wait state.
    fn on_time_wait(&mut self, segment: &TcpHeader<'_>) -> TcpOutcome {
        let flags = TcpFlags(segment.flags);
        if flags.is_set(TcpFlags::FIN) {
            // The peer did not receive our acknowledgment.
            self.queue_ack();
            return TcpOutcome::Response;
        }
        TcpOutcome::Accepted
    }

    /// Accepts the payload of an inbound segment.
    fn on_data(&mut self, segment: &TcpHeader<'_>, acknowledge: bool) -> TcpOutcome {
        let payload = segment.payload;
        if payload.is_empty() {
            return TcpOutcome::Accepted;
        }
        if seq_gt(segment.seq, self.rcv_nxt) {
            if acknowledge {
                self.queue_ack();
            }
            return TcpOutcome::OutOfWindow;
        }
        let mut skip = 0_usize;
        if seq_lt(segment.seq, self.rcv_nxt) {
            let distance =
                usize::try_from(seq_distance(segment.seq, self.rcv_nxt)).unwrap_or(usize::MAX);
            skip = distance.min(payload.len());
        }
        let fresh = payload.get(skip..).unwrap_or(&[]);
        if fresh.is_empty() {
            return TcpOutcome::Duplicate;
        }
        let space = TCP_RECV_BUFFER_SIZE.saturating_sub(self.recv_end);
        let count = fresh.len().min(space);
        if count == 0 {
            if acknowledge {
                self.queue_ack();
            }
            return TcpOutcome::OutOfWindow;
        }
        if let Some(target) = self.recv_buf.get_mut(self.recv_end..self.recv_end + count) {
            target.copy_from_slice(fresh.get(..count).unwrap_or(&[]));
        }
        self.recv_end += count;
        self.rcv_nxt = seq_add(self.rcv_nxt, u32::try_from(count).unwrap_or(u32::MAX));
        if acknowledge {
            self.queue_ack();
        }
        TcpOutcome::Response
    }

    /// Updates the window of the peer.
    fn update_window(&mut self, segment: &TcpHeader<'_>) {
        self.snd_wnd = u32::from(segment.window) << self.peer_window_scale;
    }

    /// Drops the acknowledged part of the send buffer.
    fn discard_acknowledged(&mut self) {
        let delta = seq_distance(self.send_base, self.snd_una);
        let limit = u32::try_from(self.send_sent).unwrap_or(u32::MAX);
        if delta <= limit {
            self.send_acked = usize::try_from(delta).unwrap_or(self.send_acked);
        }
    }

    /// Feeds a round trip time sample into the estimator.
    fn sample_round_trip(&mut self, ack: u32, now_millis: u64) {
        let Some(sent) = self.last_send_time else {
            return;
        };
        if !seq_ge(ack, self.last_send_seq) {
            return;
        }
        let rtt = u32::try_from(now_millis.saturating_sub(sent)).unwrap_or(u32::MAX);
        self.rto.sample(rtt);
    }

    /// Enters the time-wait state.
    fn enter_time_wait(&mut self, now_millis: u64) {
        self.state = TcpState::TimeWait;
        self.retransmit.stop();
        self.time_wait_deadline = Some(now_millis.saturating_add(TIME_WAIT_MILLIS));
    }
}
