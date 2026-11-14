//! # UEFI & Tiano Decompression and Compression Protocols and Algorithms
//!
//! Defined by the UEFI Specification Appendix A: Decompression Algorithm,
//! `EFI_DECOMPRESS_PROTOCOL`, PI Specification Volume 1: `EFI_PEI_DECOMPRESS_PPI`,
//! and custom firmware compression/decompression standards.
//!
//! Provides zero-allocation, `#![no_std]` native Huffman and LZ77 compression
//! and decompression services for firmware volumes, sections, and images.

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

/// The `EFI_DECOMPRESS_PROTOCOL_GUID`: `{d811751a-94a6-11d4-9a3a-0090273fc14d}`.
pub const EFI_DECOMPRESS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD811_751A,
    0x94A6,
    0x11D4,
    [0x9A, 0x3A, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
);

/// Tiano Custom Decompress Algorithm GUID: `{a31280ad-481e-41b6-95e8-127f4c984779}`.
pub const TIANO_CUSTOM_DECOMPRESS_GUID: EfiGuid = EfiGuid::new(
    0xA312_80AD,
    0x481E,
    0x41B6,
    [0x95, 0xE8, 0x12, 0x7F, 0x4C, 0x98, 0x47, 0x79],
);

/// LZMA Custom Decompress Algorithm GUID: `{ee4e5898-3914-4259-9d6e-dc7bd79403cf}`.
pub const LZMA_CUSTOM_DECOMPRESS_GUID: EfiGuid = EfiGuid::new(
    0xEE4E_5898,
    0x3914,
    0x4259,
    [0x9D, 0x6E, 0xDC, 0x7B, 0xD7, 0x94, 0x03, 0xCF],
);

/// LZMA with x86 code converter Custom Decompress GUID: `{d42ae6bd-1352-4bfb-909a-ca72a6eae889}`.
pub const LZMAF86_CUSTOM_DECOMPRESS_GUID: EfiGuid = EfiGuid::new(
    0xD42A_E6BD,
    0x1352,
    0x4BFB,
    [0x90, 0x9A, 0xCA, 0x72, 0xA6, 0xEA, 0xE8, 0x89],
);

/// Brotli Custom Decompress Algorithm GUID: `{3d532050-5cda-4fd0-879e-0f7f630d5afb}`.
pub const BROTLI_CUSTOM_DECOMPRESS_GUID: EfiGuid = EfiGuid::new(
    0x3D53_2050,
    0x5CDA,
    0x4FD0,
    [0x87, 0x9E, 0x0F, 0x7F, 0x63, 0x0D, 0x5A, 0xFB],
);

pub use crate::protocols::firmware::pei::{EFI_PEI_DECOMPRESS_PPI_GUID, EfiPeiDecompressPpi};

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

/// Function pointer for `EFI_PEI_DECOMPRESS_PPI.Decompress`.
pub type EfiPeiDecompressDecompress = unsafe extern "efiapi" fn(
    this: *const EfiPeiDecompressPpi,
    input_section: *const c_void,
    output_buffer: *mut *mut c_void,
    output_size: *mut usize,
) -> EfiStatus;

/// Scratch buffer size required for standard UEFI and Tiano decompression (in bytes).
pub const UEFI_DECOMPRESS_SCRATCH_SIZE: usize = 65_536;
/// Scratch buffer size required for Tiano decompression.
pub const TIANO_DECOMPRESS_SCRATCH_SIZE: usize = 65_536;

/// Parsed metadata for a compressed UEFI/Tiano stream.
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

// Internal decompressor constants conforming to UEFI Spec Appendix A.
const MAXMATCH: usize = 256;
const THRESHOLD: usize = 3;
const CODE_BIT: usize = 16;
const NC: usize = 0xFF + MAXMATCH + 2 - THRESHOLD; // 510
const CBIT: usize = 9;
const MAXPBIT: usize = 5;
const TBIT: usize = 5;
const MAXNP: usize = (1 << MAXPBIT) - 1; // 31
const NT: usize = CODE_BIT + 3; // 19
const NPT: usize = if NT > MAXNP { NT } else { MAXNP }; // 31

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
        let x = self.bit_buf.wrapping_shr(u32::from(32_u16.wrapping_sub(n)));
        self.fill_buf(n);
        x
    }
}

/// Core decompression engine supporting both UEFI (`p_bit = 4`) and Tiano (`p_bit = 5`).
fn decompress_core(
    source: &[u8],
    destination: &mut [u8],
    scratch: &mut [u8],
    p_bit: u8,
) -> Result<(), EfiStatus> {
    if scratch.len() < UEFI_DECOMPRESS_SCRATCH_SIZE {
        return Err(EfiStatus::BUFFER_TOO_SMALL);
    }
    let info = get_decompress_info(source)?;
    if info.uncompressed_size == 0 {
        return Ok(());
    }
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
    stream.fill_buf(32);

    let mut out_idx = 0_usize;
    let orig_size =
        usize::try_from(info.uncompressed_size).map_err(|_| EfiStatus::INVALID_PARAMETER)?;

    let mut left = [0_u16; 2 * NC - 1];
    let mut right = [0_u16; 2 * NC - 1];
    let mut c_len = [0_u8; NC];
    let mut pt_len = [0_u8; NPT];
    let mut c_table = [0_u16; 4096];
    let mut pt_table = [0_u16; 256];
    let mut block_size = 0_u16;

    while out_idx < orig_size {
        if block_size == 0 {
            block_size = stream.get_bits(16) as u16;
            // 1. Read Extra Set (T table)
            read_pt_len(
                &mut stream,
                NT,
                TBIT as u16,
                3,
                &mut pt_len,
                &mut pt_table,
                &mut left,
                &mut right,
            )?;
            // 2. Read Char&Len Set (C table)
            read_c_len(
                &mut stream,
                &mut c_len,
                &mut c_table,
                &pt_table,
                &pt_len,
                &mut left,
                &mut right,
            )?;
            // 3. Read Position Set (P table)
            read_pt_len(
                &mut stream,
                MAXNP,
                u16::from(p_bit),
                usize::MAX,
                &mut pt_len,
                &mut pt_table,
                &mut left,
                &mut right,
            )?;
        }

        block_size = block_size.saturating_sub(1);

        // Decode character/match length
        let mut j = c_table[(stream.bit_buf.wrapping_shr(20) & 0x0FFF) as usize] as usize;
        if j >= NC {
            let mut mask = 1_u32 << 19;
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
            // Match pointer
            let match_len = j.saturating_sub(256).saturating_add(THRESHOLD);
            let mut p = pt_table[(stream.bit_buf.wrapping_shr(24) & 0xFF) as usize] as usize;
            if p >= MAXNP {
                let mut mask = 1_u32 << 23;
                while p >= MAXNP {
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
            } else if p == 1 {
                1
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

#[allow(clippy::too_many_arguments)]
fn read_pt_len(
    stream: &mut BitStream<'_>,
    nn: usize,
    nbit: u16,
    special: usize,
    pt_len: &mut [u8],
    pt_table: &mut [u16],
    left: &mut [u16],
    right: &mut [u16],
) -> Result<(), EfiStatus> {
    let number = stream.get_bits(nbit) as usize;
    if number > NPT {
        return Err(EfiStatus::INVALID_PARAMETER);
    }
    if number == 0 {
        let c = stream.get_bits(nbit) as u16;
        pt_len[..nn].fill(0);
        pt_table.fill(c);
        return Ok(());
    }

    let mut i = 0_usize;
    while i < number && i < NPT {
        let mut c = stream.bit_buf.wrapping_shr(29) as u16;
        if c == 7 {
            let mut mask = 1_u32 << 28;
            while mask != 0 && (stream.bit_buf & mask) != 0 {
                mask >>= 1;
                c += 1;
            }
        }
        stream.fill_buf(if c < 7 { 3 } else { c - 3 });
        pt_len[i] = c as u8;
        i += 1;

        if i == special {
            let mut count = stream.get_bits(2) as usize;
            while count > 0 && i < NPT {
                pt_len[i] = 0;
                i += 1;
                count -= 1;
            }
        }
    }
    while i < NPT {
        pt_len[i] = 0;
        i += 1;
    }
    make_table(NPT, pt_len, 8, pt_table, left, right)
}

fn read_c_len(
    stream: &mut BitStream<'_>,
    c_len: &mut [u8],
    c_table: &mut [u16],
    pt_table: &[u16],
    pt_len: &[u8],
    left: &mut [u16],
    right: &mut [u16],
) -> Result<(), EfiStatus> {
    let number = stream.get_bits(CBIT as u16) as usize;
    if number > NC {
        return Err(EfiStatus::INVALID_PARAMETER);
    }
    if number == 0 {
        let c = stream.get_bits(CBIT as u16) as u16;
        c_len.fill(0);
        c_table.fill(c);
        return Ok(());
    }

    let mut i = 0_usize;
    while i < number && i < NC {
        let mut j = pt_table[(stream.bit_buf.wrapping_shr(24) & 0xFF) as usize] as usize;
        if j >= NT {
            let mut mask = 1_u32 << 23;
            while j >= NT {
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
                if i < NC {
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
    make_table(NC, c_len, 12, c_table, left, right)
}

/// Decompresses standard UEFI compressed data from `source` into `destination`.
///
/// # Errors
/// Returns `EfiStatus::INVALID_PARAMETER` if data is corrupted or buffers are too small.
pub fn decompress_uefi(
    source: &[u8],
    destination: &mut [u8],
    scratch: &mut [u8],
) -> Result<(), EfiStatus> {
    decompress_core(source, destination, scratch, 4)
}

/// Decompresses Tiano compressed data from `source` into `destination`.
///
/// # Errors
/// Returns `EfiStatus::INVALID_PARAMETER` if data is corrupted or buffers are too small.
pub fn decompress_tiano(
    source: &[u8],
    destination: &mut [u8],
    scratch: &mut [u8],
) -> Result<(), EfiStatus> {
    decompress_core(source, destination, scratch, 5)
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
    let mut start = [0_u16; 18];

    for &len in bit_len.iter().take(n_char) {
        if len > 16 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        count[usize::from(len)] = count[usize::from(len)].saturating_add(1);
    }

    start[1] = 0;
    for i in 1..=16 {
        start[i + 1] = start[i].wrapping_add(count[i].wrapping_shl(16 - i as u32));
    }

    if start[17] != 0 {
        return Err(EfiStatus::INVALID_PARAMETER);
    }

    let ju_bits = 16_usize.saturating_sub(table_bits);

    for i in 1..=table_bits {
        start[i] >>= ju_bits;
        weight[i] = 1_u16 << (table_bits - i);
    }
    for i in (table_bits + 1)..=16 {
        weight[i] = 1_u16 << (16 - i);
    }

    let idx = usize::from(start[table_bits + 1] >> ju_bits);
    let max_len = 1_usize << table_bits;
    let t_len = table.len();
    if idx != 0 && idx < max_len && idx < t_len {
        table[idx..max_len.min(t_len)].fill(0);
    }

    let mut avail = n_char as u16;
    let mask = 1_u16 << (15_usize.saturating_sub(table_bits));

    for (char_idx, &len_u8) in bit_len.iter().enumerate().take(n_char) {
        let len = usize::from(len_u8);
        if len == 0 || len >= 17 {
            continue;
        }

        let next_code = start[len].wrapping_add(weight[len]);

        if len <= table_bits {
            let s = usize::from(start[len]);
            let e = usize::from(next_code);
            if s >= e || e > max_len || e > table.len() {
                return Err(EfiStatus::INVALID_PARAMETER);
            }
            for slot in table[s..e].iter_mut() {
                *slot = char_idx as u16;
            }
        } else {
            let mut index3 = start[len];
            let entry = usize::from(index3 >> ju_bits);
            if entry >= table.len() {
                return Err(EfiStatus::INVALID_PARAMETER);
            }

            let mut node = table[entry];
            let mut remaining = len - table_bits;

            let mut parent_is_table = true;
            let mut parent_idx = entry;
            let mut is_right = false;

            while remaining > 0 {
                if node == 0 && usize::from(avail) < (2 * NC - 1) {
                    left[usize::from(avail)] = 0;
                    right[usize::from(avail)] = 0;
                    node = avail;
                    avail += 1;
                    if parent_is_table {
                        table[parent_idx] = node;
                    } else if is_right {
                        right[parent_idx] = node;
                    } else {
                        left[parent_idx] = node;
                    }
                }

                parent_is_table = false;
                parent_idx = usize::from(node);

                if parent_idx < (2 * NC - 1) {
                    if (index3 & mask) != 0 {
                        is_right = true;
                        node = right[parent_idx];
                    } else {
                        is_right = false;
                        node = left[parent_idx];
                    }
                }

                index3 <<= 1;
                remaining -= 1;
            }

            if parent_is_table {
                table[parent_idx] = char_idx as u16;
            } else if is_right {
                right[parent_idx] = char_idx as u16;
            } else {
                left[parent_idx] = char_idx as u16;
            }
        }

        start[len] = next_code;
    }

    Ok(())
}

// --- Compressor Implementation ---

struct OutBitStream<'a> {
    dst: &'a mut [u8],
    out_idx: usize,
    bit_buf: u64,
    bit_count: usize,
}

impl<'a> OutBitStream<'a> {
    fn new(dst: &'a mut [u8]) -> Self {
        Self {
            dst,
            out_idx: 0,
            bit_buf: 0,
            bit_count: 0,
        }
    }

    fn put_bits(&mut self, n: usize, x: u32) -> Result<(), EfiStatus> {
        if n == 0 {
            return Ok(());
        }
        let mask = if n >= 32 {
            u64::from(u32::MAX)
        } else {
            (1_u64 << n) - 1
        };
        let val = (u64::from(x) & mask) << (64 - self.bit_count - n);
        self.bit_buf |= val;
        self.bit_count += n;

        while self.bit_count >= 8 {
            if self.out_idx >= self.dst.len() {
                return Err(EfiStatus::BUFFER_TOO_SMALL);
            }
            let byte = (self.bit_buf >> 56) as u8;
            self.dst[self.out_idx] = byte;
            self.out_idx += 1;
            self.bit_buf <<= 8;
            self.bit_count -= 8;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), EfiStatus> {
        if self.bit_count > 0 {
            if self.out_idx >= self.dst.len() {
                return Err(EfiStatus::BUFFER_TOO_SMALL);
            }
            let byte = (self.bit_buf >> 56) as u8;
            self.dst[self.out_idx] = byte;
            self.out_idx += 1;
            self.bit_buf = 0;
            self.bit_count = 0;
        }
        // Null-terminate compressed buffer so decompressor's lookahead does not read out of bounds
        if self.out_idx < self.dst.len() {
            self.dst[self.out_idx] = 0;
            self.out_idx += 1;
        }
        Ok(())
    }
}

fn down_heap(mut i: usize, heap_size: usize, heap: &mut [usize; 1024], freq: &[u16]) {
    let k = heap[i];
    while 2 * i <= heap_size {
        let mut j = 2 * i;
        if j < heap_size && freq[heap[j]] > freq[heap[j + 1]] {
            j += 1;
        }
        if freq[k] <= freq[heap[j]] {
            break;
        }
        heap[i] = heap[j];
        i = j;
    }
    heap[i] = k;
}

fn make_huffman_tree(
    n: usize,
    freq: &mut [u16],
    len: &mut [u8],
    code: &mut [u16],
    left: &mut [u16],
    right: &mut [u16],
) -> usize {
    len[..n].fill(0);
    code[..n].fill(0);
    let mut heap = [0_usize; 1024];
    let mut heap_size = 0_usize;

    for i in 0..n {
        if freq[i] > 0 {
            heap_size += 1;
            heap[heap_size] = i;
        }
    }
    if heap_size == 0 {
        return 0;
    }
    if heap_size == 1 {
        code[heap[1]] = 0;
        return heap[1];
    }

    for i in (1..=heap_size / 2).rev() {
        down_heap(i, heap_size, &mut heap, freq);
    }

    let mut sort_order = [0_usize; 1024];
    let mut sort_count = 0_usize;

    let mut avail = n;
    while heap_size > 1 {
        let i = heap[1];
        if i < n && sort_count < sort_order.len() {
            sort_order[sort_count] = i;
            sort_count += 1;
        }
        heap[1] = heap[heap_size];
        heap_size -= 1;
        down_heap(1, heap_size, &mut heap, freq);

        let j = heap[1];
        if j < n && sort_count < sort_order.len() {
            sort_order[sort_count] = j;
            sort_count += 1;
        }
        let k = avail;
        avail += 1;
        freq[k] = freq[i].saturating_add(freq[j]);
        heap[1] = k;
        down_heap(1, heap_size, &mut heap, freq);

        left[k] = i as u16;
        right[k] = j as u16;
    }

    let root = heap[1];
    let mut len_cnt = [0_u16; 17];
    let mut stack = [(root, 0_u8); 1024];
    let mut sp = 1_usize;
    stack[0] = (root, 0);

    while sp > 0 {
        sp -= 1;
        let (node, depth) = stack[sp];
        if node < n {
            let clamped = depth.clamp(1, 16);
            len_cnt[usize::from(clamped)] += 1;
        } else {
            let next_depth = depth.saturating_add(1);
            let r = usize::from(right[node]);
            let l = usize::from(left[node]);
            if sp + 2 < stack.len() {
                stack[sp] = (r, next_depth);
                sp += 1;
                stack[sp] = (l, next_depth);
                sp += 1;
            }
        }
    }

    // Kraft adjustment
    let mut cum: u32 = 0;
    for i in (1..=16).rev() {
        cum = cum.wrapping_add(u32::from(len_cnt[i]) << (16 - i));
    }
    while cum > (1 << 16) {
        len_cnt[16] = len_cnt[16].saturating_sub(1);
        for i in (1..=15).rev() {
            if len_cnt[i] != 0 {
                len_cnt[i] -= 1;
                len_cnt[i + 1] += 2;
                break;
            }
        }
        cum = cum.saturating_sub(1);
    }

    let mut sort_idx = 0_usize;
    for i in (1..=16).rev() {
        let mut k = len_cnt[i];
        while k > 0 && sort_idx < sort_count {
            len[sort_order[sort_idx]] = i as u8;
            sort_idx += 1;
            k -= 1;
        }
    }

    let mut start = [0_u16; 18];
    start[1] = 0;
    for i in 1..=16 {
        start[i + 1] = (start[i].wrapping_add(len_cnt[i])) << 1;
    }
    for i in 0..n {
        let l = usize::from(len[i]);
        if l > 0 && l <= 16 {
            code[i] = start[l];
            start[l] = start[l].wrapping_add(1);
        }
    }

    root
}

fn count_t_freq(c_len: &[u8], t_freq: &mut [u16]) {
    t_freq.fill(0);
    let mut n = NC;
    while n > 0 && c_len[n - 1] == 0 {
        n -= 1;
    }
    let mut i = 0_usize;
    while i < n {
        let k = c_len[i];
        i += 1;
        if k == 0 {
            let mut count = 1_usize;
            while i < n && c_len[i] == 0 {
                i += 1;
                count += 1;
            }
            if count <= 2 {
                t_freq[0] = t_freq[0].saturating_add(count as u16);
            } else if count <= 18 {
                t_freq[1] = t_freq[1].saturating_add(1);
            } else if count == 19 {
                t_freq[0] = t_freq[0].saturating_add(1);
                t_freq[1] = t_freq[1].saturating_add(1);
            } else {
                t_freq[2] = t_freq[2].saturating_add(1);
            }
        } else {
            let idx = usize::from(k) + 2;
            if idx < t_freq.len() {
                t_freq[idx] = t_freq[idx].saturating_add(1);
            }
        }
    }
}

fn write_pt_len(
    stream: &mut OutBitStream<'_>,
    mut n: usize,
    nbit: usize,
    special: usize,
    pt_len: &[u8],
) -> Result<(), EfiStatus> {
    while n > 0 && pt_len[n - 1] == 0 {
        n -= 1;
    }
    stream.put_bits(nbit, n as u32)?;
    let mut i = 0_usize;
    while i < n {
        let k = usize::from(pt_len[i]);
        i += 1;
        if k <= 6 {
            stream.put_bits(3, k as u32)?;
        } else {
            stream.put_bits(k - 3, (1_u32 << (k - 3)).wrapping_sub(2))?;
        }
        if i == special {
            while i < 6 && i < pt_len.len() && pt_len[i] == 0 {
                i += 1;
            }
            stream.put_bits(2, ((i - 3) & 3) as u32)?;
        }
    }
    Ok(())
}

fn write_c_len(
    stream: &mut OutBitStream<'_>,
    c_len: &[u8],
    pt_len: &[u8],
    pt_code: &[u16],
) -> Result<(), EfiStatus> {
    let mut n = NC;
    while n > 0 && c_len[n - 1] == 0 {
        n -= 1;
    }
    stream.put_bits(CBIT, n as u32)?;
    let mut i = 0_usize;
    while i < n {
        let k = usize::from(c_len[i]);
        i += 1;
        if k == 0 {
            let mut count = 1_usize;
            while i < n && c_len[i] == 0 {
                i += 1;
                count += 1;
            }
            if count <= 2 {
                for _ in 0..count {
                    stream.put_bits(usize::from(pt_len[0]), u32::from(pt_code[0]))?;
                }
            } else if count <= 18 {
                stream.put_bits(usize::from(pt_len[1]), u32::from(pt_code[1]))?;
                stream.put_bits(4, (count - 3) as u32)?;
            } else if count == 19 {
                stream.put_bits(usize::from(pt_len[0]), u32::from(pt_code[0]))?;
                stream.put_bits(usize::from(pt_len[1]), u32::from(pt_code[1]))?;
                stream.put_bits(4, 15)?;
            } else {
                stream.put_bits(usize::from(pt_len[2]), u32::from(pt_code[2]))?;
                stream.put_bits(CBIT, (count - 20) as u32)?;
            }
        } else {
            let idx = k + 2;
            stream.put_bits(usize::from(pt_len[idx]), u32::from(pt_code[idx]))?;
        }
    }
    Ok(())
}

/// Core compression routine supporting both standard UEFI (`p_bit = 4`, 8KB window)
/// and Tiano (`p_bit = 5`, 64KB window).
fn compress_core(
    source: &[u8],
    destination: &mut [u8],
    p_bit: u8,
    window_size: usize,
) -> Result<usize, EfiStatus> {
    if destination.len() < 8 {
        return Err(EfiStatus::BUFFER_TOO_SMALL);
    }
    if source.is_empty() {
        destination[0..8].fill(0);
        return Ok(8);
    }

    let (header_slice, payload_slice) = destination.split_at_mut(8);
    let mut stream = OutBitStream::new(payload_slice);

    let mut cfreq = [0_u16; 2 * NC - 1];
    let mut clen = [0_u8; NC];
    let mut ccode = [0_u16; NC];

    let mut pfreq = [0_u16; 2 * MAXNP - 1];
    let mut plen = [0_u8; NPT];
    let mut pcode = [0_u16; NPT];

    let mut tfreq = [0_u16; 2 * NT - 1];
    let mut tlen = [0_u8; NPT];
    let mut tcode = [0_u16; NPT];

    let mut left = [0_u16; 2 * NC - 1];
    let mut right = [0_u16; 2 * NC - 1];

    let mut symbols = [0_u16; 4096];
    let mut positions = [0_u32; 4096];

    let mut src_idx = 0_usize;
    let total_len = source.len();

    while src_idx < total_len {
        let block_start = src_idx;
        let mut sym_count = 0_usize;

        while src_idx < total_len && sym_count < 4096 {
            // Find longest match in sliding window
            let win_start = block_start.saturating_sub(window_size);
            let mut best_len = 0_usize;
            let mut best_dist = 0_usize;

            if src_idx > win_start {
                let max_possible = (total_len - src_idx).min(MAXMATCH);
                if max_possible >= THRESHOLD {
                    let mut cand = src_idx.saturating_sub(1);
                    while cand >= win_start {
                        let mut m = 0_usize;
                        while m < max_possible && source[cand + m] == source[src_idx + m] {
                            m += 1;
                        }
                        if m > best_len {
                            best_len = m;
                            best_dist = src_idx - cand;
                            if best_len == max_possible {
                                break;
                            }
                        }
                        if cand == win_start {
                            break;
                        }
                        cand -= 1;
                    }
                }
            }

            if best_len >= THRESHOLD {
                symbols[sym_count] = (best_len - THRESHOLD + 256) as u16;
                positions[sym_count] = (best_dist - 1) as u32;
                sym_count += 1;
                src_idx += best_len;
            } else {
                symbols[sym_count] = u16::from(source[src_idx]);
                positions[sym_count] = 0;
                sym_count += 1;
                src_idx += 1;
            }
        }

        // Count frequencies for this block
        cfreq.fill(0);
        pfreq.fill(0);
        for i in 0..sym_count {
            let s = symbols[i];
            cfreq[usize::from(s)] = cfreq[usize::from(s)].saturating_add(1);
            if s >= 256 {
                let p = positions[i];
                let bits = if p == 0 {
                    0
                } else {
                    32 - p.leading_zeros() as usize
                };
                pfreq[bits] = pfreq[bits].saturating_add(1);
            }
        }

        // Build Char&Len tree
        let root_c =
            make_huffman_tree(NC, &mut cfreq, &mut clen, &mut ccode, &mut left, &mut right);
        stream.put_bits(16, sym_count as u32)?;

        if root_c >= NC {
            count_t_freq(&clen, &mut tfreq);
            let root_t =
                make_huffman_tree(NT, &mut tfreq, &mut tlen, &mut tcode, &mut left, &mut right);
            if root_t >= NT {
                write_pt_len(&mut stream, NT, TBIT, 3, &tlen)?;
            } else {
                stream.put_bits(TBIT, 0)?;
                stream.put_bits(TBIT, root_t as u32)?;
            }
            write_c_len(&mut stream, &clen, &tlen, &tcode)?;
        } else {
            stream.put_bits(TBIT, 0)?;
            stream.put_bits(TBIT, 0)?;
            stream.put_bits(CBIT, 0)?;
            stream.put_bits(CBIT, root_c as u32)?;
        }

        // Build Position tree
        let np = (1_usize << p_bit) - 1;
        let root_p =
            make_huffman_tree(np, &mut pfreq, &mut plen, &mut pcode, &mut left, &mut right);
        if root_p >= np {
            write_pt_len(&mut stream, np, usize::from(p_bit), usize::MAX, &plen)?;
        } else {
            stream.put_bits(usize::from(p_bit), 0)?;
            stream.put_bits(usize::from(p_bit), root_p as u32)?;
        }

        // Encode symbols
        for i in 0..sym_count {
            let s = symbols[i];
            stream.put_bits(
                usize::from(clen[usize::from(s)]),
                u32::from(ccode[usize::from(s)]),
            )?;
            if s >= 256 {
                let p = positions[i];
                let bits = if p == 0 {
                    0
                } else {
                    32 - p.leading_zeros() as usize
                };
                stream.put_bits(usize::from(plen[bits]), u32::from(pcode[bits]))?;
                if bits > 1 {
                    stream.put_bits(bits - 1, p & ((1 << (bits - 1)) - 1))?;
                }
            }
        }
    }

    stream.flush()?;

    let comp_size = stream.out_idx as u32;
    let uncomp_size = u32::try_from(source.len()).map_err(|_| EfiStatus::INVALID_PARAMETER)?;

    header_slice[0..4].copy_from_slice(&comp_size.to_le_bytes());
    header_slice[4..8].copy_from_slice(&uncomp_size.to_le_bytes());

    Ok(8 + stream.out_idx)
}

/// Compresses source data using the standard UEFI compression algorithm.
///
/// # Errors
/// Returns `EfiStatus::BUFFER_TOO_SMALL` if `destination` cannot hold the compressed output.
pub fn compress_uefi(source: &[u8], destination: &mut [u8]) -> Result<usize, EfiStatus> {
    compress_core(source, destination, 4, 8192)
}

/// Compresses source data using the Tiano compression algorithm.
///
/// # Errors
/// Returns `EfiStatus::BUFFER_TOO_SMALL` if `destination` cannot hold the compressed output.
pub fn compress_tiano(source: &[u8], destination: &mut [u8]) -> Result<usize, EfiStatus> {
    compress_core(source, destination, 5, 65_536)
}

/// Safe client wrapper around [`EfiDecompressProtocol`].
#[derive(Debug)]
pub struct Decompress<'a> {
    protocol: &'a EfiDecompressProtocol,
}

impl<'a> Decompress<'a> {
    /// Creates a borrowed wrapper around the protocol interface.
    #[must_use]
    pub const fn new(protocol: &'a EfiDecompressProtocol) -> Self {
        Self { protocol }
    }

    /// Queries uncompressed and scratch size requirements.
    ///
    /// # Errors
    /// Returns `EfiStatus` if header query fails.
    pub fn get_info(&self, source: &[u8]) -> Result<(u32, u32), EfiStatus> {
        let mut dest_size = 0_u32;
        let mut scratch_size = 0_u32;
        let status = unsafe {
            (self.protocol.get_info)(
                self.protocol,
                source.as_ptr().cast::<c_void>(),
                source.len() as u32,
                &raw mut dest_size,
                &raw mut scratch_size,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((dest_size, scratch_size))
        } else {
            Err(status)
        }
    }

    /// Decompresses data using protocol callbacks.
    ///
    /// # Errors
    /// Returns `EfiStatus` if decompression fails.
    pub fn decompress(
        &self,
        source: &[u8],
        destination: &mut [u8],
        scratch: &mut [u8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.protocol.decompress)(
                self.protocol,
                source.as_ptr().cast::<c_void>(),
                source.len() as u32,
                destination.as_mut_ptr().cast::<c_void>(),
                destination.len() as u32,
                scratch.as_mut_ptr().cast::<c_void>(),
                scratch.len() as u32,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around [`EfiPeiDecompressPpi`].
#[derive(Debug)]
pub struct PeiDecompress<'a> {
    ppi: &'a EfiPeiDecompressPpi,
}

impl<'a> PeiDecompress<'a> {
    /// Creates a borrowed wrapper around the PEI decompress PPI.
    #[must_use]
    pub const fn new(ppi: &'a EfiPeiDecompressPpi) -> Self {
        Self { ppi }
    }

    /// Decompresses data using the PEI decompress PPI.
    ///
    /// # Errors
    /// Returns `EfiStatus` if decompression fails.
    pub fn decompress(&self, source: &[u8], destination: &mut [u8]) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.ppi.decompress)(
                self.ppi,
                source.as_ptr().cast::<c_void>(),
                source.len() as u32,
                destination.as_mut_ptr().cast::<c_void>(),
                destination.len() as u32,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
