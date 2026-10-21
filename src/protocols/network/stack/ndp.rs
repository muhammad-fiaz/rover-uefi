//! # IPv6 Neighbor Discovery
//!
//! Native encoders and decoders for the `ICMPv6` Neighbor Discovery
//! messages defined by `RFC 4861`: router solicitation, router
//! advertisement, neighbor solicitation, neighbor advertisement and
//! redirect.
//!
//! The module also encodes and parses the four options that the stack
//! emits: source link-layer address, target link-layer address, prefix
//! information and `MTU`. Unknown options are preserved by the parser
//! so that a caller can inspect them without losing data.
//!
//! Nothing here allocates and no function panics. A message is built
//! into a caller-supplied buffer, and the checksum is written by
//! [`write_checksum`] once the source and destination addresses are
//! known.

use crate::protocols::network::stack::StackError;
use crate::protocols::network::stack::icmp;
use crate::protocols::network::types::{Ipv6Address, MacAddress};

/// The `ICMPv6` router solicitation type.
pub const ICMPV6_ROUTER_SOLICIT: u8 = 133;

/// The `ICMPv6` router advertisement type.
pub const ICMPV6_ROUTER_ADVERT: u8 = 134;

/// The `ICMPv6` neighbor solicitation type.
pub const ICMPV6_NEIGHBOR_SOLICIT: u8 = 135;

/// The `ICMPv6` neighbor advertisement type.
pub const ICMPV6_NEIGHBOR_ADVERT: u8 = 136;

/// The `ICMPv6` redirect type.
pub const ICMPV6_REDIRECT: u8 = 137;

/// The length of the common Neighbor Discovery header in bytes.
pub const ND_HEADER_LEN: usize = 8;

/// The length of a router advertisement header in bytes.
pub const ND_ROUTER_ADVERT_LEN: usize = 16;

/// The length of a neighbor solicitation or advertisement in bytes.
pub const ND_NEIGHBOR_LEN: usize = 24;

/// The length of a redirect message header in bytes.
pub const ND_REDIRECT_LEN: usize = 40;

/// The source link-layer address option type.
pub const ND_OPT_SOURCE_LINK_LAYER: u8 = 1;

/// The target link-layer address option type.
pub const ND_OPT_TARGET_LINK_LAYER: u8 = 2;

/// The prefix information option type.
pub const ND_OPT_PREFIX_INFORMATION: u8 = 3;

/// The `MTU` option type.
pub const ND_OPT_MTU: u8 = 5;

/// The length of a link-layer address option in bytes.
pub const ND_LINK_LAYER_OPTION_LEN: usize = 8;

/// The length of a prefix information option in bytes.
pub const ND_PREFIX_OPTION_LEN: usize = 32;

/// The length of an `MTU` option in bytes.
pub const ND_MTU_OPTION_LEN: usize = 8;

/// The prefix information carried by a router advertisement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrefixInformation {
    /// The prefix itself; the low bits are ignored.
    pub prefix: Ipv6Address,
    /// The number of leading significant bits in the prefix.
    pub prefix_length: u8,
    /// Whether the prefix may be used for on-link determination.
    pub on_link: bool,
    /// Whether the prefix may be used for stateless address
    /// autoconfiguration.
    pub autonomous: bool,
    /// The validity lifetime of the prefix in seconds.
    pub valid_lifetime: u32,
    /// The preferred lifetime of the prefix in seconds.
    pub preferred_lifetime: u32,
}

/// The options a message builder emits.
///
/// Only the options that this stack generates are represented. The
/// parser reports every option, including ones that are not listed
/// here, through [`NdpOptionIter`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NdpOptions {
    /// A source link-layer address option.
    pub source_link_layer: Option<MacAddress>,
    /// A target link-layer address option.
    pub target_link_layer: Option<MacAddress>,
    /// A prefix information option.
    pub prefix: Option<PrefixInformation>,
    /// An `MTU` option.
    pub mtu: Option<u32>,
}

impl NdpOptions {
    /// Returns the number of bytes the options occupy when encoded.
    #[must_use]
    pub const fn encoded_len(&self) -> usize {
        let mut total = 0;
        if self.source_link_layer.is_some() {
            total += ND_LINK_LAYER_OPTION_LEN;
        }
        if self.target_link_layer.is_some() {
            total += ND_LINK_LAYER_OPTION_LEN;
        }
        if self.prefix.is_some() {
            total += ND_PREFIX_OPTION_LEN;
        }
        if self.mtu.is_some() {
            total += ND_MTU_OPTION_LEN;
        }
        total
    }
}

/// One option of a Neighbor Discovery message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NdpOption<'a> {
    /// The option type.
    pub kind: u8,
    /// The option body, excluding the type and length bytes.
    pub data: &'a [u8],
}

impl NdpOption<'_> {
    /// Returns the link-layer address of a source link-layer option.
    #[must_use]
    pub fn source_link_layer(&self) -> Option<MacAddress> {
        if self.kind != ND_OPT_SOURCE_LINK_LAYER {
            return None;
        }
        let address = self.data.get(..6)?;
        let mut bytes = [0_u8; 6];
        bytes.copy_from_slice(address);
        Some(MacAddress(bytes))
    }

    /// Returns the link-layer address of a target link-layer option.
    #[must_use]
    pub fn target_link_layer(&self) -> Option<MacAddress> {
        if self.kind != ND_OPT_TARGET_LINK_LAYER {
            return None;
        }
        let address = self.data.get(..6)?;
        let mut bytes = [0_u8; 6];
        bytes.copy_from_slice(address);
        Some(MacAddress(bytes))
    }

    /// Returns the prefix information carried by a prefix option.
    #[must_use]
    pub fn prefix_information(&self) -> Option<PrefixInformation> {
        if self.kind != ND_OPT_PREFIX_INFORMATION {
            return None;
        }
        let body = self.data;
        let flags = *body.get(1)?;
        let mut prefix = [0_u8; 16];
        prefix.copy_from_slice(body.get(14..30)?);
        Some(PrefixInformation {
            prefix: Ipv6Address(prefix),
            prefix_length: *body.first()?,
            on_link: flags & 0x80 != 0,
            autonomous: flags & 0x40 != 0,
            valid_lifetime: u32::from_be_bytes([
                *body.get(2)?,
                *body.get(3)?,
                *body.get(4)?,
                *body.get(5)?,
            ]),
            preferred_lifetime: u32::from_be_bytes([
                *body.get(6)?,
                *body.get(7)?,
                *body.get(8)?,
                *body.get(9)?,
            ]),
        })
    }

    /// Returns the `MTU` carried by an `MTU` option.
    #[must_use]
    pub fn mtu(&self) -> Option<u32> {
        if self.kind != ND_OPT_MTU {
            return None;
        }
        let value = self.data.get(2..6)?;
        Some(u32::from_be_bytes([
            *value.first()?,
            *value.get(1)?,
            *value.get(2)?,
            *value.get(3)?,
        ]))
    }
}

/// An iterator over the options of a Neighbor Discovery message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NdpOptionIter<'a> {
    rest: &'a [u8],
}

impl<'a> Iterator for NdpOptionIter<'a> {
    type Item = NdpOption<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = *self.rest.first()?;
        let units = usize::from(*self.rest.get(1)?);
        let total = units.checked_mul(8)?;
        if total < 8 {
            self.rest = &[];
            return None;
        }
        let chunk = self.rest.get(..total)?;
        self.rest = self.rest.get(total..).unwrap_or(&[]);
        Some(NdpOption {
            kind,
            data: chunk.get(2..).unwrap_or(&[]),
        })
    }
}

/// Parses an option list.
///
/// Returns `None` when an option declares a zero length or runs past
/// the end of `bytes`.
#[must_use]
pub fn parse_options(bytes: &[u8]) -> Option<NdpOptionIter<'_>> {
    let mut rest = bytes;
    while !rest.is_empty() {
        let total = usize::from(*rest.get(1)?).checked_mul(8)?;
        if total < 8 || rest.len() < total {
            return None;
        }
        rest = rest.get(total..)?;
    }
    Some(NdpOptionIter { rest: bytes })
}

/// Writes the `ICMPv6` checksum over the message in `out`.
///
/// The checksum field is cleared before the value is computed, so the
/// function may be called on a message whose checksum was already set.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` is shorter than
/// the common `ICMPv6` header.
pub fn write_checksum(
    out: &mut [u8],
    source: Ipv6Address,
    destination: Ipv6Address,
) -> Result<(), StackError> {
    if out.len() < icmp::ICMP_HEADER_LEN {
        return Err(StackError::BufferTooSmall);
    }
    let field = out.get_mut(2..4).ok_or(StackError::BufferTooSmall)?;
    field.fill(0);
    let value = icmp::checksum_v6(source, destination, out);
    out.get_mut(2..4)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&value.to_be_bytes());
    Ok(())
}

/// A router solicitation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouterSolicitation<'a> {
    /// The options carried by the message.
    pub options: NdpOptionIter<'a>,
}

impl<'a> RouterSolicitation<'a> {
    /// Parses a router solicitation.
    #[must_use]
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let header = bytes.get(..ND_HEADER_LEN)?;
        if *header.first()? != ICMPV6_ROUTER_SOLICIT {
            return None;
        }
        Some(Self {
            options: parse_options(bytes.get(ND_HEADER_LEN..)?)?,
        })
    }
}

/// The parameters of a router advertisement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouterAdvertisementParams {
    /// The hop limit that hosts should place in outgoing datagrams.
    pub hop_limit: u8,
    /// Whether hosts should use `DHCPv6` for addresses.
    pub managed: bool,
    /// Whether hosts should use `DHCPv6` for other configuration.
    pub other: bool,
    /// The router lifetime in seconds; zero means not a default router.
    pub router_lifetime: u16,
    /// The reachable time in milliseconds.
    pub reachable_time: u32,
    /// The retransmission timer in milliseconds.
    pub retrans_timer: u32,
}

impl Default for RouterAdvertisementParams {
    fn default() -> Self {
        Self {
            hop_limit: 64,
            managed: false,
            other: false,
            router_lifetime: 0,
            reachable_time: 0,
            retrans_timer: 0,
        }
    }
}

/// A router advertisement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouterAdvertisement<'a> {
    /// The parameters carried by the fixed part of the message.
    pub params: RouterAdvertisementParams,
    /// The options carried by the message.
    pub options: NdpOptionIter<'a>,
}

impl<'a> RouterAdvertisement<'a> {
    /// Parses a router advertisement.
    #[must_use]
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let header = bytes.get(..ND_ROUTER_ADVERT_LEN)?;
        if *header.first()? != ICMPV6_ROUTER_ADVERT {
            return None;
        }
        let flags = *header.get(5)?;
        Some(Self {
            params: RouterAdvertisementParams {
                hop_limit: *header.get(4)?,
                managed: flags & 0x80 != 0,
                other: flags & 0x40 != 0,
                router_lifetime: u16::from_be_bytes([*header.get(6)?, *header.get(7)?]),
                reachable_time: u32::from_be_bytes([
                    *header.get(8)?,
                    *header.get(9)?,
                    *header.get(10)?,
                    *header.get(11)?,
                ]),
                retrans_timer: u32::from_be_bytes([
                    *header.get(12)?,
                    *header.get(13)?,
                    *header.get(14)?,
                    *header.get(15)?,
                ]),
            },
            options: parse_options(bytes.get(ND_ROUTER_ADVERT_LEN..)?)?,
        })
    }
}

/// A neighbor solicitation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeighborSolicitation<'a> {
    /// The address whose link-layer address is requested.
    pub target: Ipv6Address,
    /// The options carried by the message.
    pub options: NdpOptionIter<'a>,
}

impl<'a> NeighborSolicitation<'a> {
    /// Parses a neighbor solicitation.
    #[must_use]
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let header = bytes.get(..ND_NEIGHBOR_LEN)?;
        if *header.first()? != ICMPV6_NEIGHBOR_SOLICIT {
            return None;
        }
        Some(Self {
            target: address_at(header, 8)?,
            options: parse_options(bytes.get(ND_NEIGHBOR_LEN..)?)?,
        })
    }
}

/// The flags of a neighbor advertisement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NeighborAdvertisementFlags {
    /// Whether the sender is a router.
    pub router: bool,
    /// Whether the advertisement answers a solicitation.
    pub solicited: bool,
    /// Whether the advertisement overrides a cached entry.
    pub override_entry: bool,
}

/// A neighbor advertisement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeighborAdvertisement<'a> {
    /// The flag bits of the message.
    pub flags: NeighborAdvertisementFlags,
    /// The address the advertisement describes.
    pub target: Ipv6Address,
    /// The options carried by the message.
    pub options: NdpOptionIter<'a>,
}

impl<'a> NeighborAdvertisement<'a> {
    /// Parses a neighbor advertisement.
    #[must_use]
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let header = bytes.get(..ND_NEIGHBOR_LEN)?;
        if *header.first()? != ICMPV6_NEIGHBOR_ADVERT {
            return None;
        }
        let bits = *header.get(4)?;
        Some(Self {
            flags: NeighborAdvertisementFlags {
                router: bits & 0x80 != 0,
                solicited: bits & 0x40 != 0,
                override_entry: bits & 0x20 != 0,
            },
            target: address_at(header, 8)?,
            options: parse_options(bytes.get(ND_NEIGHBOR_LEN..)?)?,
        })
    }
}

/// A redirect message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Redirect<'a> {
    /// The better first hop for the destination.
    pub target: Ipv6Address,
    /// The destination that is redirected.
    pub destination: Ipv6Address,
    /// The options carried by the message.
    pub options: NdpOptionIter<'a>,
}

impl<'a> Redirect<'a> {
    /// Parses a redirect message.
    #[must_use]
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        let header = bytes.get(..ND_REDIRECT_LEN)?;
        if *header.first()? != ICMPV6_REDIRECT {
            return None;
        }
        Some(Self {
            target: address_at(header, 8)?,
            destination: address_at(header, 24)?,
            options: parse_options(bytes.get(ND_REDIRECT_LEN..)?)?,
        })
    }
}

/// Builds a router solicitation and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// message.
pub fn build_router_solicitation(
    out: &mut [u8],
    options: &NdpOptions,
) -> Result<usize, StackError> {
    let total = ND_HEADER_LEN
        .checked_add(options.encoded_len())
        .ok_or(StackError::BufferTooSmall)?;
    let buffer = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    buffer.fill(0);
    *buffer.first_mut().ok_or(StackError::BufferTooSmall)? = ICMPV6_ROUTER_SOLICIT;
    write_options(buffer, ND_HEADER_LEN, options)?;
    Ok(total)
}

/// Builds a router advertisement and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// message.
pub fn build_router_advertisement(
    out: &mut [u8],
    params: &RouterAdvertisementParams,
    options: &NdpOptions,
) -> Result<usize, StackError> {
    let total = ND_ROUTER_ADVERT_LEN
        .checked_add(options.encoded_len())
        .ok_or(StackError::BufferTooSmall)?;
    let buffer = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    buffer.fill(0);
    buffer[0] = ICMPV6_ROUTER_ADVERT;
    buffer[4] = params.hop_limit;
    let mut flags = 0_u8;
    if params.managed {
        flags |= 0x80;
    }
    if params.other {
        flags |= 0x40;
    }
    buffer[5] = flags;
    buffer
        .get_mut(6..8)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&params.router_lifetime.to_be_bytes());
    buffer
        .get_mut(8..12)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&params.reachable_time.to_be_bytes());
    buffer
        .get_mut(12..16)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&params.retrans_timer.to_be_bytes());
    write_options(buffer, ND_ROUTER_ADVERT_LEN, options)?;
    Ok(total)
}

/// Builds a neighbor solicitation and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// message.
pub fn build_neighbor_solicitation(
    out: &mut [u8],
    target: Ipv6Address,
    options: &NdpOptions,
) -> Result<usize, StackError> {
    let total = ND_NEIGHBOR_LEN
        .checked_add(options.encoded_len())
        .ok_or(StackError::BufferTooSmall)?;
    let buffer = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    buffer.fill(0);
    buffer[0] = ICMPV6_NEIGHBOR_SOLICIT;
    buffer
        .get_mut(8..24)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&target.0);
    write_options(buffer, ND_NEIGHBOR_LEN, options)?;
    Ok(total)
}

/// Builds a neighbor advertisement and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// message.
pub fn build_neighbor_advertisement(
    out: &mut [u8],
    flags: &NeighborAdvertisementFlags,
    target: Ipv6Address,
    options: &NdpOptions,
) -> Result<usize, StackError> {
    let total = ND_NEIGHBOR_LEN
        .checked_add(options.encoded_len())
        .ok_or(StackError::BufferTooSmall)?;
    let buffer = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    buffer.fill(0);
    buffer[0] = ICMPV6_NEIGHBOR_ADVERT;
    let mut bits = 0_u8;
    if flags.router {
        bits |= 0x80;
    }
    if flags.solicited {
        bits |= 0x40;
    }
    if flags.override_entry {
        bits |= 0x20;
    }
    buffer[4] = bits;
    buffer
        .get_mut(8..24)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&target.0);
    write_options(buffer, ND_NEIGHBOR_LEN, options)?;
    Ok(total)
}

/// Builds a redirect message and returns its length.
///
/// # Errors
///
/// Returns [`StackError::BufferTooSmall`] when `out` cannot hold the
/// message.
pub fn build_redirect(
    out: &mut [u8],
    target: Ipv6Address,
    destination: Ipv6Address,
    options: &NdpOptions,
) -> Result<usize, StackError> {
    let total = ND_REDIRECT_LEN
        .checked_add(options.encoded_len())
        .ok_or(StackError::BufferTooSmall)?;
    let buffer = out.get_mut(..total).ok_or(StackError::BufferTooSmall)?;
    buffer.fill(0);
    buffer[0] = ICMPV6_REDIRECT;
    buffer
        .get_mut(8..24)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&target.0);
    buffer
        .get_mut(24..40)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&destination.0);
    write_options(buffer, ND_REDIRECT_LEN, options)?;
    Ok(total)
}

/// Reads the IPv6 address stored at `offset`.
fn address_at(bytes: &[u8], offset: usize) -> Option<Ipv6Address> {
    let field = bytes.get(offset..offset.checked_add(16)?)?;
    let mut value = [0_u8; 16];
    value.copy_from_slice(field);
    Some(Ipv6Address(value))
}

/// Writes the encoded options into `out` at `offset`.
fn write_options(out: &mut [u8], offset: usize, options: &NdpOptions) -> Result<(), StackError> {
    let mut position = offset;
    if let Some(address) = options.source_link_layer {
        position = write_link_layer_option(out, position, ND_OPT_SOURCE_LINK_LAYER, address)?;
    }
    if let Some(address) = options.target_link_layer {
        position = write_link_layer_option(out, position, ND_OPT_TARGET_LINK_LAYER, address)?;
    }
    if let Some(prefix) = options.prefix {
        position = write_prefix_option(out, position, &prefix)?;
    }
    if let Some(mtu) = options.mtu {
        write_mtu_option(out, position, mtu)?;
    }
    Ok(())
}

/// Writes one link-layer address option.
fn write_link_layer_option(
    out: &mut [u8],
    offset: usize,
    kind: u8,
    address: MacAddress,
) -> Result<usize, StackError> {
    let end = offset
        .checked_add(ND_LINK_LAYER_OPTION_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    let option = out.get_mut(offset..end).ok_or(StackError::BufferTooSmall)?;
    option[0] = kind;
    option[1] = 1;
    option
        .get_mut(2..8)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&address.0);
    Ok(end)
}

/// Writes one prefix information option.
fn write_prefix_option(
    out: &mut [u8],
    offset: usize,
    prefix: &PrefixInformation,
) -> Result<usize, StackError> {
    let end = offset
        .checked_add(ND_PREFIX_OPTION_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    let option = out.get_mut(offset..end).ok_or(StackError::BufferTooSmall)?;
    option.fill(0);
    option[0] = ND_OPT_PREFIX_INFORMATION;
    option[1] = 4;
    option[2] = prefix.prefix_length;
    let mut flags = 0_u8;
    if prefix.on_link {
        flags |= 0x80;
    }
    if prefix.autonomous {
        flags |= 0x40;
    }
    option[3] = flags;
    option
        .get_mut(4..8)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&prefix.valid_lifetime.to_be_bytes());
    option
        .get_mut(8..12)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&prefix.preferred_lifetime.to_be_bytes());
    option
        .get_mut(16..32)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&prefix.prefix.0);
    Ok(end)
}

/// Writes one `MTU` option.
fn write_mtu_option(out: &mut [u8], offset: usize, mtu: u32) -> Result<usize, StackError> {
    let end = offset
        .checked_add(ND_MTU_OPTION_LEN)
        .ok_or(StackError::BufferTooSmall)?;
    let option = out.get_mut(offset..end).ok_or(StackError::BufferTooSmall)?;
    option.fill(0);
    option[0] = ND_OPT_MTU;
    option[1] = 1;
    option
        .get_mut(4..8)
        .ok_or(StackError::BufferTooSmall)?
        .copy_from_slice(&mtu.to_be_bytes());
    Ok(end)
}
