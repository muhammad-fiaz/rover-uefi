//! # SHA-512 and SHA-384 native digest
//!
//! Native `SHA-512` and truncated `SHA-384` implementation providing
//! allocation-free streaming hasher contexts, one-shot helpers, and
//! constant-time comparison.
//!
//! Input length limit: total length is stored modulo 2^64 bits, as
//! required by the specification. `finalize` consumes the hasher, so it
//! cannot be called twice.
//!
//! `SHA-384` is the `SHA-512` core with a distinct initial state and
//! truncated output. Operations remain distinct from firmware-provided
//! hash protocols, which delegate compression to firmware drivers.

/// The SHA-512 initial state (fractional parts of square roots of the
/// first eight primes).
const H512: [u64; 8] = [
    0x6a09_e667_f3bc_c908,
    0xbb67_ae85_84ca_a73b,
    0x3c6e_f372_fe94_f82b,
    0xa54f_f53a_5f1d_36f1,
    0x510e_527f_ade6_82d1,
    0x9b05_688c_2b3e_6c1f,
    0x1f83_d9ab_fb41_bd6b,
    0x5be0_cd19_137e_2179,
];

/// The SHA-384 initial state (fractional parts of square roots of the
/// 9th through 16th primes).
const H384: [u64; 8] = [
    0xcbbb_9d5d_c105_9ed8,
    0x629a_292a_367c_d507,
    0x9159_015a_3070_dd17,
    0x152f_ecd8_f70e_5939,
    0x6733_2667_ffc0_0b31,
    0x8eb4_4a87_6858_1511,
    0xdb0c_2e0d_64f9_8fa7,
    0x47b5_481d_befa_4fa4,
];

/// The SHA-512 round constants (fractional parts of cube roots of the
/// first 80 primes).
const K: [u64; 80] = [
    0x428a_2f98_d728_ae22,
    0x7137_4491_23ef_65cd,
    0xb5c0_fbcf_ec4d_3b2f,
    0xe9b5_dba5_8189_dbbc,
    0x3956_c25b_f348_b538,
    0x59f1_11f1_b605_d019,
    0x923f_82a4_af19_4f9b,
    0xab1c_5ed5_da6d_8118,
    0xd807_aa98_a303_0242,
    0x1283_5b01_4570_6fbe,
    0x2431_85be_4ee4_b28c,
    0x550c_7dc3_d5ff_b4e2,
    0x72be_5d74_f27b_896f,
    0x80de_b1fe_3b16_96b1,
    0x9bdc_06a7_25c7_1235,
    0xc19b_f174_cf69_2694,
    0xe49b_69c1_9ef1_4ad2,
    0xefbe_4786_384f_25e3,
    0x0fc1_9dc6_8b8c_d5b5,
    0x240c_a1cc_77ac_9c65,
    0x2de9_2c6f_592b_0275,
    0x4a74_84aa_6ea6_e483,
    0x5cb0_a9dc_bd41_fbd4,
    0x76f9_88da_8311_53b5,
    0x983e_5152_ee66_dfab,
    0xa831_c66d_2db4_3210,
    0xb003_27c8_98fb_213f,
    0xbf59_7fc7_beef_0ee4,
    0xc6e0_0bf3_3da8_8fc2,
    0xd5a7_9147_930a_a725,
    0x06ca_6351_e003_826f,
    0x1429_2967_0a0e_6e70,
    0x27b7_0a85_46d2_2ffc,
    0x2e1b_2138_5c26_c926,
    0x4d2c_6dfc_5ac4_2aed,
    0x5338_0d13_9d95_b3df,
    0x650a_7354_8baf_63de,
    0x766a_0abb_3c77_b2a8,
    0x81c2_c92e_47ed_aee6,
    0x9272_2c85_1482_353b,
    0xa2bf_e8a1_4cf1_0364,
    0xa81a_664b_bc42_3001,
    0xc24b_8b70_d0f8_9791,
    0xc76c_51a3_0654_be30,
    0xd192_e819_d6ef_5218,
    0xd699_0624_5565_a910,
    0xf40e_3585_5771_202a,
    0x106a_a070_32bb_d1b8,
    0x19a4_c116_b8d2_d0c8,
    0x1e37_6c08_5141_ab53,
    0x2748_774c_df8e_eb99,
    0x34b0_bcb5_e19b_48a8,
    0x391c_0cb3_c5c9_5a63,
    0x4ed8_aa4a_e341_8acb,
    0x5b9c_ca4f_7763_e373,
    0x682e_6ff3_d6b2_b8a3,
    0x748f_82ee_5def_b2fc,
    0x78a5_636f_4317_2f60,
    0x84c8_7814_a1f0_ab72,
    0x8cc7_0208_1a64_39ec,
    0x90be_fffa_2363_1e28,
    0xa450_6ceb_de82_bde9,
    0xbef9_a3f7_b2c6_7915,
    0xc671_78f2_e372_532b,
    0xca27_3ece_ea26_619c,
    0xd186_b8c7_21c0_c207,
    0xeada_7dd6_cde0_eb1e,
    0xf57d_4f7f_ee6e_d178,
    0x06f0_67aa_7217_6fba,
    0x0a63_7dc5_a2c8_98a6,
    0x113f_9804_bef9_0dae,
    0x1b71_0b35_131c_471b,
    0x28db_77f5_2304_7d84,
    0x32ca_ab7b_40c7_2493,
    0x3c9e_be0a_15c9_bebc,
    0x431d_67c4_9c10_0d4c,
    0x4cc5_d4be_cb3e_42b6,
    0x597f_299c_fc65_7e2a,
    0x5fcb_6fab_3ad6_faec,
    0x6c44_198c_4a47_5817,
];

/// A streaming SHA-512 hasher (no allocation, caller-owned state).
#[derive(Clone, Debug)]
pub struct Sha512 {
    state: [u64; 8],
    buffer: [u8; 128],
    buffer_len: usize,
    total_len: u64,
}

impl Sha512 {
    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: H512,
            buffer: [0; 128],
            buffer_len: 0,
            total_len: 0,
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        if self.buffer_len > 0 {
            let take = (128 - self.buffer_len).min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&data[..take]);
            self.buffer_len += take;
            data = &data[take..];
            if self.buffer_len == 128 {
                let block = self.buffer;
                self.compress(&block);
                self.buffer_len = 0;
            }
        }
        while data.len() >= 128 {
            let mut block = [0_u8; 128];
            block.copy_from_slice(&data[..128]);
            self.compress(&block);
            data = &data[128..];
        }
        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
            self.buffer_len = data.len();
        }
    }

    /// Finalizes and returns the 64-byte digest. Consumes the hasher.
    #[must_use]
    pub fn finalize(mut self) -> [u8; 64] {
        let bit_len = self.total_len.wrapping_mul(8);
        self.update(&[0x80]);
        while self.buffer_len != 112 {
            self.update(&[0]);
        }
        let mut len_block = [0_u8; 16];
        len_block[8..].copy_from_slice(&bit_len.to_be_bytes());
        self.update(&len_block);
        let mut out = [0_u8; 64];
        for (index, value) in self.state.iter().enumerate() {
            out[index * 8..index * 8 + 8].copy_from_slice(&value.to_be_bytes());
        }
        out
    }

    fn compress(&mut self, block: &[u8; 128]) {
        let mut words = [0_u64; 80];
        for index in 0..16 {
            words[index] = u64::from_be_bytes([
                block[index * 8],
                block[index * 8 + 1],
                block[index * 8 + 2],
                block[index * 8 + 3],
                block[index * 8 + 4],
                block[index * 8 + 5],
                block[index * 8 + 6],
                block[index * 8 + 7],
            ]);
        }
        for index in 16..80 {
            let small_sigma_0 = words[index - 15].rotate_right(1)
                ^ words[index - 15].rotate_right(8)
                ^ (words[index - 15] >> 7);
            let small_sigma_1 = words[index - 2].rotate_right(19)
                ^ words[index - 2].rotate_right(61)
                ^ (words[index - 2] >> 6);
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
        for index in 0..80 {
            let big_sigma_1 =
                state_e.rotate_right(14) ^ state_e.rotate_right(18) ^ state_e.rotate_right(41);
            let choose = (state_e & state_f) ^ ((!state_e) & state_g);
            let temp_1 = state_h
                .wrapping_add(big_sigma_1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let big_sigma_0 =
                state_a.rotate_right(28) ^ state_a.rotate_right(34) ^ state_a.rotate_right(39);
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

impl Default for Sha512 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// A streaming SHA-384 hasher (no allocation, caller-owned state).
#[derive(Clone, Debug)]
pub struct Sha384 {
    state: [u64; 8],
    buffer: [u8; 128],
    buffer_len: usize,
    total_len: u64,
}

impl Sha384 {
    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: H384,
            buffer: [0; 128],
            buffer_len: 0,
            total_len: 0,
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        if self.buffer_len > 0 {
            let take = (128 - self.buffer_len).min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&data[..take]);
            self.buffer_len += take;
            data = &data[take..];
            if self.buffer_len == 128 {
                let block = self.buffer;
                self.compress(&block);
                self.buffer_len = 0;
            }
        }
        while data.len() >= 128 {
            let mut block = [0_u8; 128];
            block.copy_from_slice(&data[..128]);
            self.compress(&block);
            data = &data[128..];
        }
        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
            self.buffer_len = data.len();
        }
    }

    /// Finalizes and returns the 48-byte digest. Consumes the hasher.
    #[must_use]
    pub fn finalize(mut self) -> [u8; 48] {
        let bit_len = self.total_len.wrapping_mul(8);
        self.update(&[0x80]);
        while self.buffer_len != 112 {
            self.update(&[0]);
        }
        let mut len_block = [0_u8; 16];
        len_block[8..].copy_from_slice(&bit_len.to_be_bytes());
        self.update(&len_block);
        let mut out = [0_u8; 48];
        for (index, value) in self.state.iter().enumerate() {
            if index < 6 {
                out[index * 8..index * 8 + 8].copy_from_slice(&value.to_be_bytes());
            }
        }
        out
    }

    fn compress(&mut self, block: &[u8; 128]) {
        let mut words = [0_u64; 80];
        for index in 0..16 {
            words[index] = u64::from_be_bytes([
                block[index * 8],
                block[index * 8 + 1],
                block[index * 8 + 2],
                block[index * 8 + 3],
                block[index * 8 + 4],
                block[index * 8 + 5],
                block[index * 8 + 6],
                block[index * 8 + 7],
            ]);
        }
        for index in 16..80 {
            let small_sigma_0 = words[index - 15].rotate_right(1)
                ^ words[index - 15].rotate_right(8)
                ^ (words[index - 15] >> 7);
            let small_sigma_1 = words[index - 2].rotate_right(19)
                ^ words[index - 2].rotate_right(61)
                ^ (words[index - 2] >> 6);
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
        for index in 0..80 {
            let big_sigma_1 =
                state_e.rotate_right(14) ^ state_e.rotate_right(18) ^ state_e.rotate_right(41);
            let choose = (state_e & state_f) ^ ((!state_e) & state_g);
            let temp_1 = state_h
                .wrapping_add(big_sigma_1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let big_sigma_0 =
                state_a.rotate_right(28) ^ state_a.rotate_right(34) ^ state_a.rotate_right(39);
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

impl Default for Sha384 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the SHA-512 digest of `data` in one call.
#[must_use]
pub fn sha512(data: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize()
}

/// Computes the SHA-384 digest of `data` in one call.
#[must_use]
pub fn sha384(data: &[u8]) -> [u8; 48] {
    let mut hasher = Sha384::new();
    hasher.update(data);
    hasher.finalize()
}

/// Constant-time comparison of two 64-byte digests (no early exit).
#[must_use]
pub fn digest_eq_512(a: &[u8; 64], b: &[u8; 64]) -> bool {
    let mut diff = 0_u8;
    for index in 0..64 {
        diff |= a[index] ^ b[index];
    }
    diff == 0
}

/// Constant-time comparison of two 48-byte digests (no early exit).
#[must_use]
pub fn digest_eq_384(a: &[u8; 48], b: &[u8; 48]) -> bool {
    let mut diff = 0_u8;
    for index in 0..48 {
        diff |= a[index] ^ b[index];
    }
    diff == 0
}
