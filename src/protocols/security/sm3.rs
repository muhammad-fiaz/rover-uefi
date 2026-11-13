//! # SM3 Cryptographic Hash Algorithm
//!
//! Native `#![no_std]` implementation of the Chinese Commercial Cryptography
//! SM3 256-bit hash standard (GB/T 32918 / ISO/IEC 10118-3:2018).
//! Operates without heap allocation on caller-provided buffers.

/// SM3 digest size in bytes (256 bits = 32 bytes).
pub const SM3_DIGEST_SIZE: usize = 32;

/// SM3 block size in bytes (512 bits = 64 bytes).
pub const SM3_BLOCK_SIZE: usize = 64;

/// Initial vector IV for SM3.
const IV: [u32; 8] = [
    0x7380_166F,
    0x4914_B2B9,
    0x1724_42D7,
    0xDA8A_0600,
    0xA96F_30BC,
    0x1631_38AA,
    0xE38D_EE4D,
    0xB0FB_0E4E,
];

const fn p0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}

const fn p1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}

const fn ff(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (x & z) | (y & z)
    }
}

const fn gg(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (!x & z)
    }
}

#[allow(clippy::many_single_char_names)]
#[allow(clippy::cast_possible_truncation)]
fn compress_block(state: &mut [u32; 8], block: &[u8; SM3_BLOCK_SIZE]) {
    let mut w = [0u32; 68];
    let mut w_prime = [0u32; 64];

    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }

    for j in 16..68 {
        let tmp = w[j - 16] ^ w[j - 9] ^ w[j - 3].rotate_left(15);
        w[j] = p1(tmp) ^ w[j - 13].rotate_left(7) ^ w[j - 6];
    }

    for j in 0..64 {
        w_prime[j] = w[j] ^ w[j + 4];
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    for j in 0..64 {
        let tj: u32 = if j < 16 { 0x79CC_4519 } else { 0x7A87_9D8A };
        let tj_rot = tj.rotate_left((j as u32) % 32);
        let ss1 = (a.rotate_left(12).wrapping_add(e).wrapping_add(tj_rot)).rotate_left(7);
        let ss2 = ss1 ^ a.rotate_left(12);
        let tt1 = ff(j, a, b, c)
            .wrapping_add(d)
            .wrapping_add(ss2)
            .wrapping_add(w_prime[j]);
        let tt2 = gg(j, e, f, g)
            .wrapping_add(h)
            .wrapping_add(ss1)
            .wrapping_add(w[j]);

        d = c;
        c = b.rotate_left(9);
        b = a;
        a = tt1;
        h = g;
        g = f.rotate_left(19);
        f = e;
        e = p0(tt2);
    }

    state[0] ^= a;
    state[1] ^= b;
    state[2] ^= c;
    state[3] ^= d;
    state[4] ^= e;
    state[5] ^= f;
    state[6] ^= g;
    state[7] ^= h;
}

/// State of an in-progress SM3 hash calculation.
#[derive(Clone, Copy, Debug)]
pub struct Sm3 {
    state: [u32; 8],
    buffer: [u8; SM3_BLOCK_SIZE],
    buf_len: usize,
    total_len: u64,
}

impl Default for Sm3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sm3 {
    /// Creates a new SM3 hasher instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: IV,
            buffer: [0u8; SM3_BLOCK_SIZE],
            buf_len: 0,
            total_len: 0,
        }
    }

    /// Resets the hasher back to initial state.
    pub fn reset(&mut self) {
        self.state = IV;
        self.buffer = [0u8; SM3_BLOCK_SIZE];
        self.buf_len = 0;
        self.total_len = 0;
    }

    /// Feeds input bytes into the hasher.
    pub fn update(&mut self, data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        let mut offset = 0;

        if self.buf_len > 0 {
            let to_fill = SM3_BLOCK_SIZE - self.buf_len;
            if data.len() >= to_fill {
                self.buffer[self.buf_len..SM3_BLOCK_SIZE].copy_from_slice(&data[..to_fill]);
                compress_block(&mut self.state, &self.buffer);
                self.buf_len = 0;
                offset = to_fill;
            } else {
                self.buffer[self.buf_len..self.buf_len + data.len()].copy_from_slice(data);
                self.buf_len += data.len();
                return;
            }
        }

        while offset + SM3_BLOCK_SIZE <= data.len() {
            let mut chunk = [0u8; SM3_BLOCK_SIZE];
            chunk.copy_from_slice(&data[offset..offset + SM3_BLOCK_SIZE]);
            compress_block(&mut self.state, &chunk);
            offset += SM3_BLOCK_SIZE;
        }

        let remaining = &data[offset..];
        if !remaining.is_empty() {
            self.buffer[..remaining.len()].copy_from_slice(remaining);
            self.buf_len = remaining.len();
        }
    }

    /// Finalizes the hash calculation and returns the 32-byte SM3 digest.
    #[must_use]
    pub fn finalize(mut self) -> [u8; SM3_DIGEST_SIZE] {
        let total_bits = self.total_len.wrapping_mul(8);

        // Append 0x80 bit
        self.buffer[self.buf_len] = 0x80;
        self.buf_len += 1;

        if self.buf_len > 56 {
            for b in &mut self.buffer[self.buf_len..SM3_BLOCK_SIZE] {
                *b = 0;
            }
            compress_block(&mut self.state, &self.buffer);
            self.buf_len = 0;
        }

        for b in &mut self.buffer[self.buf_len..56] {
            *b = 0;
        }

        self.buffer[56..64].copy_from_slice(&total_bits.to_be_bytes());
        compress_block(&mut self.state, &self.buffer);

        let mut digest = [0u8; SM3_DIGEST_SIZE];
        for (i, word) in self.state.iter().enumerate() {
            digest[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
        }
        digest
    }

    /// One-shot computation of SM3 digest on a data buffer.
    #[must_use]
    pub fn digest(data: &[u8]) -> [u8; SM3_DIGEST_SIZE] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }
}
