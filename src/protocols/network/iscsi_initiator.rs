//! # iSCSI Initiator Name Protocol
//!
//! Defined by UEFI Specification Section 24.2: iSCSI Initiator Name Protocol.
//!
//! Provides services to retrieve and configure the iSCSI Initiator Name for
//! storage area network (SAN) and remote boot configurations.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_ISCSI_INITIATOR_NAME_PROTOCOL_GUID`.
pub const EFI_ISCSI_INITIATOR_NAME_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5932_4945,
    0xEC44,
    0x4C0D,
    [0xB1, 0xCD, 0x9D, 0xB1, 0x39, 0xDF, 0x07, 0x0C],
);

/// Maximum length of an iSCSI initiator name in bytes including null terminator (RFC 3720).
pub const ISCSI_MAX_INITIATOR_NAME_LEN: usize = 224;

/// Function pointer for `EFI_ISCSI_INITIATOR_NAME_GET`.
pub type EfiIScsiInitiatorNameGet = unsafe extern "efiapi" fn(
    this: *mut EfiIScsiInitiatorNameProtocol,
    buffer_size: *mut UIntN,
    buffer: *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_ISCSI_INITIATOR_NAME_SET`.
pub type EfiIScsiInitiatorNameSet = unsafe extern "efiapi" fn(
    this: *mut EfiIScsiInitiatorNameProtocol,
    buffer_size: *mut UIntN,
    buffer: *const c_void,
) -> EfiStatus;

/// Raw ABI structure for `EFI_ISCSI_INITIATOR_NAME_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiIScsiInitiatorNameProtocol {
    /// Retrieves the current iSCSI Initiator Name.
    pub get: EfiIScsiInitiatorNameGet,
    /// Sets the current iSCSI Initiator Name.
    pub set: EfiIScsiInitiatorNameSet,
}

/// Safe client wrapper around `EfiIScsiInitiatorNameProtocol`.
pub struct IScsiInitiatorName<'a> {
    proto: &'a mut EfiIScsiInitiatorNameProtocol,
}

impl<'a> IScsiInitiatorName<'a> {
    /// Creates a new `IScsiInitiatorName` client wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiIScsiInitiatorNameProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiIScsiInitiatorNameProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiIScsiInitiatorNameProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiIScsiInitiatorNameProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiIScsiInitiatorNameProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Retrieves the current iSCSI initiator name into `buffer`.
    ///
    /// Returns the number of bytes written (including null terminator).
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if `buffer` is insufficient, or an error status.
    pub fn get(&mut self, buffer: &mut [u8]) -> Result<usize, EfiStatus> {
        let mut size = buffer.len() as UIntN;
        let status = unsafe {
            (self.proto.get)(
                self.proto,
                core::ptr::from_mut(&mut size),
                buffer.as_mut_ptr().cast::<c_void>(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(size as usize)
        } else {
            Err(status)
        }
    }

    /// Retrieves the current iSCSI initiator name as a UTF-8 string slice.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if `buffer` is too small, `INVALID_PARAMETER` if string is
    /// not valid UTF-8, or a hardware error status.
    pub fn get_name<'b>(&mut self, buffer: &'b mut [u8]) -> Result<&'b str, EfiStatus> {
        let len = self.get(buffer)?;
        let mut trimmed_len = len;
        while trimmed_len > 0 && buffer[trimmed_len - 1] == 0 {
            trimmed_len -= 1;
        }
        core::str::from_utf8(&buffer[..trimmed_len]).map_err(|_| EfiStatus::INVALID_PARAMETER)
    }

    /// Sets the iSCSI initiator name.
    ///
    /// The string must not exceed 223 characters.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if name is too long, `PROTOCOL_ERROR` if format is invalid,
    /// or an error status on hardware failure.
    pub fn set(&mut self, name: &str) -> Result<(), EfiStatus> {
        let bytes = name.as_bytes();
        if bytes.len() >= ISCSI_MAX_INITIATOR_NAME_LEN {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let mut buf = [0u8; ISCSI_MAX_INITIATOR_NAME_LEN];
        buf[..bytes.len()].copy_from_slice(bytes);
        buf[bytes.len()] = 0; // Null-terminated

        let mut size = (bytes.len() + 1) as UIntN;
        let status = unsafe {
            (self.proto.set)(
                self.proto,
                core::ptr::from_mut(&mut size),
                buf.as_ptr().cast::<c_void>(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
