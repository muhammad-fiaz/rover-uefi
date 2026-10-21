//! # AES-GCM authenticated encryption
//!
//! Native AES-GCM authenticated encryption and decryption with a
//! 128-bit authentication tag. Supports the three AES key sizes and
//! arbitrary nonce lengths; the standard 96-bit nonce is handled
//! without an extra key derivation step.
//!
//! The caller owns all buffers. Encryption fails when the output
//! buffer length differs from the plaintext length, or when the key
//! length is not 16, 24 or 32 bytes. Decryption authenticates before
//! writing plaintext and never returns unauthenticated data.

use super::aes::Aes;

/// The authentication tag size in bytes.
pub const TAG_SIZE: usize = 16;

/// Encrypts and authenticates `plaintext`.
///
/// `ciphertext` must have the same length as `plaintext`. Returns
/// `false` for a malformed key or mismatched buffer lengths.
pub fn seal(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    plaintext: &[u8],
    ciphertext: &mut [u8],
    tag: &mut [u8; TAG_SIZE],
) -> bool {
    if ciphertext.len() != plaintext.len() {
        return false;
    }
    ciphertext.copy_from_slice(plaintext);
    seal_in_place(key, nonce, aad, ciphertext, tag)
}

/// Encrypts and authenticates `data` in place.
///
/// On success `data` holds the ciphertext and `tag` the authentication
/// tag. A caller that already owns the plaintext in a buffer saves a
/// copy by using this function.
pub fn seal_in_place(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    data: &mut [u8],
    tag: &mut [u8; TAG_SIZE],
) -> bool {
    let Some(aes) = Aes::new(key) else {
        return false;
    };
    if nonce.is_empty() {
        return false;
    }
    let h = hash_key(&aes);
    let j0 = derive_j0(h, nonce);
    let mut counter = j0.to_be_bytes();
    for chunk in data.chunks_mut(16) {
        increment_counter(&mut counter);
        let mut stream = [0_u8; 16];
        aes.encrypt_block(&counter, &mut stream);
        for (index, byte) in chunk.iter_mut().enumerate() {
            *byte ^= stream[index];
        }
    }
    let authentication = ghash(h, aad, data);
    let mut mask = [0_u8; 16];
    aes.encrypt_block(&j0.to_be_bytes(), &mut mask);
    let value = u128::from_be_bytes(mask) ^ authentication;
    *tag = value.to_be_bytes();
    true
}

/// Authenticates and decrypts `ciphertext`.
///
/// Returns `false` when the key is malformed, buffer lengths differ,
/// or the tag does not match. `plaintext` is only written after the
/// tag has been verified.
pub fn open(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    ciphertext: &[u8],
    tag: &[u8; TAG_SIZE],
    plaintext: &mut [u8],
) -> bool {
    let Some(aes) = Aes::new(key) else {
        return false;
    };
    if nonce.is_empty() || plaintext.len() != ciphertext.len() {
        return false;
    }
    let h = hash_key(&aes);
    let j0 = derive_j0(h, nonce);
    if !verify_tag(&aes, h, j0, aad, ciphertext, tag) {
        return false;
    }
    let mut counter = j0.to_be_bytes();
    let mut offset = 0_usize;
    for chunk in ciphertext.chunks(16) {
        increment_counter(&mut counter);
        let mut stream = [0_u8; 16];
        aes.encrypt_block(&counter, &mut stream);
        for (index, byte) in chunk.iter().enumerate() {
            match plaintext.get_mut(offset.wrapping_add(index)) {
                Some(slot) => *slot = *byte ^ stream[index],
                None => return false,
            }
        }
        offset = offset.wrapping_add(chunk.len());
    }
    true
}

/// Verifies the authentication tag of `ciphertext`.
fn verify_tag(
    aes: &Aes,
    h: u128,
    j0: u128,
    aad: &[u8],
    ciphertext: &[u8],
    tag: &[u8; TAG_SIZE],
) -> bool {
    let authentication = ghash(h, aad, ciphertext);
    let mut mask = [0_u8; 16];
    aes.encrypt_block(&j0.to_be_bytes(), &mut mask);
    let expected = (u128::from_be_bytes(mask) ^ authentication).to_be_bytes();
    constant_time_eq(&expected, tag)
}

/// Authenticates and decrypts `data` in place.
///
/// The tag is verified before any byte is decrypted, so a failed call
/// leaves `data` untouched.
pub fn open_in_place(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    data: &mut [u8],
    tag: &[u8; TAG_SIZE],
) -> bool {
    let Some(aes) = Aes::new(key) else {
        return false;
    };
    if nonce.is_empty() {
        return false;
    }
    let h = hash_key(&aes);
    let j0 = derive_j0(h, nonce);
    if !verify_tag(&aes, h, j0, aad, data, tag) {
        return false;
    }
    let mut counter = j0.to_be_bytes();
    for chunk in data.chunks_mut(16) {
        increment_counter(&mut counter);
        let mut stream = [0_u8; 16];
        aes.encrypt_block(&counter, &mut stream);
        for (index, byte) in chunk.iter_mut().enumerate() {
            *byte ^= stream[index];
        }
    }
    true
}

/// Computes the GHASH key `E_K(0^128)`.
fn hash_key(aes: &Aes) -> u128 {
    let mut output = [0_u8; 16];
    aes.encrypt_block(&[0_u8; 16], &mut output);
    u128::from_be_bytes(output)
}

/// Derives the initial counter block.
fn derive_j0(h: u128, nonce: &[u8]) -> u128 {
    if nonce.len() == 12 {
        let mut block = [0_u8; 16];
        block[..12].copy_from_slice(nonce);
        block[12..].copy_from_slice(&1_u32.to_be_bytes());
        return u128::from_be_bytes(block);
    }
    let mut value = 0_u128;
    for chunk in nonce.chunks(16) {
        let mut block = [0_u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);
        value = multiply(value ^ u128::from_be_bytes(block), h);
    }
    let mut lengths = [0_u8; 16];
    let bits = u64::try_from(nonce.len())
        .unwrap_or(u64::MAX)
        .saturating_mul(8);
    lengths[8..].copy_from_slice(&bits.to_be_bytes());
    multiply(value ^ u128::from_be_bytes(lengths), h)
}

/// Computes GHASH over associated data and ciphertext.
fn ghash(h: u128, aad: &[u8], ciphertext: &[u8]) -> u128 {
    let mut value = 0_u128;
    for chunk in aad.chunks(16) {
        let mut block = [0_u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);
        value = multiply(value ^ u128::from_be_bytes(block), h);
    }
    for chunk in ciphertext.chunks(16) {
        let mut block = [0_u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);
        value = multiply(value ^ u128::from_be_bytes(block), h);
    }
    let mut lengths = [0_u8; 16];
    let aad_bits = u64::try_from(aad.len())
        .unwrap_or(u64::MAX)
        .saturating_mul(8);
    let text_bits = u64::try_from(ciphertext.len())
        .unwrap_or(u64::MAX)
        .saturating_mul(8);
    lengths[..8].copy_from_slice(&aad_bits.to_be_bytes());
    lengths[8..].copy_from_slice(&text_bits.to_be_bytes());
    multiply(value ^ u128::from_be_bytes(lengths), h)
}

/// Multiplies two field elements in the GCM representation.
fn multiply(left: u128, right: u128) -> u128 {
    const REDUCTION: u128 = 0xE100_0000_0000_0000_0000_0000_0000_0000;
    let mut product = 0_u128;
    let mut value = right;
    for bit in 0..128 {
        if (left >> (127 - bit)) & 1 != 0 {
            product ^= value;
        }
        let low = value & 1;
        value >>= 1;
        if low != 0 {
            value ^= REDUCTION;
        }
    }
    product
}

/// Increments the low 32 bits of a counter block with wrapping.
fn increment_counter(block: &mut [u8; 16]) {
    for byte in block[12..].iter_mut().rev() {
        let (value, overflow) = byte.overflowing_add(1);
        *byte = value;
        if !overflow {
            return;
        }
    }
}

/// Compares two 16-byte tags without early exit.
fn constant_time_eq(left: &[u8; 16], right: &[u8; 16]) -> bool {
    let mut difference = 0_u8;
    for (a, b) in left.iter().zip(right.iter()) {
        difference |= a ^ b;
    }
    difference == 0
}
