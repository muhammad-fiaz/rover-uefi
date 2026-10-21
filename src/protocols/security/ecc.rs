//! # ECDSA verification over NIST prime curves
//!
//! Native implementation of the elliptic curve arithmetic of the NIST
//! prime curves P-256 (also known as secp256r1) and P-384 (also known
//! as secp384r1) together with ECDSA signature verification as
//! specified by FIPS 186-4 and SEC 1.
//!
//! The module provides
//!
//! * modular arithmetic over the field prime and over the group order,
//! * affine point addition, doubling, and scalar multiplication on the
//!   short Weierstrass curves `y^2 = x^3 - 3x + b`,
//! * parsing of public keys from the X.509 `SubjectPublicKeyInfo`
//!   structure and from raw uncompressed points,
//! * parsing of signatures from the raw `(r, s)` form and from the DER
//!   `ECDSA-Sig-Value` structure, and
//! * ECDSA signature verification.
//!
//! Only public-key operations are provided; signing is out of scope.
//! The implementation performs no heap allocation and depends on no
//! external crate.
//!
//! # Constant time
//!
//! The scalar multiplication uses a Montgomery ladder that performs the
//! same sequence of point operations for every scalar, and the modular
//! exponentiation uses a fixed window. The field inversion of the
//! affine point arithmetic is a binary extended Euclidean algorithm
//! whose running time depends on the inverted value, so this
//! implementation does not claim constant-time behaviour overall.

use super::x509::{decode_oid, oid_eq, parse_tlv};
use core::cmp::Ordering;

/// The number of 64-bit limbs of the widest supported integer.
const MAX_LIMBS: usize = 6;

/// The number of bytes of the widest supported integer.
const MAX_BYTES: usize = MAX_LIMBS * 8;

/// Returns the low 64 bits of `value`.
const fn low_u64(value: u128) -> u64 {
    let bytes = value.to_le_bytes();
    u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

/// Returns the high 64 bits of `value`.
const fn high_u64(value: u128) -> u64 {
    let bytes = value.to_le_bytes();
    u64::from_le_bytes([
        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
    ])
}

/// The object identifier of an elliptic curve public key.
const OID_EC_PUBLIC_KEY: [u64; 6] = [1, 2, 840, 10045, 2, 1];

/// The object identifier of the P-256 named curve.
const OID_PRIME256V1: [u64; 7] = [1, 2, 840, 10045, 3, 1, 7];

/// The object identifier of the P-384 named curve.
const OID_SECP384R1: [u64; 5] = [1, 3, 132, 0, 34];

/// The field prime of P-256.
const P256_FIELD_LIMBS: [u64; MAX_LIMBS] = [
    0xffff_ffff_ffff_ffff,
    0x0000_0000_ffff_ffff,
    0x0000_0000_0000_0000,
    0xffff_ffff_0000_0001,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
];

/// The group order of P-256.
const P256_ORDER_LIMBS: [u64; MAX_LIMBS] = [
    0xf3b9_cac2_fc63_2551,
    0xbce6_faad_a717_9e84,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_0000_0000,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
];

/// The coefficient b of P-256.
const P256_B_LIMBS: [u64; MAX_LIMBS] = [
    0x3bce_3c3e_27d2_604b,
    0x651d_06b0_cc53_b0f6,
    0xb3eb_bd55_7698_86bc,
    0x5ac6_35d8_aa3a_93e7,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
];

/// The x coordinate of the base point of P-256.
const P256_GX_LIMBS: [u64; MAX_LIMBS] = [
    0xf4a1_3945_d898_c296,
    0x7703_7d81_2deb_33a0,
    0xf8bc_e6e5_63a4_40f2,
    0x6b17_d1f2_e12c_4247,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
];

/// The y coordinate of the base point of P-256.
const P256_GY_LIMBS: [u64; MAX_LIMBS] = [
    0xcbb6_4068_37bf_51f5,
    0x2bce_3357_6b31_5ece,
    0x8ee7_eb4a_7c0f_9e16,
    0x4fe3_42e2_fe1a_7f9b,
    0x0000_0000_0000_0000,
    0x0000_0000_0000_0000,
];

/// The field prime of P-384.
const P384_FIELD_LIMBS: [u64; MAX_LIMBS] = [
    0x0000_0000_ffff_ffff,
    0xffff_ffff_0000_0000,
    0xffff_ffff_ffff_fffe,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
];

/// The group order of P-384.
const P384_ORDER_LIMBS: [u64; MAX_LIMBS] = [
    0xecec_196a_ccc5_2973,
    0x581a_0db2_48b0_a77a,
    0xc763_4d81_f437_2ddf,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
];

/// The coefficient b of P-384.
const P384_B_LIMBS: [u64; MAX_LIMBS] = [
    0x2a85_c8ed_d3ec_2aef,
    0xc656_398d_8a2e_d19d,
    0x0314_088f_5013_875a,
    0x181d_9c6e_fe81_4112,
    0x988e_056b_e3f8_2d19,
    0xb331_2fa7_e23e_e7e4,
];

/// The x coordinate of the base point of P-384.
const P384_GX_LIMBS: [u64; MAX_LIMBS] = [
    0x3a54_5e38_7276_0ab7,
    0x5502_f25d_bf55_296c,
    0x59f7_41e0_8254_2a38,
    0x6e1d_3b62_8ba7_9b98,
    0x8eb1_c71e_f320_ad74,
    0xaa87_ca22_be8b_0537,
];

/// The y coordinate of the base point of P-384.
const P384_GY_LIMBS: [u64; MAX_LIMBS] = [
    0x7a43_1d7c_90ea_0e5f,
    0x0a60_b1ce_1d7e_819d,
    0xe9da_3113_b5f0_b8c0,
    0xf8f4_1dbd_289a_147c,
    0x5d9e_98bf_9292_dc29,
    0x3617_de4a_9626_2c6f,
];

/// The field prime of P-256 as bytes.
const P256_FIELD_BYTES: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

/// The group order of P-256 as bytes.
const P256_ORDER_BYTES: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xbc, 0xe6, 0xfa, 0xad, 0xa7, 0x17, 0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2, 0xfc, 0x63, 0x25, 0x51,
];

/// The coefficient b of P-256 as bytes.
const P256_B_BYTES: [u8; 32] = [
    0x5a, 0xc6, 0x35, 0xd8, 0xaa, 0x3a, 0x93, 0xe7, 0xb3, 0xeb, 0xbd, 0x55, 0x76, 0x98, 0x86, 0xbc,
    0x65, 0x1d, 0x06, 0xb0, 0xcc, 0x53, 0xb0, 0xf6, 0x3b, 0xce, 0x3c, 0x3e, 0x27, 0xd2, 0x60, 0x4b,
];

/// The field prime of P-384 as bytes.
const P384_FIELD_BYTES: [u8; 48] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
];

/// The group order of P-384 as bytes.
const P384_ORDER_BYTES: [u8; 48] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc7, 0x63, 0x4d, 0x81, 0xf4, 0x37, 0x2d, 0xdf,
    0x58, 0x1a, 0x0d, 0xb2, 0x48, 0xb0, 0xa7, 0x7a, 0xec, 0xec, 0x19, 0x6a, 0xcc, 0xc5, 0x29, 0x73,
];

/// The coefficient b of P-384 as bytes.
const P384_B_BYTES: [u8; 48] = [
    0xb3, 0x31, 0x2f, 0xa7, 0xe2, 0x3e, 0xe7, 0xe4, 0x98, 0x8e, 0x05, 0x6b, 0xe3, 0xf8, 0x2d, 0x19,
    0x18, 0x1d, 0x9c, 0x6e, 0xfe, 0x81, 0x41, 0x12, 0x03, 0x14, 0x08, 0x8f, 0x50, 0x13, 0x87, 0x5a,
    0xc6, 0x56, 0x39, 0x8d, 0x8a, 0x2e, 0xd1, 0x9d, 0x2a, 0x85, 0xc8, 0xed, 0xd3, 0xec, 0x2a, 0xef,
];

/// A fixed-capacity unsigned integer used for the curve arithmetic.
///
/// The value is stored as little-endian limbs with a significant length
/// so that comparisons and shifts stay cheap. The type is private; the
/// public API exchanges values as big-endian byte strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Big {
    limbs: [u64; MAX_LIMBS],
    len: usize,
}

impl Big {
    /// Creates the value zero.
    const fn zero() -> Self {
        Self {
            limbs: [0; MAX_LIMBS],
            len: 0,
        }
    }

    /// Creates the value one.
    const fn one() -> Self {
        Self {
            limbs: [1, 0, 0, 0, 0, 0],
            len: 1,
        }
    }

    /// Creates a value from little-endian limbs.
    fn from_limbs(limbs: [u64; MAX_LIMBS]) -> Self {
        let mut value = Self {
            limbs,
            len: MAX_LIMBS,
        };
        value.normalize();
        value
    }

    /// Decodes a big-endian byte string.
    ///
    /// The least significant limb receives the last bytes of the input.
    /// Returns `None` when the input does not fit into the capacity.
    fn from_be_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() > MAX_BYTES {
            return None;
        }
        let mut limbs = [0_u64; MAX_LIMBS];
        let mut len = 0_usize;
        let mut end = bytes.len();
        while end > 0 {
            let start = end.saturating_sub(8);
            let mut chunk = [0_u8; 8];
            let take = end - start;
            if let (Some(slot), Some(source)) = (chunk.get_mut(8 - take..), bytes.get(start..end)) {
                slot.copy_from_slice(source);
            }
            if let Some(slot) = limbs.get_mut(len) {
                *slot = u64::from_be_bytes(chunk);
            }
            len += 1;
            end = start;
        }
        let mut value = Self { limbs, len };
        value.normalize();
        Some(value)
    }

    /// Writes the value as big-endian bytes into `out`.
    ///
    /// The value is left padded with zeros. Returns `false` when the
    /// buffer is too small.
    fn write_be(&self, out: &mut [u8]) -> bool {
        let needed = self.len * 8;
        if out.len() < needed {
            return false;
        }
        let offset = out.len() - needed;
        for index in 0..self.len {
            let value = self.limbs.get(index).copied().unwrap_or(0);
            let bytes = value.to_be_bytes();
            let base = offset + needed - (index + 1) * 8;
            for position in 0..8 {
                if let Some(slot) = out.get_mut(base + position) {
                    *slot = bytes.get(position).copied().unwrap_or(0);
                }
            }
        }
        true
    }

    /// Removes the leading zero limbs.
    fn normalize(&mut self) {
        while self.len > 0 && self.limbs.get(self.len - 1).copied().unwrap_or(0) == 0 {
            self.len -= 1;
        }
    }

    /// Returns whether the value is zero.
    const fn is_zero(&self) -> bool {
        self.len == 0
    }

    /// Returns whether the value is one.
    const fn is_one(&self) -> bool {
        self.len == 1 && self.limbs[0] == 1
    }

    /// Returns whether the value is even.
    const fn is_even(&self) -> bool {
        self.limbs[0] & 1 == 0
    }

    /// Returns the bit at `index`.
    fn bit(&self, index: usize) -> bool {
        let value = self.limbs.get(index / 64).copied().unwrap_or(0);
        (value >> (index % 64)) & 1 == 1
    }

    /// Shifts the value right by one bit.
    fn shift_right(&mut self) {
        for index in 0..self.len {
            let next = self.limbs.get(index + 1).copied().unwrap_or(0);
            if let Some(slot) = self.limbs.get_mut(index) {
                *slot = (*slot >> 1) | (next << 63);
            }
        }
        self.normalize();
    }

    /// Compares two values.
    fn compare(&self, other: &Self) -> Ordering {
        if self.len != other.len {
            return self.len.cmp(&other.len);
        }
        for index in (0..self.len).rev() {
            let left = self.limbs.get(index).copied().unwrap_or(0);
            let right = other.limbs.get(index).copied().unwrap_or(0);
            let ordering = left.cmp(&right);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        Ordering::Equal
    }

    /// Adds two values.
    ///
    /// Returns `None` when the sum does not fit into the capacity.
    fn add(&self, other: &Self) -> Option<Self> {
        let mut limbs = [0_u64; MAX_LIMBS];
        let count = self.len.max(other.len);
        let mut carry = 0_u64;
        for index in 0..count {
            let left = self.limbs.get(index).copied().unwrap_or(0);
            let right = other.limbs.get(index).copied().unwrap_or(0);
            let (sum, first) = left.overflowing_add(right);
            let (sum, second) = sum.overflowing_add(carry);
            if let Some(slot) = limbs.get_mut(index) {
                *slot = sum;
            }
            carry = u64::from(first || second);
        }
        if carry != 0 {
            return None;
        }
        let mut value = Self { limbs, len: count };
        value.normalize();
        Some(value)
    }

    /// Subtracts `other` from the value.
    ///
    /// Returns `None` when `other` is larger than the value.
    fn subtract(&self, other: &Self) -> Option<Self> {
        if self.compare(other) == Ordering::Less {
            return None;
        }
        let mut limbs = [0_u64; MAX_LIMBS];
        let mut borrow = 0_u64;
        for index in 0..self.len {
            let left = self.limbs.get(index).copied().unwrap_or(0);
            let right = other.limbs.get(index).copied().unwrap_or(0);
            let (difference, first) = left.overflowing_sub(right);
            let (difference, second) = difference.overflowing_sub(borrow);
            if let Some(slot) = limbs.get_mut(index) {
                *slot = difference;
            }
            borrow = u64::from(first || second);
        }
        let mut value = Self {
            limbs,
            len: self.len,
        };
        value.normalize();
        Some(value)
    }

    /// Computes the remainder of the division by `modulus`.
    fn remainder(&self, modulus: &Self) -> Self {
        if modulus.is_zero() {
            return Self::zero();
        }
        let mut remainder = [0_u64; 2 * MAX_LIMBS];
        for index in (0..(2 * MAX_LIMBS * 64)).rev() {
            let bit = (self.limbs.get(index / 64).copied().unwrap_or(0) >> (index % 64)) & 1;
            let mut carry = bit;
            for limb in &mut remainder {
                let next = *limb >> 63;
                *limb = (*limb << 1) | carry;
                carry = next;
            }
            if compare_wide(&remainder, &modulus.limbs) != Ordering::Less {
                remainder = subtract_wide(&remainder, &modulus.limbs);
            }
        }
        let mut limbs = [0_u64; MAX_LIMBS];
        for index in 0..MAX_LIMBS {
            if let Some(slot) = limbs.get_mut(index) {
                *slot = remainder.get(index).copied().unwrap_or(0);
            }
        }
        Self::from_limbs(limbs)
    }

    /// Computes the modular inverse modulo the odd `modulus`.
    ///
    /// The binary extended Euclidean algorithm is used. Returns `None`
    /// when the value has no inverse.
    fn inverse(&self, modulus: &Self) -> Option<Self> {
        if self.is_zero() || modulus.is_zero() {
            return None;
        }
        let mut u = *self;
        let mut v = *modulus;
        let mut x1 = Self::one();
        let mut x2 = Self::zero();
        while !u.is_one() && !v.is_one() {
            if u.is_zero() || v.is_zero() {
                return None;
            }
            while u.is_even() {
                u.shift_right();
                x1 = halve_modulo(&x1, modulus);
            }
            while v.is_even() {
                v.shift_right();
                x2 = halve_modulo(&x2, modulus);
            }
            if u.compare(&v).is_ge() {
                u = u.subtract(&v).unwrap_or_else(Self::zero);
                x1 = subtract_modulo(&x1, &x2, modulus);
            } else {
                v = v.subtract(&u).unwrap_or_else(Self::zero);
                x2 = subtract_modulo(&x2, &x1, modulus);
            }
        }
        if u.is_one() { Some(x1) } else { Some(x2) }
    }
}

/// Returns the half of `value` modulo `modulus`.
///
/// The value is reduced modulo the odd modulus before the shift, so the
/// result stays in the range `0 .. modulus`.
fn halve_modulo(value: &Big, modulus: &Big) -> Big {
    if value.is_even() {
        let mut half = *value;
        half.shift_right();
        return half;
    }
    let mut shifted = *value;
    shifted.shift_right();
    let mut modulus_half = *modulus;
    modulus_half.shift_right();
    let sum = shifted.add(&modulus_half).unwrap_or_else(Big::zero);
    sum.add(&Big::one()).unwrap_or_else(Big::zero)
}

/// Returns `left - right` reduced modulo `modulus`.
fn subtract_modulo(left: &Big, right: &Big, modulus: &Big) -> Big {
    match left.compare(right) {
        Ordering::Less => {
            let difference = right.subtract(left).unwrap_or_else(Big::zero);
            modulus.subtract(&difference).unwrap_or_else(Big::zero)
        }
        _ => left.subtract(right).unwrap_or_else(Big::zero),
    }
}

/// Compares a wide value with a narrow modulus.
fn compare_wide(left: &[u64; 2 * MAX_LIMBS], right: &[u64; MAX_LIMBS]) -> Ordering {
    for index in (0..(2 * MAX_LIMBS)).rev() {
        let left_limb = left.get(index).copied().unwrap_or(0);
        let right_limb = right.get(index).copied().unwrap_or(0);
        let ordering = left_limb.cmp(&right_limb);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    Ordering::Equal
}

/// Subtracts a narrow modulus from a wide value.
fn subtract_wide(left: &[u64; 2 * MAX_LIMBS], right: &[u64; MAX_LIMBS]) -> [u64; 2 * MAX_LIMBS] {
    let mut result = *left;
    let mut borrow = 0_u64;
    for index in 0..(2 * MAX_LIMBS) {
        let left_limb = left.get(index).copied().unwrap_or(0);
        let right_limb = right.get(index).copied().unwrap_or(0);
        let (difference, first) = left_limb.overflowing_sub(right_limb);
        let (difference, second) = difference.overflowing_sub(borrow);
        if let Some(slot) = result.get_mut(index) {
            *slot = difference;
        }
        borrow = u64::from(first || second);
    }
    result
}

/// An odd modulus together with the constants of the Montgomery
/// representation used for the modular multiplication.
#[derive(Clone, Copy, Debug)]
struct Prime {
    limbs: [u64; MAX_LIMBS],
    len: usize,
    n0inv: u64,
    r2: [u64; MAX_LIMBS],
}

impl Prime {
    /// Creates a modulus from little-endian limbs.
    const fn new(limbs: [u64; MAX_LIMBS], len: usize) -> Self {
        let n0inv = Self::compute_n0inv(limbs);
        let r2 = Self::compute_power_of_two(limbs, len, 2);
        Self {
            limbs,
            len,
            n0inv,
            r2,
        }
    }

    /// Computes `-modulus^-1 mod 2^64` for the odd modulus.
    const fn compute_n0inv(modulus: [u64; MAX_LIMBS]) -> u64 {
        let mut inverse = 1_u64;
        let mut count = 0_usize;
        while count < 6 {
            inverse = inverse.wrapping_mul(2_u64.wrapping_sub(modulus[0].wrapping_mul(inverse)));
            count += 1;
        }
        inverse.wrapping_neg()
    }

    /// Computes `2^(64 * len * exponent) mod modulus`.
    const fn compute_power_of_two(
        modulus: [u64; MAX_LIMBS],
        len: usize,
        exponent: usize,
    ) -> [u64; MAX_LIMBS] {
        let mut value = [0_u64; MAX_LIMBS];
        value[0] = 1;
        let mut count = 0_usize;
        while count < 64 * len * exponent {
            value = Self::double_modulo(value, modulus, len);
            count += 1;
        }
        value
    }

    /// Computes `2 * value mod modulus` for a value smaller than the
    /// modulus.
    const fn double_modulo(
        value: [u64; MAX_LIMBS],
        modulus: [u64; MAX_LIMBS],
        len: usize,
    ) -> [u64; MAX_LIMBS] {
        let mut result = value;
        let mut carry = 0_u64;
        let mut index = 0_usize;
        while index < len {
            let next = result[index] >> 63;
            result[index] = (result[index] << 1) | carry;
            carry = next;
            index += 1;
        }
        if carry == 0 {
            if Self::compare_or_equal(result, modulus, len) {
                result = Self::subtract_limbs(result, modulus, len);
            }
        } else {
            let complement = Self::complement(modulus, len);
            result = Self::add_limbs(result, complement, len);
        }
        result
    }

    /// Returns whether `left` is greater than or equal to `right`.
    const fn compare_or_equal(left: [u64; MAX_LIMBS], right: [u64; MAX_LIMBS], len: usize) -> bool {
        let mut index = len;
        while index > 0 {
            index -= 1;
            let left_limb = left[index];
            let right_limb = right[index];
            if left_limb > right_limb {
                return true;
            }
            if left_limb < right_limb {
                return false;
            }
        }
        true
    }

    /// Subtracts `right` from `left`.
    const fn subtract_limbs(
        left: [u64; MAX_LIMBS],
        right: [u64; MAX_LIMBS],
        len: usize,
    ) -> [u64; MAX_LIMBS] {
        let mut result = left;
        let mut borrow = 0_u64;
        let mut index = 0_usize;
        while index < len {
            let (difference, first) = result[index].overflowing_sub(right[index]);
            let (difference, second) = difference.overflowing_sub(borrow);
            result[index] = difference;
            borrow = if first || second { 1 } else { 0 };
            index += 1;
        }
        result
    }

    /// Adds `right` to `left`.
    const fn add_limbs(
        left: [u64; MAX_LIMBS],
        right: [u64; MAX_LIMBS],
        len: usize,
    ) -> [u64; MAX_LIMBS] {
        let mut result = left;
        let mut carry = 0_u64;
        let mut index = 0_usize;
        while index < len {
            let (sum, first) = result[index].overflowing_add(right[index]);
            let (sum, second) = sum.overflowing_add(carry);
            result[index] = sum;
            carry = if first || second { 1 } else { 0 };
            index += 1;
        }
        result
    }

    /// Computes `2^(64 * len) - modulus`.
    const fn complement(modulus: [u64; MAX_LIMBS], len: usize) -> [u64; MAX_LIMBS] {
        let mut result = [0_u64; MAX_LIMBS];
        let mut index = 0_usize;
        while index < len {
            result[index] = !modulus[index];
            index += 1;
        }
        let mut carry = 1_u64;
        let mut index = 0_usize;
        while carry != 0 && index < len {
            let (sum, overflow) = result[index].overflowing_add(carry);
            result[index] = sum;
            carry = if overflow { 1 } else { 0 };
            index += 1;
        }
        result
    }

    /// Returns the modulus as an integer.
    fn big(&self) -> Big {
        Big::from_limbs(self.limbs)
    }

    /// Converts a plain value into the Montgomery representation.
    fn montgomery_form(&self, value: &[u64; MAX_LIMBS]) -> [u64; MAX_LIMBS] {
        self.montgomery_mul(value, &self.r2)
    }

    /// Converts a Montgomery value into the plain representation.
    fn plain_form(&self, value: &[u64; MAX_LIMBS]) -> [u64; MAX_LIMBS] {
        let plain_one = [1_u64, 0, 0, 0, 0, 0];
        self.montgomery_mul(value, &plain_one)
    }

    /// Computes `left * right * R^-1 mod modulus` with the coarsely
    /// integrated operand scanning method.
    fn montgomery_mul(
        &self,
        left: &[u64; MAX_LIMBS],
        right: &[u64; MAX_LIMBS],
    ) -> [u64; MAX_LIMBS] {
        let mut accumulator = [0_u64; MAX_LIMBS + 2];
        for index in 0..self.len {
            let right_limb = right.get(index).copied().unwrap_or(0);
            let mut carry = 0_u64;
            let mut position = 0_usize;
            while position < self.len {
                let current = accumulator.get(position).copied().unwrap_or(0);
                let left_limb = left.get(position).copied().unwrap_or(0);
                let total = u128::from(current)
                    + u128::from(left_limb) * u128::from(right_limb)
                    + u128::from(carry);
                if let Some(slot) = accumulator.get_mut(position) {
                    *slot = low_u64(total);
                }
                carry = high_u64(total);
                position += 1;
            }
            let current = accumulator.get(self.len).copied().unwrap_or(0);
            let total = u128::from(current) + u128::from(carry);
            if let Some(slot) = accumulator.get_mut(self.len) {
                *slot = low_u64(total);
            }
            if let Some(slot) = accumulator.get_mut(self.len + 1) {
                *slot = high_u64(total);
            }

            let factor = accumulator
                .first()
                .copied()
                .unwrap_or(0)
                .wrapping_mul(self.n0inv);
            let current = accumulator.first().copied().unwrap_or(0);
            let modulus_limb = self.limbs.first().copied().unwrap_or(0);
            let total = u128::from(current) + u128::from(factor) * u128::from(modulus_limb);
            let mut carry = high_u64(total);
            let mut position = 1_usize;
            while position < self.len {
                let current = accumulator.get(position).copied().unwrap_or(0);
                let modulus_limb = self.limbs.get(position).copied().unwrap_or(0);
                let total = u128::from(current)
                    + u128::from(factor) * u128::from(modulus_limb)
                    + u128::from(carry);
                if let Some(slot) = accumulator.get_mut(position - 1) {
                    *slot = low_u64(total);
                }
                carry = high_u64(total);
                position += 1;
            }
            let current = accumulator.get(self.len).copied().unwrap_or(0);
            let total = u128::from(current) + u128::from(carry);
            if let Some(slot) = accumulator.get_mut(self.len - 1) {
                *slot = low_u64(total);
            }
            let top = accumulator.get(self.len + 1).copied().unwrap_or(0);
            let top = top.wrapping_add(high_u64(total));
            if let Some(slot) = accumulator.get_mut(self.len) {
                *slot = top;
            }
        }

        let mut result = [0_u64; MAX_LIMBS];
        for index in 0..self.len {
            if let Some(slot) = result.get_mut(index) {
                *slot = accumulator.get(index).copied().unwrap_or(0);
            }
        }
        let overflow = accumulator.get(self.len).copied().unwrap_or(0) & 1;
        if overflow != 0 {
            // The product is `2^(64 * len) + result`; subtracting the
            // modulus needs the two's complement of the modulus.
            let complement = Self::complement(self.limbs, self.len);
            result = Self::add_limbs(result, complement, self.len);
        } else if self.compare_or_equal_runtime(&result, &self.limbs) {
            result = self.subtract_runtime(&result, &self.limbs);
        }
        result
    }

    /// Returns whether `left` is greater than or equal to `right`.
    fn compare_or_equal_runtime(&self, left: &[u64; MAX_LIMBS], right: &[u64; MAX_LIMBS]) -> bool {
        let mut index = self.len;
        while index > 0 {
            index -= 1;
            let left_limb = left.get(index).copied().unwrap_or(0);
            let right_limb = right.get(index).copied().unwrap_or(0);
            if left_limb > right_limb {
                return true;
            }
            if left_limb < right_limb {
                return false;
            }
        }
        true
    }

    /// Subtracts `right` from `left`.
    fn subtract_runtime(
        &self,
        left: &[u64; MAX_LIMBS],
        right: &[u64; MAX_LIMBS],
    ) -> [u64; MAX_LIMBS] {
        let mut result = *left;
        let mut borrow = 0_u64;
        let mut index = 0_usize;
        while index < self.len {
            let left_limb = left.get(index).copied().unwrap_or(0);
            let right_limb = right.get(index).copied().unwrap_or(0);
            let (difference, first) = left_limb.overflowing_sub(right_limb);
            let (difference, second) = difference.overflowing_sub(borrow);
            if let Some(slot) = result.get_mut(index) {
                *slot = difference;
            }
            borrow = u64::from(first || second);
            index += 1;
        }
        result
    }

    /// Adds two plain values modulo the modulus.
    fn add(&self, left: &Big, right: &Big) -> Big {
        let sum = left.add(right).unwrap_or_else(Big::zero);
        self.reduce_once(&sum)
    }

    /// Subtracts two plain values modulo the modulus.
    fn subtract(&self, left: &Big, right: &Big) -> Big {
        match left.compare(right) {
            Ordering::Less => {
                let difference = right.subtract(left).unwrap_or_else(Big::zero);
                self.big().subtract(&difference).unwrap_or_else(Big::zero)
            }
            _ => left.subtract(right).unwrap_or_else(Big::zero),
        }
    }

    /// Multiplies two plain values modulo the modulus.
    fn multiply(&self, left: &Big, right: &Big) -> Big {
        let left_limbs = left.limbs;
        let right_limbs = right.limbs;
        let product = self.montgomery_mul(
            &self.montgomery_form(&left_limbs),
            &self.montgomery_form(&right_limbs),
        );
        Big::from_limbs(self.plain_form(&product))
    }

    /// Computes the modular inverse of a plain value.
    fn inverse(&self, value: &Big) -> Option<Big> {
        let modulus = self.big();
        value.inverse(&modulus)
    }

    /// Reduces a plain value modulo the modulus.
    fn reduce(&self, value: &Big) -> Big {
        if value.compare(&self.big()) == Ordering::Less {
            return *value;
        }
        value.remainder(&self.big())
    }

    /// Reduces a value that is at most twice the modulus.
    fn reduce_once(&self, value: &Big) -> Big {
        let modulus = self.big();
        match value.compare(&modulus) {
            Ordering::Less => *value,
            _ => value.subtract(&modulus).unwrap_or_else(Big::zero),
        }
    }

    /// Computes `base^exponent mod modulus` with a fixed four-bit
    /// window.
    ///
    /// The same number of squarings and multiplications is performed for
    /// every exponent and no branch depends on the exponent bits.
    fn exponentiate(&self, base: &Big, exponent: &[u8]) -> Big {
        let mut table = [Big::zero(); 16];
        table[0] = Big::one();
        table[1] = self.reduce(base);
        for index in 2..16 {
            table[index] = self.multiply(&table[index - 1], &table[1]);
        }
        let mut result = Big::one();
        for byte in exponent.iter().rev() {
            let nibbles = [byte >> 4, byte & 0x0f];
            for nibble in nibbles {
                for _ in 0..4 {
                    result = self.multiply(&result, &result);
                }
                let factor = table.get(nibble as usize).copied().unwrap_or_else(Big::one);
                result = self.multiply(&result, &factor);
            }
        }
        result
    }
}

/// The field prime of P-256.
const P256_FIELD: Prime = Prime::new(P256_FIELD_LIMBS, 4);

/// The group order of P-256.
const P256_ORDER: Prime = Prime::new(P256_ORDER_LIMBS, 4);

/// The field prime of P-384.
const P384_FIELD: Prime = Prime::new(P384_FIELD_LIMBS, 6);

/// The group order of P-384.
const P384_ORDER: Prime = Prime::new(P384_ORDER_LIMBS, 6);

/// The NIST prime curves supported by this module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Curve {
    /// The NIST P-256 curve, also known as secp256r1.
    P256,
    /// The NIST P-384 curve, also known as secp384r1.
    P384,
}

impl Curve {
    /// Returns the size of a field element in bytes.
    #[must_use]
    pub const fn field_size(self) -> usize {
        match self {
            Self::P256 => 32,
            Self::P384 => 48,
        }
    }

    /// Returns the size of the group order in bytes.
    #[must_use]
    pub const fn order_size(self) -> usize {
        self.field_size()
    }

    /// Returns the field prime as big-endian bytes.
    #[must_use]
    pub const fn field_prime(self) -> &'static [u8] {
        match self {
            Self::P256 => &P256_FIELD_BYTES,
            Self::P384 => &P384_FIELD_BYTES,
        }
    }

    /// Returns the group order as big-endian bytes.
    #[must_use]
    pub const fn order(self) -> &'static [u8] {
        match self {
            Self::P256 => &P256_ORDER_BYTES,
            Self::P384 => &P384_ORDER_BYTES,
        }
    }

    /// Returns the curve coefficient `b` as big-endian bytes.
    #[must_use]
    pub const fn coefficient_b(self) -> &'static [u8] {
        match self {
            Self::P256 => &P256_B_BYTES,
            Self::P384 => &P384_B_BYTES,
        }
    }

    /// Returns the base point of the curve.
    #[must_use]
    pub fn generator(self) -> Point {
        let field = self.field();
        let (x, y) = match self {
            Self::P256 => (P256_GX_LIMBS, P256_GY_LIMBS),
            Self::P384 => (P384_GX_LIMBS, P384_GY_LIMBS),
        };
        Point {
            curve: self,
            x: field.montgomery_form(&x),
            y: field.montgomery_form(&y),
            infinity: false,
        }
    }

    /// Parses an uncompressed public point.
    ///
    /// The encoding is the `0x04` prefix followed by the big-endian `x`
    /// and `y` coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`EcdsaError::Malformed`] when the length or the prefix
    /// is wrong, and [`EcdsaError::NotOnCurve`] when the coordinates do
    /// not satisfy the curve equation.
    pub fn point_from_uncompressed(self, bytes: &[u8]) -> Result<Point, EcdsaError> {
        let size = self.field_size();
        if bytes.len() != 1 + 2 * size {
            return Err(EcdsaError::Malformed);
        }
        if bytes.first() != Some(&0x04) {
            return Err(EcdsaError::Malformed);
        }
        let x_bytes = bytes.get(1..1 + size).ok_or(EcdsaError::Malformed)?;
        let y_bytes = bytes.get(1 + size..).ok_or(EcdsaError::Malformed)?;
        let x = Big::from_be_bytes(x_bytes).ok_or(EcdsaError::Malformed)?;
        let y = Big::from_be_bytes(y_bytes).ok_or(EcdsaError::Malformed)?;
        let field = self.field();
        if x.compare(&field.big()) != Ordering::Less || y.compare(&field.big()) != Ordering::Less {
            return Err(EcdsaError::NotOnCurve);
        }
        let point = Point {
            curve: self,
            x: field.montgomery_form(&x.limbs),
            y: field.montgomery_form(&y.limbs),
            infinity: false,
        };
        if !point.is_on_curve() {
            return Err(EcdsaError::NotOnCurve);
        }
        Ok(point)
    }

    /// Adds two points of the curve.
    ///
    /// The addition is performed in affine coordinates and uses a
    /// modular inversion.
    #[must_use]
    pub fn point_add(self, left: Point, right: Point) -> Point {
        let field = self.field();
        if left.infinity {
            return right;
        }
        if right.infinity {
            return left;
        }
        if left.x == right.x {
            if left.y == right.y {
                return self.point_double(left);
            }
            return Point::infinity(self);
        }
        let numerator = field.montgomery_sub(&right.y, &left.y);
        let denominator = field.montgomery_sub(&right.x, &left.x);
        let Some(inverse) = field.montgomery_inverse(&denominator) else {
            return Point::infinity(self);
        };
        let lambda = field.montgomery_mul(&numerator, &inverse);
        let x3 = field.montgomery_sub(
            &field.montgomery_mul(&lambda, &lambda),
            &field.montgomery_add(&left.x, &right.x),
        );
        let y3 = field.montgomery_sub(
            &field.montgomery_mul(&lambda, &field.montgomery_sub(&left.x, &x3)),
            &left.y,
        );
        Point {
            curve: self,
            x: x3,
            y: y3,
            infinity: false,
        }
    }

    /// Doubles a point of the curve.
    ///
    /// The doubling is performed in affine coordinates and uses a
    /// modular inversion.
    #[must_use]
    pub fn point_double(self, point: Point) -> Point {
        let field = self.field();
        if point.infinity {
            return Point::infinity(self);
        }
        let zero = [0_u64; MAX_LIMBS];
        if point.y == zero {
            return Point::infinity(self);
        }
        let curve_a = self.coefficient_a();
        let three = field.montgomery_form(&[3, 0, 0, 0, 0, 0]);
        let two = field.montgomery_form(&[2, 0, 0, 0, 0, 0]);
        let square = field.montgomery_mul(&point.x, &point.x);
        let three_square = field.montgomery_mul(&three, &square);
        let numerator = field.montgomery_add(&three_square, &curve_a);
        let denominator = field.montgomery_mul(&two, &point.y);
        let Some(inverse) = field.montgomery_inverse(&denominator) else {
            return Point::infinity(self);
        };
        let lambda = field.montgomery_mul(&numerator, &inverse);
        let x3 = field.montgomery_sub(
            &field.montgomery_mul(&lambda, &lambda),
            &field.montgomery_add(&point.x, &point.x),
        );
        let y3 = field.montgomery_sub(
            &field.montgomery_mul(&lambda, &field.montgomery_sub(&point.x, &x3)),
            &point.y,
        );
        Point {
            curve: self,
            x: x3,
            y: y3,
            infinity: false,
        }
    }

    /// Multiplies a point by a scalar.
    ///
    /// The scalar is a big-endian integer that is reduced modulo the
    /// group order. The multiplication uses a Montgomery ladder that
    /// performs the same point operations for every scalar.
    ///
    /// Returns `None` when the scalar does not fit into the supported
    /// integer size.
    #[must_use]
    pub fn scalar_mul(self, scalar: &[u8], point: Point) -> Option<Point> {
        let value = Big::from_be_bytes(scalar)?;
        Some(self.scalar_mul_reduced(&value, point))
    }

    /// Returns the field arithmetic of the curve.
    fn field(self) -> &'static Prime {
        match self {
            Self::P256 => &P256_FIELD,
            Self::P384 => &P384_FIELD,
        }
    }

    /// Returns the group order arithmetic of the curve.
    fn order_modulus(self) -> &'static Prime {
        match self {
            Self::P256 => &P256_ORDER,
            Self::P384 => &P384_ORDER,
        }
    }

    /// Returns the curve coefficient `a` in the Montgomery
    /// representation.
    fn coefficient_a(self) -> [u64; MAX_LIMBS] {
        let field = self.field();
        let mut limbs = [0_u64; MAX_LIMBS];
        let mut borrow = 0_u64;
        let mut index = 0_usize;
        while index < field.len {
            let modulus_limb = field.limbs.get(index).copied().unwrap_or(0);
            let subtrahend = if index == 0 { 3 } else { 0 };
            let (difference, first) = modulus_limb.overflowing_sub(subtrahend);
            let (difference, second) = difference.overflowing_sub(borrow);
            if let Some(slot) = limbs.get_mut(index) {
                *slot = difference;
            }
            borrow = u64::from(first || second);
            index += 1;
        }
        field.montgomery_form(&limbs)
    }

    /// Multiplies a point by a scalar that is already reduced modulo
    /// the group order.
    fn scalar_mul_reduced(self, scalar: &Big, point: Point) -> Point {
        let order = self.order_modulus();
        let reduced = order.reduce(scalar);
        let bits = self.field_size() * 8;
        let mut r0 = Point::infinity(self);
        let mut r1 = point;
        for index in (0..bits).rev() {
            let sum = self.point_add(r0, r1);
            let double0 = self.point_double(r0);
            let double1 = self.point_double(r1);
            let (next0, next1) = select_pair(reduced.bit(index), (sum, double1), (double0, sum));
            r0 = next0;
            r1 = next1;
        }
        r0
    }
}

impl Prime {
    /// Adds two Montgomery values modulo the modulus.
    fn montgomery_add(
        &self,
        left: &[u64; MAX_LIMBS],
        right: &[u64; MAX_LIMBS],
    ) -> [u64; MAX_LIMBS] {
        let difference = self.subtract_runtime(&self.limbs, right);
        if self.compare_or_equal_runtime(left, &difference) {
            difference
        } else {
            let mut sum = *left;
            let mut carry = 0_u64;
            let mut index = 0_usize;
            while index < self.len {
                let left_limb = left.get(index).copied().unwrap_or(0);
                let right_limb = right.get(index).copied().unwrap_or(0);
                let (total, first) = left_limb.overflowing_add(right_limb);
                let (total, second) = total.overflowing_add(carry);
                if let Some(slot) = sum.get_mut(index) {
                    *slot = total;
                }
                carry = u64::from(first || second);
                index += 1;
            }
            sum
        }
    }

    /// Subtracts two Montgomery values modulo the modulus.
    fn montgomery_sub(
        &self,
        left: &[u64; MAX_LIMBS],
        right: &[u64; MAX_LIMBS],
    ) -> [u64; MAX_LIMBS] {
        if self.compare_or_equal_runtime(left, right) {
            self.subtract_runtime(left, right)
        } else {
            let difference = self.subtract_runtime(right, left);
            self.subtract_runtime(&self.limbs, &difference)
        }
    }

    /// Computes the inverse of a Montgomery value.
    fn montgomery_inverse(&self, value: &[u64; MAX_LIMBS]) -> Option<[u64; MAX_LIMBS]> {
        let modulus = self.big();
        let plain = Big::from_limbs(self.plain_form(value));
        let inverse = plain.inverse(&modulus)?;
        Some(self.montgomery_form(&inverse.limbs))
    }
}

/// Selects between two pairs of points without branching on the bit.
fn select_pair(bit: bool, when_one: (Point, Point), when_zero: (Point, Point)) -> (Point, Point) {
    let mask = if bit { u64::MAX } else { 0 };
    let inverse = !mask;
    (
        select_point(when_one.0, when_zero.0, mask, inverse),
        select_point(when_one.1, when_zero.1, mask, inverse),
    )
}

/// Selects between two points without branching on the mask.
fn select_point(one: Point, zero: Point, mask: u64, inverse: u64) -> Point {
    let mut result = zero;
    for index in 0..MAX_LIMBS {
        let one_limb = one.x.get(index).copied().unwrap_or(0);
        let zero_limb = zero.x.get(index).copied().unwrap_or(0);
        let value = (one_limb & mask) | (zero_limb & inverse);
        if let Some(slot) = result.x.get_mut(index) {
            *slot = value;
        }
        let one_limb = one.y.get(index).copied().unwrap_or(0);
        let zero_limb = zero.y.get(index).copied().unwrap_or(0);
        let value = (one_limb & mask) | (zero_limb & inverse);
        if let Some(slot) = result.y.get_mut(index) {
            *slot = value;
        }
    }
    let infinity = (u64::from(one.infinity) & mask) | (u64::from(zero.infinity) & inverse);
    result.infinity = infinity != 0;
    result
}

/// A point of a NIST prime curve in affine coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    curve: Curve,
    x: [u64; MAX_LIMBS],
    y: [u64; MAX_LIMBS],
    infinity: bool,
}

impl Point {
    /// Creates the point at infinity of the curve.
    fn infinity(curve: Curve) -> Self {
        Self {
            curve,
            x: [0; MAX_LIMBS],
            y: [0; MAX_LIMBS],
            infinity: true,
        }
    }

    /// Returns the curve of the point.
    #[must_use]
    pub const fn curve(&self) -> Curve {
        self.curve
    }

    /// Returns whether the point is the point at infinity.
    #[must_use]
    pub const fn is_infinity(&self) -> bool {
        self.infinity
    }

    /// Writes the affine `x` coordinate as big-endian bytes into `out`.
    ///
    /// Returns `false` when the buffer is too small.
    #[must_use]
    pub fn x_bytes(&self, out: &mut [u8]) -> bool {
        self.coordinate(&self.x, out)
    }

    /// Writes the affine `y` coordinate as big-endian bytes into `out`.
    ///
    /// Returns `false` when the buffer is too small.
    #[must_use]
    pub fn y_bytes(&self, out: &mut [u8]) -> bool {
        self.coordinate(&self.y, out)
    }

    /// Writes the uncompressed encoding of the point into `out`.
    ///
    /// The encoding is the `0x04` prefix followed by the big-endian `x`
    /// and `y` coordinates. Returns `false` when the buffer is too
    /// small or when the point is the point at infinity.
    #[must_use]
    pub fn to_uncompressed(&self, out: &mut [u8]) -> bool {
        let size = self.curve.field_size();
        if out.len() != 1 + 2 * size || self.infinity {
            return false;
        }
        if let Some(prefix) = out.first_mut() {
            *prefix = 0x04;
        }
        let mut buffer = [0_u8; MAX_BYTES];
        if !self.coordinate(&self.x, &mut buffer) {
            return false;
        }
        let start = MAX_BYTES - size;
        if let (Some(slot), Some(source)) = (out.get_mut(1..1 + size), buffer.get(start..)) {
            slot.copy_from_slice(source);
        }
        if !self.coordinate(&self.y, &mut buffer) {
            return false;
        }
        if let (Some(slot), Some(source)) = (out.get_mut(1 + size..), buffer.get(start..)) {
            slot.copy_from_slice(source);
        }
        true
    }

    /// Returns whether the point satisfies the curve equation.
    #[must_use]
    pub fn is_on_curve(&self) -> bool {
        if self.infinity {
            return true;
        }
        let field = self.curve.field();
        let plain_x = Big::from_limbs(field.plain_form(&self.x));
        let plain_y = Big::from_limbs(field.plain_form(&self.y));
        if plain_x.compare(&field.big()) != Ordering::Less
            || plain_y.compare(&field.big()) != Ordering::Less
        {
            return false;
        }
        let square = field.multiply(&plain_x, &plain_x);
        let cube = field.multiply(&square, &plain_x);
        let ax = field.multiply(
            &Big::from_limbs(field.plain_form(&self.coefficient_a_value())),
            &plain_x,
        );
        let right = field.add(
            &field.add(&cube, &ax),
            &Big::from_limbs(field.plain_form(&self.coefficient_b_value())),
        );
        let left = field.multiply(&plain_y, &plain_y);
        left.compare(&right) == Ordering::Equal
    }

    /// Writes a coordinate as big-endian bytes into `out`.
    fn coordinate(&self, value: &[u64; MAX_LIMBS], out: &mut [u8]) -> bool {
        let field = self.curve.field();
        let plain = Big::from_limbs(field.plain_form(value));
        plain.write_be(out)
    }

    /// Returns the curve coefficient `a` in the Montgomery
    /// representation.
    fn coefficient_a_value(&self) -> [u64; MAX_LIMBS] {
        self.curve.coefficient_a()
    }

    /// Returns the curve coefficient `b` in the Montgomery
    /// representation.
    fn coefficient_b_value(&self) -> [u64; MAX_LIMBS] {
        let field = self.curve.field();
        let limbs = match self.curve {
            Curve::P256 => P256_B_LIMBS,
            Curve::P384 => P384_B_LIMBS,
        };
        field.montgomery_form(&limbs)
    }
}

/// A modulus for the modular arithmetic helpers of this module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Modulus {
    /// The field prime of the curve.
    Field(Curve),
    /// The group order of the curve.
    Order(Curve),
}

impl Modulus {
    /// Returns the size of the modulus in bytes.
    #[must_use]
    pub const fn size(&self) -> usize {
        match self {
            Self::Field(curve) | Self::Order(curve) => curve.field_size(),
        }
    }

    /// Adds two values modulo the modulus.
    ///
    /// Returns `false` when an input does not fit into the supported
    /// integer size or when the output buffer is too small.
    #[must_use]
    pub fn add(&self, left: &[u8], right: &[u8], out: &mut [u8]) -> bool {
        self.compute(out, |prime| {
            let left = prime.reduce(&Big::from_be_bytes(left).unwrap_or_else(Big::zero));
            let right = prime.reduce(&Big::from_be_bytes(right).unwrap_or_else(Big::zero));
            prime.add(&left, &right)
        })
    }

    /// Subtracts two values modulo the modulus.
    ///
    /// Returns `false` when an input does not fit into the supported
    /// integer size or when the output buffer is too small.
    #[must_use]
    pub fn sub(&self, left: &[u8], right: &[u8], out: &mut [u8]) -> bool {
        self.compute(out, |prime| {
            let left = prime.reduce(&Big::from_be_bytes(left).unwrap_or_else(Big::zero));
            let right = prime.reduce(&Big::from_be_bytes(right).unwrap_or_else(Big::zero));
            prime.subtract(&left, &right)
        })
    }

    /// Multiplies two values modulo the modulus.
    ///
    /// Returns `false` when an input does not fit into the supported
    /// integer size or when the output buffer is too small.
    #[must_use]
    pub fn mul(&self, left: &[u8], right: &[u8], out: &mut [u8]) -> bool {
        self.compute(out, |prime| {
            let left = prime.reduce(&Big::from_be_bytes(left).unwrap_or_else(Big::zero));
            let right = prime.reduce(&Big::from_be_bytes(right).unwrap_or_else(Big::zero));
            prime.multiply(&left, &right)
        })
    }

    /// Computes the modular inverse of a value.
    ///
    /// Returns `false` when the value has no inverse or when the output
    /// buffer is too small.
    #[must_use]
    pub fn inverse(&self, value: &[u8], out: &mut [u8]) -> bool {
        let Some(parsed) = Big::from_be_bytes(value) else {
            return false;
        };
        let prime = self.prime();
        let reduced = prime.reduce(&parsed);
        let Some(inverse) = prime.inverse(&reduced) else {
            return false;
        };
        self.write(out, &inverse)
    }

    /// Reduces a value modulo the modulus.
    ///
    /// Returns `false` when the input does not fit into the supported
    /// integer size or when the output buffer is too small.
    #[must_use]
    pub fn reduce(&self, value: &[u8], out: &mut [u8]) -> bool {
        let Some(parsed) = Big::from_be_bytes(value) else {
            return false;
        };
        let prime = self.prime();
        let reduced = prime.reduce(&parsed);
        self.write(out, &reduced)
    }

    /// Computes `base^exponent mod modulus` with a fixed four-bit
    /// window.
    ///
    /// The same number of squarings and multiplications is performed for
    /// every exponent and no branch depends on the exponent bits.
    ///
    /// Returns `false` when an input does not fit into the supported
    /// integer size or when the output buffer is too small.
    #[must_use]
    pub fn exp(&self, base: &[u8], exponent: &[u8], out: &mut [u8]) -> bool {
        let Some(parsed) = Big::from_be_bytes(base) else {
            return false;
        };
        let prime = self.prime();
        let reduced = prime.reduce(&parsed);
        let result = prime.exponentiate(&reduced, exponent);
        self.write(out, &result)
    }

    /// Returns the arithmetic of the modulus.
    fn prime(self) -> &'static Prime {
        match self {
            Self::Field(curve) => curve.field(),
            Self::Order(curve) => curve.order_modulus(),
        }
    }

    /// Runs an operation and writes the result into `out`.
    fn compute(self, out: &mut [u8], operation: impl FnOnce(&Prime) -> Big) -> bool {
        let result = operation(self.prime());
        self.write(out, &result)
    }

    /// Writes a value into `out` using the size of the modulus.
    fn write(self, out: &mut [u8], value: &Big) -> bool {
        let size = self.size();
        if out.len() < size {
            return false;
        }
        let mut buffer = [0_u8; MAX_BYTES];
        if !value.write_be(&mut buffer) {
            return false;
        }
        let start = MAX_BYTES - size;
        if let (Some(slot), Some(source)) = (out.get_mut(..size), buffer.get(start..)) {
            slot.copy_from_slice(source);
        }
        true
    }
}

/// An ECDSA signature.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Signature {
    r: Big,
    s: Big,
}

impl Signature {
    /// Builds a signature from raw big-endian values.
    ///
    /// The values are not checked against the group order; the range is
    /// validated during verification.
    ///
    /// # Errors
    ///
    /// Returns [`EcdsaError::Malformed`] when a value does not fit into
    /// the supported integer size.
    pub fn from_raw(r: &[u8], s: &[u8]) -> Result<Self, EcdsaError> {
        let r = Big::from_be_bytes(r).ok_or(EcdsaError::Malformed)?;
        let s = Big::from_be_bytes(s).ok_or(EcdsaError::Malformed)?;
        Ok(Self { r, s })
    }

    /// Parses a DER encoded `ECDSA-Sig-Value` structure.
    ///
    /// # Errors
    ///
    /// Returns [`EcdsaError::Malformed`] when the structure is
    /// truncated, when a component is negative, or when a component does
    /// not fit into the supported integer size.
    pub fn from_der(der: &[u8]) -> Result<Self, EcdsaError> {
        let outer = parse_tlv(der).map_err(|_| EcdsaError::Malformed)?;
        if outer.tag() != 0x30 {
            return Err(EcdsaError::Malformed);
        }
        let content = outer.value();
        let first = parse_tlv(content).map_err(|_| EcdsaError::Malformed)?;
        if first.tag() != 0x02 {
            return Err(EcdsaError::Malformed);
        }
        let r = der_integer(first.value())?;
        let consumed = first.header_len() + first.value().len();
        let rest = content.get(consumed..).ok_or(EcdsaError::Malformed)?;
        let second = parse_tlv(rest).map_err(|_| EcdsaError::Malformed)?;
        if second.tag() != 0x02 {
            return Err(EcdsaError::Malformed);
        }
        let s = der_integer(second.value())?;
        Ok(Self { r, s })
    }

    /// Writes the `r` component as big-endian bytes into `out`.
    ///
    /// Returns `false` when the buffer is too small.
    #[must_use]
    pub fn r_bytes(&self, out: &mut [u8]) -> bool {
        self.r.write_be(out)
    }

    /// Writes the `s` component as big-endian bytes into `out`.
    ///
    /// Returns `false` when the buffer is too small.
    #[must_use]
    pub fn s_bytes(&self, out: &mut [u8]) -> bool {
        self.s.write_be(out)
    }
}

/// Decodes a positive DER integer content.
fn der_integer(content: &[u8]) -> Result<Big, EcdsaError> {
    if content.is_empty() {
        return Err(EcdsaError::Malformed);
    }
    if content.first().copied().unwrap_or(0) & 0x80 != 0 {
        return Err(EcdsaError::Malformed);
    }
    let mut start = 0_usize;
    while start < content.len() && content.get(start).copied().unwrap_or(0) == 0 {
        start += 1;
    }
    let value = content.get(start..).ok_or(EcdsaError::Malformed)?;
    Big::from_be_bytes(value).ok_or(EcdsaError::Malformed)
}

/// An ECDSA public key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicKey {
    curve: Curve,
    point: Point,
}

impl PublicKey {
    /// Parses an uncompressed public point.
    ///
    /// # Errors
    ///
    /// Returns [`EcdsaError::Malformed`] when the length or the prefix
    /// is wrong, and [`EcdsaError::NotOnCurve`] when the coordinates do
    /// not satisfy the curve equation.
    pub fn from_uncompressed(curve: Curve, bytes: &[u8]) -> Result<Self, EcdsaError> {
        let point = curve.point_from_uncompressed(bytes)?;
        Ok(Self { curve, point })
    }

    /// Parses an X.509 `SubjectPublicKeyInfo` structure.
    ///
    /// Only the `id-ecPublicKey` algorithm with the `prime256v1` and
    /// `secp384r1` named curves is supported, and the public key must
    /// use the uncompressed point format.
    ///
    /// # Errors
    ///
    /// Returns [`EcdsaError::Malformed`] when the structure is
    /// truncated or uses an unsupported algorithm, and
    /// [`EcdsaError::NotOnCurve`] when the public point is not on the
    /// curve.
    pub fn from_spki_der(der: &[u8]) -> Result<Self, EcdsaError> {
        let outer = parse_tlv(der).map_err(|_| EcdsaError::Malformed)?;
        if outer.tag() != 0x30 {
            return Err(EcdsaError::Malformed);
        }
        let content = outer.value();
        let algorithm = parse_tlv(content).map_err(|_| EcdsaError::Malformed)?;
        if algorithm.tag() != 0x30 {
            return Err(EcdsaError::Malformed);
        }
        let algorithm_content = algorithm.value();
        let oid_tlv = parse_tlv(algorithm_content).map_err(|_| EcdsaError::Malformed)?;
        if oid_tlv.tag() != 0x06 {
            return Err(EcdsaError::Malformed);
        }
        let oid = decode_oid(oid_tlv.value()).map_err(|_| EcdsaError::Malformed)?;
        if !oid_eq(&oid, &OID_EC_PUBLIC_KEY) {
            return Err(EcdsaError::Malformed);
        }
        let consumed = oid_tlv.header_len() + oid_tlv.value().len();
        let rest = algorithm_content
            .get(consumed..)
            .ok_or(EcdsaError::Malformed)?;
        let curve_tlv = parse_tlv(rest).map_err(|_| EcdsaError::Malformed)?;
        if curve_tlv.tag() != 0x06 {
            return Err(EcdsaError::Malformed);
        }
        let curve_oid = decode_oid(curve_tlv.value()).map_err(|_| EcdsaError::Malformed)?;
        let curve = if oid_eq(&curve_oid, &OID_PRIME256V1) {
            Curve::P256
        } else if oid_eq(&curve_oid, &OID_SECP384R1) {
            Curve::P384
        } else {
            return Err(EcdsaError::Malformed);
        };
        let consumed = curve_tlv.header_len() + curve_tlv.value().len();
        let algorithm_length = algorithm.value().len();
        if consumed != algorithm_length {
            return Err(EcdsaError::Malformed);
        }
        let rest = content
            .get(algorithm.header_len() + algorithm_length..)
            .ok_or(EcdsaError::Malformed)?;
        let bits_tlv = parse_tlv(rest).map_err(|_| EcdsaError::Malformed)?;
        if bits_tlv.tag() != 0x03 {
            return Err(EcdsaError::Malformed);
        }
        let bits = bits_tlv.value();
        if bits.first() != Some(&0) {
            return Err(EcdsaError::Malformed);
        }
        let point = bits.get(1..).ok_or(EcdsaError::Malformed)?;
        Self::from_uncompressed(curve, point)
    }

    /// Returns the curve of the key.
    #[must_use]
    pub const fn curve(&self) -> Curve {
        self.curve
    }

    /// Returns the public point of the key.
    #[must_use]
    pub const fn point(&self) -> &Point {
        &self.point
    }

    /// Verifies an ECDSA signature over a message digest.
    ///
    /// The digest is interpreted as a big-endian integer; only the
    /// leftmost bytes that fit into the group order are used.
    ///
    /// # Errors
    ///
    /// Returns [`EcdsaError::InvalidRange`] when a signature component
    /// is zero or not smaller than the group order, and
    /// [`EcdsaError::InvalidSignature`] when the signature does not
    /// verify.
    pub fn verify(&self, digest: &[u8], signature: &Signature) -> Result<(), EcdsaError> {
        let order = self.curve.order_modulus();
        let order_value = order.big();
        if signature.r.is_zero() || signature.s.is_zero() {
            return Err(EcdsaError::InvalidRange);
        }
        if signature.r.compare(&order_value) != Ordering::Less
            || signature.s.compare(&order_value) != Ordering::Less
        {
            return Err(EcdsaError::InvalidRange);
        }
        let size = self.curve.order_size();
        let take = digest.len().min(size);
        let e = Big::from_be_bytes(digest.get(..take).unwrap_or(&[])).unwrap_or_else(Big::zero);
        let w = order
            .inverse(&signature.s)
            .ok_or(EcdsaError::InvalidRange)?;
        let u1 = order.multiply(&e, &w);
        let u2 = order.multiply(&signature.r, &w);
        let first = self.curve.scalar_mul_reduced(&u1, self.curve.generator());
        let second = self.curve.scalar_mul_reduced(&u2, self.point);
        let sum = self.curve.point_add(first, second);
        if sum.is_infinity() {
            return Err(EcdsaError::InvalidSignature);
        }
        let x = Big::from_limbs(self.curve.field().plain_form(&sum.x));
        let reduced = order.reduce(&x);
        if reduced.compare(&signature.r) != Ordering::Equal {
            return Err(EcdsaError::InvalidSignature);
        }
        Ok(())
    }
}

/// An ECDSA verification failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EcdsaError {
    /// A signature component was zero or not smaller than the group
    /// order.
    InvalidRange,
    /// The public key point is not on the curve.
    NotOnCurve,
    /// The signature does not verify.
    InvalidSignature,
    /// The encoding is truncated, malformed, or unsupported.
    Malformed,
}

impl core::fmt::Display for EcdsaError {
    /// Formats the error without allocation.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            Self::InvalidRange => "signature component out of range",
            Self::NotOnCurve => "public key point is not on the curve",
            Self::InvalidSignature => "signature verification failed",
            Self::Malformed => "malformed ECDSA encoding",
        };
        formatter.write_str(text)
    }
}
