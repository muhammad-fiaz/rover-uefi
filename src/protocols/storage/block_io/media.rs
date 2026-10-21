//! # Block media
//!
//! The `EFI_BLOCK_IO_MEDIA` structure.
//!
//! The struct is `#[repr(C)]`. The lowest
//! aligned LBA, logical blocks per physical
//! block, and optimal transfer granularity
//! fields are present in revision 2 and 3
//! revisions. The struct size is version
//! dependent; all fields are defined so the
//! layout matches the specification order.

use crate::abi::types::Bool;

/// Block I/O media information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiBlockIoMedia {
    /// The media identifier.
    pub media_id: u32,
    /// Whether the media is removable.
    pub removable_media: Bool,
    /// Whether the media is present.
    pub media_present: Bool,
    /// Whether LBA 0 starts a partition.
    pub logical_partition: Bool,
    /// Whether the media is read-only.
    pub read_only: Bool,
    /// Whether writes are cached.
    pub write_caching: Bool,
    /// The intrinsic block size.
    pub block_size: u32,
    /// The buffer alignment requirement.
    pub io_align: u32,
    /// The last logical block address.
    pub last_block: u64,
    /// The lowest aligned LBA.
    pub lowest_aligned_lba: u64,
    /// Logical blocks per physical block.
    pub logical_blocks_per_physical_block: u32,
    /// The optimal transfer granularity.
    pub optimal_transfer_length_granularity: u32,
}

const _: () = {
    assert!(core::mem::size_of::<EfiBlockIoMedia>() == 48);
    assert!(core::mem::offset_of!(EfiBlockIoMedia, media_id) == 0);
    assert!(core::mem::offset_of!(EfiBlockIoMedia, block_size) == 12);
    assert!(core::mem::offset_of!(EfiBlockIoMedia, last_block) == 24);
    assert!(core::mem::offset_of!(EfiBlockIoMedia, optimal_transfer_length_granularity) == 44);
};
