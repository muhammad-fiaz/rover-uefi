//! # Direct memory access descriptors
//!
//! The description of the buffers that a device transfers
//! through direct memory access.
//!
//! A direct memory access engine does not read a pointer. It
//! reads a physical address and a length, and it may require
//! the buffer to satisfy an alignment and a maximum length. The
//! descriptors in this module capture those requirements so that
//! a driver can validate a buffer before it hands it to the
//! hardware.
//!
//! A transfer is described by a [`DmaBuffer`] for a contiguous
//! region, or by a [`DmaSegmentList`] when the data is spread
//! over several physical regions. The segment list has a fixed
//! capacity so that it can be stored without an allocator.

/// The smallest alignment that general purpose bus masters
/// accept for a transfer buffer.
pub const DMA_MIN_ALIGNMENT: usize = 16;

/// The alignment required for buffers that are mapped through a
/// page table.
pub const DMA_PAGE_ALIGNMENT: usize = 4096;

/// The number of segments that a single descriptor list holds.
pub const DMA_MAX_SEGMENTS: usize = 64;

/// The direction of a direct memory access transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaDirection {
    /// The device reads from the buffer.
    ToDevice,
    /// The device writes into the buffer.
    FromDevice,
    /// The device reads from and writes into the buffer.
    Bidirectional,
}

/// The reason a direct memory access descriptor is not valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaError {
    /// The descriptor list is already full.
    TooManySegments,
    /// The segment has a length of zero.
    ZeroLength,
    /// The buffer does not satisfy the requested alignment.
    Misaligned,
}

/// One physical region of a scattering or gathering transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaSegment {
    /// The physical address of the first byte.
    pub address: u64,
    /// The number of bytes in the segment.
    pub length: usize,
}

impl DmaSegment {
    /// Creates a segment for a physical region.
    #[must_use]
    pub const fn new(address: u64, length: usize) -> Self {
        Self { address, length }
    }

    /// Returns the physical address of the first byte.
    #[must_use]
    pub const fn address(&self) -> u64 {
        self.address
    }

    /// Returns the number of bytes in the segment.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.length
    }

    /// Returns whether the segment satisfies an alignment.
    #[must_use]
    pub fn is_aligned(&self, alignment: usize) -> bool {
        is_aligned(self.address, alignment)
    }
}

/// A contiguous buffer that a device transfers directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaBuffer {
    /// The physical address of the first byte.
    pub address: u64,
    /// The number of bytes in the buffer.
    pub length: usize,
    /// The direction of the transfer.
    pub direction: DmaDirection,
    /// The alignment that the buffer satisfies.
    pub alignment: usize,
}

impl DmaBuffer {
    /// Creates a buffer descriptor for a contiguous region.
    ///
    /// The alignment defaults to [`DMA_MIN_ALIGNMENT`].
    #[must_use]
    pub const fn new(address: u64, length: usize, direction: DmaDirection) -> Self {
        Self {
            address,
            length,
            direction,
            alignment: DMA_MIN_ALIGNMENT,
        }
    }

    /// Creates a buffer descriptor with an explicit alignment.
    #[must_use]
    pub const fn with_alignment(
        address: u64,
        length: usize,
        direction: DmaDirection,
        alignment: usize,
    ) -> Self {
        Self {
            address,
            length,
            direction,
            alignment,
        }
    }

    /// Returns the physical address of the first byte.
    #[must_use]
    pub const fn address(&self) -> u64 {
        self.address
    }

    /// Returns the number of bytes in the buffer.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.length
    }

    /// Returns the direction of the transfer.
    #[must_use]
    pub const fn direction(&self) -> DmaDirection {
        self.direction
    }

    /// Returns the alignment that the buffer satisfies.
    #[must_use]
    pub const fn alignment(&self) -> usize {
        self.alignment
    }

    /// Returns whether the buffer satisfies its alignment.
    #[must_use]
    pub fn is_aligned(&self) -> bool {
        is_aligned(self.address, self.alignment)
    }

    /// Validates the buffer against the requirements of a
    /// transfer.
    ///
    /// # Errors
    ///
    /// Returns [`DmaError::ZeroLength`] for an empty buffer and
    /// [`DmaError::Misaligned`] when the address does not
    /// satisfy the recorded alignment.
    pub fn validate(&self) -> Result<(), DmaError> {
        if self.length == 0 {
            return Err(DmaError::ZeroLength);
        }
        if self.is_aligned() {
            Ok(())
        } else {
            Err(DmaError::Misaligned)
        }
    }

    /// Returns the single segment descriptor of the buffer.
    #[must_use]
    pub const fn as_segment(&self) -> DmaSegment {
        DmaSegment {
            address: self.address,
            length: self.length,
        }
    }
}

/// Returns whether a physical address satisfies an alignment.
///
/// An alignment of zero, or an alignment that is not a power of
/// two, is never satisfied.
#[must_use]
pub fn is_aligned(address: u64, alignment: usize) -> bool {
    if alignment == 0 || !alignment.is_power_of_two() {
        return false;
    }
    match u64::try_from(alignment) {
        Ok(modulus) => modulus != 0 && address % modulus == 0,
        Err(_) => false,
    }
}

/// Builds the segment descriptor of a contiguous physical
/// region.
#[must_use]
pub const fn contiguous_segment(address: u64, length: usize) -> DmaSegment {
    DmaSegment { address, length }
}

/// Builds the buffer descriptor of a contiguous physical region.
#[must_use]
pub const fn contiguous_buffer(address: u64, length: usize, direction: DmaDirection) -> DmaBuffer {
    DmaBuffer::new(address, length, direction)
}

/// A bounded list of scattering or gathering segments.
///
/// The list stores its segments inline so that it needs no
/// allocator. The capacity is chosen when the type is
/// instantiated; [`DMA_MAX_SEGMENTS`] is the default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaSegmentList<const N: usize = DMA_MAX_SEGMENTS> {
    segments: [DmaSegment; N],
    count: usize,
}

impl<const N: usize> Default for DmaSegmentList<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> DmaSegmentList<N> {
    /// Creates an empty segment list.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            segments: [DmaSegment {
                address: 0,
                length: 0,
            }; N],
            count: 0,
        }
    }

    /// Returns the number of segments in the list.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Returns whether the list holds no segment.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns the number of segments the list can hold.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns whether the list is full.
    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.count >= N
    }

    /// Returns the segment at an index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<DmaSegment> {
        if index >= self.count {
            return None;
        }
        self.segments.get(index).copied()
    }

    /// Returns an iterator over the segments of the list.
    pub fn iter(&self) -> core::slice::Iter<'_, DmaSegment> {
        self.segments.get(..self.count).unwrap_or(&[]).iter()
    }

    /// Returns the total number of bytes described by the list.
    ///
    /// The sum saturates instead of wrapping around.
    #[must_use]
    pub fn total_length(&self) -> usize {
        self.iter().fold(0usize, |total, segment| {
            total.saturating_add(segment.length)
        })
    }

    /// Appends a segment to the list.
    ///
    /// # Errors
    ///
    /// Returns [`DmaError::ZeroLength`] for an empty segment and
    /// [`DmaError::TooManySegments`] when the list is full.
    pub fn push(&mut self, segment: DmaSegment) -> Result<(), DmaError> {
        if segment.length == 0 {
            return Err(DmaError::ZeroLength);
        }
        if self.is_full() {
            return Err(DmaError::TooManySegments);
        }
        if let Some(slot) = self.segments.get_mut(self.count) {
            *slot = segment;
            self.count += 1;
            Ok(())
        } else {
            Err(DmaError::TooManySegments)
        }
    }

    /// Removes every segment from the list.
    pub fn clear(&mut self) {
        self.count = 0;
    }

    /// Returns whether every segment satisfies an alignment.
    #[must_use]
    pub fn is_aligned(&self, alignment: usize) -> bool {
        self.iter().all(|segment| segment.is_aligned(alignment))
    }
}

impl<'a, const N: usize> IntoIterator for &'a DmaSegmentList<N> {
    type Item = &'a DmaSegment;
    type IntoIter = core::slice::Iter<'a, DmaSegment>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
