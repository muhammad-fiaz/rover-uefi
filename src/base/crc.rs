//! # Cyclic redundancy checks
//!
//! IEEE 802.3 CRC-32 used by firmware volumes, `GUID` partition tables,
//! and firmware table headers.
//!
//! The implementation uses the reflected polynomial `0xEDB8_8320`, an
//! initial value of `0xFFFF_FFFF`, and a final XOR of `0xFFFF_FFFF`.
//! This is the same parameter set that firmware services and the
//! `UEFI` specification use for `CalculateCrc32` and for the checksum
//! stored in [`crate::tables::header::EfiTableHeader`].
//!
//! The routines are allocation-free, operate on little-endian byte
//! streams, and never index outside the supplied slice.

/// The reflected CRC-32 polynomial.
pub const CRC32_POLYNOMIAL: u32 = 0xEDB8_8320;

/// Computes the CRC-32 of `data`.
///
/// This is the value returned by the firmware `CalculateCrc32` service
/// and the value stored in firmware table headers and `GUID` partition
/// tables.
#[must_use]
pub fn crc32(data: &[u8]) -> u32 {
    crc32_update(0xFFFF_FFFF, data) ^ 0xFFFF_FFFF
}

/// Continues a CRC-32 computation from a saved intermediate state.
///
/// `state` is the value produced by a previous [`crc32_update`] call
/// (or `0xFFFF_FFFF` to begin a new stream). The final XOR is applied
/// by the caller when the whole stream has been consumed.
#[must_use]
pub fn crc32_update(mut state: u32, data: &[u8]) -> u32 {
    for &byte in data {
        state ^= u32::from(byte);
        for _ in 0..8 {
            if state & 1 != 0 {
                state = (state >> 1) ^ CRC32_POLYNOMIAL;
            } else {
                state >>= 1;
            }
        }
    }
    state
}

/// Computes the CRC-32 of `data` with a table-based inner loop.
///
/// The result is identical to [`crc32`]; the table avoids the
/// per-bit branch on platforms where the constant table has already
/// been generated.
#[must_use]
pub fn crc32_table(data: &[u8]) -> u32 {
    let mut state = 0xFFFF_FFFF_u32;
    for &byte in data {
        let low = (state ^ u32::from(byte)).to_le_bytes()[0];
        let index = usize::from(low);
        state = (state >> 8) ^ CRC32_TABLE[index];
    }
    state ^ 0xFFFF_FFFF
}

const CRC32_TABLE: [u32; 256] = {
    let mut table = [0_u32; 256];
    let mut index = 0_usize;
    while index < 256 {
        let mut value = index.to_le_bytes()[0] as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 != 0 {
                (value >> 1) ^ CRC32_POLYNOMIAL
            } else {
                value >> 1
            };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
};

/// Returns whether the sum of every byte in `data`, modulo 256, is zero.
///
/// This is the checksum rule used by `ACPI` tables and by the `SMBIOS`
/// entry point.
#[must_use]
pub fn byte_sum_checksum_ok(data: &[u8]) -> bool {
    data.iter()
        .fold(0_u8, |accumulator, byte| accumulator.wrapping_add(*byte))
        == 0
}

/// Computes the value that must be stored so the byte sum is zero.
///
/// Pass the buffer with the checksum field set to zero; the result is
/// the value to write into the checksum field.
#[must_use]
pub fn byte_sum_checksum(data: &[u8]) -> u8 {
    0_u8.wrapping_sub(
        data.iter()
            .fold(0_u8, |accumulator, byte| accumulator.wrapping_add(*byte)),
    )
}

#[cfg(test)]
mod tests {
    use super::{byte_sum_checksum, byte_sum_checksum_ok, crc32, crc32_table, crc32_update};

    #[test]
    fn crc32_known_answer() {
        // The standard check value for CRC-32/ISO-HDLC.
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32_table(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn crc32_empty_is_zero() {
        assert_eq!(crc32(b""), 0);
        assert_eq!(crc32_table(b""), 0);
    }

    #[test]
    fn crc32_streaming_matches_single_shot() {
        let state = crc32_update(0xFFFF_FFFF, b"1234");
        let state = crc32_update(state, b"56789");
        assert_eq!(state ^ 0xFFFF_FFFF, crc32(b"123456789"));
    }

    #[test]
    fn byte_sum_round_trip() {
        let mut buffer = [0_u8; 8];
        buffer[..7].copy_from_slice(b"rover!!");
        let checksum = byte_sum_checksum(&buffer);
        buffer[7] = checksum;
        assert!(byte_sum_checksum_ok(&buffer));
    }
}
