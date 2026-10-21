//! # X.509 DER parsing
//!
//! Bounded, allocation-free Distinguished Encoding Rules parsing for
//! certificates. The parser decodes tag-length-value structure, object
//! identifiers, validity times, and subject and issuer names.
//!
//! The module also walks a certificate chain from a leaf up to a
//! configured trust anchor, verifying each signature and the issuer
//! and subject name binding. Parsing a certificate is still not the
//! same as trusting it: callers must supply the anchors they trust.
//! Chain walking does not evaluate name constraints, policies,
//! key-usage extensions, or revocation lists.

/// The maximum number of intermediate certificates a chain walk visits.
pub const MAX_CHAIN_LENGTH: usize = 32;

/// The reason a certificate chain could not be validated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChainError {
    /// No trust anchor was supplied, so nothing can be trusted.
    NoTrustAnchor,
    /// A certificate in the chain is outside its validity interval.
    Expired,
    /// The issuer named by a certificate was not supplied.
    IssuerNotFound,
    /// A supplied issuer names the certificate but the signature does
    /// not verify.
    SignatureInvalid,
    /// The supplied candidate chain is longer than the supported walk.
    ChainTooLong,
    /// A candidate issuer would be visited twice.
    Loop,
}

/// The maximum OID arcs retained in a decoded object identifier.
pub const MAX_OID_ARCS: usize = 16;

/// A decoded object identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Oid {
    arcs: [u64; MAX_OID_ARCS],
    len: usize,
}

impl Oid {
    /// Returns the number of arcs.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether no arcs are present.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the arc at `index`, or `None` out of range.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<u64> {
        self.arcs.get(index).copied()
    }

    /// Returns the arcs slice.
    #[must_use]
    pub fn arcs(&self) -> &[u64] {
        &self.arcs[..self.len]
    }
}

/// A DER decoding failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DerError {
    /// The input ended before a complete value.
    Truncated,
    /// A tag used indefinite length or an invalid class.
    InvalidTag,
    /// A length overflowed `usize` or exceeded the buffer.
    LengthOverflow,
    /// Extra bytes followed a complete value.
    TrailingData,
    /// An object identifier was malformed.
    InvalidOid,
    /// A length used non-minimal long form.
    NonMinimalLength,
    /// A structure did not match the expected ASN.1 shape.
    UnsupportedStructure,
    /// A validity time was malformed.
    InvalidTime,
    /// A version value was outside the allowed range.
    InvalidVersion,
}

impl core::fmt::Display for DerError {
    /// Formats the error without allocation.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Self::Truncated => "truncated DER input",
            Self::InvalidTag => "invalid DER tag",
            Self::LengthOverflow => "DER length overflow",
            Self::TrailingData => "trailing DER data",
            Self::InvalidOid => "invalid object identifier",
            Self::NonMinimalLength => "non-minimal DER length",
            Self::UnsupportedStructure => "unsupported DER structure",
            Self::InvalidTime => "invalid validity time",
            Self::InvalidVersion => "invalid certificate version",
        };
        formatter.write_str(text)
    }
}

/// A parsed tag-length-value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tlv<'a> {
    tag: u8,
    constructed: bool,
    header_len: usize,
    value: &'a [u8],
    total_len: usize,
}

impl<'a> Tlv<'a> {
    /// Returns the tag byte.
    #[must_use]
    pub const fn tag(&self) -> u8 {
        self.tag
    }

    /// Returns whether the constructed bit is set.
    #[must_use]
    pub const fn constructed(&self) -> bool {
        self.constructed
    }

    /// Returns the header length in bytes.
    #[must_use]
    pub const fn header_len(&self) -> usize {
        self.header_len
    }

    /// Returns the value bytes without tag and length.
    #[must_use]
    pub const fn value(&self) -> &'a [u8] {
        self.value
    }

    /// Returns the total encoded length including tag and length.
    #[must_use]
    pub const fn total_len(&self) -> usize {
        self.total_len
    }
}

/// Parses one tag-length-value from the front of `input`.
///
/// Rejects indefinite lengths, non-minimal long forms, and lengths that
/// overflow `usize` or exceed the buffer.
///
/// # Errors
///
/// Returns [`DerError::Truncated`] when fewer than two bytes are
/// present, [`DerError::InvalidTag`] for indefinite form,
/// [`DerError::NonMinimalLength`] for long form encoding a short
/// length, and [`DerError::LengthOverflow`] when the declared length
/// exceeds the buffer.
pub fn parse_tlv(input: &[u8]) -> Result<Tlv<'_>, DerError> {
    let tag = *input.first().ok_or(DerError::Truncated)?;
    let length_byte = *input.get(1).ok_or(DerError::Truncated)?;
    let constructed = (tag & 0x20) != 0;
    let (length, header_len) = if length_byte < 0x80 {
        (length_byte as usize, 2)
    } else {
        let count = (length_byte & 0x7f) as usize;
        if count == 0 {
            return Err(DerError::InvalidTag);
        }
        if count > 4 {
            return Err(DerError::LengthOverflow);
        }
        if input.len() < 2 + count {
            return Err(DerError::Truncated);
        }
        let mut value = 0_usize;
        for index in 0..count {
            let byte = *input.get(2 + index).ok_or(DerError::Truncated)?;
            value = value
                .checked_mul(256)
                .and_then(|v| v.checked_add(byte as usize))
                .ok_or(DerError::LengthOverflow)?;
        }
        if value < 0x80 {
            return Err(DerError::NonMinimalLength);
        }
        (value, 2 + count)
    };
    if input.len() < header_len {
        return Err(DerError::Truncated);
    }
    let value = input
        .get(
            header_len
                ..header_len
                    .checked_add(length)
                    .ok_or(DerError::LengthOverflow)?,
        )
        .ok_or(DerError::Truncated)?;
    Ok(Tlv {
        tag,
        constructed,
        header_len,
        value,
        total_len: header_len + length,
    })
}

/// Decodes an object identifier content bytes (without tag and length).
///
/// The first byte encodes the first two arcs. Each later arc uses base
/// 128. Leading zero groups are rejected as non-minimal.
///
/// # Errors
///
/// Returns [`DerError::InvalidOid`] for empty input, truncated arcs,
/// non-minimal encodings, or values that overflow `u64`.
pub fn decode_oid(content: &[u8]) -> Result<Oid, DerError> {
    if content.is_empty() {
        return Err(DerError::InvalidOid);
    }
    let first = u64::from(*content.first().ok_or(DerError::InvalidOid)?);
    let mut arcs = [0_u64; MAX_OID_ARCS];
    if first < 40 {
        arcs[0] = 0;
        arcs[1] = first;
    } else if first < 80 {
        arcs[0] = 1;
        arcs[1] = first - 40;
    } else {
        arcs[0] = 2;
        arcs[1] = first - 80;
    }
    let mut len = 2_usize;
    let mut index = 1_usize;
    while index < content.len() {
        if len >= MAX_OID_ARCS {
            return Err(DerError::InvalidOid);
        }
        let mut value = 0_u64;
        let mut groups = 0_usize;
        loop {
            let byte = *content.get(index).ok_or(DerError::InvalidOid)?;
            index += 1;
            groups += 1;
            if groups > 9 {
                return Err(DerError::InvalidOid);
            }
            if groups == 1 && byte == 0x80 {
                return Err(DerError::InvalidOid);
            }
            value = value
                .checked_mul(128)
                .and_then(|v| v.checked_add(u64::from(byte & 0x7f)))
                .ok_or(DerError::InvalidOid)?;
            if (byte & 0x80) == 0 {
                break;
            }
            if index >= content.len() {
                return Err(DerError::InvalidOid);
            }
        }
        arcs[len] = value;
        len += 1;
    }
    Ok(Oid { arcs, len })
}

/// Checks whether an OID equals a fixed arc array.
///
/// Returns `false` when lengths differ.
#[must_use]
pub fn oid_eq(oid: &Oid, expected: &[u64]) -> bool {
    if oid.len() != expected.len() {
        return false;
    }
    let mut equal = true;
    for index in 0..expected.len() {
        let a = oid.get(index).unwrap_or(0);
        let b = expected.get(index).copied().unwrap_or(0);
        equal &= a == b;
    }
    equal
}

/// Parses a validity time into seconds since the Unix epoch.
///
/// Accepts `UTCTime` (`YYMMDDHHMMSSZ`) and `GeneralizedTime`
/// (`YYYYMMDDHHMMSSZ`). Other encodings (offsets, fractional seconds)
/// are rejected.
///
/// # Errors
///
/// Returns [`DerError::InvalidTime`] for wrong length, non-digit
/// content, missing `Z`, or out-of-range calendar fields.
pub fn parse_validity_time(content: &[u8]) -> Result<i64, DerError> {
    if content.len() == 13 && *content.last().ok_or(DerError::InvalidTime)? == b'Z' {
        let year2 = digits(content, 0, 2)?;
        let year = if year2 < 50 {
            2000 + year2
        } else {
            1900 + year2
        };
        let month = digits(content, 2, 2)?;
        let day = digits(content, 4, 2)?;
        let hour = digits(content, 6, 2)?;
        let minute = digits(content, 8, 2)?;
        let second = digits(content, 10, 2)?;
        make_unix(year, month, day, hour, minute, second)
    } else if content.len() == 15 && *content.last().ok_or(DerError::InvalidTime)? == b'Z' {
        let year = digits(content, 0, 4)?;
        let month = digits(content, 4, 2)?;
        let day = digits(content, 6, 2)?;
        let hour = digits(content, 8, 2)?;
        let minute = digits(content, 10, 2)?;
        let second = digits(content, 12, 2)?;
        make_unix(year, month, day, hour, minute, second)
    } else {
        Err(DerError::InvalidTime)
    }
}

/// A parsed certificate view. All slices borrow from the input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Certificate<'a> {
    /// The full `tbsCertificate` encoding.
    pub tbs: &'a [u8],
    /// The signature algorithm OID.
    pub signature_algorithm: Oid,
    /// The signature bit string content after the unused-bits byte.
    pub signature: &'a [u8],
    /// The `TBSCertificate` version (`0` means `v1`), or `None` when absent.
    pub version: Option<u64>,
    /// The serial number content bytes.
    pub serial: &'a [u8],
    /// The issuer full encoding.
    pub issuer: &'a [u8],
    /// The subject full encoding.
    pub subject: &'a [u8],
    /// The `notBefore` time in Unix seconds.
    pub not_before: i64,
    /// The `notAfter` time in Unix seconds.
    pub not_after: i64,
}

/// Parses a DER encoded certificate.
///
/// # Errors
///
/// Returns DER errors when the ASN.1 shape does not match
/// `Certificate ::= SEQUENCE { tbsCertificate SEQUENCE,
/// signatureAlgorithm SEQUENCE, signatureValue BIT STRING }`.
pub fn parse_certificate(input: &[u8]) -> Result<Certificate<'_>, DerError> {
    let outer = parse_tlv(input)?;
    if outer.tag() != 0x30 {
        return Err(DerError::UnsupportedStructure);
    }
    let rest = outer.value();
    let tbs_tlv = parse_tlv(rest)?;
    if tbs_tlv.tag() != 0x30 {
        return Err(DerError::UnsupportedStructure);
    }
    let tbs_total = tbs_tlv.total_len();
    let tbs_bytes = rest.get(..tbs_total).ok_or(DerError::Truncated)?;
    let mut offset = tbs_total;
    let alg_tlv = parse_tlv(rest.get(offset..).ok_or(DerError::Truncated)?)?;
    if alg_tlv.tag() != 0x30 {
        return Err(DerError::UnsupportedStructure);
    }
    let alg_oid = parse_algorithm_oid(alg_tlv.value())?;
    offset += alg_tlv.header_len() + alg_tlv.value().len();
    let sig_tlv = parse_tlv(rest.get(offset..).ok_or(DerError::Truncated)?)?;
    if sig_tlv.tag() != 0x03 {
        return Err(DerError::UnsupportedStructure);
    }
    let sig_content = sig_tlv.value();
    let unused = *sig_content.first().ok_or(DerError::Truncated)?;
    if unused > 7 {
        return Err(DerError::UnsupportedStructure);
    }
    let signature = sig_content.get(1..).ok_or(DerError::Truncated)?;
    let tbs_fields = parse_tbs(tbs_tlv.value())?;
    Ok(Certificate {
        tbs: tbs_bytes,
        signature_algorithm: alg_oid,
        signature,
        version: tbs_fields.version,
        serial: tbs_fields.serial,
        issuer: tbs_fields.issuer,
        subject: tbs_fields.subject,
        not_before: tbs_fields.not_before,
        not_after: tbs_fields.not_after,
    })
}

fn parse_algorithm_oid(content: &[u8]) -> Result<Oid, DerError> {
    let oid_tlv = parse_tlv(content)?;
    if oid_tlv.tag() != 0x06 {
        return Err(DerError::UnsupportedStructure);
    }
    decode_oid(oid_tlv.value())
}

impl<'a> Certificate<'a> {
    /// Returns the full `SubjectPublicKeyInfo` encoding from the
    /// `TBSCertificate`, or `None` for a malformed structure.
    #[must_use]
    pub fn subject_public_key_info(&self) -> Option<&'a [u8]> {
        let outer = parse_tlv(self.tbs).ok()?;
        if outer.tag() != 0x30 {
            return None;
        }
        let content = outer.value();
        let mut offset = 0_usize;
        let first = parse_tlv(content).ok()?;
        if first.tag() == 0xa0 {
            offset = offset.checked_add(first.total_len())?;
        }
        for _ in 0..5 {
            let value = parse_tlv(content.get(offset..)?).ok()?;
            offset = offset.checked_add(value.total_len())?;
        }
        let key = parse_tlv(content.get(offset..)?).ok()?;
        if key.tag() != 0x30 {
            return None;
        }
        let end = offset.checked_add(key.total_len())?;
        content.get(offset..end)
    }

    /// Returns whether the certificate validity interval covers
    /// `unix_seconds`.
    #[must_use]
    pub const fn is_valid_at(&self, unix_seconds: i64) -> bool {
        unix_seconds >= self.not_before && unix_seconds <= self.not_after
    }

    /// Verifies this certificate's signature with the subject public
    /// key of `issuer`.
    ///
    /// Supported algorithms are SHA-1, SHA-256, SHA-384 and SHA-512
    /// with RSA PKCS #1 v1.5. This verifies the cryptographic binding
    /// to `issuer`; it does not by itself establish trust, validate
    /// name constraints, or check revocation. Callers must build a
    /// trusted chain from a configured root.
    #[must_use]
    pub fn verify_signature(&self, issuer: &Certificate<'_>) -> bool {
        let Some(spki) = issuer.subject_public_key_info() else {
            return false;
        };
        let Ok(key) = super::rsa::parse_rsa_public_key(spki) else {
            return false;
        };
        let algorithm = &self.signature_algorithm;
        if oid_eq(algorithm, &[1, 2, 840, 113_549, 1, 1, 5]) {
            super::rsa::verify_sha1(&key, self.tbs, self.signature)
        } else if oid_eq(algorithm, &[1, 2, 840, 113_549, 1, 1, 11]) {
            super::rsa::verify_sha256(&key, self.tbs, self.signature)
        } else if oid_eq(algorithm, &[1, 2, 840, 113_549, 1, 1, 12]) {
            super::rsa::verify_sha384(&key, self.tbs, self.signature)
        } else {
            oid_eq(algorithm, &[1, 2, 840, 113_549, 1, 1, 13])
                && super::rsa::verify_sha512(&key, self.tbs, self.signature)
        }
    }
}

/// Verifies a certificate chain from `leaf` to one of `anchors`.
///
/// Each step requires that the current certificate is valid at
/// `unix_seconds`, that the candidate issuer's subject encoding equals
/// the current certificate's issuer encoding, and that the candidate
/// verifies the current certificate's signature. The walk succeeds
/// when a trust anchor verifies the current certificate, and the
/// number of certificates in the accepted path is returned.
///
/// The walk is bounded by [`MAX_CHAIN_LENGTH`] and never revisits an
/// intermediate certificate. Name constraints, policies, key-usage
/// extensions, and revocation are not evaluated.
///
/// # Errors
///
/// Returns [`ChainError::NoTrustAnchor`] when `anchors` is empty,
/// [`ChainError::Expired`] for a certificate outside its validity
/// interval, [`ChainError::SignatureInvalid`] for a bad signature,
/// [`ChainError::IssuerNotFound`] when no supplied certificate names
/// the current issuer, and [`ChainError::ChainTooLong`] or
/// [`ChainError::Loop`] for an unusable candidate set.
pub fn verify_chain<'a>(
    leaf: &'a Certificate<'a>,
    intermediates: &'a [Certificate<'a>],
    anchors: &'a [Certificate<'a>],
    unix_seconds: i64,
) -> Result<usize, ChainError> {
    if anchors.is_empty() {
        return Err(ChainError::NoTrustAnchor);
    }
    if intermediates.len() > MAX_CHAIN_LENGTH {
        return Err(ChainError::ChainTooLong);
    }
    let mut current = leaf;
    let mut visited = 0_u64;
    let mut depth = 1_usize;
    loop {
        if depth
            > MAX_CHAIN_LENGTH
                .checked_add(1)
                .ok_or(ChainError::ChainTooLong)?
        {
            return Err(ChainError::ChainTooLong);
        }
        if !current.is_valid_at(unix_seconds) {
            return Err(ChainError::Expired);
        }
        let mut signature_failed = false;
        for anchor in anchors {
            if anchor.subject != current.issuer {
                continue;
            }
            if current.verify_signature(anchor) {
                return Ok(depth);
            }
            signature_failed = true;
        }
        if signature_failed {
            return Err(ChainError::SignatureInvalid);
        }
        let mut matched = None;
        for (index, candidate) in intermediates.iter().enumerate() {
            if candidate.subject != current.issuer {
                continue;
            }
            if !current.verify_signature(candidate) {
                return Err(ChainError::SignatureInvalid);
            }
            matched = Some(index);
            break;
        }
        let Some(index) = matched else {
            return Err(ChainError::IssuerNotFound);
        };
        let bit = 1_u64
            .checked_shl(u32::try_from(index).unwrap_or(u32::MAX))
            .ok_or(ChainError::ChainTooLong)?;
        if visited & bit != 0 {
            return Err(ChainError::Loop);
        }
        visited |= bit;
        current = intermediates.get(index).ok_or(ChainError::IssuerNotFound)?;
        depth = depth.checked_add(1).ok_or(ChainError::ChainTooLong)?;
    }
}

/// Fields extracted from a `TBSCertificate` sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
struct TbsFields<'a> {
    version: Option<u64>,
    serial: &'a [u8],
    issuer: &'a [u8],
    subject: &'a [u8],
    not_before: i64,
    not_after: i64,
}

#[allow(clippy::type_complexity)]
fn parse_tbs(content: &[u8]) -> Result<TbsFields<'_>, DerError> {
    let mut offset = 0_usize;
    let mut version = None;
    let first = parse_tlv(content.get(offset..).ok_or(DerError::Truncated)?)?;
    if first.tag() == 0xa0 {
        let inner = parse_tlv(first.value())?;
        if inner.tag() != 0x02 || inner.value().is_empty() {
            return Err(DerError::UnsupportedStructure);
        }
        let value = parse_integer(inner.value())?;
        if value > 2 {
            return Err(DerError::InvalidVersion);
        }
        version = Some(value);
        offset += first.header_len() + first.value().len();
    }
    let serial_tlv = parse_tlv(content.get(offset..).ok_or(DerError::Truncated)?)?;
    if serial_tlv.tag() != 0x02 {
        return Err(DerError::UnsupportedStructure);
    }
    let serial = serial_tlv.value();
    offset += serial_tlv.header_len() + serial_tlv.value().len();
    let sig_tlv = parse_tlv(content.get(offset..).ok_or(DerError::Truncated)?)?;
    offset += sig_tlv.header_len() + sig_tlv.value().len();
    let issuer_tlv = parse_tlv(content.get(offset..).ok_or(DerError::Truncated)?)?;
    if issuer_tlv.tag() != 0x30 {
        return Err(DerError::UnsupportedStructure);
    }
    let issuer_full_len = issuer_tlv.header_len() + issuer_tlv.value().len();
    let issuer = content
        .get(offset..offset + issuer_full_len)
        .ok_or(DerError::Truncated)?;
    offset += issuer_full_len;
    let validity_tlv = parse_tlv(content.get(offset..).ok_or(DerError::Truncated)?)?;
    if validity_tlv.tag() != 0x30 {
        return Err(DerError::UnsupportedStructure);
    }
    let first_time = parse_tlv(validity_tlv.value())?;
    let first_end = first_time.header_len() + first_time.value().len();
    let second_time = parse_tlv(
        validity_tlv
            .value()
            .get(first_end..)
            .ok_or(DerError::Truncated)?,
    )?;
    let not_before_real = parse_validity_time(first_time.value())?;
    let not_after_real = parse_validity_time(second_time.value())?;
    offset += validity_tlv.header_len() + validity_tlv.value().len();
    let subject_tlv = parse_tlv(content.get(offset..).ok_or(DerError::Truncated)?)?;
    if subject_tlv.tag() != 0x30 {
        return Err(DerError::UnsupportedStructure);
    }
    let subject_len = subject_tlv.header_len() + subject_tlv.value().len();
    let subject = content
        .get(offset..offset + subject_len)
        .ok_or(DerError::Truncated)?;
    Ok(TbsFields {
        version,
        serial,
        issuer,
        subject,
        not_before: not_before_real,
        not_after: not_after_real,
    })
}

fn parse_integer(content: &[u8]) -> Result<u64, DerError> {
    if content.is_empty() || content.len() > 8 {
        return Err(DerError::UnsupportedStructure);
    }
    let mut value = 0_u64;
    for byte in content {
        value = value
            .checked_mul(256)
            .and_then(|v| v.checked_add(u64::from(*byte)))
            .ok_or(DerError::LengthOverflow)?;
    }
    if *content.first().ok_or(DerError::UnsupportedStructure)? & 0x80 != 0 {
        return Err(DerError::UnsupportedStructure);
    }
    Ok(value)
}

fn digits(content: &[u8], start: usize, count: usize) -> Result<i64, DerError> {
    let mut value = 0_i64;
    for index in 0..count {
        let byte = *content.get(start + index).ok_or(DerError::InvalidTime)?;
        if !byte.is_ascii_digit() {
            return Err(DerError::InvalidTime);
        }
        value = value * 10 + i64::from(byte - b'0');
    }
    Ok(value)
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let mut y = year;
    if month <= 2 {
        y -= 1;
    }
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn make_unix(
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
) -> Result<i64, DerError> {
    if !(1..=12).contains(&month) {
        return Err(DerError::InvalidTime);
    }
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if leap_year(year) {
                29
            } else {
                28
            }
        }
    };
    if day < 1 || day > days_in_month || hour > 23 || minute > 59 || second > 60 {
        return Err(DerError::InvalidTime);
    }
    let days = days_from_civil(year, month, day);
    Ok(days * 86_400 + hour * 3600 + minute * 60 + second)
}
