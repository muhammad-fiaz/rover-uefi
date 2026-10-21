//! # DHCP client
//!
//! A dynamic host configuration protocol client.
//!
//! The client follows the discover, offer, request and acknowledgment
//! exchange described in `RFC 2131`. The bootstrap protocol header of
//! `RFC 951` is built from scratch and the options are encoded and
//! decoded as type, length and value triples.
//!
//! No memory is allocated: the lease record is stored by value and the
//! message buffers are supplied by the caller.

use crate::protocols::network::application::{
    DHCP_COOKIE_OFFSET, DHCP_MAGIC_COOKIE, DHCP_MIN_LEN, DhcpPacket, dhcp_find_option, parse_dhcp,
};

pub use crate::protocols::network::application::{
    DHCP_MSG_ACK, DHCP_MSG_DISCOVER, DHCP_MSG_NAK, DHCP_MSG_OFFER, DHCP_MSG_REQUEST,
    DHCP_OPT_DNS_SERVERS, DHCP_OPT_END, DHCP_OPT_HOSTNAME, DHCP_OPT_MAX_MESSAGE_SIZE,
    DHCP_OPT_MESSAGE_TYPE, DHCP_OPT_ROUTER, DHCP_OPT_SUBNET_MASK,
};
use crate::protocols::network::stack::StackError;
use crate::protocols::network::types::{Ipv4Address, MacAddress};

/// DHCP option code for the broadcast flag.
pub const DHCP_OPT_BROADCAST: u8 = 28;
/// DHCP option code for the requested address.
pub const DHCP_OPT_REQUESTED_IP: u8 = 50;
/// DHCP option code for the lease time.
pub const DHCP_OPT_LEASE_TIME: u8 = 51;
/// DHCP option code for the server identifier.
pub const DHCP_OPT_SERVER_IDENTIFIER: u8 = 54;
/// DHCP option code for the parameter request list.
pub const DHCP_OPT_PARAMETER_REQUEST_LIST: u8 = 55;
/// DHCP option code for the client identifier.
pub const DHCP_OPT_CLIENT_IDENTIFIER: u8 = 61;

/// The bootstrap operation of a request.
pub const DHCP_OP_REQUEST: u8 = 1;
/// The bootstrap operation of a reply.
pub const DHCP_OP_REPLY: u8 = 2;
/// The hardware type of Ethernet.
pub const DHCP_HTYPE_ETHERNET: u8 = 1;

/// The largest number of DNS servers that are remembered.
pub const MAX_DNS_SERVERS: usize = 4;

/// The length of the bootstrap header before the magic cookie.
pub const BOOTP_HEADER_LEN: usize = DHCP_COOKIE_OFFSET;

/// The states of the DHCP client.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DhcpState {
    /// The client has not started yet.
    Init,
    /// The client waits for an offer.
    Select,
    /// The client waits for the acknowledgment.
    Request,
    /// The client holds a lease.
    Bound,
    /// The client renews the lease.
    Renew,
    /// The client rebinds the lease.
    Rebind,
    /// The server refused the address.
    Declined,
    /// The client gave up.
    Fail,
}

/// A bounded list of DNS server addresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DnsServerList {
    servers: [Ipv4Address; MAX_DNS_SERVERS],
    len: usize,
}

impl Default for DnsServerList {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsServerList {
    /// Creates an empty list.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            servers: [Ipv4Address([0, 0, 0, 0]); MAX_DNS_SERVERS],
            len: 0,
        }
    }

    /// Returns the number of stored addresses.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the list is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the address at `index`.
    ///
    /// # Errors
    ///
    /// Returns `None` when the index is out of range.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<Ipv4Address> {
        self.servers
            .get(index)
            .copied()
            .filter(|_| index < self.len)
    }

    /// Returns the first address.
    ///
    /// # Errors
    ///
    /// Returns `None` when the list is empty.
    #[must_use]
    pub fn first(&self) -> Option<Ipv4Address> {
        self.get(0)
    }

    /// Returns the stored addresses.
    #[must_use]
    pub fn as_slice(&self) -> &[Ipv4Address] {
        self.servers.get(..self.len).unwrap_or(&[])
    }

    /// Appends an address.
    ///
    /// Returns `false` when the list is full.
    pub fn push(&mut self, address: Ipv4Address) -> bool {
        if self.len >= MAX_DNS_SERVERS {
            return false;
        }
        if let Some(slot) = self.servers.get_mut(self.len) {
            *slot = address;
        }
        self.len += 1;
        true
    }
}

impl Default for DhcpOffer {
    fn default() -> Self {
        Self {
            address: Ipv4Address([0, 0, 0, 0]),
            server_identifier: Ipv4Address([0, 0, 0, 0]),
            subnet_mask: None,
            router: None,
            dns_servers: DnsServerList::new(),
            lease_seconds: 0,
        }
    }
}

/// The parameters a server offers to a client.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DhcpOffer {
    /// The address the server offers.
    pub address: Ipv4Address,
    /// The identifier of the server that made the offer.
    pub server_identifier: Ipv4Address,
    /// The subnet mask of the offer.
    pub subnet_mask: Option<Ipv4Address>,
    /// The default gateway of the offer.
    pub router: Option<Ipv4Address>,
    /// The name servers of the offer.
    pub dns_servers: DnsServerList,
    /// The lease time in seconds.
    pub lease_seconds: u32,
}

/// A lease that the client holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DhcpLease {
    /// The leased address.
    pub address: Ipv4Address,
    /// The identifier of the server that granted the lease.
    pub server_identifier: Ipv4Address,
    /// The subnet mask of the lease.
    pub subnet_mask: Option<Ipv4Address>,
    /// The default gateway of the lease.
    pub router: Option<Ipv4Address>,
    /// The name servers of the lease.
    pub dns_servers: DnsServerList,
    /// The lease time in seconds.
    pub lease_seconds: u32,
    /// The moment at which the lease was granted, in milliseconds.
    pub acquired_at_millis: u64,
}

impl Default for DhcpLease {
    fn default() -> Self {
        Self {
            address: Ipv4Address([0, 0, 0, 0]),
            server_identifier: Ipv4Address([0, 0, 0, 0]),
            subnet_mask: None,
            router: None,
            dns_servers: DnsServerList::new(),
            lease_seconds: 0,
            acquired_at_millis: 0,
        }
    }
}

impl DhcpLease {
    /// Returns the moment at which the lease expires.
    #[must_use]
    pub fn expires_at_millis(&self) -> u64 {
        self.acquired_at_millis
            .saturating_add(u64::from(self.lease_seconds).saturating_mul(1000))
    }

    /// Returns whether the lease expired at `now_millis`.
    #[must_use]
    pub fn is_expired(&self, now_millis: u64) -> bool {
        now_millis >= self.expires_at_millis()
    }
}

/// The result of processing an inbound DHCP message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DhcpEvent {
    /// The message was not relevant for the current state.
    Ignored,
    /// An offer was received and a request has to be sent.
    Offer(DhcpOffer),
    /// A lease was granted.
    Bound(DhcpLease),
    /// The server refused the requested address.
    Nak,
}

/// A writer for the option area of a DHCP message.
#[derive(Debug)]
pub struct DhcpOptionWriter<'a> {
    out: &'a mut [u8],
    offset: usize,
}

impl<'a> DhcpOptionWriter<'a> {
    /// Creates a writer over `out`.
    #[must_use]
    pub const fn new(out: &'a mut [u8]) -> Self {
        Self { out, offset: 0 }
    }

    /// Returns the number of bytes written.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.offset
    }

    /// Returns whether nothing was written yet.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.offset == 0
    }

    /// Writes one option.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when the option does not
    /// fit into the buffer.
    pub fn write(&mut self, code: u8, value: &[u8]) -> Result<(), StackError> {
        let length = u8::try_from(value.len()).map_err(|_| StackError::BufferTooSmall)?;
        let end = self
            .offset
            .checked_add(2)
            .and_then(|position| position.checked_add(value.len()))
            .ok_or(StackError::BufferTooSmall)?;
        let slot = self
            .out
            .get_mut(self.offset..end)
            .ok_or(StackError::BufferTooSmall)?;
        slot[0] = code;
        slot[1] = length;
        slot.get_mut(2..)
            .ok_or(StackError::BufferTooSmall)?
            .copy_from_slice(value);
        self.offset = end;
        Ok(())
    }

    /// Writes an option with a single byte value.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when the option does not
    /// fit into the buffer.
    pub fn write_u8(&mut self, code: u8, value: u8) -> Result<(), StackError> {
        self.write(code, &[value])
    }

    /// Writes an option with a 16-bit value.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when the option does not
    /// fit into the buffer.
    pub fn write_u16(&mut self, code: u8, value: u16) -> Result<(), StackError> {
        self.write(code, &value.to_be_bytes())
    }

    /// Writes an option with a 32-bit value.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when the option does not
    /// fit into the buffer.
    pub fn write_u32(&mut self, code: u8, value: u32) -> Result<(), StackError> {
        self.write(code, &value.to_be_bytes())
    }

    /// Writes an option with an address value.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when the option does not
    /// fit into the buffer.
    pub fn write_ipv4(&mut self, code: u8, value: &Ipv4Address) -> Result<(), StackError> {
        self.write(code, &value.0)
    }

    /// Writes the end-of-options marker.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when the marker does not
    /// fit into the buffer.
    pub fn write_end(&mut self) -> Result<(), StackError> {
        let slot = self
            .out
            .get_mut(self.offset..self.offset + 1)
            .ok_or(StackError::BufferTooSmall)?;
        slot[0] = DHCP_OPT_END;
        self.offset += 1;
        Ok(())
    }
}

/// Returns the message type of an option area.
///
/// # Errors
///
/// Returns `None` when the option is absent or malformed.
#[must_use]
pub fn option_message_type(options: &[u8]) -> Option<u8> {
    dhcp_find_option(options, DHCP_OPT_MESSAGE_TYPE).and_then(|value| value.first().copied())
}

/// Returns a 32-bit option value.
///
/// # Errors
///
/// Returns `None` when the option is absent or has another length.
#[must_use]
pub fn option_u32(options: &[u8], code: u8) -> Option<u32> {
    let value = dhcp_find_option(options, code)?;
    if value.len() != 4 {
        return None;
    }
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(value);
    Some(u32::from_be_bytes(raw))
}

/// Returns an address option value.
///
/// # Errors
///
/// Returns `None` when the option is absent or has another length.
#[must_use]
pub fn option_ipv4(options: &[u8], code: u8) -> Option<Ipv4Address> {
    let value = dhcp_find_option(options, code)?;
    if value.len() != 4 {
        return None;
    }
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(value);
    Some(Ipv4Address(raw))
}

/// Returns the lease time of an option area.
///
/// # Errors
///
/// Returns `None` when the option is absent.
#[must_use]
pub fn option_lease_seconds(options: &[u8]) -> Option<u32> {
    option_u32(options, DHCP_OPT_LEASE_TIME)
}

/// A DHCP client.
#[derive(Clone, Copy, Debug)]
pub struct DhcpClient {
    state: DhcpState,
    xid: u32,
    mac: MacAddress,
    elapsed_seconds: u16,
    offer: Option<DhcpOffer>,
    lease: Option<DhcpLease>,
}

impl DhcpClient {
    /// Creates a client that uses `xid` as the transaction
    /// identifier.
    #[must_use]
    pub const fn new(mac: MacAddress, xid: u32) -> Self {
        Self {
            state: DhcpState::Init,
            xid,
            mac,
            elapsed_seconds: 0,
            offer: None,
            lease: None,
        }
    }

    /// Returns the state of the client.
    #[must_use]
    pub const fn state(&self) -> DhcpState {
        self.state
    }

    /// Returns the transaction identifier.
    #[must_use]
    pub const fn transaction_id(&self) -> u32 {
        self.xid
    }

    /// Returns the hardware address of the client.
    #[must_use]
    pub const fn hardware_address(&self) -> MacAddress {
        self.mac
    }

    /// Sets the seconds elapsed field of the next message.
    pub fn set_elapsed_seconds(&mut self, seconds: u16) {
        self.elapsed_seconds = seconds;
    }

    /// Returns the offer that the client received.
    ///
    /// # Errors
    ///
    /// Returns `None` when no offer was received.
    #[must_use]
    pub const fn offer(&self) -> Option<&DhcpOffer> {
        self.offer.as_ref()
    }

    /// Returns the lease that the client holds.
    ///
    /// # Errors
    ///
    /// Returns `None` when the client has no lease.
    #[must_use]
    pub const fn lease(&self) -> Option<&DhcpLease> {
        self.lease.as_ref()
    }

    /// Builds a discover message in `out`.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::BufferTooSmall`] when `out` cannot hold
    /// the message.
    pub fn build_discover(&mut self, out: &mut [u8]) -> Result<usize, StackError> {
        self.state = DhcpState::Select;
        self.offer = None;
        self.lease = None;
        let header_len = self.write_header(out, DHCP_OP_REQUEST, Ipv4Address([0, 0, 0, 0]))?;
        let mut writer = DhcpOptionWriter::new(
            out.get_mut(header_len..)
                .ok_or(StackError::BufferTooSmall)?,
        );
        writer.write_u8(DHCP_OPT_MESSAGE_TYPE, DHCP_MSG_DISCOVER)?;
        self.write_client_identifier(&mut writer)?;
        writer.write_u16(
            DHCP_OPT_MAX_MESSAGE_SIZE,
            u16::try_from(1500).unwrap_or(u16::MAX),
        )?;
        writer.write(
            DHCP_OPT_PARAMETER_REQUEST_LIST,
            &[
                DHCP_OPT_SUBNET_MASK,
                DHCP_OPT_ROUTER,
                DHCP_OPT_DNS_SERVERS,
                DHCP_OPT_HOSTNAME,
            ],
        )?;
        writer.write_end()?;
        Ok(header_len + writer.len())
    }

    /// Builds a request message in `out`.
    ///
    /// The request refers to the offer that the client received.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::InvalidState`] when the client has no
    /// offer and [`StackError::BufferTooSmall`] when `out` cannot hold
    /// the message.
    pub fn build_request(&mut self, out: &mut [u8]) -> Result<usize, StackError> {
        let offer = self.offer.ok_or(StackError::InvalidState)?;
        self.state = DhcpState::Request;
        let header_len = self.write_header(out, DHCP_OP_REQUEST, offer.address)?;
        let mut writer = DhcpOptionWriter::new(
            out.get_mut(header_len..)
                .ok_or(StackError::BufferTooSmall)?,
        );
        writer.write_u8(DHCP_OPT_MESSAGE_TYPE, DHCP_MSG_REQUEST)?;
        writer.write_ipv4(DHCP_OPT_REQUESTED_IP, &offer.address)?;
        writer.write_ipv4(DHCP_OPT_SERVER_IDENTIFIER, &offer.server_identifier)?;
        self.write_client_identifier(&mut writer)?;
        writer.write(
            DHCP_OPT_PARAMETER_REQUEST_LIST,
            &[
                DHCP_OPT_SUBNET_MASK,
                DHCP_OPT_ROUTER,
                DHCP_OPT_DNS_SERVERS,
                DHCP_OPT_HOSTNAME,
            ],
        )?;
        writer.write_end()?;
        Ok(header_len + writer.len())
    }

    /// Processes an inbound message.
    ///
    /// `now_millis` is the current time and is stored in the lease.
    pub fn on_message(&mut self, packet: &DhcpPacket<'_>, now_millis: u64) -> DhcpEvent {
        if packet.xid != self.xid {
            return DhcpEvent::Ignored;
        }
        let Some(message_type) = option_message_type(packet.options) else {
            return DhcpEvent::Ignored;
        };
        match message_type {
            DHCP_MSG_OFFER => {
                if self.state != DhcpState::Select {
                    return DhcpEvent::Ignored;
                }
                let offer = Self::parse_offer(packet);
                self.offer = Some(offer);
                self.state = DhcpState::Request;
                DhcpEvent::Offer(offer)
            }
            DHCP_MSG_ACK => {
                if self.state != DhcpState::Request {
                    return DhcpEvent::Ignored;
                }
                let Some(offer) = self.offer else {
                    return DhcpEvent::Ignored;
                };
                let lease = DhcpLease {
                    address: offer.address,
                    server_identifier: offer.server_identifier,
                    subnet_mask: offer.subnet_mask,
                    router: offer.router,
                    dns_servers: offer.dns_servers,
                    lease_seconds: offer.lease_seconds,
                    acquired_at_millis: now_millis,
                };
                self.lease = Some(lease);
                self.state = DhcpState::Bound;
                DhcpEvent::Bound(lease)
            }
            DHCP_MSG_NAK => {
                self.state = DhcpState::Declined;
                self.offer = None;
                DhcpEvent::Nak
            }
            _ => DhcpEvent::Ignored,
        }
    }

    /// Parses a raw DHCP message.
    ///
    /// # Errors
    ///
    /// Returns `None` when the message is malformed.
    #[must_use]
    pub fn parse_message(data: &[u8]) -> Option<DhcpPacket<'_>> {
        parse_dhcp(data)
    }

    /// Reads the offer from a reply.
    fn parse_offer(packet: &DhcpPacket<'_>) -> DhcpOffer {
        let server_identifier = option_ipv4(packet.options, DHCP_OPT_SERVER_IDENTIFIER)
            .unwrap_or(Ipv4Address([0, 0, 0, 0]));
        let mut dns_servers = DnsServerList::new();
        if let Some(value) = dhcp_find_option(packet.options, DHCP_OPT_DNS_SERVERS) {
            for chunk in value.chunks(4) {
                if chunk.len() != 4 {
                    break;
                }
                let mut raw = [0_u8; 4];
                raw.copy_from_slice(chunk);
                if !dns_servers.push(Ipv4Address(raw)) {
                    break;
                }
            }
        }
        DhcpOffer {
            address: packet.yiaddr,
            server_identifier,
            subnet_mask: option_ipv4(packet.options, DHCP_OPT_SUBNET_MASK),
            router: option_ipv4(packet.options, DHCP_OPT_ROUTER),
            dns_servers,
            lease_seconds: option_lease_seconds(packet.options).unwrap_or(0),
        }
    }

    /// Writes the bootstrap header and the magic cookie.
    fn write_header(
        &self,
        out: &mut [u8],
        oper: u8,
        yiaddr: Ipv4Address,
    ) -> Result<usize, StackError> {
        let header = out
            .get_mut(..DHCP_MIN_LEN)
            .ok_or(StackError::BufferTooSmall)?;
        header[0] = oper;
        header[1] = DHCP_HTYPE_ETHERNET;
        header[2] = 6;
        header[3] = 0;
        header[4..8].copy_from_slice(&self.xid.to_be_bytes());
        header[8..10].copy_from_slice(&self.elapsed_seconds.to_be_bytes());
        header[10..12].copy_from_slice(&0x8000_u16.to_be_bytes());
        header[12..16].copy_from_slice(&[0, 0, 0, 0]);
        header[16..20].copy_from_slice(&yiaddr.0);
        header[20..24].copy_from_slice(&[0, 0, 0, 0]);
        header[24..28].copy_from_slice(&[0, 0, 0, 0]);
        header[28..34].copy_from_slice(&self.mac.0);
        for byte in header
            .get_mut(34..BOOTP_HEADER_LEN)
            .ok_or(StackError::BufferTooSmall)?
        {
            *byte = 0;
        }
        header
            .get_mut(DHCP_COOKIE_OFFSET..DHCP_MIN_LEN)
            .ok_or(StackError::BufferTooSmall)?
            .copy_from_slice(&DHCP_MAGIC_COOKIE);
        Ok(DHCP_MIN_LEN)
    }

    /// Writes the client identifier option.
    fn write_client_identifier(&self, writer: &mut DhcpOptionWriter<'_>) -> Result<(), StackError> {
        let mut value = [0_u8; 7];
        value[0] = DHCP_HTYPE_ETHERNET;
        value[1..7].copy_from_slice(&self.mac.0);
        writer.write(DHCP_OPT_CLIENT_IDENTIFIER, &value)
    }
}
