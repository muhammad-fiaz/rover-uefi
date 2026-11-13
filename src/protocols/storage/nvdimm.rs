//! # `NVDIMM` Label Protocol and Client API
//!
//! Provides services to manage labels and namespaces contained in the Label Storage
//! Area (`LSA`) of Non-Volatile `DIMMs` (`NVDIMMs` / persistent memory).
//!
//! Defined in UEFI Specification 2.7+ (Section 13.20).

use crate::base::{EfiGuid, EfiStatus};

/// EFI `NVDIMM` Label Protocol GUID.
pub const EFI_NVDIMM_LABEL_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD40B_6B80,
    0x97D5,
    0x4282,
    [0xBB, 0x1D, 0x22, 0x3A, 0x16, 0x91, 0x80, 0x58],
);

/// Signature length of index block ("`NAMESPACE_INDEX\0`").
pub const EFI_NVDIMM_LABEL_INDEX_SIG_LEN: usize = 16;
/// Index block alignment in bytes.
pub const EFI_NVDIMM_LABEL_INDEX_ALIGN: usize = 256;

/// `NVDIMM` Label Storage Area Index Block.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiNvdimmLabelIndexBlock {
    /// Signature of index block data structure (`NAMESPACE_INDEX\0`).
    pub sig: [u8; EFI_NVDIMM_LABEL_INDEX_SIG_LEN],
    /// Attributes of this Label Storage Area.
    pub flags: [u8; 3],
    /// Size of each label: `128 << label_size` bytes.
    pub label_size: u8,
    /// Sequence number used to identify current index block.
    pub seq: u32,
    /// Byte offset of this index block in `LSA`.
    pub my_off: u64,
    /// Size of this index block in bytes.
    pub my_size: u64,
    /// Offset of the alternate index block in `LSA`.
    pub other_off: u64,
    /// Offset of the label area in `LSA`.
    pub label_off: u64,
    /// Number of label slots in the label area.
    pub n_slot: u32,
    /// Major revision number.
    pub major: u16,
    /// Minor revision number.
    pub minor: u16,
    /// 32-bit checksum of the index block.
    pub checksum: u64,
}

/// Retrieves Label Storage Area size and maximum transfer length.
pub type EfiNvdimmLabelStorageInformation = unsafe extern "efiapi" fn(
    this: *mut EfiNvdimmLabelProtocol,
    size_of_label_storage_area: *mut u32,
    max_transfer_length: *mut u32,
) -> EfiStatus;

/// Reads label data from Label Storage Area at specified offset.
pub type EfiNvdimmLabelStorageRead = unsafe extern "efiapi" fn(
    this: *const EfiNvdimmLabelProtocol,
    offset: u32,
    transfer_length: u32,
    label_data: *mut u8,
) -> EfiStatus;

/// Writes label data into Label Storage Area at specified offset.
pub type EfiNvdimmLabelStorageWrite = unsafe extern "efiapi" fn(
    this: *const EfiNvdimmLabelProtocol,
    offset: u32,
    transfer_length: u32,
    label_data: *const u8,
) -> EfiStatus;

/// EFI `NVDIMM` Label Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiNvdimmLabelProtocol {
    /// Retrieves `LSA` size and transfer capability.
    pub label_storage_information: Option<EfiNvdimmLabelStorageInformation>,
    /// Reads `LSA` data.
    pub label_storage_read: Option<EfiNvdimmLabelStorageRead>,
    /// Writes `LSA` data.
    pub label_storage_write: Option<EfiNvdimmLabelStorageWrite>,
}

/// Safe client wrapper for `NVDIMM` Label Storage Area operations.
#[derive(Debug)]
pub struct NvdimmLabel<'a> {
    protocol: &'a mut EfiNvdimmLabelProtocol,
}

impl<'a> NvdimmLabel<'a> {
    /// Create a new safe view of the `NVDIMM` label protocol.
    #[must_use]
    pub fn new(protocol: &'a mut EfiNvdimmLabelProtocol) -> Self {
        Self { protocol }
    }

    /// Retrieve the total size of the Label Storage Area and maximum transfer length.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn info(&mut self) -> Result<(u32, u32), EfiStatus> {
        if let Some(info_fn) = self.protocol.label_storage_information {
            let mut lsa_size = 0u32;
            let mut max_transfer = 0u32;
            let status =
                unsafe { info_fn(self.protocol, &raw mut lsa_size, &raw mut max_transfer) };
            if status == EfiStatus::SUCCESS {
                Ok((lsa_size, max_transfer))
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Read label data from the Label Storage Area into buffer.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn read(&self, offset: u32, buffer: &mut [u8]) -> Result<(), EfiStatus> {
        if let Some(read_fn) = self.protocol.label_storage_read {
            let len = u32::try_from(buffer.len()).map_err(|_| EfiStatus::BAD_BUFFER_SIZE)?;
            let status = unsafe { read_fn(self.protocol, offset, len, buffer.as_mut_ptr()) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Write label data into the Label Storage Area from buffer.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn write(&self, offset: u32, buffer: &[u8]) -> Result<(), EfiStatus> {
        if let Some(write_fn) = self.protocol.label_storage_write {
            let len = u32::try_from(buffer.len()).map_err(|_| EfiStatus::BAD_BUFFER_SIZE)?;
            let status = unsafe { write_fn(self.protocol, offset, len, buffer.as_ptr()) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}
