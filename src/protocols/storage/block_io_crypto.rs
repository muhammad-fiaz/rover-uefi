//! UEFI Block I/O Crypto Protocol definition and safe wrapper.
//!
//! Defined in UEFI Specification 2.5+ Section 13.
//! The Block I/O Crypto protocol provides hardware-accelerated inline cryptographic
//! services for block storage devices such as UFS, `NVMe`, and SATA.

use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use crate::protocols::storage::block_io::EfiBlockIoMedia;
use core::ffi::c_void;

/// UEFI Block I/O Crypto Protocol GUID: `{a00490ba-3f1a-4b4c-ab90-4fa99726a1e8}`.
pub const EFI_BLOCK_IO_CRYPTO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA004_90BA,
    0x3F1A,
    0x4B4C,
    [0xAB, 0x90, 0x4F, 0xA9, 0x97, 0x26, 0xA1, 0xE8],
);

/// AES-XTS Algorithm GUID: `{2f87ba6a-5c04-4385-a780-f3bf78a97bec}`.
pub const EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_XTS: EfiGuid = EfiGuid::new(
    0x2F87_BA6A,
    0x5C04,
    0x4385,
    [0xA7, 0x80, 0xF3, 0xBF, 0x78, 0xA9, 0x7B, 0xEC],
);

/// AES-CBC `BitLocker` Algorithm GUID: `{689e4c62-70bf-4cf3-88bb-33b318268670}`.
pub const EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_CBC_BITLOCKER: EfiGuid = EfiGuid::new(
    0x689E_4C62,
    0x70BF,
    0x4CF3,
    [0x88, 0xBB, 0x33, 0xB3, 0x18, 0x26, 0x86, 0x70],
);

/// Special index indicating the implementation should allocate any available configuration slot.
pub const EFI_BLOCK_IO_CRYPTO_INDEX_ANY: u64 = 0xFFFF_FFFF_FFFF_FFFF;

/// Asynchronous transaction token for Block I/O Crypto operations.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiBlockIoCryptoToken {
    /// Event signaled on completion (null for blocking synchronous I/O).
    pub event: EfiEvent,
    /// Result status of the cryptographic operation.
    pub transaction_status: EfiStatus,
}

/// Description of an inline cryptographic algorithm capability.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiBlockIoCryptoCapability {
    /// Algorithm identifier GUID.
    pub algorithm: EfiGuid,
    /// Key size in bits (e.g. 128, 256, 512).
    pub key_size: u64,
    /// Bitmask of supported crypto block sizes (bit `j` set indicates 2^j bytes supported).
    pub crypto_block_size_bit_mask: u64,
}

impl Default for EfiBlockIoCryptoCapability {
    fn default() -> Self {
        Self {
            algorithm: EfiGuid::ZERO,
            key_size: 0,
            crypto_block_size_bit_mask: 0,
        }
    }
}

/// Common header for cryptographic initialization vector inputs.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiBlockIoCryptoIvInput {
    /// Total byte size of the IV input parameter buffer.
    pub input_size: u64,
}

/// Initialization vector input structure for AES-XTS.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiBlockIoCryptoIvInputAesXts {
    /// Common IV header.
    pub header: EfiBlockIoCryptoIvInput,
    /// Logical block/tweak sequence number.
    pub crypto_block_number: u64,
    /// Byte size of each crypto block.
    pub crypto_block_byte_size: u64,
}

/// Initialization vector input structure for AES-CBC with `BitLocker`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiBlockIoCryptoIvInputAesCbcBitlocker {
    /// Common IV header.
    pub header: EfiBlockIoCryptoIvInput,
    /// Byte offset into the storage device.
    pub crypto_block_byte_offset: u64,
    /// Byte size of each crypto block.
    pub crypto_block_byte_size: u64,
}

/// Capabilities descriptor for inline cryptographic hardware engine.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiBlockIoCryptoCapabilities {
    /// True if inline cryptography is supported and ready.
    pub supported: bool,
    /// Maximum number of keys that can be configured simultaneously.
    pub key_count: u64,
    /// Number of supported algorithm capabilities.
    pub capability_count: u64,
}

/// Configuration table entry for loading a key into the crypto engine.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiBlockIoCryptoConfigurationTableEntry {
    /// Slot index or `EFI_BLOCK_IO_CRYPTO_INDEX_ANY`.
    pub index: u64,
    /// Component GUID owning this key slot.
    pub key_owner_guid: EfiGuid,
    /// Chosen capability for this key slot.
    pub capability: EfiBlockIoCryptoCapability,
    /// Pointer to raw key data buffer.
    pub crypto_key: *const c_void,
}

/// Response entry describing a configured key slot.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfiBlockIoCryptoResponseConfigurationEntry {
    /// Slot index.
    pub index: u64,
    /// Component GUID owning this key slot.
    pub key_owner_guid: EfiGuid,
    /// Active capability for this key slot.
    pub capability: EfiBlockIoCryptoCapability,
}

/// Function prototype for resetting crypto engine hardware.
pub type EfiBlockIoCryptoReset = unsafe extern "efiapi" fn(
    this: *const EfiBlockIoCryptoProtocol,
    extended_verification: bool,
) -> EfiStatus;

/// Function prototype for querying crypto engine capabilities.
pub type EfiBlockIoCryptoGetCapabilities = unsafe extern "efiapi" fn(
    this: *const EfiBlockIoCryptoProtocol,
    capabilities: *mut EfiBlockIoCryptoCapabilities,
) -> EfiStatus;

/// Function prototype for configuring key slots.
pub type EfiBlockIoCryptoSetConfiguration = unsafe extern "efiapi" fn(
    this: *const EfiBlockIoCryptoProtocol,
    configuration_count: u64,
    configuration_table: *const EfiBlockIoCryptoConfigurationTableEntry,
    resulting_table: *mut EfiBlockIoCryptoResponseConfigurationEntry,
) -> EfiStatus;

/// Function prototype for querying configured key slots.
pub type EfiBlockIoCryptoGetConfiguration = unsafe extern "efiapi" fn(
    this: *const EfiBlockIoCryptoProtocol,
    start_index: u64,
    configuration_count: u64,
    key_owner_guid: *const EfiGuid,
    configuration_table: *mut EfiBlockIoCryptoResponseConfigurationEntry,
) -> EfiStatus;

/// Function prototype for reading blocks with inline decryption.
pub type EfiBlockIoCryptoReadExtended = unsafe extern "efiapi" fn(
    this: *const EfiBlockIoCryptoProtocol,
    media_id: u32,
    lba: u64,
    token: *mut EfiBlockIoCryptoToken,
    buffer_size: u64,
    buffer: *mut c_void,
    index: *const u64,
    crypto_iv_input: *const c_void,
) -> EfiStatus;

/// Function prototype for writing blocks with inline encryption.
pub type EfiBlockIoCryptoWriteExtended = unsafe extern "efiapi" fn(
    this: *const EfiBlockIoCryptoProtocol,
    media_id: u32,
    lba: u64,
    token: *mut EfiBlockIoCryptoToken,
    buffer_size: u64,
    buffer: *const c_void,
    index: *const u64,
    crypto_iv_input: *const c_void,
) -> EfiStatus;

/// Function prototype for flushing cached modified blocks.
pub type EfiBlockIoCryptoFlush = unsafe extern "efiapi" fn(
    this: *const EfiBlockIoCryptoProtocol,
    token: *mut EfiBlockIoCryptoToken,
) -> EfiStatus;

/// UEFI Block I/O Crypto Protocol structure (`EFI_BLOCK_IO_CRYPTO_PROTOCOL`).
#[repr(C)]
pub struct EfiBlockIoCryptoProtocol {
    /// Pointer to underlying `EFI_BLOCK_IO_MEDIA`.
    pub media: *const EfiBlockIoMedia,
    /// Resets the block device and crypto hardware.
    pub reset: EfiBlockIoCryptoReset,
    /// Queries crypto engine capabilities.
    pub get_capabilities: EfiBlockIoCryptoGetCapabilities,
    /// Sets key configuration entries.
    pub set_configuration: EfiBlockIoCryptoSetConfiguration,
    /// Queries current key configuration entries.
    pub get_configuration: EfiBlockIoCryptoGetConfiguration,
    /// Reads and optionally decrypts blocks inline.
    pub read_extended: EfiBlockIoCryptoReadExtended,
    /// Optionally encrypts inline and writes blocks.
    pub write_extended: EfiBlockIoCryptoWriteExtended,
    /// Flushes modified data to disk.
    pub flush_blocks: EfiBlockIoCryptoFlush,
}

/// Safe idiomatic wrapper around `EfiBlockIoCryptoProtocol`.
pub struct BlockIoCrypto {
    proto: *const EfiBlockIoCryptoProtocol,
}

impl BlockIoCrypto {
    /// Creates a new `BlockIoCrypto` client wrapper.
    #[must_use]
    pub const fn new(proto: *const EfiBlockIoCryptoProtocol) -> Self {
        Self { proto }
    }

    /// Returns the raw protocol pointer.
    #[must_use]
    pub const fn raw(&self) -> *const EfiBlockIoCryptoProtocol {
        self.proto
    }

    /// Returns a reference to the block media descriptor, if valid.
    #[must_use]
    pub fn media(&self) -> Option<&EfiBlockIoMedia> {
        let media_ptr = unsafe { (*self.proto).media };
        if media_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*media_ptr })
        }
    }

    /// Resets the block device and crypto hardware.
    ///
    /// # Errors
    ///
    /// Returns an error status if reset fails.
    pub fn reset(&self, extended_verification: bool) -> Result<(), EfiStatus> {
        let status = unsafe { ((*self.proto).reset)(self.proto, extended_verification) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries capabilities of the inline crypto engine.
    ///
    /// # Errors
    ///
    /// Returns an error status if query fails or buffer is too small.
    pub fn get_capabilities(
        &self,
        capabilities: &mut EfiBlockIoCryptoCapabilities,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.proto).get_capabilities)(self.proto, core::ptr::from_mut(capabilities))
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Configures key slots in the inline crypto engine.
    ///
    /// # Errors
    ///
    /// Returns an error status if configuration fails.
    pub fn set_configuration(
        &self,
        entries: &[EfiBlockIoCryptoConfigurationTableEntry],
        mut resulting: Option<&mut [EfiBlockIoCryptoResponseConfigurationEntry]>,
    ) -> Result<(), EfiStatus> {
        let res_ptr = match resulting.as_mut() {
            Some(slice) => slice.as_mut_ptr(),
            None => core::ptr::null_mut(),
        };

        let status = unsafe {
            ((*self.proto).set_configuration)(
                self.proto,
                entries.len() as u64,
                entries.as_ptr(),
                res_ptr,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries currently configured key slots.
    ///
    /// # Errors
    ///
    /// Returns an error status if query fails.
    pub fn get_configuration(
        &self,
        start_index: u64,
        entries: &mut [EfiBlockIoCryptoResponseConfigurationEntry],
        owner_filter: Option<&EfiGuid>,
    ) -> Result<(), EfiStatus> {
        let filter_ptr = match owner_filter {
            Some(guid) => core::ptr::from_ref(guid),
            None => core::ptr::null(),
        };

        let status = unsafe {
            ((*self.proto).get_configuration)(
                self.proto,
                start_index,
                entries.len() as u64,
                filter_ptr,
                entries.as_mut_ptr(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads blocks with optional inline decryption.
    ///
    /// # Errors
    ///
    /// Returns an error status on I/O or decryption failure.
    pub fn read_extended(
        &self,
        media_id: u32,
        lba: u64,
        token: &mut EfiBlockIoCryptoToken,
        buffer: &mut [u8],
        index: Option<u64>,
        iv_input: Option<&[u8]>,
    ) -> Result<(), EfiStatus> {
        let index_ptr = match &index {
            Some(idx) => core::ptr::from_ref(idx),
            None => core::ptr::null(),
        };
        let iv_ptr = match iv_input {
            Some(slice) => slice.as_ptr().cast::<c_void>(),
            None => core::ptr::null(),
        };

        let status = unsafe {
            ((*self.proto).read_extended)(
                self.proto,
                media_id,
                lba,
                core::ptr::from_mut(token),
                buffer.len() as u64,
                buffer.as_mut_ptr().cast::<c_void>(),
                index_ptr,
                iv_ptr,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Writes blocks with optional inline encryption.
    ///
    /// # Errors
    ///
    /// Returns an error status on I/O or encryption failure.
    pub fn write_extended(
        &self,
        media_id: u32,
        lba: u64,
        token: &mut EfiBlockIoCryptoToken,
        buffer: &[u8],
        index: Option<u64>,
        iv_input: Option<&[u8]>,
    ) -> Result<(), EfiStatus> {
        let index_ptr = match &index {
            Some(idx) => core::ptr::from_ref(idx),
            None => core::ptr::null(),
        };
        let iv_ptr = match iv_input {
            Some(slice) => slice.as_ptr().cast::<c_void>(),
            None => core::ptr::null(),
        };

        let status = unsafe {
            ((*self.proto).write_extended)(
                self.proto,
                media_id,
                lba,
                core::ptr::from_mut(token),
                buffer.len() as u64,
                buffer.as_ptr().cast::<c_void>(),
                index_ptr,
                iv_ptr,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Flushes cached modified blocks to physical media.
    ///
    /// # Errors
    ///
    /// Returns an error status on device failure.
    pub fn flush_blocks(&self, token: &mut EfiBlockIoCryptoToken) -> Result<(), EfiStatus> {
        let status =
            unsafe { ((*self.proto).flush_blocks)(self.proto, core::ptr::from_mut(token)) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
