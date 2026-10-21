//! # Bounded byte readers
//!
//! Checked little-endian readers shared by every binary-format parser in
//! the crate. Each reader returns `None` when the requested field would
//! extend past the end of the slice, so callers never index out of bounds
//! and never panic on truncated input.

/// Reads a little-endian `u16` at `offset`.
#[must_use]
pub fn read_u16_le(bytes: &[u8], offset: usize) -> Option<u16> {
    let slice = bytes.get(offset..offset.checked_add(2)?)?;
    let mut raw = [0_u8; 2];
    raw.copy_from_slice(slice);
    Some(u16::from_le_bytes(raw))
}

/// Reads a little-endian `u32` at `offset`.
#[must_use]
pub fn read_u32_le(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(slice);
    Some(u32::from_le_bytes(raw))
}

/// Reads a little-endian `u64` at `offset`.
#[must_use]
pub fn read_u64_le(bytes: &[u8], offset: usize) -> Option<u64> {
    let slice = bytes.get(offset..offset.checked_add(8)?)?;
    let mut raw = [0_u8; 8];
    raw.copy_from_slice(slice);
    Some(u64::from_le_bytes(raw))
}

/// Reads a little-endian `i16` at `offset`.
#[must_use]
pub fn read_i16_le(bytes: &[u8], offset: usize) -> Option<i16> {
    let slice = bytes.get(offset..offset.checked_add(2)?)?;
    let mut raw = [0_u8; 2];
    raw.copy_from_slice(slice);
    Some(i16::from_le_bytes(raw))
}

/// Reads a big-endian `u16` at `offset`.
#[must_use]
pub fn read_u16_be(bytes: &[u8], offset: usize) -> Option<u16> {
    let slice = bytes.get(offset..offset.checked_add(2)?)?;
    let mut raw = [0_u8; 2];
    raw.copy_from_slice(slice);
    Some(u16::from_be_bytes(raw))
}

/// Reads a big-endian `u32` at `offset`.
#[must_use]
pub fn read_u32_be(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(slice);
    Some(u32::from_be_bytes(raw))
}

/// Reads a big-endian `u64` at `offset`.
#[must_use]
pub fn read_u64_be(bytes: &[u8], offset: usize) -> Option<u64> {
    let slice = bytes.get(offset..offset.checked_add(8)?)?;
    let mut raw = [0_u8; 8];
    raw.copy_from_slice(slice);
    Some(u64::from_be_bytes(raw))
}

/// Reads a single byte at `offset`.
#[must_use]
pub fn read_u8(bytes: &[u8], offset: usize) -> Option<u8> {
    bytes.get(offset).copied()
}

/// Reads a little-endian `i32` at `offset`.
#[must_use]
pub fn read_i32_le(bytes: &[u8], offset: usize) -> Option<i32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(slice);
    Some(i32::from_le_bytes(raw))
}

/// Reads a little-endian `i64` at `offset`.
#[must_use]
pub fn read_i64_le(bytes: &[u8], offset: usize) -> Option<i64> {
    let slice = bytes.get(offset..offset.checked_add(8)?)?;
    let mut raw = [0_u8; 8];
    raw.copy_from_slice(slice);
    Some(i64::from_le_bytes(raw))
}

/// Reads a big-endian `i16` at `offset`.
#[must_use]
pub fn read_i16_be(bytes: &[u8], offset: usize) -> Option<i16> {
    let slice = bytes.get(offset..offset.checked_add(2)?)?;
    let mut raw = [0_u8; 2];
    raw.copy_from_slice(slice);
    Some(i16::from_be_bytes(raw))
}

/// Reads a big-endian `i32` at `offset`.
#[must_use]
pub fn read_i32_be(bytes: &[u8], offset: usize) -> Option<i32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(slice);
    Some(i32::from_be_bytes(raw))
}

/// Writes a little-endian `u16` at `offset`.
///
/// Returns `false` when the write would extend past the end of `bytes`.
pub fn write_u16_le(bytes: &mut [u8], offset: usize, value: u16) -> bool {
    let Some(slice) = bytes.get_mut(offset..offset.saturating_add(2)) else {
        return false;
    };
    slice.copy_from_slice(&value.to_le_bytes());
    true
}

/// Writes a little-endian `u32` at `offset`.
///
/// Returns `false` when the write would extend past the end of `bytes`.
pub fn write_u32_le(bytes: &mut [u8], offset: usize, value: u32) -> bool {
    let Some(slice) = bytes.get_mut(offset..offset.saturating_add(4)) else {
        return false;
    };
    slice.copy_from_slice(&value.to_le_bytes());
    true
}

/// Writes a little-endian `u64` at `offset`.
///
/// Returns `false` when the write would extend past the end of `bytes`.
pub fn write_u64_le(bytes: &mut [u8], offset: usize, value: u64) -> bool {
    let Some(slice) = bytes.get_mut(offset..offset.saturating_add(8)) else {
        return false;
    };
    slice.copy_from_slice(&value.to_le_bytes());
    true
}

/// A cursor over a byte slice with bounds-checked little-endian reads.
///
/// The cursor never panics on truncated input; every read returns
/// `None` and leaves the position unchanged when the requested field
/// would extend past the end of the slice.
#[derive(Clone, Copy, Debug)]
pub struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    /// Creates a reader positioned at the start of `bytes`.
    #[must_use]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    /// Returns the current position.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns the number of bytes remaining.
    #[must_use]
    pub const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }

    /// Returns whether the reader has reached the end of the input.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    /// Advances the position by `count` bytes.
    ///
    /// Returns `false` and leaves the position unchanged when fewer than
    /// `count` bytes remain.
    pub fn skip(&mut self, count: usize) -> bool {
        match self.position.checked_add(count) {
            Some(next) if next <= self.bytes.len() => {
                self.position = next;
                true
            }
            _ => false,
        }
    }

    /// Returns the next `count` bytes and advances past them.
    pub fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let end = self.position.checked_add(count)?;
        let slice = self.bytes.get(self.position..end)?;
        self.position = end;
        Some(slice)
    }

    /// Reads a `u8`.
    pub fn u8(&mut self) -> Option<u8> {
        let value = *self.bytes.get(self.position)?;
        self.position += 1;
        Some(value)
    }

    /// Reads a little-endian `u16`.
    pub fn u16_le(&mut self) -> Option<u16> {
        let slice = self.take(2)?;
        Some(u16::from_le_bytes([slice[0], slice[1]]))
    }

    /// Reads a little-endian `u32`.
    pub fn u32_le(&mut self) -> Option<u32> {
        let slice = self.take(4)?;
        Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Reads a little-endian `u64`.
    pub fn u64_le(&mut self) -> Option<u64> {
        let slice = self.take(8)?;
        let mut raw = [0_u8; 8];
        raw.copy_from_slice(slice);
        Some(u64::from_le_bytes(raw))
    }

    /// Reads a big-endian `u16`.
    pub fn u16_be(&mut self) -> Option<u16> {
        let slice = self.take(2)?;
        Some(u16::from_be_bytes([slice[0], slice[1]]))
    }

    /// Reads a big-endian `u32`.
    pub fn u32_be(&mut self) -> Option<u32> {
        let slice = self.take(4)?;
        Some(u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
    }

    /// Reads a big-endian `u64`.
    pub fn u64_be(&mut self) -> Option<u64> {
        let slice = self.take(8)?;
        let mut raw = [0_u8; 8];
        raw.copy_from_slice(slice);
        Some(u64::from_be_bytes(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::{Reader, read_i32_le, read_u16_be, write_u32_le};

    #[test]
    fn reader_bounds() {
        let mut reader = Reader::new(&[0x01, 0x02]);
        assert_eq!(reader.u16_le(), Some(0x0201));
        assert_eq!(reader.u8(), None);
        assert_eq!(reader.position(), 2);
        assert!(reader.is_empty());
    }

    #[test]
    fn reader_skip_and_take() {
        let mut reader = Reader::new(&[0, 1, 2, 3, 4]);
        assert!(reader.skip(2));
        assert_eq!(reader.take(2), Some(&[2_u8, 3][..]));
        assert_eq!(reader.remaining(), 1);
        assert!(!reader.skip(2));
    }

    #[test]
    fn endian_helpers() {
        assert_eq!(read_u16_be(&[0x12, 0x34], 0), Some(0x1234));
        assert_eq!(read_i32_le(&[0xFF, 0xFF, 0xFF, 0xFF], 0), Some(-1));
        assert_eq!(read_i32_le(&[0; 4], 1), None);
    }

    #[test]
    fn writer_bounds() {
        let mut buffer = [0_u8; 4];
        assert!(write_u32_le(&mut buffer, 0, 0xDEAD_BEEF));
        assert_eq!(buffer, [0xEF, 0xBE, 0xAD, 0xDE]);
        assert!(!write_u32_le(&mut buffer, 1, 0));
    }
}
