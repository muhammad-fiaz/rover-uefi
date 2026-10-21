//! # Hash-based message authentication
//!
//! Native HMAC construction over the SHA-1 and SHA-2 digests.
//!
//! HMAC is implemented from the digest primitives; it does not call
//! firmware authentication services. Keys longer than the digest
//! block size are hashed first, as required by the construction.
//! No allocation is performed.

use super::sha1::Sha1;
use super::sha224::Sha224;
use super::sha256::Sha256;
use super::sha512::{Sha384, Sha512};

/// The largest block size of the supported digests.
const MAX_BLOCK: usize = 128;

/// The largest digest size of the supported digests.
pub(crate) const MAX_OUTPUT: usize = 64;

/// A streaming digest that can back HMAC.
///
/// The trait exposes only the primitives needed by HMAC so that the
/// construction is shared by every supported digest.
pub trait HashFunction {
    /// The digest block size in bytes.
    const BLOCK_SIZE: usize;
    /// The digest output size in bytes.
    const OUTPUT_SIZE: usize;

    /// Creates a digest in the initial state.
    fn new() -> Self;

    /// Absorbs `data`.
    fn update(&mut self, data: &[u8]);

    /// Finalizes into `output`, returning `false` when the buffer is
    /// shorter than [`Self::OUTPUT_SIZE`].
    fn finalize_into(self, output: &mut [u8]) -> bool;
}

macro_rules! impl_hash_function {
    ($type:ty, $block:expr, $output:expr) => {
        impl HashFunction for $type {
            const BLOCK_SIZE: usize = $block;
            const OUTPUT_SIZE: usize = $output;

            fn new() -> Self {
                <$type>::new()
            }

            fn update(&mut self, data: &[u8]) {
                <$type>::update(self, data);
            }

            fn finalize_into(self, output: &mut [u8]) -> bool {
                let digest = self.finalize();
                if output.len() < $output {
                    return false;
                }
                match output.get_mut(..$output) {
                    Some(slot) => {
                        slot.copy_from_slice(&digest);
                        true
                    }
                    None => false,
                }
            }
        }
    };
}

impl_hash_function!(Sha1, 64, 20);
impl_hash_function!(Sha224, 64, 28);
impl_hash_function!(Sha256, 64, 32);
impl_hash_function!(Sha384, 128, 48);
impl_hash_function!(Sha512, 128, 64);

/// A streaming HMAC state over a [`HashFunction`].
#[derive(Clone, Debug)]
pub struct Hmac<H: HashFunction> {
    inner: H,
    outer_key: [u8; MAX_BLOCK],
}

impl<H: HashFunction> Hmac<H> {
    /// Creates a keyed HMAC state.
    #[must_use]
    pub fn new(key: &[u8]) -> Self {
        let mut normalized = [0_u8; MAX_BLOCK];
        if key.len() > H::BLOCK_SIZE {
            let mut digest = [0_u8; MAX_OUTPUT];
            let mut hasher = H::new();
            hasher.update(key);
            let _ = hasher.finalize_into(&mut digest[..H::OUTPUT_SIZE]);
            normalized[..H::OUTPUT_SIZE].copy_from_slice(&digest[..H::OUTPUT_SIZE]);
        } else {
            normalized[..key.len()].copy_from_slice(key);
        }

        let mut inner_pad = [0_u8; MAX_BLOCK];
        let mut outer_key = [0_u8; MAX_BLOCK];
        for ((inner_slot, outer_slot), source) in inner_pad
            .iter_mut()
            .zip(outer_key.iter_mut())
            .zip(normalized.iter())
            .take(H::BLOCK_SIZE)
        {
            *inner_slot = source ^ 0x36;
            *outer_slot = source ^ 0x5c;
        }

        let mut inner = H::new();
        inner.update(&inner_pad[..H::BLOCK_SIZE]);
        Self { inner, outer_key }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalizes into `output`, returning `false` when the buffer is
    /// shorter than the hash output size (`H::OUTPUT_SIZE`).
    #[must_use]
    pub fn finalize_into(self, output: &mut [u8]) -> bool {
        let mut inner_digest = [0_u8; MAX_OUTPUT];
        if !self
            .inner
            .finalize_into(&mut inner_digest[..H::OUTPUT_SIZE])
        {
            return false;
        }
        let mut outer = H::new();
        outer.update(&self.outer_key[..H::BLOCK_SIZE]);
        outer.update(&inner_digest[..H::OUTPUT_SIZE]);
        let mut digest = [0_u8; MAX_OUTPUT];
        if !outer.finalize_into(&mut digest[..H::OUTPUT_SIZE]) {
            return false;
        }
        if output.len() < H::OUTPUT_SIZE {
            return false;
        }
        match output.get_mut(..H::OUTPUT_SIZE) {
            Some(slot) => {
                slot.copy_from_slice(&digest[..H::OUTPUT_SIZE]);
                true
            }
            None => false,
        }
    }
}

/// HMAC-SHA-1 state.
pub type HmacSha1 = Hmac<Sha1>;
/// HMAC-SHA-224 state.
pub type HmacSha224 = Hmac<Sha224>;
/// HMAC-SHA-256 state.
pub type HmacSha256 = Hmac<Sha256>;
/// HMAC-SHA-384 state.
pub type HmacSha384 = Hmac<Sha384>;
/// HMAC-SHA-512 state.
pub type HmacSha512 = Hmac<Sha512>;

/// Computes HMAC over `data` with `key` into `output`.
///
/// Returns `false` when `output` cannot hold the digest.
pub fn hmac<H: HashFunction>(key: &[u8], data: &[u8], output: &mut [u8]) -> bool {
    let mut state = Hmac::<H>::new(key);
    state.update(data);
    state.finalize_into(output)
}

/// Computes HMAC-SHA-1.
#[must_use]
pub fn hmac_sha1(key: &[u8], data: &[u8]) -> [u8; 20] {
    let mut output = [0_u8; 20];
    let _ = hmac::<Sha1>(key, data, &mut output);
    output
}

/// Computes HMAC-SHA-224.
#[must_use]
pub fn hmac_sha224(key: &[u8], data: &[u8]) -> [u8; 28] {
    let mut output = [0_u8; 28];
    let _ = hmac::<Sha224>(key, data, &mut output);
    output
}

/// Computes HMAC-SHA-256.
#[must_use]
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut output = [0_u8; 32];
    let _ = hmac::<Sha256>(key, data, &mut output);
    output
}

/// Computes HMAC-SHA-384.
#[must_use]
pub fn hmac_sha384(key: &[u8], data: &[u8]) -> [u8; 48] {
    let mut output = [0_u8; 48];
    let _ = hmac::<Sha384>(key, data, &mut output);
    output
}

/// Computes HMAC-SHA-512.
#[must_use]
pub fn hmac_sha512(key: &[u8], data: &[u8]) -> [u8; 64] {
    let mut output = [0_u8; 64];
    let _ = hmac::<Sha512>(key, data, &mut output);
    output
}
