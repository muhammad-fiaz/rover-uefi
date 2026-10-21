//! # Network application protocols
//!
//! Bounded, heapless parsers for DNS, DHCP and HTTP messages.
//!
//! All parsers operate on borrowed slices and return `Option` rather
//! than panicking. DNS name decompression follows at most two
//! compression pointers, DHCP option iteration handles pad and end
//! codes, and HTTP header iteration is line-oriented. Nothing here
//! allocates.

use crate::base::bytes::{read_u16_be, read_u32_be};

use crate::protocols::network::types::{Ipv4Address, Ipv6Address};

/// Length of the DNS header in bytes.
pub const DNS_HEADER_LEN: usize = 12;
/// DNS resource record type for an IPv4 address.
pub const DNS_TYPE_A: u16 = 1;
/// DNS resource record type for a name server.
pub const DNS_TYPE_NS: u16 = 2;
/// DNS resource record type for a canonical name.
pub const DNS_TYPE_CNAME: u16 = 5;
/// DNS resource record type for an IPv6 address.
pub const DNS_TYPE_AAAA: u16 = 28;
/// DNS class for the internet.
pub const DNS_CLASS_IN: u16 = 1;
/// Maximum compression jumps followed while decoding a name.
pub const DNS_MAX_JUMPS: u32 = 2;

/// Length of a fixed BOOTP / DHCP header before the magic cookie.
pub const DHCP_COOKIE_OFFSET: usize = 236;
/// Minimum DHCP packet length including the magic cookie.
pub const DHCP_MIN_LEN: usize = 240;
/// DHCP magic cookie: `63 53 82 63` in wire order.
pub const DHCP_MAGIC_COOKIE: [u8; 4] = [0x63, 0x53, 0x82, 0x63];
/// DHCP option code for padding.
pub const DHCP_OPT_PAD: u8 = 0;
/// DHCP option code for subnet mask.
pub const DHCP_OPT_SUBNET_MASK: u8 = 1;
/// DHCP option code for router.
pub const DHCP_OPT_ROUTER: u8 = 3;
/// DHCP option code for DNS servers.
pub const DHCP_OPT_DNS_SERVERS: u8 = 6;
/// DHCP option code for hostname.
pub const DHCP_OPT_HOSTNAME: u8 = 12;
/// DHCP option code for domain name.
pub const DHCP_OPT_DOMAIN_NAME: u8 = 15;
/// DHCP option code for maximum message size.
pub const DHCP_OPT_MAX_MESSAGE_SIZE: u8 = 57;
/// DHCP option code for message type.
pub const DHCP_OPT_MESSAGE_TYPE: u8 = 53;
/// DHCP option code for end.
pub const DHCP_OPT_END: u8 = 255;

/// DHCP message type for discovery.
pub const DHCP_MSG_DISCOVER: u8 = 1;
/// DHCP message type for offer.
pub const DHCP_MSG_OFFER: u8 = 2;
/// DHCP message type for request.
pub const DHCP_MSG_REQUEST: u8 = 3;
/// DHCP message type for decline.
pub const DHCP_MSG_DECLINE: u8 = 4;
/// DHCP message type for acknowledgment.
pub const DHCP_MSG_ACK: u8 = 5;
/// DHCP message type for negative acknowledgment.
pub const DHCP_MSG_NAK: u8 = 6;
/// DHCP message type for release.
pub const DHCP_MSG_RELEASE: u8 = 7;
/// DHCP message type for inform.
pub const DHCP_MSG_INFORM: u8 = 8;

/// Reads a big-endian `u16` as four ASCII digits into a number, if valid.
fn parse_status_code(digits: &[u8]) -> Option<u16> {
    if digits.len() != 3 {
        return None;
    }
    let mut value: u32 = 0;
    for &digit in digits {
        if !digit.is_ascii_digit() {
            return None;
        }
        value = value
            .checked_mul(10)?
            .checked_add(u32::from(digit - b'0'))?;
    }
    u16::try_from(value).ok()
}

// ---------------------------------------------------------------------------
// DNS
// ---------------------------------------------------------------------------

/// A parsed DNS header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DnsHeader {
    /// Transaction identifier.
    pub id: u16,
    /// Flags field.
    pub flags: u16,
    /// Number of questions.
    pub qd_count: u16,
    /// Number of answers.
    pub an_count: u16,
    /// Number of authority records.
    pub ns_count: u16,
    /// Number of additional records.
    pub ar_count: u16,
}

impl DnsHeader {
    /// Parses a DNS header from `data`.
    #[must_use]
    pub fn parse(data: &[u8]) -> Option<Self> {
        parse_dns_header(data)
    }

    /// Returns the query / response bit.
    #[must_use]
    pub const fn is_response(&self) -> bool {
        (self.flags & 0x8000) != 0
    }

    /// Returns the opcode.
    #[must_use]
    pub fn opcode(&self) -> u8 {
        u8::try_from((self.flags >> 11) & 0x0F).unwrap_or(0)
    }

    /// Returns the authoritative-answer bit.
    #[must_use]
    pub const fn is_authoritative(&self) -> bool {
        (self.flags & 0x0400) != 0
    }

    /// Returns the truncation bit.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        (self.flags & 0x0200) != 0
    }

    /// Returns the recursion-desired bit.
    #[must_use]
    pub const fn is_recursion_desired(&self) -> bool {
        (self.flags & 0x0100) != 0
    }

    /// Returns the recursion-available bit.
    #[must_use]
    pub const fn is_recursion_available(&self) -> bool {
        (self.flags & 0x0080) != 0
    }

    /// Returns the response code.
    #[must_use]
    pub fn rcode(&self) -> u8 {
        u8::try_from(self.flags & 0x000F).unwrap_or(0)
    }
}

/// Parses a DNS header from `data`, requiring at least 12 bytes.
#[must_use]
#[allow(clippy::similar_names)]
pub fn parse_dns_header(data: &[u8]) -> Option<DnsHeader> {
    if data.len() < DNS_HEADER_LEN {
        return None;
    }
    let id = read_u16_be(data, 0)?;
    let flags = read_u16_be(data, 2)?;
    let qd_count = read_u16_be(data, 4)?;
    let an_count = read_u16_be(data, 6)?;
    let ns_count = read_u16_be(data, 8)?;
    let ar_count = read_u16_be(data, 10)?;
    Some(DnsHeader {
        id,
        flags,
        qd_count,
        an_count,
        ns_count,
        ar_count,
    })
}

/// Decodes a DNS name at `offset` into `out`.
///
/// The decoded form is dot-separated labels without a trailing dot or
/// terminator, for example `www.example.com`. Returns the decoded
/// length and the offset immediately after the name, which is
/// `offset + 2` when a compression pointer was followed. At most
/// [`DNS_MAX_JUMPS`] compression jumps are followed; pointers outside
/// the message, truncated labels and invalid label types return
/// `None`.
#[must_use]
pub fn decode_dns_name(message: &[u8], offset: usize, out: &mut [u8]) -> Option<(usize, usize)> {
    let mut position = offset;
    let mut next: Option<usize> = None;
    let mut jumps: u32 = 0;
    let mut out_pos = 0_usize;
    loop {
        let len_byte = *message.get(position)?;
        if len_byte == 0 {
            let next_offset = next.unwrap_or(position.checked_add(1)?);
            return Some((out_pos, next_offset));
        }
        if (len_byte & 0xC0) == 0xC0 {
            let low = *message.get(position.checked_add(1)?)?;
            let pointer = usize::from(u16::from_be_bytes([len_byte & 0x3F, low]) & 0x3FFF);
            jumps = jumps.checked_add(1)?;
            if jumps > DNS_MAX_JUMPS {
                return None;
            }
            if pointer >= message.len() {
                return None;
            }
            if next.is_none() {
                next = Some(position.checked_add(2)?);
            }
            position = pointer;
            continue;
        }
        if (len_byte & 0xC0) != 0 {
            return None;
        }
        let label_len = usize::from(len_byte);
        if label_len == 0 || label_len > 63 {
            return None;
        }
        let label_start = position.checked_add(1)?;
        let label_end = label_start.checked_add(label_len)?;
        let label = message.get(label_start..label_end)?;
        let needed = if out_pos == 0 {
            label_len
        } else {
            label_len.checked_add(1)?
        };
        let out_end = out_pos.checked_add(needed)?;
        if out_end > out.len() {
            return None;
        }
        if out_pos != 0 {
            *out.get_mut(out_pos)? = b'.';
            out_pos = out_pos.checked_add(1)?;
        }
        out.get_mut(out_pos..out_end)?.copy_from_slice(label);
        out_pos = out_end;
        position = label_end;
    }
}

/// A parsed DNS question.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DnsQuestion<'a> {
    /// Offset where the `QNAME` starts in the message.
    pub name_offset: usize,
    /// Question type.
    pub qtype: u16,
    /// Question class.
    pub qclass: u16,
    /// Offset immediately after the question.
    pub next_offset: usize,
    /// The full message this question borrows from.
    pub message: &'a [u8],
}

impl<'a> DnsQuestion<'a> {
    /// Parses a question at `offset` in `message`.
    #[must_use]
    pub fn parse(message: &'a [u8], offset: usize) -> Option<Self> {
        parse_question(message, offset)
    }

    /// Decodes the question name into `out`.
    #[must_use]
    pub fn decoded_name(&self, out: &mut [u8]) -> Option<usize> {
        decode_dns_name(self.message, self.name_offset, out).map(|(len, _)| len)
    }
}

/// A parsed DNS resource record answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DnsAnswer<'a> {
    /// Offset where the `NAME` starts in the message.
    pub name_offset: usize,
    /// Record type.
    pub rtype: u16,
    /// Record class.
    pub rclass: u16,
    /// Time to live in seconds.
    pub ttl: u32,
    /// `RDATA` bytes.
    pub rdata: &'a [u8],
    /// Offset immediately after the record.
    pub next_offset: usize,
    /// The full message this record borrows from.
    pub message: &'a [u8],
}

impl<'a> DnsAnswer<'a> {
    /// Parses a record at `offset` in `message`.
    #[must_use]
    pub fn parse(message: &'a [u8], offset: usize) -> Option<Self> {
        parse_answer(message, offset)
    }

    /// Returns whether the type is `A`.
    #[must_use]
    pub const fn is_a(&self) -> bool {
        self.rtype == DNS_TYPE_A
    }

    /// Returns whether the type is `AAAA`.
    #[must_use]
    pub const fn is_aaaa(&self) -> bool {
        self.rtype == DNS_TYPE_AAAA
    }

    /// Returns whether the type is `CNAME`.
    #[must_use]
    pub const fn is_cname(&self) -> bool {
        self.rtype == DNS_TYPE_CNAME
    }

    /// Returns whether the type is `NS`.
    #[must_use]
    pub const fn is_ns(&self) -> bool {
        self.rtype == DNS_TYPE_NS
    }

    /// Decodes the owner name into `out`.
    #[must_use]
    pub fn decoded_name(&self, out: &mut [u8]) -> Option<usize> {
        decode_dns_name(self.message, self.name_offset, out).map(|(len, _)| len)
    }

    /// Returns the IPv4 address for an `A` record, otherwise `None`.
    #[must_use]
    pub fn a_address(&self) -> Option<Ipv4Address> {
        if self.rtype != DNS_TYPE_A || self.rdata.len() != 4 {
            return None;
        }
        let mut addr = [0_u8; 4];
        addr.copy_from_slice(self.rdata);
        Some(Ipv4Address(addr))
    }

    /// Returns the IPv6 address for an `AAAA` record, otherwise `None`.
    #[must_use]
    pub fn aaaa_address(&self) -> Option<Ipv6Address> {
        if self.rtype != DNS_TYPE_AAAA || self.rdata.len() != 16 {
            return None;
        }
        let mut addr = [0_u8; 16];
        addr.copy_from_slice(self.rdata);
        Some(Ipv6Address(addr))
    }

    /// Decodes a `CNAME` or `NS` target name into `out`.
    ///
    /// The target is read from `RDATA`, which may use compression
    /// relative to the whole message. Returns the decoded length.
    #[must_use]
    pub fn target_name(&self, out: &mut [u8]) -> Option<usize> {
        if self.rtype != DNS_TYPE_CNAME && self.rtype != DNS_TYPE_NS {
            return None;
        }
        // `RDATA` starts immediately before the offset after the
        // record, less the length of `RDATA` itself.
        let rd_start = self.next_offset.checked_sub(self.rdata.len())?;
        decode_dns_name(self.message, rd_start, out).map(|(len, _)| len)
    }
}

/// Parses a DNS question at `offset`.
///
/// The name must be well-formed, the four trailing bytes must
/// fit, and the computed `next_offset` must stay in bounds.
#[must_use]
pub fn parse_question(message: &[u8], offset: usize) -> Option<DnsQuestion<'_>> {
    let after = question_end_offset(message, offset)?;
    let qtype = read_u16_be(message, after)?;
    let qclass = read_u16_be(message, after.checked_add(2)?)?;
    let next_offset = after.checked_add(4)?;
    if next_offset > message.len() {
        return None;
    }
    Some(DnsQuestion {
        name_offset: offset,
        qtype,
        qclass,
        next_offset,
        message,
    })
}

/// Finds the offset immediately after a wire-format name without
/// decoding, following at most [`DNS_MAX_JUMPS`] pointers.
fn question_end_offset(message: &[u8], offset: usize) -> Option<usize> {
    let mut position = offset;
    let mut next: Option<usize> = None;
    let mut jumps: u32 = 0;
    loop {
        let len_byte = *message.get(position)?;
        if len_byte == 0 {
            return Some(next.unwrap_or(position.checked_add(1)?));
        }
        if (len_byte & 0xC0) == 0xC0 {
            let low = *message.get(position.checked_add(1)?)?;
            let pointer = usize::from(u16::from_be_bytes([len_byte & 0x3F, low]) & 0x3FFF);
            jumps = jumps.checked_add(1)?;
            if jumps > DNS_MAX_JUMPS {
                return None;
            }
            if pointer >= message.len() {
                return None;
            }
            if next.is_none() {
                next = Some(position.checked_add(2)?);
            }
            position = pointer;
            continue;
        }
        if (len_byte & 0xC0) != 0 {
            return None;
        }
        let label_len = usize::from(len_byte);
        if label_len == 0 || label_len > 63 {
            return None;
        }
        position = position.checked_add(1)?.checked_add(label_len)?;
        if position > message.len() {
            return None;
        }
    }
}

/// Parses a DNS resource record at `offset`.
///
/// Validates the owner name, the ten fixed bytes that follow it and
/// that `RDLENGTH` fits in the buffer.
#[must_use]
pub fn parse_answer(message: &[u8], offset: usize) -> Option<DnsAnswer<'_>> {
    let after_name = question_end_offset(message, offset)?;
    let rtype = read_u16_be(message, after_name)?;
    let rclass = read_u16_be(message, after_name.checked_add(2)?)?;
    let ttl = read_u32_be(message, after_name.checked_add(4)?)?;
    let rd_len = usize::from(read_u16_be(message, after_name.checked_add(8)?)?);
    let rdata_start = after_name.checked_add(10)?;
    let next_offset = rdata_start.checked_add(rd_len)?;
    if next_offset > message.len() {
        return None;
    }
    let rdata = message.get(rdata_start..next_offset)?;
    Some(DnsAnswer {
        name_offset: offset,
        rtype,
        rclass,
        ttl,
        rdata,
        next_offset,
        message,
    })
}

/// A parsed DNS message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DnsMessage<'a> {
    /// Parsed header.
    pub header: DnsHeader,
    /// The full message.
    pub raw: &'a [u8],
}

impl<'a> DnsMessage<'a> {
    /// Parses a DNS message from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_dns_message(data)
    }

    /// Returns the first question, if any.
    #[must_use]
    pub fn question(&self) -> Option<DnsQuestion<'a>> {
        if self.header.qd_count == 0 {
            return None;
        }
        parse_question(self.raw, DNS_HEADER_LEN)
    }

    /// Returns the offset after all questions, or `None` on error.
    #[must_use]
    pub fn after_questions(&self) -> Option<usize> {
        let mut offset = DNS_HEADER_LEN;
        let mut remaining = u32::from(self.header.qd_count);
        while remaining > 0 {
            let question = parse_question(self.raw, offset)?;
            offset = question.next_offset;
            remaining = remaining.checked_sub(1)?;
        }
        Some(offset)
    }

    /// Returns the answer at `index`, zero-based.
    #[must_use]
    pub fn answer(&self, index: usize) -> Option<DnsAnswer<'a>> {
        let mut offset = self.after_questions()?;
        let mut remaining = index;
        loop {
            let record = parse_answer(self.raw, offset)?;
            if remaining == 0 {
                return Some(record);
            }
            offset = record.next_offset;
            remaining = remaining.checked_sub(1)?;
        }
    }
}

/// Parses a DNS message from `data`.
#[must_use]
pub fn parse_dns_message(data: &[u8]) -> Option<DnsMessage<'_>> {
    let header = parse_dns_header(data)?;
    Some(DnsMessage { header, raw: data })
}

// ---------------------------------------------------------------------------
// DHCP
// ---------------------------------------------------------------------------

/// A single DHCP option.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DhcpOption<'a> {
    /// Option code.
    pub code: u8,
    /// Option length.
    pub length: u8,
    /// Option value bytes.
    pub value: &'a [u8],
}

/// An iterator over DHCP options in wire order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DhcpOptions<'a> {
    /// Options bytes after the magic cookie.
    pub data: &'a [u8],
    /// Current offset into `data`.
    pub offset: usize,
}

impl<'a> DhcpOptions<'a> {
    /// Creates an iterator over `data`.
    #[must_use]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    /// Returns the next option, skipping pads.
    ///
    /// Returns `None` at the end code or when the remaining bytes
    /// are truncated.
    #[must_use]
    pub fn next_option(&mut self) -> Option<DhcpOption<'a>> {
        loop {
            let code = *self.data.get(self.offset)?;
            if code == DHCP_OPT_PAD {
                self.offset = self.offset.checked_add(1)?;
                continue;
            }
            if code == DHCP_OPT_END {
                return None;
            }
            let length = *self.data.get(self.offset.checked_add(1)?)?;
            let value_start = self.offset.checked_add(2)?;
            let value_end = value_start.checked_add(usize::from(length))?;
            if value_end > self.data.len() {
                return None;
            }
            let value = self.data.get(value_start..value_end)?;
            self.offset = value_end;
            return Some(DhcpOption {
                code,
                length,
                value,
            });
        }
    }
}

/// A parsed DHCP packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DhcpPacket<'a> {
    /// Operation: 1 is boot request, 2 is boot reply.
    pub op: u8,
    /// Hardware type.
    pub htype: u8,
    /// Hardware address length.
    pub hlen: u8,
    /// Hop count.
    pub hops: u8,
    /// Transaction identifier.
    pub xid: u32,
    /// Seconds elapsed.
    pub secs: u16,
    /// Flags.
    pub flags: u16,
    /// Client IP address.
    pub ciaddr: Ipv4Address,
    /// Your IP address.
    pub yiaddr: Ipv4Address,
    /// Server IP address.
    pub siaddr: Ipv4Address,
    /// Gateway IP address.
    pub giaddr: Ipv4Address,
    /// Client hardware address, 16 bytes as on the wire.
    pub chaddr: [u8; 16],
    /// Options bytes after the magic cookie.
    pub options: &'a [u8],
    /// The full packet as supplied.
    pub raw: &'a [u8],
}

impl<'a> DhcpPacket<'a> {
    /// Parses a DHCP packet from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_dhcp(data)
    }

    /// Returns an iterator over the options.
    #[must_use]
    pub const fn options_iter(&self) -> DhcpOptions<'a> {
        DhcpOptions::new(self.options)
    }

    /// Returns the message-type option value, if present.
    #[must_use]
    pub fn message_type(&self) -> Option<u8> {
        dhcp_message_type(self.options)
    }
}

/// Parses a DHCP packet.
///
/// Requires at least 240 bytes and the magic cookie at offset 236.
/// The options slice is whatever remains after the cookie.
#[must_use]
#[allow(clippy::similar_names)]
pub fn parse_dhcp(data: &[u8]) -> Option<DhcpPacket<'_>> {
    if data.len() < DHCP_MIN_LEN {
        return None;
    }
    let cookie = data.get(DHCP_COOKIE_OFFSET..DHCP_MIN_LEN)?;
    if cookie != DHCP_MAGIC_COOKIE {
        return None;
    }
    let op = *data.first()?;
    let htype = *data.get(1)?;
    let hlen = *data.get(2)?;
    let hops = *data.get(3)?;
    let xid = read_u32_be(data, 4)?;
    let secs = read_u16_be(data, 8)?;
    let flags = read_u16_be(data, 10)?;
    let ciaddr = read_ipv4_addr(data, 12)?;
    let yiaddr = read_ipv4_addr(data, 16)?;
    let siaddr = read_ipv4_addr(data, 20)?;
    let giaddr = read_ipv4_addr(data, 24)?;
    let chaddr_bytes = data.get(28..44)?;
    let mut chaddr = [0_u8; 16];
    chaddr.copy_from_slice(chaddr_bytes);
    let options = data.get(DHCP_MIN_LEN..)?;
    Some(DhcpPacket {
        op,
        htype,
        hlen,
        hops,
        xid,
        secs,
        flags,
        ciaddr,
        yiaddr,
        siaddr,
        giaddr,
        chaddr,
        options,
        raw: data,
    })
}

/// Returns the DHCP message-type option from raw `options` bytes.
///
/// Handles pad and end codes and returns `None` when the option is
/// absent, empty or truncated.
#[must_use]
pub fn dhcp_message_type(options: &[u8]) -> Option<u8> {
    let mut iter = DhcpOptions::new(options);
    while let Some(option) = iter.next_option() {
        if option.code == DHCP_OPT_MESSAGE_TYPE {
            return option.value.first().copied();
        }
    }
    None
}

/// Finds a DHCP option by `code` in raw `options` bytes.
#[must_use]
pub fn dhcp_find_option(options: &[u8], code: u8) -> Option<&[u8]> {
    let mut iter = DhcpOptions::new(options);
    while let Some(option) = iter.next_option() {
        if option.code == code {
            return Some(option.value);
        }
    }
    None
}

/// Reads four bytes at `offset` into an `Ipv4Address`.
fn read_ipv4_addr(data: &[u8], offset: usize) -> Option<Ipv4Address> {
    let bytes = data.get(offset..offset.checked_add(4)?)?;
    let mut addr = [0_u8; 4];
    addr.copy_from_slice(bytes);
    Some(Ipv4Address(addr))
}

// ---------------------------------------------------------------------------
// HTTP
// ---------------------------------------------------------------------------

/// A single HTTP header field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpHeader<'a> {
    /// Field name bytes.
    pub name: &'a [u8],
    /// Field value bytes with leading whitespace removed.
    pub value: &'a [u8],
}

/// An iterator over HTTP headers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpHeaders<'a> {
    /// The message bytes.
    pub data: &'a [u8],
    /// Current offset.
    pub offset: usize,
    /// Offset at the end of the header section.
    pub end: usize,
}

impl<'a> HttpHeaders<'a> {
    /// Returns the next header field, or `None` at the end.
    #[must_use]
    pub fn next_header(&mut self) -> Option<HttpHeader<'a>> {
        if self.offset >= self.end {
            return None;
        }
        let line = take_line(self.data, self.offset, self.end)?;
        if line.line.is_empty() {
            // Should not happen: `end` already excludes the blank line.
            self.offset = line.next;
            return None;
        }
        let colon = position_of(line.line, b':')?;
        let name = line.line.get(..colon)?;
        if name.is_empty() {
            return None;
        }
        let value_start = colon.checked_add(1)?;
        let mut value = line.line.get(value_start..)?;
        while let Some((&first, rest)) = value.split_first() {
            if first != b' ' && first != b'\t' {
                break;
            }
            value = rest;
        }
        // Trim trailing whitespace as well.
        while let Some((prefix, &last)) = value.split_last_rev_pair() {
            if last != b' ' && last != b'\t' && last != b'\r' {
                break;
            }
            value = prefix;
        }
        self.offset = line.next;
        Some(HttpHeader { name, value })
    }
}

/// Helper pair for trimming a trailing byte from a slice.
trait SplitLastRevPair {
    /// Splits at the last byte, returning `(prefix, last)`.
    fn split_last_rev_pair(&self) -> Option<(&[u8], &u8)>;
}

impl SplitLastRevPair for [u8] {
    fn split_last_rev_pair(&self) -> Option<(&[u8], &u8)> {
        if self.is_empty() {
            return None;
        }
        let at = self.len().checked_sub(1)?;
        let (all_but_last, last_slice) = self.split_at(at);
        let last = last_slice.first()?;
        Some((all_but_last, last))
    }
}

/// A line slice plus the offset after its line ending.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Line<'a> {
    /// Line without its terminator.
    line: &'a [u8],
    /// Offset after the terminator.
    next: usize,
}

/// Takes one `CRLF` or `LF` terminated line starting at `offset`.
fn take_line(data: &[u8], offset: usize, end: usize) -> Option<Line<'_>> {
    if offset > end || end > data.len() {
        return None;
    }
    let slice = data.get(offset..end)?;
    let mut index = 0_usize;
    while index < slice.len() {
        let byte = *slice.get(index)?;
        if byte == b'\n' {
            let line_end = if index > 0 && *slice.get(index.checked_sub(1)?)? == b'\r' {
                index.checked_sub(1)?
            } else {
                index
            };
            let line = slice.get(..line_end)?;
            let next = offset.checked_add(index)?.checked_add(1)?;
            return Some(Line { line, next });
        }
        index = index.checked_add(1)?;
    }
    // No terminator: the rest of the slice is the line.
    let next = end;
    Some(Line { line: slice, next })
}

/// Returns the position of the first `needle` byte in `haystack`.
fn position_of(haystack: &[u8], needle: u8) -> Option<usize> {
    let mut index = 0_usize;
    while index < haystack.len() {
        if *haystack.get(index)? == needle {
            return Some(index);
        }
        index = index.checked_add(1)?;
    }
    None
}

/// Trims ASCII whitespace on both ends of `bytes`.
#[must_use]
fn trim_ascii(bytes: &[u8]) -> &[u8] {
    let mut start = 0_usize;
    let mut end = bytes.len();
    while start < end {
        let Some(&byte) = bytes.get(start) else {
            break;
        };
        if byte != b' ' && byte != b'\t' && byte != b'\r' && byte != b'\n' {
            break;
        }
        let Some(next) = start.checked_add(1) else {
            break;
        };
        start = next;
    }
    while end > start {
        let Some(predecessor) = end.checked_sub(1) else {
            break;
        };
        let Some(&byte) = bytes.get(predecessor) else {
            break;
        };
        if byte != b' ' && byte != b'\t' && byte != b'\r' && byte != b'\n' {
            break;
        }
        end = predecessor;
    }
    bytes.get(start..end).unwrap_or(&[])
}

/// Parses an HTTP request line into method, target and version.
#[must_use]
pub fn parse_request_line(line: &[u8]) -> Option<(&[u8], &[u8], &[u8])> {
    let first_space = position_of(line, b' ')?;
    let after_first = line.get(first_space.checked_add(1)?..)?;
    let second_rel = position_of(after_first, b' ')?;
    let second_space = second_rel.checked_add(first_space)?.checked_add(1)?;
    // Exactly two spaces separate the three fields.
    let after_second = line.get(second_space.checked_add(1)?..)?;
    if position_of(after_second, b' ').is_some() {
        return None;
    }
    let method = line.get(..first_space)?;
    let target_start = first_space.checked_add(1)?;
    let target = line.get(target_start..second_space)?;
    let version_start = second_space.checked_add(1)?;
    let version = line.get(version_start..)?;
    if method.is_empty()
        || target.is_empty()
        || version.is_empty()
        || !version.starts_with(b"HTTP/")
    {
        return None;
    }
    Some((method, target, version))
}

/// Parses an HTTP status line into version, code and reason.
#[must_use]
pub fn parse_status_line(line: &[u8]) -> Option<(&[u8], u16, &[u8])> {
    let first_space = position_of(line, b' ')?;
    let version = line.get(..first_space)?;
    let code_start = first_space.checked_add(1)?;
    let after_code = line.get(code_start..)?;
    if after_code.len() < 3 {
        return None;
    }
    let digits = after_code.get(..3)?;
    let code = parse_status_code(digits)?;
    let rest = after_code.get(3..)?;
    let reason = if rest.is_empty() {
        &[]
    } else if rest.first() == Some(&b' ') {
        rest.get(1..)?
    } else {
        return None;
    };
    if version.is_empty() || !version.starts_with(b"HTTP/") {
        return None;
    }
    Some((version, code, reason))
}

/// Finds the header/body split.
///
/// Returns `(headers_end, body_start)` where `headers_end` is the
/// offset at the blank line and `body_start` is the offset after it.
/// Handles both `CRLF` and `LF` endings. Returns `None` when the
/// header section is unterminated.
fn header_body_split(data: &[u8], start: usize) -> Option<(usize, usize)> {
    let mut offset = start;
    loop {
        let line = take_line(data, offset, data.len())?;
        if line.line.is_empty() {
            let headers_end = offset;
            return Some((headers_end, line.next));
        }
        if line.next <= offset || line.next > data.len() {
            return None;
        }
        offset = line.next;
        if offset >= data.len() {
            // Headers ran to EOF with no blank line: no body.
            return Some((offset, offset));
        }
    }
}

/// A parsed HTTP request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpRequest<'a> {
    /// HTTP method bytes.
    pub method: &'a [u8],
    /// Request target bytes.
    pub target: &'a [u8],
    /// HTTP version bytes, for example `HTTP/1.1`.
    pub version: &'a [u8],
    /// The full message.
    pub raw: &'a [u8],
    /// Offset where the header section starts.
    pub headers_offset: usize,
    /// Offset at the end of the header section.
    pub headers_end: usize,
    /// Offset where the body starts.
    pub body_offset: usize,
}

impl<'a> HttpRequest<'a> {
    /// Parses an HTTP request from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_http_request(data)
    }

    /// Returns the body bytes after the header section.
    #[must_use]
    pub fn body(&self) -> &'a [u8] {
        self.raw.get(self.body_offset..).unwrap_or(&[])
    }

    /// Returns an iterator over the headers.
    #[must_use]
    pub const fn headers_iter(&self) -> HttpHeaders<'a> {
        HttpHeaders {
            data: self.raw,
            offset: self.headers_offset,
            end: self.headers_end,
        }
    }
}

/// A parsed HTTP response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HttpResponse<'a> {
    /// HTTP version bytes.
    pub version: &'a [u8],
    /// Status code.
    pub status_code: u16,
    /// Reason phrase bytes.
    pub reason: &'a [u8],
    /// The full message.
    pub raw: &'a [u8],
    /// Offset where the header section starts.
    pub headers_offset: usize,
    /// Offset at the end of the header section.
    pub headers_end: usize,
    /// Offset where the body starts.
    pub body_offset: usize,
}

impl<'a> HttpResponse<'a> {
    /// Parses an HTTP response from `data`.
    #[must_use]
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        parse_http_response(data)
    }

    /// Returns the body bytes after the header section.
    #[must_use]
    pub fn body(&self) -> &'a [u8] {
        self.raw.get(self.body_offset..).unwrap_or(&[])
    }

    /// Returns an iterator over the headers.
    #[must_use]
    pub const fn headers_iter(&self) -> HttpHeaders<'a> {
        HttpHeaders {
            data: self.raw,
            offset: self.headers_offset,
            end: self.headers_end,
        }
    }
}

/// Parses an HTTP request from `data`.
#[must_use]
pub fn parse_http_request(data: &[u8]) -> Option<HttpRequest<'_>> {
    let first = take_line(data, 0, data.len())?;
    if first.line.is_empty() {
        return None;
    }
    let (method, target, version) = parse_request_line(first.line)?;
    let (headers_end, body_offset) = header_body_split(data, first.next)?;
    Some(HttpRequest {
        method,
        target,
        version,
        raw: data,
        headers_offset: first.next,
        headers_end,
        body_offset,
    })
}

/// Parses an HTTP response from `data`.
#[must_use]
pub fn parse_http_response(data: &[u8]) -> Option<HttpResponse<'_>> {
    let first = take_line(data, 0, data.len())?;
    if first.line.is_empty() {
        return None;
    }
    let (version, status_code, reason) = parse_status_line(first.line)?;
    let (headers_end, body_offset) = header_body_split(data, first.next)?;
    Some(HttpResponse {
        version,
        status_code,
        reason,
        raw: data,
        headers_offset: first.next,
        headers_end,
        body_offset,
    })
}

/// Trims and returns `bytes` after removing ASCII whitespace.
///
/// Exposed for callers that parse header values themselves.
#[must_use]
pub fn trim_http_token(bytes: &[u8]) -> &[u8] {
    trim_ascii(bytes)
}
