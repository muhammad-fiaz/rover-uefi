//! # SHA-224 native digest
//!
//! Native `SHA-224` implementation providing an allocation-free
//! streaming hasher, a one-shot helper, and constant-time comparison.
//!
//! `SHA-224` is the `SHA-256` core with a distinct initial state and
//! truncated output. Operations remain distinct from firmware-provided
//! hash protocols, which delegate compression to firmware drivers.
//!
//! Input length limit: total length is stored modulo 2^64 bits, as
//! required by the specification. `finalize` consumes the hasher, so it
//! cannot be called twice.

/// The SHA-224 initial state (low 32 bits of the SHA-384 initial state).
const H224: [u32; 8] = [
    0xc105_9ed8,
    0x367c_d507,
    0x3070_dd17,
    0xf70e_5939,
    0xffc0_0b31,
    0x6858_1511,
    0x64f9_8fa7,
    0xbefa_4fa4,
];

/// The SHA-256 round constants shared by SHA-224.
const K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// A streaming SHA-224 hasher (no allocation, caller-owned state).
#[derive(Clone, Debug)]
pub struct Sha224 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

impl Sha224 {
    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: H224,
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

    /// Finalizes and returns the 28-byte digest. Consumes the hasher.
    #[must_use]
    pub fn finalize(mut self) -> [u8; 28] {
        let bit_len = self.total_len.wrapping_mul(8);
        self.update(&[0x80]);
        while self.buffer_len != 56 {
            self.update(&[0]);
        }
        let mut len_block = [0_u8; 8];
        len_block.copy_from_slice(&bit_len.to_be_bytes());
        self.update(&len_block);
        let mut out = [0_u8; 28];
        for index in 0..7 {
            out[index * 4..index * 4 + 4].copy_from_slice(&self.state[index].to_be_bytes());
        }
        out
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut words = [0_u32; 64];
        for index in 0..16 {
            words[index] = u32::from_be_bytes([
                block[index * 4],
                block[index * 4 + 1],
                block[index * 4 + 2],
                block[index * 4 + 3],
            ]);
        }
        for index in 16..64 {
            let small_sigma_0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let small_sigma_1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(small_sigma_0)
                .wrapping_add(words[index - 7])
                .wrapping_add(small_sigma_1);
        }
        let mut state_a = self.state[0];
        let mut state_b = self.state[1];
        let mut state_c = self.state[2];
        let mut state_d = self.state[3];
        let mut state_e = self.state[4];
        let mut state_f = self.state[5];
        let mut state_g = self.state[6];
        let mut state_h = self.state[7];
        for index in 0..64 {
            let big_sigma_1 =
                state_e.rotate_right(6) ^ state_e.rotate_right(11) ^ state_e.rotate_right(25);
            let choose = (state_e & state_f) ^ ((!state_e) & state_g);
            let temp_1 = state_h
                .wrapping_add(big_sigma_1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let big_sigma_0 =
                state_a.rotate_right(2) ^ state_a.rotate_right(13) ^ state_a.rotate_right(22);
            let majority = (state_a & state_b) ^ (state_a & state_c) ^ (state_b & state_c);
            let temp_2 = big_sigma_0.wrapping_add(majority);
            state_h = state_g;
            state_g = state_f;
            state_f = state_e;
            state_e = state_d.wrapping_add(temp_1);
            state_d = state_c;
            state_c = state_b;
            state_b = state_a;
            state_a = temp_1.wrapping_add(temp_2);
        }
        self.state[0] = self.state[0].wrapping_add(state_a);
        self.state[1] = self.state[1].wrapping_add(state_b);
        self.state[2] = self.state[2].wrapping_add(state_c);
        self.state[3] = self.state[3].wrapping_add(state_d);
        self.state[4] = self.state[4].wrapping_add(state_e);
        self.state[5] = self.state[5].wrapping_add(state_f);
        self.state[6] = self.state[6].wrapping_add(state_g);
        self.state[7] = self.state[7].wrapping_add(state_h);
    }
}

impl Default for Sha224 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the SHA-224 digest of `data` in one call.
#[must_use]
pub fn sha224(data: &[u8]) -> [u8; 28] {
    let mut hasher = Sha224::new();
    hasher.update(data);
    hasher.finalize()
}

/// Constant-time comparison of two 28-byte digests (no early exit).
#[must_use]
pub fn digest_eq_224(a: &[u8; 28], b: &[u8; 28]) -> bool {
    let mut diff = 0_u8;
    for index in 0..28 {
        diff |= a[index] ^ b[index];
    }
    diff == 0
}
