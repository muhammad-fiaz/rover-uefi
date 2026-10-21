//! # Variables
//!
//! Variable storage and enumeration.
//!
//! Variable names are null-terminated UTF-16
//! strings. The safe methods take `Char16`
//! slices that must already include the trailing
//! null. No internal allocation occurs; all
//! buffers are caller-provided. Size
//! calculations use checked arithmetic. The
//! `BUFFER_TOO_SMALL` status preserves the
//! firmware-reported required size in the
//! returned length.

use crate::abi::types::{Char16, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use crate::services::runtime::RuntimeServices;

/// Variable is non-volatile.
pub const VARIABLE_NON_VOLATILE: u32 = 0x0000_0001;

/// Variable is accessible in boot services.
pub const VARIABLE_BOOTSERVICE_ACCESS: u32 = 0x0000_0002;

/// Variable is accessible in runtime services.
pub const VARIABLE_RUNTIME_ACCESS: u32 = 0x0000_0004;

/// Variable is a hardware error record.
pub const VARIABLE_HARDWARE_ERROR_RECORD: u32 = 0x0000_0008;

/// Variable uses authenticated write access.
pub const VARIABLE_AUTHENTICATED_WRITE_ACCESS: u32 = 0x0000_0010;

/// Variable uses time-based authenticated writes.
pub const VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS: u32 = 0x0000_0020;

/// Variable appends on write.
pub const VARIABLE_APPEND_WRITE: u32 = 0x0000_0040;

/// Returns whether the name is a valid
/// null-terminated UTF-16 name.
///
/// The name must be non-empty and end with
/// zero. No other validation is performed.
#[must_use]
pub fn validate_variable_name(name: &[Char16]) -> bool {
    !name.is_empty() && name.last().is_some_and(|last| *last == 0)
}

/// Returns the byte size of a UTF-16 name.
///
/// Returns `None` on overflow.
#[must_use]
pub fn variable_name_bytes(name: &[Char16]) -> Option<UIntN> {
    name.len().checked_mul(core::mem::size_of::<Char16>())
}

/// The result of a variable retrieval.
pub struct VariableResult {
    /// The firmware status.
    pub status: EfiStatus,
    /// The variable attributes.
    pub attributes: u32,
    /// The data size in bytes.
    pub data_size: UIntN,
}

impl core::fmt::Debug for VariableResult {
    /// Formats the result as its fields.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("VariableResult")
            .field("status", &self.status)
            .field("attributes", &self.attributes)
            .field("data_size", &self.data_size)
            .finish()
    }
}

impl RuntimeServices<'_> {
    /// Retrieves a variable value.
    ///
    /// `name` must include the trailing null.
    /// `data` must be large enough. When the
    /// buffer is too small the status is
    /// `BUFFER_TOO_SMALL` and `data_size` holds
    /// the required size.
    #[must_use]
    pub fn get_variable(&self, name: &[Char16], guid: &EfiGuid, data: &mut [u8]) -> VariableResult {
        let mut attributes: u32 = 0;
        let mut data_size = data.len();
        // SAFETY: All pointers are derived from valid slices and stack locals for the duration of the call.
        let status = unsafe {
            (self.table.get_variable)(
                name.as_ptr().cast_mut(),
                core::ptr::from_ref(guid).cast_mut(),
                &raw mut attributes,
                &raw mut data_size,
                data.as_mut_ptr().cast(),
            )
        };
        VariableResult {
            status,
            attributes,
            data_size,
        }
    }

    /// Enumerates variable names.
    ///
    /// `name` and `guid` hold the previous
    /// entry on input and the next entry on
    /// output. For the first entry the caller
    /// provides a buffer sized for the data;
    /// firmware ignores the initial contents
    /// when the name is empty per platform
    /// behavior. The returned length is the
    /// updated name size in bytes.
    #[must_use]
    pub fn get_next_variable_name(
        &self,
        name: &mut [Char16],
        guid: &mut EfiGuid,
    ) -> (EfiStatus, UIntN) {
        let Some(name_bytes) = variable_name_bytes(name) else {
            return (EfiStatus::INVALID_PARAMETER, 0);
        };
        let mut name_size = name_bytes;
        // SAFETY: All pointers are derived from valid slices and references for the duration of the call.
        let status = unsafe {
            (self.table.get_next_variable_name)(
                &raw mut name_size,
                name.as_mut_ptr(),
                core::ptr::from_mut(guid),
            )
        };
        (status, name_size)
    }

    /// Stores a variable value.
    ///
    /// An empty `data` slice has size zero and
    /// deletes the variable unless the append
    /// or authentication attributes are set.
    #[must_use]
    pub fn set_variable(
        &self,
        name: &[Char16],
        guid: &EfiGuid,
        attributes: u32,
        data: &[u8],
    ) -> EfiStatus {
        // SAFETY: All pointers are derived from valid slices and references for the duration of the call.
        unsafe {
            (self.table.set_variable)(
                name.as_ptr().cast_mut(),
                core::ptr::from_ref(guid).cast_mut(),
                attributes,
                data.len(),
                data.as_ptr().cast::<core::ffi::c_void>().cast_mut(),
            )
        }
    }

    /// Queries variable storage information.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// query fails.
    pub fn query_variable_info(&self, attributes: u32) -> Result<(u64, u64, u64), EfiStatus> {
        let mut maximum_storage: u64 = 0;
        let mut remaining_storage: u64 = 0;
        let mut maximum_size: u64 = 0;
        // SAFETY: All stack locals are valid for their out parameters.
        let status = unsafe {
            (self.table.query_variable_info)(
                attributes,
                &raw mut maximum_storage,
                &raw mut remaining_storage,
                &raw mut maximum_size,
            )
        };
        if status.is_success() {
            Ok((maximum_storage, remaining_storage, maximum_size))
        } else {
            Err(status)
        }
    }
}
