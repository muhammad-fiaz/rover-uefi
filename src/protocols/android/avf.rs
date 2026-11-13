//! # Android Virtualization Framework (AVF) Protocol
//!
//! Provides hardware-backed DICE handover artifacts and Secretkeeper public key
//! material required for secure guest microdroid and VM bootstrapping.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};

/// Protocol GUID for the Android Virtualization Framework (`AVF`) interface.
///
/// `{e7f1c4a6-0a52-4f61-bd98-9e60b559452a}`
pub const GBL_EFI_AVF_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xe7f1_c4a6,
    0x0a52,
    0x4f61,
    [0xbd, 0x98, 0x9e, 0x60, 0xb5, 0x59, 0x45, 0x2a],
);

/// Initial revision of the AVF protocol.
pub const GBL_EFI_AVF_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Raw function pointer for reading vendor DICE handover data.
pub type GblEfiAvfReadVendorDiceHandover = unsafe extern "efiapi" fn(
    this: *mut GblEfiAvfProtocol,
    handover_size: *mut UIntN,
    handover: *mut u8,
) -> EfiStatus;

/// Raw function pointer for reading the Secretkeeper public key.
pub type GblEfiAvfReadSecretKeeperPublicKey = unsafe extern "efiapi" fn(
    this: *mut GblEfiAvfProtocol,
    public_key_size: *mut UIntN,
    public_key: *mut u8,
) -> EfiStatus;

/// Raw Android Virtualization Framework (`AVF`) Protocol structure.
#[derive(Debug)]
#[repr(C)]
pub struct GblEfiAvfProtocol {
    /// Revision of the protocol.
    pub revision: u64,
    /// Function to retrieve vendor DICE handover payload.
    pub read_vendor_dice_handover: GblEfiAvfReadVendorDiceHandover,
    /// Function to retrieve the Secretkeeper public key.
    pub read_secret_keeper_public_key: GblEfiAvfReadSecretKeeperPublicKey,
}

/// High-level, safe wrapper around [`GblEfiAvfProtocol`].
pub struct Avf<'a> {
    proto: &'a mut GblEfiAvfProtocol,
}

impl<'a> Avf<'a> {
    /// Creates a safe wrapper around an active protocol instance.
    #[must_use]
    pub fn new(proto: &'a mut GblEfiAvfProtocol) -> Self {
        Self { proto }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.proto.revision
    }

    /// Reads vendor DICE handover data into `buffer`.
    ///
    /// Returns the number of bytes written on success.
    ///
    /// # Errors
    ///
    /// Returns an error status if the underlying firmware protocol fails, or if
    /// `buffer` is too small to receive the payload.
    pub fn read_vendor_dice_handover(&mut self, buffer: &mut [u8]) -> Result<usize, EfiStatus> {
        let mut size = buffer.len();
        let status = unsafe {
            (self.proto.read_vendor_dice_handover)(self.proto, &raw mut size, buffer.as_mut_ptr())
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Reads Secretkeeper public key material into `buffer`.
    ///
    /// Returns the number of bytes written on success.
    ///
    /// # Errors
    ///
    /// Returns an error status if the underlying firmware protocol fails, or if
    /// `buffer` is too small to receive the public key.
    pub fn read_secret_keeper_public_key(&mut self, buffer: &mut [u8]) -> Result<usize, EfiStatus> {
        let mut size = buffer.len();
        let status = unsafe {
            (self.proto.read_secret_keeper_public_key)(
                self.proto,
                &raw mut size,
                buffer.as_mut_ptr(),
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }
}
