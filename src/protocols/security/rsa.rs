//! # RSA signature verification
//!
//! Native arbitrary-precision modular exponentiation and PKCS #1 v1.5
//! signature verification. The implementation supports the RSA public
//! key encodings used by X.509 subject public key information and by
//! Secure Boot signature databases.
//!
//! Verification is deliberately limited to public-key operations.
//! Private-key signing, blinding, and padding generation are not part
//! of a verifier and are not implemented here.
//!
//! All arithmetic uses fixed-capacity stack buffers supporting moduli
//! up to 8192 bits. Inputs are validated for length and range before
//! use; no input can trigger a panic or an out-of-bounds access.

use super::sha1::sha1;
use super::sha256::sha256;
use super::sha512::{sha384, sha512};
use super::x509::{DerError, decode_oid, oid_eq, parse_tlv};
use core::cmp::Ordering;

/// The number of 64-bit limbs in a value.
const LIMBS: usize = 129;

/// The number of 64-bit limbs in a product.
const WIDE_LIMBS: usize = LIMBS * 2;

/// The DER prefix of a SHA-1 `DigestInfo` value.
const SHA1_DIGEST_INFO: [u8; 15] = [
    0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05, 0x00, 0x04, 0x14,
];

/// The DER prefix of a SHA-256 `DigestInfo` value.
const SHA256_DIGEST_INFO: [u8; 19] = [
    0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05,
    0x00, 0x04, 0x20,
];

/// The DER prefix of a SHA-384 `DigestInfo` value.
const SHA384_DIGEST_INFO: [u8; 19] = [
    0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02, 0x05,
    0x00, 0x04, 0x30,
];

/// The DER prefix of a SHA-512 `DigestInfo` value.
const SHA512_DIGEST_INFO: [u8; 19] = [
    0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05,
    0x00, 0x04, 0x40,
];

/// An RSA public key view borrowed from a DER encoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RsaPublicKey<'a> {
    /// The big-endian modulus without leading zero bytes.
    pub modulus: &'a [u8],
    /// The big-endian public exponent without leading zero bytes.
    pub exponent: &'a [u8],
}

/// An RSA key or signature decoding failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RsaError {
    /// The input ended before a complete value.
    Truncated,
    /// The input was not a supported DER shape.
    UnsupportedStructure,
    /// The key was empty, negative, or too large.
    InvalidKey,
    /// The algorithm identifier was not RSA encryption.
    UnsupportedAlgorithm,
    /// The signature length did not match the modulus.
    SignatureLength,
}

impl core::fmt::Display for RsaError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Self::Truncated => "truncated RSA encoding",
            Self::UnsupportedStructure => "unsupported RSA structure",
            Self::InvalidKey => "invalid RSA key",
            Self::UnsupportedAlgorithm => "unsupported public key algorithm",
            Self::SignatureLength => "RSA signature length mismatch",
        };
        formatter.write_str(text)
    }
}

impl From<DerError> for RsaError {
    fn from(error: DerError) -> Self {
        match error {
            DerError::Truncated => Self::Truncated,
            _ => Self::UnsupportedStructure,
        }
    }
}

/// A fixed-capacity unsigned integer used for modular arithmetic.
#[derive(Clone, Copy)]
struct Big {
    limbs: [u64; LIMBS],
    len: usize,
}

impl Big {
    /// Creates zero.
    const fn zero() -> Self {
        Self {
            limbs: [0; LIMBS],
            len: 0,
        }
    }

    /// Creates one.
    fn one() -> Self {
        let mut value = Self::zero();
        value.limbs[0] = 1;
        value.len = 1;
        value
    }

    /// Decodes big-endian bytes, rejecting oversized input.
    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() > LIMBS * 8 {
            return None;
        }
        let mut value = Self::zero();
        let mut end = bytes.len();
        let mut limb = 0_usize;
        while end > 0 && limb < LIMBS {
            let start = end.saturating_sub(8);
            let mut chunk = [0_u8; 8];
            let take = end - start;
            chunk[8 - take..].copy_from_slice(&bytes[start..end]);
            value.limbs[limb] = u64::from_be_bytes(chunk);
            limb += 1;
            end = start;
        }
        value.len = limb;
        value.normalize();
        Some(value)
    }

    /// Removes leading zero limbs.
    fn normalize(&mut self) {
        while self.len > 0 && self.limbs[self.len - 1] == 0 {
            self.len -= 1;
        }
    }

    /// Writes the value big-endian into `output`, zero-padding on the left.
    fn write_be(&self, output: &mut [u8]) -> bool {
        if output.len() < self.len * 8 {
            return false;
        }
        for (index, byte) in output.iter_mut().rev().enumerate() {
            let limb = index / 8;
            let bytes = if limb < self.len {
                self.limbs[limb].to_le_bytes()
            } else {
                [0_u8; 8]
            };
            *byte = bytes[index % 8];
        }
        true
    }

    /// Compares two values.
    fn cmp(&self, other: &Self) -> Ordering {
        if self.len != other.len {
            return self.len.cmp(&other.len);
        }
        for (left, right) in self.limbs[..self.len]
            .iter()
            .rev()
            .zip(other.limbs[..other.len].iter().rev())
        {
            let ordering = left.cmp(right);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        Ordering::Equal
    }

    /// Shifts left by one bit and inserts `bit`.
    fn shift_in(&mut self, bit: u64) {
        let mut carry = bit & 1;
        for limb in &mut self.limbs[..self.len] {
            let next = *limb >> 63;
            *limb = (*limb << 1) | carry;
            carry = next;
        }
        if carry != 0 {
            if self.len < LIMBS {
                self.limbs[self.len] = carry;
                self.len += 1;
            }
        } else {
            self.normalize();
        }
    }

    /// Subtracts `other` in place; `self` must be at least `other`.
    fn sub_assign(&mut self, other: &Self) {
        let mut borrow = 0_u64;
        for (index, limb) in self.limbs[..self.len].iter_mut().enumerate() {
            let rhs = if index < other.len {
                other.limbs[index]
            } else {
                0
            };
            let (difference, first) = limb.overflowing_sub(rhs);
            let (difference, second) = difference.overflowing_sub(borrow);
            *limb = difference;
            borrow = u64::from(first || second);
        }
        self.normalize();
    }
}

/// A fixed-capacity product.
#[derive(Clone, Copy)]
struct Wide {
    limbs: [u64; WIDE_LIMBS],
    len: usize,
}

/// Returns the low 64 bits of a 128-bit value.
fn low(value: u128) -> u64 {
    u64::try_from(value & u128::from(u64::MAX)).unwrap_or(0)
}

/// Returns the high 64 bits of a 128-bit value.
fn high(value: u128) -> u64 {
    u64::try_from(value >> 64).unwrap_or(0)
}

/// Multiplies two values.
fn multiply(left: &Big, right: &Big) -> Wide {
    let mut product = Wide {
        limbs: [0; WIDE_LIMBS],
        len: 0,
    };
    if left.len == 0 || right.len == 0 {
        return product;
    }
    for (i, left_limb) in left.limbs[..left.len].iter().enumerate() {
        let mut carry = 0_u128;
        for (j, right_limb) in right.limbs[..right.len].iter().enumerate() {
            let index = i + j;
            let accumulator = u128::from(*left_limb) * u128::from(*right_limb)
                + u128::from(product.limbs[index])
                + carry;
            product.limbs[index] = low(accumulator);
            carry = u128::from(high(accumulator));
        }
        let mut index = i + right.len;
        while carry != 0 && index < WIDE_LIMBS {
            let accumulator = u128::from(product.limbs[index]) + carry;
            product.limbs[index] = low(accumulator);
            carry = u128::from(high(accumulator));
            index += 1;
        }
    }
    product.len = left.len + right.len;
    while product.len > 0 && product.limbs[product.len - 1] == 0 {
        product.len -= 1;
    }
    product
}

/// Reduces a product modulo `modulus` by binary long division.
fn reduce(product: &Wide, modulus: &Big) -> Big {
    let mut remainder = Big::zero();
    if modulus.len == 0 {
        return remainder;
    }
    for index in (0..product.len * 64).rev() {
        let bit = (product.limbs[index / 64] >> (index % 64)) & 1;
        remainder.shift_in(bit);
        if remainder.cmp(modulus) != Ordering::Less {
            remainder.sub_assign(modulus);
        }
    }
    remainder
}

/// Computes `base * multiplier mod modulus`.
fn multiply_mod(left: &Big, right: &Big, modulus: &Big) -> Big {
    reduce(&multiply(left, right), modulus)
}

/// Computes `base^exponent mod modulus` for a big-endian exponent.
fn power_mod(base: &Big, exponent: &[u8], modulus: &Big) -> Option<Big> {
    if modulus.len == 0 {
        return None;
    }
    let mut result = Big::one();
    let base = reduce(
        &Wide {
            limbs: {
                let mut limbs = [0_u64; WIDE_LIMBS];
                limbs[..LIMBS].copy_from_slice(&base.limbs);
                limbs
            },
            len: base.len,
        },
        modulus,
    );
    for byte in exponent {
        for shift in (0..8).rev() {
            result = multiply_mod(&result, &result, modulus);
            if (byte >> shift) & 1 != 0 {
                result = multiply_mod(&result, &base, modulus);
            }
        }
    }
    Some(result)
}

/// Strips leading zero bytes from a positive DER integer.
fn integer_bytes(content: &[u8]) -> Result<&[u8], RsaError> {
    if content.is_empty() || content[0] & 0x80 != 0 {
        return Err(RsaError::InvalidKey);
    }
    let mut start = 0_usize;
    while start < content.len() && content[start] == 0 {
        start += 1;
    }
    let value = content.get(start..).ok_or(RsaError::Truncated)?;
    if value.is_empty() {
        return Err(RsaError::InvalidKey);
    }
    Ok(value)
}

/// Parses a DER `RSAPublicKey` or `SubjectPublicKeyInfo` encoding.
///
/// Both `RSAPublicKey ::= SEQUENCE { INTEGER, INTEGER }` and the
/// X.509 `SubjectPublicKeyInfo` wrapper carrying the `rsaEncryption`
/// algorithm identifier are accepted.
///
/// # Errors
///
/// Returns [`RsaError`] for truncated, negative, oversized, or
/// unsupported encodings.
pub fn parse_rsa_public_key(input: &[u8]) -> Result<RsaPublicKey<'_>, RsaError> {
    let outer = parse_tlv(input)?;
    if outer.tag() != 0x30 {
        return Err(RsaError::UnsupportedStructure);
    }
    let content = outer.value();
    let first = parse_tlv(content)?;
    match first.tag() {
        0x02 => {
            let modulus = integer_bytes(first.value())?;
            let consumed = first.header_len() + first.value().len();
            let rest = content.get(consumed..).ok_or(RsaError::Truncated)?;
            if rest.is_empty() {
                return Err(RsaError::Truncated);
            }
            let exponent_tlv = parse_tlv(rest)?;
            if exponent_tlv.tag() != 0x02 {
                return Err(RsaError::UnsupportedStructure);
            }
            let exponent = integer_bytes(exponent_tlv.value())?;
            Ok(RsaPublicKey { modulus, exponent })
        }
        0x30 => {
            let algorithm = parse_tlv(first.value())?;
            if algorithm.tag() != 0x06 {
                return Err(RsaError::UnsupportedStructure);
            }
            let oid = decode_oid(algorithm.value())?;
            if !oid_eq(&oid, &[1, 2, 840, 113_549, 1, 1, 1]) {
                return Err(RsaError::UnsupportedAlgorithm);
            }
            let consumed = first.header_len() + first.value().len();
            let rest = content.get(consumed..).ok_or(RsaError::Truncated)?;
            let key_tlv = parse_tlv(rest)?;
            if key_tlv.tag() != 0x03 {
                return Err(RsaError::UnsupportedStructure);
            }
            let bits = key_tlv.value();
            if bits.first() != Some(&0) {
                return Err(RsaError::UnsupportedStructure);
            }
            let inner = bits.get(1..).ok_or(RsaError::Truncated)?;
            parse_rsa_public_key(inner)
        }
        _ => Err(RsaError::UnsupportedStructure),
    }
}

/// Verifies a PKCS #1 v1.5 signature over a pre-encoded `DigestInfo`.
///
/// Returns `false` for malformed keys, out-of-range signatures, and
/// any padding or digest mismatch.
#[must_use]
pub fn verify_digest_info(key: &RsaPublicKey<'_>, signature: &[u8], digest_info: &[u8]) -> bool {
    let Some(modulus) = Big::from_be_bytes(key.modulus) else {
        return false;
    };
    let Some(exponent) = Big::from_be_bytes(key.exponent) else {
        return false;
    };
    if modulus.len == 0 || exponent.len == 0 || key.modulus.len() > LIMBS * 8 - 1 {
        return false;
    }
    if signature.len() != key.modulus.len() {
        return false;
    }
    let Some(value) = Big::from_be_bytes(signature) else {
        return false;
    };
    if value.cmp(&modulus) != Ordering::Less {
        return false;
    }
    let Some(decoded) = power_mod(&value, key.exponent, &modulus) else {
        return false;
    };
    let mut encoded = [0_u8; LIMBS * 8];
    let length = key.modulus.len();
    if !decoded.write_be(&mut encoded[..length]) {
        return false;
    }
    if encoded.first() != Some(&0x00) || encoded.get(1) != Some(&0x01) {
        return false;
    }
    let mut index = 2_usize;
    while index < length && encoded[index] == 0xFF {
        index += 1;
    }
    if index < 10 || encoded.get(index) != Some(&0x00) {
        return false;
    }
    match encoded.get(index + 1..length) {
        Some(fields) => fields == digest_info,
        None => false,
    }
}

/// Verifies a PKCS #1 v1.5 SHA-1 signature over `message`.
#[must_use]
pub fn verify_sha1(key: &RsaPublicKey<'_>, message: &[u8], signature: &[u8]) -> bool {
    let digest = sha1(message);
    let mut info = [0_u8; 15 + 20];
    info[..15].copy_from_slice(&SHA1_DIGEST_INFO);
    info[15..].copy_from_slice(&digest);
    verify_digest_info(key, signature, &info)
}

/// Verifies a PKCS #1 v1.5 SHA-256 signature over `message`.
#[must_use]
pub fn verify_sha256(key: &RsaPublicKey<'_>, message: &[u8], signature: &[u8]) -> bool {
    let digest = sha256(message);
    let mut info = [0_u8; 19 + 32];
    info[..19].copy_from_slice(&SHA256_DIGEST_INFO);
    info[19..].copy_from_slice(&digest);
    verify_digest_info(key, signature, &info)
}

/// Verifies a PKCS #1 v1.5 SHA-384 signature over `message`.
#[must_use]
pub fn verify_sha384(key: &RsaPublicKey<'_>, message: &[u8], signature: &[u8]) -> bool {
    let digest = sha384(message);
    let mut info = [0_u8; 19 + 48];
    info[..19].copy_from_slice(&SHA384_DIGEST_INFO);
    info[19..].copy_from_slice(&digest);
    verify_digest_info(key, signature, &info)
}

/// Verifies a PKCS #1 v1.5 SHA-512 signature over `message`.
#[must_use]
pub fn verify_sha512(key: &RsaPublicKey<'_>, message: &[u8], signature: &[u8]) -> bool {
    let digest = sha512(message);
    let mut info = [0_u8; 19 + 64];
    info[..19].copy_from_slice(&SHA512_DIGEST_INFO);
    info[19..].copy_from_slice(&digest);
    verify_digest_info(key, signature, &info)
}
