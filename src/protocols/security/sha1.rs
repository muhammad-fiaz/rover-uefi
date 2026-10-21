//! # SHA-1 native digest
//!
//! Native SHA-1 implementation for compatibility with legacy firmware,
//! signature databases, and measured-boot event logs.
//!
//! SHA-1 is not collision resistant and must not be used for new
//! signature generation. It is provided because several firmware
//! formats still reference SHA-1 digests.
//!
//! The implementation operates on caller-owned buffers and performs
//! no allocation. `finalize` consumes the hasher, so it cannot be
//! called twice.

/// The SHA-1 initial hash values.
const H0: [u32; 5] = [
    0x6745_2301,
    0xEFCD_AB89,
    0x98BA_DCFE,
    0x1032_5476,
    0xC3D2_E1F0,
];

/// The SHA-1 round constants.
const K: [u32; 4] = [0x5A82_7999, 0x6ED9_EBA1, 0x8F1B_BCDC, 0xCA62_C1D6];

/// A streaming SHA-1 hasher (no allocation, caller-owned state).
#[derive(Clone, Debug)]
pub struct Sha1 {
    state: [u32; 5],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

impl Sha1 {
    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: H0,
            buffer: [0; 64],
            buffer_len: 0,
            total_len: 0,
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        if self.buffer_len > 0 {
            let take = (64 - self.buffer_len).min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&data[..take]);
            self.buffer_len += take;
            data = &data[take..];
            if self.buffer_len == 64 {
                let block = self.buffer;
                self.compress(&block);
                self.buffer_len = 0;
            }
        }
        while data.len() >= 64 {
            let mut block = [0_u8; 64];
            block.copy_from_slice(&data[..64]);
            self.compress(&block);
            data = &data[64..];
        }
        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
            self.buffer_len = data.len();
        }
    }

    /// Finalizes and returns the 20-byte digest. Consumes the hasher.
    #[must_use]
    pub fn finalize(mut self) -> [u8; 20] {
        let bit_len = self.total_len.wrapping_mul(8);
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;
        if self.buffer_len > 56 {
            for byte in &mut self.buffer[self.buffer_len..] {
                *byte = 0;
            }
            let block = self.buffer;
            self.compress(&block);
            self.buffer_len = 0;
        }
        for byte in &mut self.buffer[self.buffer_len..56] {
            *byte = 0;
        }
        self.buffer[56..64].copy_from_slice(&bit_len.to_be_bytes());
        let block = self.buffer;
        self.compress(&block);

        let mut digest = [0_u8; 20];
        for (chunk, word) in digest.chunks_mut(4).zip(self.state) {
            chunk.copy_from_slice(&word.to_be_bytes());
        }
        digest
    }

    /// Compresses one 64-byte block into the state.
    fn compress(&mut self, block: &[u8; 64]) {
        let mut schedule = [0_u32; 80];
        for (index, chunk) in block.chunks(4).enumerate() {
            let mut word = [0_u8; 4];
            word.copy_from_slice(chunk);
            schedule[index] = u32::from_be_bytes(word);
        }
        for index in 16..80 {
            let value = schedule[index - 3]
                ^ schedule[index - 8]
                ^ schedule[index - 14]
                ^ schedule[index - 16];
            schedule[index] = value.rotate_left(1);
        }

        let mut hash = self.state;
        for (index, word) in schedule.iter().enumerate() {
            let (function, constant) = match index / 20 {
                0 => ((hash[1] & hash[2]) | (!hash[1] & hash[3]), K[0]),
                1 => (hash[1] ^ hash[2] ^ hash[3], K[1]),
                2 => (
                    (hash[1] & hash[2]) | (hash[1] & hash[3]) | (hash[2] & hash[3]),
                    K[2],
                ),
                _ => (hash[1] ^ hash[2] ^ hash[3], K[3]),
            };
            let temp = hash[0]
                .rotate_left(5)
                .wrapping_add(function)
                .wrapping_add(hash[4])
                .wrapping_add(constant)
                .wrapping_add(*word);
            hash[4] = hash[3];
            hash[3] = hash[2];
            hash[2] = hash[1].rotate_left(30);
            hash[1] = hash[0];
            hash[0] = temp;
        }

        for (state, value) in self.state.iter_mut().zip(hash) {
            *state = state.wrapping_add(value);
        }
    }
}

impl Default for Sha1 {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the SHA-1 digest of `data`.
#[must_use]
pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize()
}

/// Compares two digests without early exit.
#[must_use]
pub fn digest_eq(a: &[u8; 20], b: &[u8; 20]) -> bool {
    let mut difference = 0_u8;
    for (left, right) in a.iter().zip(b.iter()) {
        difference |= left ^ right;
    }
    difference == 0
}
