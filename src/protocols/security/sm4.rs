//! # SM4 Block Cipher Algorithm
//!
//! Native `#![no_std]` implementation of the Chinese Commercial Cryptography
//! SM4 128-bit block cipher standard (GB/T 32907-2016 / ISO/IEC 18033-3:2010/Amd 1:2021).
//! Supports 128-bit key expansion, raw block encryption/decryption, and CBC mode.

/// SM4 key size in bytes (128 bits = 16 bytes).
pub const SM4_KEY_SIZE: usize = 16;

/// SM4 block size in bytes (128 bits = 16 bytes).
pub const SM4_BLOCK_SIZE: usize = 16;

/// Number of rounds in SM4 cipher.
pub const SM4_NUM_ROUNDS: usize = 32;

/// SM4 S-Box lookup table.
const SBOX: [u8; 256] = [
    0xD6, 0x90, 0xE9, 0xFE, 0xCC, 0xE1, 0x3D, 0xB7, 0x16, 0xB6, 0x14, 0xC2, 0x28, 0xFB, 0x2C, 0x05,
    0x2B, 0x67, 0x9A, 0x76, 0x2A, 0xBE, 0x04, 0xC3, 0xAA, 0x44, 0x13, 0x26, 0x49, 0x86, 0x06, 0x99,
    0x9C, 0x42, 0x50, 0xF4, 0x91, 0xEF, 0x98, 0x7A, 0x33, 0x54, 0x0B, 0x43, 0xED, 0xCF, 0xAC, 0x62,
    0xE4, 0xB3, 0x1C, 0xA9, 0xC9, 0x08, 0xE8, 0x95, 0x80, 0xDF, 0x94, 0xFA, 0x75, 0x8F, 0x3F, 0xA6,
    0x47, 0x07, 0xA7, 0xFC, 0xF3, 0x73, 0x17, 0xBA, 0x83, 0x59, 0x3C, 0x19, 0xE6, 0x85, 0x4F, 0xA8,
    0x68, 0x6B, 0x81, 0xB2, 0x71, 0x64, 0xDA, 0x8B, 0xF8, 0xEB, 0x0F, 0x4B, 0x70, 0x56, 0x9D, 0x35,
    0x1E, 0x24, 0x0E, 0x5E, 0x63, 0x58, 0xD1, 0xA2, 0x25, 0x22, 0x7C, 0x3B, 0x01, 0x21, 0x78, 0x87,
    0xD4, 0x00, 0x46, 0x57, 0x9F, 0xD3, 0x27, 0x52, 0x4C, 0x36, 0x02, 0xE7, 0xA0, 0xC4, 0xC8, 0x9E,
    0xEA, 0xBF, 0x8A, 0xD2, 0x40, 0xC7, 0x38, 0xB5, 0xA3, 0xF7, 0xF2, 0xCE, 0xF9, 0x61, 0x15, 0xA1,
    0xE0, 0xAE, 0x5D, 0xA4, 0x9B, 0x34, 0x1A, 0x55, 0xAD, 0x93, 0x32, 0x30, 0xF5, 0x8C, 0xB1, 0xE3,
    0x1D, 0xF6, 0xE2, 0x2E, 0x82, 0x66, 0xCA, 0x60, 0xC0, 0x29, 0x23, 0xAB, 0x0D, 0x53, 0x4E, 0x6F,
    0xD5, 0xDB, 0x37, 0x45, 0xDE, 0xFD, 0x8E, 0x2F, 0x03, 0xFF, 0x6A, 0x72, 0x6D, 0x6C, 0x5B, 0x51,
    0x8D, 0x1B, 0xAF, 0x92, 0xBB, 0xDD, 0xBC, 0x7F, 0x11, 0xD9, 0x5C, 0x41, 0x1F, 0x10, 0x5A, 0xD8,
    0x0A, 0xC1, 0x31, 0x88, 0xA5, 0xCD, 0x7B, 0xBD, 0x2D, 0x74, 0xD0, 0x12, 0xB8, 0xE5, 0xB4, 0xB0,
    0x89, 0x69, 0x97, 0x4A, 0x0C, 0x96, 0x77, 0x7E, 0x65, 0xB9, 0xF1, 0x09, 0xC5, 0x6E, 0xC6, 0x84,
    0x18, 0xF0, 0x7D, 0xEC, 0x3A, 0xDC, 0x4D, 0x20, 0x79, 0xEE, 0x5F, 0x3E, 0xD7, 0xCB, 0x39, 0x48,
];

/// Family key constants for key schedule expansion.
const FK: [u32; 4] = [0xA3B1_BAC6, 0x56AA_3350, 0x677D_9197, 0xB270_22DC];

/// Computes the constant CK for round i.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_lossless)]
const fn ck(i: usize) -> u32 {
    let b0 = ((4 * i) * 7) as u8;
    let b1 = ((4 * i + 1) * 7) as u8;
    let b2 = ((4 * i + 2) * 7) as u8;
    let b3 = ((4 * i + 3) * 7) as u8;
    ((b0 as u32) << 24) | ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32)
}

fn tau(a: u32) -> u32 {
    let b0 = u32::from(SBOX[((a >> 24) & 0xFF) as usize]);
    let b1 = u32::from(SBOX[((a >> 16) & 0xFF) as usize]);
    let b2 = u32::from(SBOX[((a >> 8) & 0xFF) as usize]);
    let b3 = u32::from(SBOX[(a & 0xFF) as usize]);
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

fn l_transform(b: u32) -> u32 {
    b ^ b.rotate_left(2) ^ b.rotate_left(10) ^ b.rotate_left(18) ^ b.rotate_left(24)
}

fn l_prime(b: u32) -> u32 {
    b ^ b.rotate_left(13) ^ b.rotate_left(23)
}

/// SM4 Cipher instance with precomputed round keys.
#[derive(Clone, Copy, Debug)]
pub struct Sm4 {
    rk: [u32; SM4_NUM_ROUNDS],
}

impl Sm4 {
    /// Initializes SM4 cipher with a 128-bit key.
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub fn new(key: &[u8; SM4_KEY_SIZE]) -> Self {
        let mut mk = [0u32; 4];
        for i in 0..4 {
            mk[i] =
                u32::from_be_bytes([key[i * 4], key[i * 4 + 1], key[i * 4 + 2], key[i * 4 + 3]]);
        }

        let mut k = [0u32; 36];
        k[0] = mk[0] ^ FK[0];
        k[1] = mk[1] ^ FK[1];
        k[2] = mk[2] ^ FK[2];
        k[3] = mk[3] ^ FK[3];

        let mut rk = [0u32; SM4_NUM_ROUNDS];
        for i in 0..32 {
            let tmp = k[i + 1] ^ k[i + 2] ^ k[i + 3] ^ ck(i);
            k[i + 4] = k[i] ^ l_prime(tau(tmp));
            rk[i] = k[i + 4];
        }

        Self { rk }
    }

    /// Encrypts a single 16-byte block.
    #[allow(clippy::many_single_char_names)]
    pub fn encrypt_block(&self, block: &[u8; SM4_BLOCK_SIZE], out: &mut [u8; SM4_BLOCK_SIZE]) {
        let mut x = [0u32; 36];
        for i in 0..4 {
            x[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }

        for i in 0..32 {
            let tmp = x[i + 1] ^ x[i + 2] ^ x[i + 3] ^ self.rk[i];
            x[i + 4] = x[i] ^ l_transform(tau(tmp));
        }

        for i in 0..4 {
            let word = x[35 - i].to_be_bytes();
            out[i * 4..(i + 1) * 4].copy_from_slice(&word);
        }
    }

    /// Decrypts a single 16-byte block.
    #[allow(clippy::many_single_char_names)]
    pub fn decrypt_block(&self, block: &[u8; SM4_BLOCK_SIZE], out: &mut [u8; SM4_BLOCK_SIZE]) {
        let mut x = [0u32; 36];
        for i in 0..4 {
            x[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }

        for i in 0..32 {
            let tmp = x[i + 1] ^ x[i + 2] ^ x[i + 3] ^ self.rk[31 - i];
            x[i + 4] = x[i] ^ l_transform(tau(tmp));
        }

        for i in 0..4 {
            let word = x[35 - i].to_be_bytes();
            out[i * 4..(i + 1) * 4].copy_from_slice(&word);
        }
    }

    /// Encrypts data in-place using Cipher Block Chaining (CBC) mode.
    ///
    /// # Panics
    ///
    /// Panics if `data` length is not a multiple of 16 bytes.
    #[allow(clippy::chunks_exact_to_as_chunks)]
    pub fn cbc_encrypt(&self, iv: &[u8; SM4_BLOCK_SIZE], data: &mut [u8]) -> [u8; SM4_BLOCK_SIZE] {
        assert!(
            data.len() % SM4_BLOCK_SIZE == 0,
            "Data length must be multiple of 16"
        );
        let mut current_iv = *iv;
        let mut block = [0u8; SM4_BLOCK_SIZE];

        for chunk in data.chunks_exact_mut(SM4_BLOCK_SIZE) {
            for j in 0..SM4_BLOCK_SIZE {
                block[j] = chunk[j] ^ current_iv[j];
            }
            self.encrypt_block(&block, &mut current_iv);
            chunk.copy_from_slice(&current_iv);
        }

        current_iv
    }

    /// Decrypts data in-place using Cipher Block Chaining (CBC) mode.
    ///
    /// # Panics
    ///
    /// Panics if `data` length is not a multiple of 16 bytes.
    #[allow(clippy::chunks_exact_to_as_chunks)]
    pub fn cbc_decrypt(&self, iv: &[u8; SM4_BLOCK_SIZE], data: &mut [u8]) {
        assert!(
            data.len() % SM4_BLOCK_SIZE == 0,
            "Data length must be multiple of 16"
        );
        let mut current_iv = *iv;
        let mut block = [0u8; SM4_BLOCK_SIZE];
        let mut next_iv = [0u8; SM4_BLOCK_SIZE];

        for chunk in data.chunks_exact_mut(SM4_BLOCK_SIZE) {
            next_iv.copy_from_slice(chunk);
            let mut in_block = [0u8; SM4_BLOCK_SIZE];
            in_block.copy_from_slice(chunk);
            self.decrypt_block(&in_block, &mut block);
            for j in 0..SM4_BLOCK_SIZE {
                chunk[j] = block[j] ^ current_iv[j];
            }
            current_iv = next_iv;
        }
    }
}
