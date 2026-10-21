//! # DNS client
//!
//! Building standard queries and parsing responses.
//!
//! The message format follows `RFC 1035`. Names are encoded as label
//! sequences, and the parser follows the compression pointers of the
//! specification with a bounded number of jumps so that a malformed
//! message cannot loop forever. The header, question and answer
//! parsers are reused from [`crate::protocols::network::application`].

use crate::protocols::network::application::{
    DNS_CLASS_IN, DNS_HEADER_LEN, DNS_MAX_JUMPS, DNS_TYPE_A, DNS_TYPE_AAAA, DNS_TYPE_CNAME,
    DnsAnswer, DnsHeader, DnsQuestion, decode_dns_name, parse_answer, parse_dns_header,
    parse_question,
};
use crate::protocols::network::stack::StackError;
use crate::protocols::network::types::{Ipv4Address, Ipv6Address};

/// The largest number of compression pointers that are followed.
pub const MAX_POINTER_JUMPS: u32 = DNS_MAX_JUMPS;

/// The largest name that can be encoded in a query.
pub const MAX_NAME_LEN: usize = 255;

/// Builds a standard query in `out`.
///
/// The query carries one question. `name` is the domain name in
/// presentation form, for example `www.example.com`.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// query and [`StackError::Malformed`] when `name` is not a valid
/// domain name.
pub fn build_query(
    out: &mut [u8],
    id: u16,
    name: &[u8],
    qtype: u16,
    qclass: u16,
    recursion_desired: bool,
) -> Result<usize, StackError> {
    let name_len = encoded_name_len(name)?;
    let total = DNS_HEADER_LEN
        .checked_add(name_len)
        .and_then(|value| value.checked_add(4))
        .ok_or(StackError::BufferTooSmall)?;
    let buffer = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    buffer[0..2].copy_from_slice(&id.to_be_bytes());
    let flags = if recursion_desired { 0x0100_u16 } else { 0 };
    buffer[2..4].copy_from_slice(&flags.to_be_bytes());
    buffer[4..6].copy_from_slice(&1_u16.to_be_bytes());
    buffer[6..8].copy_from_slice(&[0, 0]);
    buffer[8..10].copy_from_slice(&[0, 0]);
    buffer[10..12].copy_from_slice(&[0, 0]);
    let mut offset = DNS_HEADER_LEN;
    offset = write_name(buffer, offset, name)?;
    let tail = buffer.get_mut(offset..).ok_or(StackError::BufferTooSmall)?;
    tail[0..2].copy_from_slice(&qtype.to_be_bytes());
    tail[2..4].copy_from_slice(&qclass.to_be_bytes());
    Ok(total)
}

/// Returns the length of the wire form of a domain name.
///
/// # Errors
///
/// Returns [`StackError::Malformed`] when a label is empty or longer
/// than 63 bytes, or when the name is longer than 255 bytes.
pub fn encoded_name_len(name: &[u8]) -> Result<usize, StackError> {
    if name.is_empty() {
        return Err(StackError::Malformed);
    }
    let mut total = 1_usize;
    for label in name.split(|byte| *byte == b'.') {
        if label.is_empty() || label.len() > 63 {
            return Err(StackError::Malformed);
        }
        total = total
            .checked_add(label.len() + 1)
            .ok_or(StackError::Malformed)?;
    }
    if total > MAX_NAME_LEN {
        return Err(StackError::Malformed);
    }
    Ok(total)
}

/// Writes the wire form of a domain name into `buffer` at `offset`.
///
/// # Errors
///
/// Returns [`StackError::Malformed`] when the name is invalid and
/// [`StackError::BufferTooSmall`] when the buffer is too small.
pub fn write_name(buffer: &mut [u8], offset: usize, name: &[u8]) -> Result<usize, StackError> {
    let total = encoded_name_len(name)?;
    let end = offset
        .checked_add(total)
        .ok_or(StackError::BufferTooSmall)?;
    let mut position = offset;
    for label in name.split(|byte| *byte == b'.') {
        let length = u8::try_from(label.len()).map_err(|_| StackError::Malformed)?;
        let slot = buffer.get_mut(position).ok_or(StackError::BufferTooSmall)?;
        *slot = length;
        position += 1;
        let target = buffer
            .get_mut(position..position + label.len())
            .ok_or(StackError::BufferTooSmall)?;
        target.copy_from_slice(label);
        position += label.len();
    }
    let terminator = buffer.get_mut(position).ok_or(StackError::BufferTooSmall)?;
    *terminator = 0;
    let _ = end;
    Ok(position + 1)
}

/// Decodes a domain name at `offset` into `out`.
///
/// The returned tuple holds the length of the decoded name and the
/// offset just after the name. At most [`MAX_POINTER_JUMPS`]
/// compression pointers are followed, which rejects messages whose
/// pointers form a loop.
///
/// # Errors
///
/// Returns `None` when the name is malformed, when a pointer leaves
/// the message or when the pointer limit is reached.
#[must_use]
pub fn decode_name(message: &[u8], offset: usize, out: &mut [u8]) -> Option<(usize, usize)> {
    decode_dns_name(message, offset, out)
}

/// An iterator over the answers of a DNS response.
#[derive(Clone, Debug)]
pub struct DnsAnswerIter<'a> {
    message: &'a [u8],
    offset: usize,
    remaining: u16,
}

impl<'a> Iterator for DnsAnswerIter<'a> {
    type Item = DnsAnswer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let record = parse_answer(self.message, self.offset)?;
        self.offset = record.next_offset;
        self.remaining -= 1;
        Some(record)
    }
}

impl<'a> IntoIterator for &DnsAnswerIter<'a> {
    type Item = DnsAnswer<'a>;
    type IntoIter = DnsAnswerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.clone()
    }
}

impl DnsAnswerIter<'_> {
    /// Returns an iterator over the answers.
    #[must_use]
    pub fn iter(&self) -> DnsAnswerIter<'_> {
        self.clone()
    }
}

/// A DNS response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DnsResponse<'a> {
    /// The header of the response.
    pub header: DnsHeader,
    /// The complete message.
    pub raw: &'a [u8],
}

impl<'a> DnsResponse<'a> {
    /// Parses a response from `data`.
    ///
    /// # Errors
    ///
    /// Returns `None` when the message is shorter than the header.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let header = parse_dns_header(data)?;
        Some(DnsResponse { header, raw: data })
    }

    /// Returns the response code.
    #[must_use]
    pub fn rcode(&self) -> u8 {
        self.header.rcode()
    }

    /// Returns the first question of the response.
    ///
    /// # Errors
    ///
    /// Returns `None` when the message has no question or when the
    /// question is malformed.
    #[must_use]
    pub fn question(&self) -> Option<DnsQuestion<'a>> {
        if self.header.qd_count == 0 {
            return None;
        }
        parse_question(self.raw, DNS_HEADER_LEN)
    }

    /// Returns the answer at `index`, zero-based.
    ///
    /// # Errors
    ///
    /// Returns `None` when the answer does not exist or is malformed.
    #[must_use]
    pub fn answer(&self, index: usize) -> Option<DnsAnswer<'a>> {
        self.answers().nth(index)
    }

    /// Returns an iterator over the answers.
    #[must_use]
    pub fn answers(&self) -> DnsAnswerIter<'a> {
        let offset = self.after_questions().unwrap_or(DNS_HEADER_LEN);
        DnsAnswerIter {
            message: self.raw,
            offset,
            remaining: self.header.an_count,
        }
    }

    /// Returns the first IPv4 address of the answer section.
    ///
    /// # Errors
    ///
    /// Returns `None` when the response holds no address record.
    #[must_use]
    pub fn first_ipv4(&self) -> Option<Ipv4Address> {
        self.answers()
            .find(DnsAnswer::is_a)
            .and_then(|answer| answer.a_address())
    }

    /// Returns the first IPv6 address of the answer section.
    ///
    /// # Errors
    ///
    /// Returns `None` when the response holds no address record.
    #[must_use]
    pub fn first_ipv6(&self) -> Option<Ipv6Address> {
        self.answers()
            .find(DnsAnswer::is_aaaa)
            .and_then(|answer| answer.aaaa_address())
    }

    /// Returns the offset after the question section.
    ///
    /// # Errors
    ///
    /// Returns `None` when a question is malformed.
    fn after_questions(&self) -> Option<usize> {
        let mut offset = DNS_HEADER_LEN;
        let mut remaining = self.header.qd_count;
        while remaining > 0 {
            let question = parse_question(self.raw, offset)?;
            offset = question.next_offset;
            remaining -= 1;
        }
        Some(offset)
    }
}

/// Builds a query for an IPv4 address record.
#[must_use]
pub const fn address_record_type() -> u16 {
    DNS_TYPE_A
}

/// Builds a query for an IPv6 address record.
#[must_use]
pub const fn address_record_type_v6() -> u16 {
    DNS_TYPE_AAAA
}

/// Returns the query type of a canonical name record.
#[must_use]
pub const fn canonical_name_type() -> u16 {
    DNS_TYPE_CNAME
}

/// Returns the query class of the internet.
#[must_use]
pub const fn internet_class() -> u16 {
    DNS_CLASS_IN
}
