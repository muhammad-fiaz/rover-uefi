//! # `IGMP`
//!
//! Native encoders and decoders for the Internet Group Management
//! Protocol used over `IPv4`.
//!
//! Membership queries, `IGMPv1` and `IGMPv2` reports, `IGMPv2` leave
//! messages and `IGMPv3` reports are supported. The `IGMPv3` report
//! body is a list of group records, each with a list of source
//! addresses and an opaque auxiliary block.
//!
//! Nothing here allocates and no function panics. Encoders write into
//! a caller-supplied buffer and validate every length before use.

use crate::protocols::network::stack::StackError;
use crate::protocols::network::stack::icmp;
use crate::protocols::network::types::Ipv4Address;

/// The `IPv4` protocol number of `IGMP`.
pub const IP_PROTOCOL_IGMP: u8 = 2;

/// The length of every `IGMP` message header in bytes.
pub const IGMP_HEADER_LEN: usize = 8;

/// The length of an `IGMPv3` membership query header in bytes.
pub const IGMP_V3_QUERY_LEN: usize = 12;

/// The membership query type.
pub const IGMP_MEMBERSHIP_QUERY: u8 = 0x11;

/// The `IGMPv1` membership report type.
pub const IGMP_V1_MEMBERSHIP_REPORT: u8 = 0x12;

/// The `IGMPv2` membership report type.
pub const IGMP_V2_MEMBERSHIP_REPORT: u8 = 0x16;

/// The `IGMPv2` leave group type.
pub const IGMP_V2_LEAVE_GROUP: u8 = 0x17;

/// The `IGMPv3` membership report type.
pub const IGMP_V3_MEMBERSHIP_REPORT: u8 = 0x22;

/// Record type: the source list is in include mode.
pub const RECORD_MODE_IS_INCLUDE: u8 = 1;

/// Record type: the source list is in exclude mode.
pub const RECORD_MODE_IS_EXCLUDE: u8 = 2;

/// Record type: the source list changes from include to exclude.
pub const RECORD_CHANGE_TO_INCLUDE: u8 = 3;

/// Record type: the source list changes from exclude to include.
pub const RECORD_CHANGE_TO_EXCLUDE: u8 = 4;

/// Record type: allow new sources.
pub const RECORD_ALLOW_NEW_SOURCES: u8 = 5;

/// Record type: block old sources.
pub const RECORD_BLOCK_OLD_SOURCES: u8 = 6;

/// A group membership query.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IgmpQuery<'a> {
    /// The encoded maximum response time, in tenths of a second for
    /// the low half of the range.
    pub max_response_code: u8,
    /// The group being queried; zero asks about all groups.
    pub group: Ipv4Address,
    /// Whether routers should suppress their own reports.
    pub suppress_router_processing: bool,
    /// The robustness variable; zero means the default of two.
    pub robustness: u8,
    /// The query interval code.
    pub query_interval_code: u8,
    /// The trailing source address block, or an empty slice when the
    /// message carries no source list.
    pub sources: &'a [u8],
}

impl IgmpQuery<'_> {
    /// Returns the maximum response time encoded by the message, in
    /// milliseconds.
    ///
    /// Codes below 128 are expressed in tenths of a second; larger
    /// codes use the floating-point encoding of `RFC 3376`.
    #[must_use]
    // `u32::from` is not usable in a constant function on stable
    // const traits, so the call is kept out of the constant evaluator.
    #[allow(clippy::missing_const_for_fn)]
    pub fn max_response_time_millis(&self) -> u32 {
        let code = u32::from(self.max_response_code);
        if code < 128 {
            return code * 100;
        }
        let mantissa = (code & 0x0F) | 0x10;
        let exponent = (code >> 4) & 0x07;
        // The shift is bounded by the three-bit exponent field.
        let units = mantissa << (exponent + 3);
        units * 100
    }

    /// Returns the number of source addresses carried by the query.
    #[must_use]
    pub const fn source_count(&self) -> usize {
        self.sources.len() / 4
    }
}

/// One record of an `IGMPv3` membership report.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupRecord<'a> {
    /// The record type.
    pub record_type: u8,
    /// The multicast group the record describes.
    pub multicast_address: Ipv4Address,
    /// The source address list, four bytes per address.
    pub sources: &'a [u8],
    /// The opaque auxiliary data block.
    pub auxiliary: &'a [u8],
}

impl GroupRecord<'_> {
    /// Returns the number of source addresses in the record.
    #[must_use]
    pub const fn source_count(&self) -> usize {
        self.sources.len() / 4
    }
}

/// An `IGMPv3` membership report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IgmpV3Report<'a> {
    /// The group records carried by the report.
    pub records: GroupRecordIter<'a>,
}

/// An iterator over the group records of a membership report.
///
/// The iterator is `Clone` so that a caller can re-walk the record
/// list without copying the underlying message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupRecordIter<'a> {
    rest: &'a [u8],
    remaining: usize,
}

impl<'a> Iterator for GroupRecordIter<'a> {
    type Item = GroupRecord<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let record_type = *self.rest.first()?;
        let auxiliary_words = usize::from(*self.rest.get(1)?);
        let source_count =
            usize::from(u16::from_be_bytes([*self.rest.get(2)?, *self.rest.get(3)?]));
        let group = self.rest.get(4..8)?;
        let source_bytes = source_count.checked_mul(4)?;
        let auxiliary_bytes = auxiliary_words.checked_mul(4)?;
        let header_end = 8_usize;
        let sources_end = header_end.checked_add(source_bytes)?;
        let auxiliary_end = sources_end.checked_add(auxiliary_bytes)?;
        let record = self.rest.get(..auxiliary_end)?;
        self.rest = self.rest.get(auxiliary_end..).unwrap_or(&[]);
        self.remaining -= 1;
        Some(GroupRecord {
            record_type,
            multicast_address: Ipv4Address([
                *group.first()?,
                *group.get(1)?,
                *group.get(2)?,
                *group.get(3)?,
            ]),
            sources: record.get(header_end..sources_end).unwrap_or(&[]),
            auxiliary: record.get(sources_end..auxiliary_end).unwrap_or(&[]),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.remaining))
    }
}

/// A parsed `IGMP` message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IgmpMessage<'a> {
    /// A membership query.
    Query(IgmpQuery<'a>),
    /// A version 1 membership report.
    V1Report(Ipv4Address),
    /// A version 2 membership report.
    V2Report(Ipv4Address),
    /// A version 2 leave group message.
    LeaveGroup(Ipv4Address),
    /// A version 3 membership report.
    V3Report(IgmpV3Report<'a>),
}

impl<'a> IgmpMessage<'a> {
    /// Parses an `IGMP` message.
    ///
    /// Returns `None` for an unknown type, a truncated message or a
    /// report whose record list does not match its record count.
    #[must_use]
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let header = bytes.get(..IGMP_HEADER_LEN)?;
        let kind = *header.first()?;
        let group = group_at(header)?;
        match kind {
            IGMP_MEMBERSHIP_QUERY => {
                let mut query = IgmpQuery {
                    max_response_code: *header.get(1)?,
                    group,
                    suppress_router_processing: false,
                    robustness: 0,
                    query_interval_code: 0,
                    sources: &[],
                };
                if bytes.len() >= IGMP_V3_QUERY_LEN {
                    let body = bytes.get(IGMP_HEADER_LEN..)?;
                    let flags = *body.first()?;
                    query.suppress_router_processing = flags & 0x08 != 0;
                    query.robustness = flags & 0x07;
                    query.query_interval_code = *body.get(1)?;
                    let sources = bytes.get(IGMP_V3_QUERY_LEN..)?;
                    if sources.len() % 4 != 0 {
                        return None;
                    }
                    query.sources = sources;
                }
                Some(Self::Query(query))
            }
            IGMP_V1_MEMBERSHIP_REPORT => Some(Self::V1Report(group)),
            IGMP_V2_MEMBERSHIP_REPORT => Some(Self::V2Report(group)),
            IGMP_V2_LEAVE_GROUP => Some(Self::LeaveGroup(group)),
            IGMP_V3_MEMBERSHIP_REPORT => {
                let count = usize::from(u16::from_be_bytes([*header.get(6)?, *header.get(7)?]));
                let records = bytes.get(IGMP_HEADER_LEN..)?;
                let iter = GroupRecordIter {
                    rest: records,
                    remaining: count,
                };
                // Validate the whole record list up front so that a
                // truncated report is rejected rather than silently
                // shortened.
                let mut remaining = count;
                let mut rest = records;
                while remaining > 0 {
                    let head = rest.get(..4)?;
                    let auxiliary_words = usize::from(*head.get(1)?);
                    let source_count =
                        usize::from(u16::from_be_bytes([*head.get(2)?, *head.get(3)?]));
                    let total = 8_usize
                        .checked_add(source_count.checked_mul(4)?)?
                        .checked_add(auxiliary_words.checked_mul(4)?)?;
                    rest = rest.get(total..)?;
                    remaining -= 1;
                }
                Some(Self::V3Report(IgmpV3Report { records: iter }))
            }
            _ => None,
        }
    }

    /// Returns the message type.
    #[must_use]
    pub const fn message_type(&self) -> u8 {
        match self {
            Self::Query(_) => IGMP_MEMBERSHIP_QUERY,
            Self::V1Report(_) => IGMP_V1_MEMBERSHIP_REPORT,
            Self::V2Report(_) => IGMP_V2_MEMBERSHIP_REPORT,
            Self::LeaveGroup(_) => IGMP_V2_LEAVE_GROUP,
            Self::V3Report(_) => IGMP_V3_MEMBERSHIP_REPORT,
        }
    }
}

/// Builds a version 2 membership query and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// message.
pub fn build_query(
    out: &mut [u8],
    group: Ipv4Address,
    max_response_code: u8,
) -> Result<usize, StackError> {
    build_fixed(out, IGMP_MEMBERSHIP_QUERY, group, max_response_code)
}

/// Builds a version 2 membership report and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// message.
pub fn build_v2_report(out: &mut [u8], group: Ipv4Address) -> Result<usize, StackError> {
    build_fixed(out, IGMP_V2_MEMBERSHIP_REPORT, group, 0)
}

/// Builds a version 2 leave group message and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// message.
pub fn build_leave_group(out: &mut [u8], group: Ipv4Address) -> Result<usize, StackError> {
    build_fixed(out, IGMP_V2_LEAVE_GROUP, group, 0)
}

/// Builds a version 3 membership report and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// header and every record.
pub fn build_v3_report(out: &mut [u8], records: &[GroupRecord<'_>]) -> Result<usize, StackError> {
    let mut total = IGMP_HEADER_LEN;
    for record in records {
        let size = IGMP_HEADER_LEN
            .checked_add(record.sources.len())
            .and_then(|value| value.checked_add(record.auxiliary.len()))
            .ok_or(StackError::BufferTooSmall)?;
        total = total.checked_add(size).ok_or(StackError::BufferTooSmall)?;
    }
    let count = u16::try_from(records.len()).map_err(|_| StackError::BufferTooSmall)?;
    let buffer = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    buffer.fill(0);
    buffer[0] = IGMP_V3_MEMBERSHIP_REPORT;
    buffer
        .get_mut(6..8)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&count.to_be_bytes());
    let mut position = IGMP_HEADER_LEN;
    for record in records {
        let sources =
            u16::try_from(record.source_count()).map_err(|_| StackError::BufferTooSmall)?;
        let auxiliary_words =
            u8::try_from(record.auxiliary.len() / 4).map_err(|_| StackError::BufferTooSmall)?;
        let end = position
            .checked_add(IGMP_HEADER_LEN)
            .and_then(|value| value.checked_add(record.sources.len()))
            .and_then(|value| value.checked_add(record.auxiliary.len()))
            .ok_or(StackError::BufferTooSmall)?;
        let slot = buffer
            .get_mut(position..end)
            .ok_or(StackError::BufferTooSmall)?;
        slot[0] = record.record_type;
        slot[1] = auxiliary_words;
        slot.get_mut(2..4)
            .ok_or(StackError::BufferTooSmall)?
            .copy_from_slice(&sources.to_be_bytes());
        slot.get_mut(4..8)
            .ok_or(StackError::BufferTooSmall)?
            .copy_from_slice(&record.multicast_address.0);
        let sources_start = IGMP_HEADER_LEN;
        let sources_end = sources_start
            .checked_add(record.sources.len())
            .ok_or(StackError::BufferTooSmall)?;
        let auxiliary_end = sources_end
            .checked_add(record.auxiliary.len())
            .ok_or(StackError::BufferTooSmall)?;
        slot.get_mut(sources_start..sources_end)
            .ok_or(StackError::BufferTooSmall)?
            .copy_from_slice(record.sources);
        slot.get_mut(sources_end..auxiliary_end)
            .ok_or(StackError::BufferTooSmall)?
            .copy_from_slice(record.auxiliary);
        position = end;
    }
    write_checksum(buffer)?;
    Ok(total)
}

/// Writes the `IGMP` checksum over the complete message in `out`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// the message header.
pub fn write_checksum(out: &mut [u8]) -> Result<(), StackError> {
    if out.len() < IGMP_HEADER_LEN {
        return Err(StackError::BufferTooSmall);
    }
    out.get_mut(2..4).ok_or(StackError::BufferTooSmall)?.fill(0);
    let value = icmp::checksum(out);
    out.get_mut(2..4)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&value.to_be_bytes());
    Ok(())
}

/// Returns whether the checksum of a complete `IGMP` message is
/// correct.
#[must_use]
pub fn verify_checksum(bytes: &[u8]) -> bool {
    bytes.len() >= IGMP_HEADER_LEN && icmp::checksum(bytes) == 0
}

/// Writes a fixed-size message and returns its length.
fn build_fixed(
    out: &mut [u8],
    kind: u8,
    group: Ipv4Address,
    max_response_code: u8,
) -> Result<usize, StackError> {
    let buffer = out
        .get_mut(..IGMP_HEADER_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    buffer.fill(0);
    buffer[0] = kind;
    buffer[1] = max_response_code;
    buffer
        .get_mut(4..8)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&group.0);
    write_checksum(buffer)?;
    Ok(IGMP_HEADER_LEN)
}

/// Reads the group address carried by a fixed-size message.
fn group_at(header: &[u8]) -> Option<Ipv4Address> {
    let field = header.get(4..8)?;
    Some(Ipv4Address([
        *field.first()?,
        *field.get(1)?,
        *field.get(2)?,
        *field.get(3)?,
    ]))
}
