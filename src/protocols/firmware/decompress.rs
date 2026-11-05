//! # UEFI Decompression Protocol and Algorithm
//!
//! Defined by the UEFI Specification Appendix A: Decompression Algorithm and
//! `EFI_DECOMPRESS_PROTOCOL`.
//!
//! Provides decompression services for compressed firmware volumes, sections,
//! and images using the standard UEFI Huffman/LZ77 algorithm.

#![allow(
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::similar_names,
    clippy::doc_markdown,
    clippy::cast_lossless,
    clippy::needless_range_loop,
    clippy::explicit_iter_loop
)]

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_DECOMPRESS_PROTOCOL_GUID`.
pub const EFI_DECOMPRESS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD811_751A,
    0x94A6,
    0x11D4,
    [0x9A, 0x3A, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Function pointer for `EFI_DECOMPRESS_PROTOCOL.GetInfo`.
pub type EfiDecompressGetInfo = unsafe extern "efiapi" fn(
    this: *const EfiDecompressProtocol,
    source: *const c_void,
    source_size: u32,
    destination_size: *mut u32,
    scratch_size: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_DECOMPRESS_PROTOCOL.Decompress`.
pub type EfiDecompress = unsafe extern "efiapi" fn(
    this: *const EfiDecompressProtocol,
    source: *const c_void,
    source_size: u32,
    destination: *mut c_void,
    destination_size: u32,
    scratch: *mut c_void,
    scratch_size: u32,
) -> EfiStatus;

/// The raw ABI structure for `EFI_DECOMPRESS_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiDecompressProtocol {
    /// Retrieves uncompressed size and required scratch buffer size.
    pub get_info: EfiDecompressGetInfo,
    /// Decompresses source into destination using scratch space.
    pub decompress: EfiDecompress,
}

/// Scratch buffer size required for UEFI standard decompression (in bytes).
pub const UEFI_DECOMPRESS_SCRATCH_SIZE: usize = 65_536;

/// Parsed metadata for a compressed UEFI stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecompressInfo {
    /// Size of the compressed data payload excluding the 8-byte header.
    pub compressed_size: u32,
    /// Uncompressed size of the original data.
    pub uncompressed_size: u32,
    /// Minimum size of the scratch buffer needed for decompression.
    pub scratch_size: u32,
}

/// Inspects the header of a compressed buffer and returns its decompressed and scratch requirements.
///
/// # Errors
/// Returns `EfiStatus::INVALID_PARAMETER` if the header is truncated or malformed.
pub fn get_decompress_info(source: &[u8]) -> Result<DecompressInfo, EfiStatus> {
    if source.len() < 8 {
        return Err(EfiStatus::INVALID_PARAMETER);
    }
    let comp_size = u32::from_le_bytes([source[0], source[1], source[2], source[3]]);
    let uncomp_size = u32::from_le_bytes([source[4], source[5], source[6], source[7]]);

    let total_req = match comp_size.checked_add(8) {
        Some(val) => usize::try_from(val).map_err(|_| EfiStatus::INVALID_PARAMETER)?,
        None => return Err(EfiStatus::INVALID_PARAMETER),
    };

    if source.len() < total_req {
        return Err(EfiStatus::INVALID_PARAMETER);
    }

    Ok(DecompressInfo {
        compressed_size: comp_size,
        uncompressed_size: uncomp_size,
        scratch_size: u32::try_from(UEFI_DECOMPRESS_SCRATCH_SIZE)
            .map_err(|_| EfiStatus::OUT_OF_RESOURCES)?,
    })
}

// Internal decompressor structures conforming to UEFI Spec Appendix A
const MAXMATCH: usize = 256;
const THRESHOLD: usize = 3;
const CODE_BIT: usize = 16;
const NC: usize = 0xFF + MAXMATCH + 2 - THRESHOLD; // 510
const CBIT: usize = 9;
const MAXPBIT: usize = 5;
const TBIT: usize = 5;
const MAXNP: usize = (1 << MAXPBIT) - 1;
const NT: usize = CODE_BIT + 3;
const NPT: usize = if NT > MAXNP { NT } else { MAXNP };

struct BitStream<'a> {
    src: &'a [u8],
    in_idx: usize,
    comp_size: usize,
    bit_count: u16,
    bit_buf: u32,
    sub_bit_buf: u32,
}

impl<'a> BitStream<'a> {
    fn new(src: &'a [u8], comp_size: usize) -> Self {
        Self {
            src,
            in_idx: 0,
            comp_size,
            bit_count: 0,
            bit_buf: 0,
            sub_bit_buf: 0,
        }
    }

    fn fill_buf(&mut self, mut n: u16) {
        self.bit_buf = self.bit_buf.wrapping_shl(u32::from(n));
        while n > self.bit_count {
            n -= self.bit_count;
            self.bit_buf |= self.sub_bit_buf.wrapping_shl(u32::from(n));
            if self.comp_size > 0 {
                self.comp_size -= 1;
                self.sub_bit_buf = u32::from(self.src.get(self.in_idx).copied().unwrap_or(0));
                self.in_idx += 1;
                self.bit_count = 8;
            } else {
                self.sub_bit_buf = 0;
                self.bit_count = 8;
            }
        }
        self.bit_count -= n;
        self.bit_buf |= self.sub_bit_buf.wrapping_shr(u32::from(self.bit_count));
    }

    fn get_bits(&mut self, n: u16) -> u32 {
        let x = self.bit_buf.wrapping_shr(u32::from(16_u16.wrapping_sub(n)));
        self.fill_buf(n);
        x
    }
}

/// Decompresses UEFI compressed data from `source` into `destination`.
///
/// If `source` has an 8-byte header, it will be validated and skipped.
/// `scratch` must be at least `UEFI_DECOMPRESS_SCRATCH_SIZE` bytes.
///
/// # Errors
/// Returns `EfiStatus::INVALID_PARAMETER` if data is corrupted or buffers are too small.
pub fn decompress_uefi(
    source: &[u8],
    destination: &mut [u8],
    scratch: &mut [u8],
) -> Result<(), EfiStatus> {
    if scratch.len() < UEFI_DECOMPRESS_SCRATCH_SIZE {
        return Err(EfiStatus::BUFFER_TOO_SMALL);
    }
    let info = get_decompress_info(source)?;
    let dest_len = destination.len();
    if dest_len
        < usize::try_from(info.uncompressed_size).map_err(|_| EfiStatus::INVALID_PARAMETER)?
    {
        return Err(EfiStatus::BUFFER_TOO_SMALL);
    }

    let compressed_payload = source
        .get(
            8..8 + usize::try_from(info.compressed_size)
                .map_err(|_| EfiStatus::INVALID_PARAMETER)?,
        )
        .ok_or(EfiStatus::INVALID_PARAMETER)?;

    let mut stream = BitStream::new(compressed_payload, compressed_payload.len());
    stream.fill_buf(16);

    let mut out_idx = 0_usize;
    let orig_size =
        usize::try_from(info.uncompressed_size).map_err(|_| EfiStatus::INVALID_PARAMETER)?;

    // Scratch data tables
    let mut left = [0_u16; 2 * NC - 1];
    let mut right = [0_u16; 2 * NC - 1];
    let mut c_len = [0_u8; NC];
    let mut pt_len = [0_u8; NPT];
    let mut c_table = [0_u16; 4096];
    let mut pt_table = [0_u16; 256];
    let mut block_size = 0_u16;

    while out_idx < orig_size {
        // Decode character/length
        if block_size == 0 {
            block_size = stream.get_bits(16) as u16;
            // Read PT table
            let n = stream.get_bits(TBIT as u16) as usize;
            if n > NPT {
                return Err(EfiStatus::INVALID_PARAMETER);
            }
            if n == 0 {
                let c = stream.get_bits(TBIT as u16) as u16;
                pt_len.fill(0);
                pt_table.fill(c);
            } else {
                let mut i = 0_usize;
                while i < n {
                    let c = stream.bit_buf.wrapping_shr(13) as usize;
                    if c == 7 {
                        let mut mask = 1_u32 << 12;
                        while mask != 0 && (stream.bit_buf & mask) != 0 {
                            mask >>= 1;
                        }
                    }
                    let len = stream.get_bits(3) as u8;
                    pt_len[i] = len;
                    i += 1;
                    if i == 3 {
                        let c = stream.get_bits(2) as usize;
                        for _ in 0..c {
                            if i < n {
                                pt_len[i] = 0;
                                i += 1;
                            }
                        }
                    }
                }
                while i < NPT {
                    pt_len[i] = 0;
                    i += 1;
                }
                make_table(NPT, &pt_len, 8, &mut pt_table, &mut left, &mut right)?;
            }

            // Read C table
            let n = stream.get_bits(CBIT as u16) as usize;
            if n > NC {
                return Err(EfiStatus::INVALID_PARAMETER);
            }
            if n == 0 {
                let c = stream.get_bits(CBIT as u16) as u16;
                c_len.fill(0);
                c_table.fill(c);
            } else {
                let mut i = 0_usize;
                while i < n {
                    let mut j = pt_table[(stream.bit_buf.wrapping_shr(8) & 0xFF) as usize] as usize;
                    if j >= NPT {
                        let mut mask = 1_u32 << 7;
                        while j >= NPT {
                            if (stream.bit_buf & mask) != 0 {
                                j = right.get(j).copied().unwrap_or(0) as usize;
                            } else {
                                j = left.get(j).copied().unwrap_or(0) as usize;
                            }
                            mask >>= 1;
                        }
                    }
                    stream.fill_buf(pt_len.get(j).copied().unwrap_or(0) as u16);
                    if j <= 2 {
                        let count = match j {
                            0 => 1,
                            1 => (stream.get_bits(4) as usize).saturating_add(3),
                            _ => (stream.get_bits(CBIT as u16) as usize).saturating_add(20),
                        };
                        for _ in 0..count {
                            if i < n {
                                c_len[i] = 0;
                                i += 1;
                            }
                        }
                    } else {
                        c_len[i] = (j as u8).saturating_sub(2);
                        i += 1;
                    }
                }
                while i < NC {
                    c_len[i] = 0;
                    i += 1;
                }
                make_table(NC, &c_len, 12, &mut c_table, &mut left, &mut right)?;
            }
        }

        block_size = block_size.saturating_sub(1);

        // Decode symbol
        let mut j = c_table[(stream.bit_buf.wrapping_shr(4) & 0x0FFF) as usize] as usize;
        if j >= NC {
            let mut mask = 1_u32 << 3;
            while j >= NC {
                if (stream.bit_buf & mask) != 0 {
                    j = right.get(j).copied().unwrap_or(0) as usize;
                } else {
                    j = left.get(j).copied().unwrap_or(0) as usize;
                }
                mask >>= 1;
            }
        }
        stream.fill_buf(c_len.get(j).copied().unwrap_or(0) as u16);

        if j < 256 {
            destination[out_idx] = j as u8;
            out_idx += 1;
        } else {
            // Match reference
            let match_len = j.saturating_sub(256).saturating_add(THRESHOLD);
            let mut p = pt_table[(stream.bit_buf.wrapping_shr(8) & 0xFF) as usize] as usize;
            if p >= NPT {
                let mut mask = 1_u32 << 7;
                while p >= NPT {
                    if (stream.bit_buf & mask) != 0 {
                        p = right.get(p).copied().unwrap_or(0) as usize;
                    } else {
                        p = left.get(p).copied().unwrap_or(0) as usize;
                    }
                    mask >>= 1;
                }
            }
            stream.fill_buf(pt_len.get(p).copied().unwrap_or(0) as u16);

            let offset = if p == 0 {
                0
            } else {
                let bits = (p as u16).saturating_sub(1);
                (1_usize << bits).saturating_add(stream.get_bits(bits) as usize)
            };

            let dist = offset.saturating_add(1);
            if dist > out_idx {
                return Err(EfiStatus::INVALID_PARAMETER);
            }
            for _ in 0..match_len {
                if out_idx >= orig_size {
                    break;
                }
                let byte = destination[out_idx - dist];
                destination[out_idx] = byte;
                out_idx += 1;
            }
        }
    }

    Ok(())
}

fn make_table(
    n_char: usize,
    bit_len: &[u8],
    table_bits: usize,
    table: &mut [u16],
    left: &mut [u16],
    right: &mut [u16],
) -> Result<(), EfiStatus> {
    let mut count = [0_u16; 17];
    let mut weight = [0_u16; 17];
    let mut start = [0_u16; 17];

    for &len in bit_len.iter().take(n_char) {
        if len > 16 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        count[usize::from(len)] = count[usize::from(len)].saturating_add(1);
    }

    start[1] = 0;
    for i in 1..16 {
        start[i + 1] = start[i].wrapping_add(count[i].wrapping_shl(16 - i as u32));
    }

    let k = 1_usize << table_bits;
    for (i, _) in count.iter().enumerate().skip(1).take(table_bits) {
        weight[i] = 1_u16 << (table_bits - i);
    }
    for i in (table_bits + 1)..=16 {
        weight[i] = 1_u16 << (16 - i);
    }

    let mut avail = n_char as u16;
    for (i, &len_u8) in bit_len.iter().enumerate().take(n_char) {
        let len = usize::from(len_u8);
        if len == 0 {
            continue;
        }
        let next_code = start[len].wrapping_add(weight[len]);
        if len <= table_bits {
            let s = usize::from(start[len]);
            let e = usize::from(next_code);
            if s > k || e > k {
                return Err(EfiStatus::INVALID_PARAMETER);
            }
            for slot in table[s..e].iter_mut() {
                *slot = i as u16;
            }
        } else {
            let p = usize::from(start[len] >> (16 - table_bits));
            let mut node = table.get(p).copied().unwrap_or(0);
            if node == 0 {
                node = avail;
                avail = avail.saturating_add(1);
                if p < table.len() {
                    table[p] = node;
                }
            }
            for bit in (16 - len)..(16 - table_bits) {
                let mask = 1_u16 << bit;
                if (start[len] & mask) != 0 {
                    if right.get(usize::from(node)).copied().unwrap_or(0) == 0 {
                        right[usize::from(node)] = avail;
                        node = avail;
                        avail = avail.saturating_add(1);
                    } else {
                        node = right[usize::from(node)];
                    }
                } else if left.get(usize::from(node)).copied().unwrap_or(0) == 0 {
                    left[usize::from(node)] = avail;
                    node = avail;
                    avail = avail.saturating_add(1);
                } else {
                    node = left[usize::from(node)];
                }
            }
        }
        start[len] = next_code;
    }

    Ok(())
}
