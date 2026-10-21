//! # HKDF key derivation
//!
//! Native HKDF extract-and-expand derivation built on the native HMAC
//! implementation. This follows the standardised HMAC-based
//! extract-and-expand construction and performs no allocation.
//!
//! Output length is limited to `255 * HashLen` bytes. A return value
//! of `false` indicates an output buffer that is too long or too
//! short; no partial output should be trusted in that case.

use super::hmac::{HashFunction, Hmac, MAX_OUTPUT};

/// Extracts a pseudorandom key from input keying material.
///
/// `prk` must hold at least `H::OUTPUT_SIZE` bytes. Returns `false`
/// when the buffer is too short.
pub fn extract<H: HashFunction>(salt: &[u8], input_key_material: &[u8], prk: &mut [u8]) -> bool {
    let mut state = Hmac::<H>::new(salt);
    state.update(input_key_material);
    state.finalize_into(prk)
}

/// Expands a pseudorandom key into output keying material.
///
/// Returns `false` when `output` exceeds `255 * H::OUTPUT_SIZE` bytes
/// or when the underlying HMAC fails.
pub fn expand<H: HashFunction>(prk: &[u8], info: &[u8], output: &mut [u8]) -> bool {
    let max_length = 255_usize.saturating_mul(H::OUTPUT_SIZE);
    if output.len() > max_length {
        return false;
    }
    let mut previous = [0_u8; MAX_OUTPUT];
    let mut previous_len = 0_usize;
    let mut offset = 0_usize;
    let mut counter = 1_u8;
    while offset < output.len() {
        let mut state = Hmac::<H>::new(prk);
        state.update(&previous[..previous_len]);
        state.update(info);
        state.update(&[counter]);
        let mut block = [0_u8; MAX_OUTPUT];
        if !state.finalize_into(&mut block[..H::OUTPUT_SIZE]) {
            return false;
        }
        let take = (output.len() - offset).min(H::OUTPUT_SIZE);
        match output.get_mut(offset..offset + take) {
            Some(slot) => slot.copy_from_slice(&block[..take]),
            None => return false,
        }
        previous[..H::OUTPUT_SIZE].copy_from_slice(&block[..H::OUTPUT_SIZE]);
        previous_len = H::OUTPUT_SIZE;
        offset += take;
        counter = counter.wrapping_add(1);
    }
    true
}

/// Performs HKDF-SHA-256 extract and expand in one call.
///
/// Returns `false` when `output` is longer than `255 * 32` bytes.
pub fn hkdf_sha256(salt: &[u8], input_key_material: &[u8], info: &[u8], output: &mut [u8]) -> bool {
    use super::sha256::Sha256;
    let mut prk = [0_u8; 32];
    if !extract::<Sha256>(salt, input_key_material, &mut prk) {
        return false;
    }
    expand::<Sha256>(&prk, info, output)
}

/// Performs HKDF-SHA-512 extract and expand in one call.
///
/// Returns `false` when `output` is longer than `255 * 64` bytes.
pub fn hkdf_sha512(salt: &[u8], input_key_material: &[u8], info: &[u8], output: &mut [u8]) -> bool {
    use super::sha512::Sha512;
    let mut prk = [0_u8; 64];
    if !extract::<Sha512>(salt, input_key_material, &mut prk) {
        return false;
    }
    expand::<Sha512>(&prk, info, output)
}
