//! # HII Configuration Keyword Handler Protocol
//!
//! Provides services to set and get values associated with configuration keywords
//! exposed through an `x-UEFI-` prefixed configuration language namespace.
//! Defined in UEFI Specification 2.5+.

use crate::base::{EfiGuid, EfiStatus};

/// HII Configuration Keyword Handler Protocol GUID: `{0a8badd5-03b8-4d19-b128-7b8f0edaa596}`.
pub const EFI_CONFIG_KEYWORD_HANDLER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0A8B_ADD5,
    0x03B8,
    0x4D19,
    [0xB1, 0x28, 0x7B, 0x8F, 0x0E, 0xDA, 0xA5, 0x96],
);

/// Progress error code: no error occurred.
pub const KEYWORD_HANDLER_NO_ERROR: u32 = 0x0000_0000;
/// Progress error code: configuration namespace was not found.
pub const KEYWORD_HANDLER_NAMESPACE_ID_NOT_FOUND: u32 = 0x0000_0001;
/// Progress error code: keyword string is malformed.
pub const KEYWORD_HANDLER_MALFORMED_STRING: u32 = 0x0000_0002;
/// Progress error code: requested keyword was not found.
pub const KEYWORD_HANDLER_KEYWORD_NOT_FOUND: u32 = 0x0000_0004;
/// Progress error code: incompatible value detected for keyword.
pub const KEYWORD_HANDLER_INCOMPATIBLE_VALUE_DETECTED: u32 = 0x0000_0008;
/// Progress error code: access to keyword setting is not permitted by policy.
pub const KEYWORD_HANDLER_ACCESS_NOT_PERMITTED: u32 = 0x0000_0010;
/// Progress error code: undefined or internal processing error.
pub const KEYWORD_HANDLER_UNDEFINED_PROCESSING_ERROR: u32 = 0x8000_0000;

/// Function pointer for `EFI_CONFIG_KEYWORD_HANDLER_SET_DATA`.
pub type EfiConfigKeywordHandlerSetData = unsafe extern "efiapi" fn(
    this: *mut EfiConfigKeywordHandlerProtocol,
    keyword_string: *const u16,
    progress: *mut *mut u16,
    progress_err: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_CONFIG_KEYWORD_HANDLER_GET_DATA`.
pub type EfiConfigKeywordHandlerGetData = unsafe extern "efiapi" fn(
    this: *mut EfiConfigKeywordHandlerProtocol,
    namespace_id: *const u16,
    keyword_string: *const u16,
    progress: *mut *mut u16,
    progress_err: *mut u32,
    results: *mut *mut u16,
) -> EfiStatus;

/// Raw ABI structure for `EFI_CONFIG_KEYWORD_HANDLER_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiConfigKeywordHandlerProtocol {
    /// Sets keyword values in system storage.
    pub set_data: EfiConfigKeywordHandlerSetData,
    /// Retrieves keyword values from system storage.
    pub get_data: EfiConfigKeywordHandlerGetData,
}

/// Safe client wrapper around [`EfiConfigKeywordHandlerProtocol`].
pub struct ConfigKeywordHandler<'a> {
    proto: &'a mut EfiConfigKeywordHandlerProtocol,
}

impl<'a> ConfigKeywordHandler<'a> {
    /// Creates a new `ConfigKeywordHandler` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiConfigKeywordHandlerProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiConfigKeywordHandlerProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Sets keyword values in system storage from a `<MultiKeywordResp>` string.
    ///
    /// # Safety
    ///
    /// `keyword_string` must be a null-terminated UCS-2 string.
    ///
    /// # Errors
    ///
    /// Returns an error status if processing fails, along with progress pointer and error code.
    pub unsafe fn set_data_raw(
        &mut self,
        keyword_string: *const u16,
    ) -> Result<(), (EfiStatus, *mut u16, u32)> {
        let mut progress: *mut u16 = core::ptr::null_mut();
        let mut progress_err: u32 = KEYWORD_HANDLER_NO_ERROR;
        let status = unsafe {
            (self.proto.set_data)(
                self.proto,
                keyword_string,
                core::ptr::from_mut(&mut progress),
                core::ptr::from_mut(&mut progress_err),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err((status, progress, progress_err))
        }
    }

    /// Retrieves keyword values from system storage for requested keywords.
    ///
    /// # Safety
    ///
    /// `namespace_id` and `keyword_string` (if non-null) must be null-terminated UCS-2 strings.
    ///
    /// # Errors
    ///
    /// Returns an error status if retrieval fails, along with progress info.
    pub unsafe fn get_data_raw(
        &mut self,
        namespace_id: *const u16,
        keyword_string: *const u16,
    ) -> Result<*mut u16, (EfiStatus, *mut u16, u32)> {
        let mut progress: *mut u16 = core::ptr::null_mut();
        let mut progress_err: u32 = KEYWORD_HANDLER_NO_ERROR;
        let mut results: *mut u16 = core::ptr::null_mut();
        let status = unsafe {
            (self.proto.get_data)(
                self.proto,
                namespace_id,
                keyword_string,
                core::ptr::from_mut(&mut progress),
                core::ptr::from_mut(&mut progress_err),
                core::ptr::from_mut(&mut results),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(results)
        } else {
            Err((status, progress, progress_err))
        }
    }
}
