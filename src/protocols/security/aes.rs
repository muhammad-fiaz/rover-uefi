//! # AES block cipher
//!
//! Native AES-128, AES-192 and AES-256 block encryption and
//! decryption. The implementation is table-driven and operates on
//! caller-owned buffers without allocation.
//!
//! This module provides the raw block transformation only. Use
//! [`super::gcm`] for authenticated encryption.

/// The AES substitution box.
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// The AES inverse substitution box.
const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/// The round constants for key expansion.
const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

/// An expanded AES key supporting 128, 192 and 256 bit keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aes {
    round_keys: [u32; 60],
    rounds: usize,
}

impl Aes {
    /// Expands 128-bit `key`.
    #[must_use]
    pub fn new_128(key: &[u8; 16]) -> Self {
        Self::expand(key, 4)
    }

    /// Expands 192-bit `key`.
    #[must_use]
    pub fn new_192(key: &[u8; 24]) -> Self {
        Self::expand(key, 6)
    }

    /// Expands 256-bit `key`.
    #[must_use]
    pub fn new_256(key: &[u8; 32]) -> Self {
        Self::expand(key, 8)
    }

    /// Expands a key slice of length 16, 24 or 32.
    ///
    /// Returns `None` for any other length.
    #[must_use]
    pub fn new(key: &[u8]) -> Option<Self> {
        match key.len() {
            16 => {
                let mut array = [0_u8; 16];
                array.copy_from_slice(key);
                Some(Self::new_128(&array))
            }
            24 => {
                let mut array = [0_u8; 24];
                array.copy_from_slice(key);
                Some(Self::new_192(&array))
            }
            32 => {
                let mut array = [0_u8; 32];
                array.copy_from_slice(key);
                Some(Self::new_256(&array))
            }
            _ => None,
        }
    }

    /// Returns the number of rounds.
    #[must_use]
    pub const fn rounds(&self) -> usize {
        self.rounds
    }

    /// Expands `key` with `words` 32-bit key words.
    fn expand(key: &[u8], words: usize) -> Self {
        let total = 4 * (words + 7);
        let mut round_keys = [0_u32; 60];
        for (index, chunk) in key.chunks(4).enumerate() {
            let mut word = [0_u8; 4];
            word.copy_from_slice(chunk);
            round_keys[index] = u32::from_be_bytes(word);
        }
        for index in words..total {
            let mut temp = round_keys[index - 1];
            if index % words == 0 {
                temp = sub_word(temp.rotate_left(8)) ^ (u32::from(RCON[index / words - 1]) << 24);
            } else if words > 6 && index % words == 4 {
                temp = sub_word(temp);
            }
            round_keys[index] = round_keys[index - words] ^ temp;
        }
        Self {
            round_keys,
            rounds: words + 6,
        }
    }

    /// Encrypts one 16-byte `block` into `output`.
    pub fn encrypt_block(&self, block: &[u8; 16], output: &mut [u8; 16]) {
        let mut state = load_state(block);
        add_round_key(&mut state, &self.round_keys[..4]);
        for round in 1..self.rounds {
            sub_bytes(&mut state);
            shift_rows(&mut state);
            mix_columns(&mut state);
            add_round_key(&mut state, &self.round_keys[round * 4..round * 4 + 4]);
        }
        sub_bytes(&mut state);
        shift_rows(&mut state);
        add_round_key(
            &mut state,
            &self.round_keys[self.rounds * 4..self.rounds * 4 + 4],
        );
        store_state(&state, output);
    }

    /// Decrypts one 16-byte `block` into `output`.
    pub fn decrypt_block(&self, block: &[u8; 16], output: &mut [u8; 16]) {
        let mut state = load_state(block);
        add_round_key(
            &mut state,
            &self.round_keys[self.rounds * 4..self.rounds * 4 + 4],
        );
        for round in (1..self.rounds).rev() {
            inv_shift_rows(&mut state);
            inv_sub_bytes(&mut state);
            add_round_key(&mut state, &self.round_keys[round * 4..round * 4 + 4]);
            inv_mix_columns(&mut state);
        }
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(&mut state, &self.round_keys[..4]);
        store_state(&state, output);
    }
}

/// Loads a block into column-major state.
fn load_state(block: &[u8; 16]) -> [[u8; 4]; 4] {
    let mut state = [[0_u8; 4]; 4];
    for (column, chunk) in block.chunks(4).enumerate() {
        for (row, byte) in chunk.iter().enumerate() {
            state[row][column] = *byte;
        }
    }
    state
}

/// Stores column-major state into a block.
fn store_state(state: &[[u8; 4]; 4], output: &mut [u8; 16]) {
    for column in 0..4 {
        for row in 0..4 {
            output[column * 4 + row] = state[row][column];
        }
    }
}

/// Applies the round key to the state.
fn add_round_key(state: &mut [[u8; 4]; 4], key: &[u32]) {
    for (column, word) in key.iter().enumerate() {
        let bytes = word.to_be_bytes();
        for row in 0..4 {
            state[row][column] ^= bytes[row];
        }
    }
}

/// Applies the substitution box.
fn sub_bytes(state: &mut [[u8; 4]; 4]) {
    for column in state.iter_mut() {
        for byte in column.iter_mut() {
            *byte = SBOX[usize::from(*byte)];
        }
    }
}

/// Applies the inverse substitution box.
fn inv_sub_bytes(state: &mut [[u8; 4]; 4]) {
    for column in state.iter_mut() {
        for byte in column.iter_mut() {
            *byte = INV_SBOX[usize::from(*byte)];
        }
    }
}

/// Rotates each state row left.
fn shift_rows(state: &mut [[u8; 4]; 4]) {
    let original = *state;
    for row in 0..4 {
        for column in 0..4 {
            state[row][column] = original[row][(column + row) % 4];
        }
    }
}

/// Rotates each state row right.
fn inv_shift_rows(state: &mut [[u8; 4]; 4]) {
    let original = *state;
    for row in 0..4 {
        for column in 0..4 {
            state[row][column] = original[row][(column + 4 - row) % 4];
        }
    }
}

/// Multiplies by two in the AES field.
const fn xtime(value: u8) -> u8 {
    (value << 1) ^ if value & 0x80 != 0 { 0x1b } else { 0 }
}

/// Multiplies two values in the AES field.
fn multiply(left: u8, right: u8) -> u8 {
    let mut result = 0_u8;
    let mut a = left;
    let mut b = right;
    for _ in 0..8 {
        if b & 1 != 0 {
            result ^= a;
        }
        a = xtime(a);
        b >>= 1;
    }
    result
}

/// Applies the `MixColumns` transformation.
fn mix_columns(state: &mut [[u8; 4]; 4]) {
    for index in 0..4 {
        let a = state[0][index];
        let b = state[1][index];
        let c = state[2][index];
        let d = state[3][index];
        state[0][index] = multiply(a, 2) ^ multiply(b, 3) ^ c ^ d;
        state[1][index] = a ^ multiply(b, 2) ^ multiply(c, 3) ^ d;
        state[2][index] = a ^ b ^ multiply(c, 2) ^ multiply(d, 3);
        state[3][index] = multiply(a, 3) ^ b ^ c ^ multiply(d, 2);
    }
}

/// Applies the inverse `MixColumns` transformation.
fn inv_mix_columns(state: &mut [[u8; 4]; 4]) {
    for index in 0..4 {
        let a = state[0][index];
        let b = state[1][index];
        let c = state[2][index];
        let d = state[3][index];
        state[0][index] = multiply(a, 14) ^ multiply(b, 11) ^ multiply(c, 13) ^ multiply(d, 9);
        state[1][index] = multiply(a, 9) ^ multiply(b, 14) ^ multiply(c, 11) ^ multiply(d, 13);
        state[2][index] = multiply(a, 13) ^ multiply(b, 9) ^ multiply(c, 14) ^ multiply(d, 11);
        state[3][index] = multiply(a, 11) ^ multiply(b, 13) ^ multiply(c, 9) ^ multiply(d, 14);
    }
}

/// Substitutes each byte of a word.
fn sub_word(word: u32) -> u32 {
    let bytes = word.to_be_bytes();
    u32::from_be_bytes([
        SBOX[usize::from(bytes[0])],
        SBOX[usize::from(bytes[1])],
        SBOX[usize::from(bytes[2])],
        SBOX[usize::from(bytes[3])],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes128_expansion_matches_standard() {
        let aes = Aes::new_128(&[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ]);
        assert_eq!(
            aes.round_keys[..8],
            [
                0x0001_0203,
                0x0405_0607,
                0x0809_0a0b,
                0x0c0d_0e0f,
                0xd6aa_74fd,
                0xd2af_72fa,
                0xdaa6_78f1,
                0xd6ab_76fe,
            ]
        );
    }

    #[test]
    fn aes_round_trip() {
        let aes = Aes::new_256(&[0x42; 32]);
        let plaintext = [0x11; 16];
        let mut ciphertext = [0; 16];
        aes.encrypt_block(&plaintext, &mut ciphertext);
        let mut recovered = [0; 16];
        aes.decrypt_block(&ciphertext, &mut recovered);
        assert_eq!(recovered, plaintext);
    }
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn aes128_encrypts_fips_vector() {
        let key: [u8; 16] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ];
        let plaintext: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let aes = Aes::new_128(&key);
        let mut output = [0_u8; 16];
        aes.encrypt_block(&plaintext, &mut output);
        assert_eq!(
            output,
            [
                0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4,
                0xc5, 0x5a
            ]
        );
    }
}
