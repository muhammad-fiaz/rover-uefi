//! # Memory map
//!
//! Retrieval and parsing of the firmware memory map.
//!
//! The safe method takes a byte buffer and returns the map key,
//! descriptor size, and descriptor version. The caller provides the
//! buffer, so no allocation occurs. The memory map key is valid only
//! until the next allocation or other operation that changes the map.
//! The exit service must use the most recent key without intervening
//! allocations.
//!
//! Descriptors are parsed with the firmware descriptor size as stride
//! because future descriptor versions may be larger than the current
//! Rust struct. The helpers validate bounds before constructing views.
//! The current `EFI_MEMORY_DESCRIPTOR` is always 40 bytes because the
//! physical and virtual address fields are 64-bit on every target.

use crate::abi::types::UIntN;
use crate::base::{EfiPhysicalAddress, EfiStatus, EfiVirtualAddress, MemoryType};
use crate::services::boot::BootServices;

/// The size of the fixed portion of a memory descriptor in bytes.
pub const MEMORY_DESCRIPTOR_SIZE: usize = 40;

/// A single memory map descriptor view.
///
/// The view borrows bytes from the caller buffer.
pub struct MemoryDescriptorRef<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl MemoryDescriptorRef<'_> {
    /// Returns the memory type.
    #[must_use]
    pub fn memory_type(&self) -> MemoryType {
        let mut raw = [0_u8; 4];
        if let Some(slice) = self.bytes.get(self.offset..self.offset + 4) {
            raw.copy_from_slice(slice);
        }
        MemoryType::from_raw(u32::from_le_bytes(raw))
    }

    /// Returns the physical start address.
    #[must_use]
    pub fn physical_start(&self) -> EfiPhysicalAddress {
        let mut raw = [0_u8; 8];
        if let Some(slice) = self.bytes.get(self.offset + 8..self.offset + 16) {
            raw.copy_from_slice(slice);
        }
        EfiPhysicalAddress::from_u64(u64::from_le_bytes(raw))
    }

    /// Returns the virtual start address.
    ///
    /// Before the virtual address transition this field is zero.
    #[must_use]
    pub fn virtual_start(&self) -> EfiVirtualAddress {
        let mut raw = [0_u8; 8];
        if let Some(slice) = self.bytes.get(self.offset + 16..self.offset + 24) {
            raw.copy_from_slice(slice);
        }
        EfiVirtualAddress::from_u64(u64::from_le_bytes(raw))
    }

    /// Returns the number of pages.
    #[must_use]
    pub fn number_of_pages(&self) -> u64 {
        let mut raw = [0_u8; 8];
        if let Some(slice) = self.bytes.get(self.offset + 24..self.offset + 32) {
            raw.copy_from_slice(slice);
        }
        u64::from_le_bytes(raw)
    }

    /// Returns the attribute mask.
    #[must_use]
    pub fn attribute(&self) -> u64 {
        let mut raw = [0_u8; 8];
        if let Some(slice) = self.bytes.get(self.offset + 32..self.offset + 40) {
            raw.copy_from_slice(slice);
        }
        u64::from_le_bytes(raw)
    }

    /// Copies the descriptor into an owned value.
    #[must_use]
    pub fn to_owned_descriptor(&self) -> MemoryDescriptor {
        MemoryDescriptor {
            memory_type: self.memory_type(),
            physical_start: self.physical_start(),
            virtual_start: self.virtual_start(),
            number_of_pages: self.number_of_pages(),
            attribute: self.attribute(),
        }
    }

    /// Returns the end physical address of the region.
    ///
    /// Returns `None` on arithmetic overflow.
    #[must_use]
    pub fn physical_end(&self) -> Option<u64> {
        let pages = self.number_of_pages();
        let length = pages.checked_mul(crate::abi::types::EFI_PAGE_SIZE as u64)?;
        self.physical_start().to_u64().checked_add(length)
    }
}

/// An owned memory map descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemoryDescriptor {
    /// The type of the memory region.
    pub memory_type: MemoryType,
    /// The physical address of the first byte.
    pub physical_start: EfiPhysicalAddress,
    /// The virtual address of the first byte.
    pub virtual_start: EfiVirtualAddress,
    /// The number of 4 KiB pages in the region.
    pub number_of_pages: u64,
    /// The attribute mask of the region.
    pub attribute: u64,
}

/// Returns whether the buffer length is a valid memory map image.
///
/// The descriptor size must be at least [`MEMORY_DESCRIPTOR_SIZE`] and
/// the buffer length must be an exact multiple of it.
#[must_use]
pub fn validate_memory_map_buffer(buffer_len: usize, descriptor_size: usize) -> bool {
    descriptor_size >= MEMORY_DESCRIPTOR_SIZE && buffer_len.is_multiple_of(descriptor_size)
}

/// Validates a returned memory map.
///
/// Unlike [`validate_memory_map_buffer`], this rejects a zero-length
/// map and requires at least one descriptor, which is the condition
/// for a usable `ExitBootServices` key.
#[must_use]
pub fn validate_returned_memory_map(
    buffer_len: usize,
    descriptor_size: usize,
    descriptor_version: u32,
) -> bool {
    buffer_len > 0
        && validate_memory_map_buffer(buffer_len, descriptor_size)
        && descriptor_version == crate::base::EFI_MEMORY_DESCRIPTOR_VERSION
}

/// Iterates memory map descriptors.
pub struct MemoryMapIter<'a> {
    bytes: &'a [u8],
    descriptor_size: usize,
    offset: usize,
}

impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = MemoryDescriptorRef<'a>;

    /// Returns the next descriptor.
    fn next(&mut self) -> Option<Self::Item> {
        if self.descriptor_size < MEMORY_DESCRIPTOR_SIZE {
            return None;
        }
        let end = self.offset.checked_add(MEMORY_DESCRIPTOR_SIZE)?;
        if end > self.bytes.len() {
            return None;
        }
        let current = self.offset;
        self.offset = current.checked_add(self.descriptor_size)?;
        Some(MemoryDescriptorRef {
            bytes: self.bytes,
            offset: current,
        })
    }
}

/// Creates a memory map iterator.
///
/// Returns `None` when the descriptor size is smaller than the fixed
/// descriptor size. The stride is the firmware value, not the Rust
/// struct size, so forward-compatible descriptors still parse.
#[must_use]
pub fn memory_map_descriptors(bytes: &[u8], descriptor_size: usize) -> Option<MemoryMapIter<'_>> {
    if descriptor_size < MEMORY_DESCRIPTOR_SIZE {
        None
    } else {
        Some(MemoryMapIter {
            bytes,
            descriptor_size,
            offset: 0,
        })
    }
}

/// A parsed firmware memory map.
///
/// The value borrows the caller-supplied buffer and carries the
/// firmware metadata needed for `ExitBootServices`.
pub struct MemoryMap<'a> {
    bytes: &'a [u8],
    descriptor_size: UIntN,
    descriptor_version: u32,
    map_key: UIntN,
}

impl<'a> MemoryMap<'a> {
    /// Parses a memory map from the values returned by the firmware.
    ///
    /// Returns `None` when the descriptor size is too small or the
    /// buffer length is not a multiple of the descriptor size.
    #[must_use]
    pub fn new(
        bytes: &'a [u8],
        map_key: UIntN,
        descriptor_size: UIntN,
        descriptor_version: u32,
    ) -> Option<Self> {
        if !validate_returned_memory_map(bytes.len(), descriptor_size, descriptor_version) {
            return None;
        }
        Some(Self {
            bytes,
            descriptor_size,
            descriptor_version,
            map_key,
        })
    }

    /// Returns the firmware map key.
    #[must_use]
    pub const fn map_key(&self) -> UIntN {
        self.map_key
    }

    /// Returns the descriptor stride in bytes.
    #[must_use]
    pub const fn descriptor_size(&self) -> UIntN {
        self.descriptor_size
    }

    /// Returns the descriptor version reported by firmware.
    #[must_use]
    pub const fn descriptor_version(&self) -> u32 {
        self.descriptor_version
    }

    /// Returns the raw map bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Iterates over borrowed descriptor views.
    #[must_use]
    pub fn descriptors(&self) -> MemoryMapIter<'a> {
        MemoryMapIter {
            bytes: self.bytes,
            descriptor_size: self.descriptor_size,
            offset: 0,
        }
    }

    /// Copies the map into `output`.
    ///
    /// Returns the number of descriptors copied. When `output` is too
    /// small the leading descriptors are copied and the return value is
    /// the number actually written; callers can size the output with
    /// [`MemoryMap::descriptor_count`].
    pub fn copy_descriptors(&self, output: &mut [MemoryDescriptor]) -> usize {
        let mut copied = 0;
        for (slot, descriptor) in output.iter_mut().zip(self.descriptors()) {
            *slot = descriptor.to_owned_descriptor();
            copied += 1;
        }
        copied
    }

    /// Returns the number of descriptors in the map.
    #[must_use]
    pub fn descriptor_count(&self) -> usize {
        self.descriptors().count()
    }

    /// Returns the total number of pages described by the map.
    ///
    /// Returns `None` on arithmetic overflow.
    #[must_use]
    pub fn total_pages(&self) -> Option<u64> {
        self.descriptors().try_fold(0_u64, |total, descriptor| {
            total.checked_add(descriptor.number_of_pages())
        })
    }
}

impl BootServices<'_> {
    /// Retrieves the memory map into a byte buffer.
    ///
    /// The buffer must be large enough for the map. The returned tuple
    /// holds the map key, descriptor size, and descriptor version. The
    /// map key is valid only until the next map-changing call.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the call fails. When the status
    /// is [`EfiStatus::BUFFER_TOO_SMALL`] the buffer is unchanged; call
    /// [`BootServices::memory_map_size`] first or retry with a larger
    /// buffer.
    pub fn get_memory_map(&self, buffer: &mut [u8]) -> Result<(UIntN, UIntN, u32), EfiStatus> {
        let (status, _used, map_key, descriptor_size, descriptor_version) =
            self.get_memory_map_raw(buffer);
        if status.is_success() {
            Ok((map_key, descriptor_size, descriptor_version))
        } else {
            Err(status)
        }
    }

    /// Retrieves the memory map and reports the used buffer length.
    ///
    /// The first tuple element is the firmware status, the second is the
    /// number of bytes written, and the remaining elements are the map
    /// key, descriptor size, and descriptor version.
    fn get_memory_map_raw(&self, buffer: &mut [u8]) -> (EfiStatus, usize, UIntN, UIntN, u32) {
        let requested = buffer.len();
        let mut buffer_len: UIntN = requested;
        let mut map_key: UIntN = 0;
        let mut descriptor_size: UIntN = 0;
        let mut descriptor_version: u32 = 0;
        // SAFETY: The raw pointer derived from the mutable slice is valid
        // for `buffer_len` bytes for the call, and every out parameter
        // points to a live stack local.
        let status = unsafe {
            (self.table.get_memory_map)(
                &raw mut buffer_len,
                buffer.as_mut_ptr().cast(),
                &raw mut map_key,
                &raw mut descriptor_size,
                &raw mut descriptor_version,
            )
        };
        let used = buffer_len.min(requested);
        (status, used, map_key, descriptor_size, descriptor_version)
    }

    /// Queries the buffer size required for the memory map.
    ///
    /// Calls the firmware with a zero-length buffer. Firmware returns
    /// [`EfiStatus::BUFFER_TOO_SMALL`] and stores the required size in
    /// the size input, which is returned as the success value of this
    /// method.
    ///
    /// # Errors
    ///
    /// Returns any other firmware status unchanged.
    pub fn memory_map_size(&self) -> Result<UIntN, EfiStatus> {
        let mut buffer_len: UIntN = 0;
        let mut map_key: UIntN = 0;
        let mut descriptor_size: UIntN = 0;
        let mut descriptor_version: u32 = 0;
        // SAFETY: A zero-length buffer is permitted by the specification
        // for a size query, and every out parameter points to a live
        // stack local.
        let status = unsafe {
            (self.table.get_memory_map)(
                &raw mut buffer_len,
                core::ptr::null_mut(),
                &raw mut map_key,
                &raw mut descriptor_size,
                &raw mut descriptor_version,
            )
        };
        if status == EfiStatus::BUFFER_TOO_SMALL || status.is_success() {
            Ok(buffer_len)
        } else {
            Err(status)
        }
    }

    /// Retrieves and parses the memory map in one call.
    ///
    /// Returns the parsed map on success and the firmware status when
    /// the buffer is too small or the map is malformed.
    ///
    /// # Errors
    ///
    /// Returns [`EfiStatus::BAD_BUFFER_SIZE`] when the firmware reports
    /// an unusable map shape, and the raw firmware status otherwise.
    pub fn memory_map<'a>(&self, buffer: &'a mut [u8]) -> Result<MemoryMap<'a>, EfiStatus> {
        let (status, used, map_key, descriptor_size, descriptor_version) =
            self.get_memory_map_raw(buffer);
        if !status.is_success() {
            return Err(status);
        }
        let length = used.min(buffer.len());
        MemoryMap::new(
            &buffer[..length],
            map_key,
            descriptor_size,
            descriptor_version,
        )
        .ok_or(EfiStatus::BAD_BUFFER_SIZE)
    }
}
