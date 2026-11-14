//! # `GUIDed` Section Extraction Protocol
//!
//! Provides services for extracting and authenticating encapsulated sections within
//! Firmware File System (FFS) files using format-specific algorithms identified by GUID.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::EfiStatus;

/// Attribute flag: Section requires processing before contents can be used.
pub const EFI_GUIDED_SECTION_PROCESSING_REQUIRED: u16 = 0x0001;
/// Attribute flag: Section contains authentication status information.
pub const EFI_GUIDED_SECTION_AUTH_STATUS_VALID: u16 = 0x0002;

/// Authentication bit: Platform security policy override was applied.
pub const EFI_AUTH_STATUS_PLATFORM_OVERRIDE: u32 = 0x0001;
/// Authentication bit: Encapsulated image or section is cryptographically signed.
pub const EFI_AUTH_STATUS_IMAGE_SIGNED: u32 = 0x0002;
/// Authentication bit: Signature verification was not tested.
pub const EFI_AUTH_STATUS_NOT_TESTED: u32 = 0x0004;
/// Authentication bit: Signature verification was executed and failed.
pub const EFI_AUTH_STATUS_TEST_FAILED: u32 = 0x0008;
/// Mask for all standard authentication bits.
pub const EFI_AUTH_STATUS_ALL: u32 = 0x000F;

/// Function pointer for extracting encapsulated section stream.
pub type EfiExtractGuidedSection = unsafe extern "efiapi" fn(
    this: *const EfiGuidedSectionExtractionProtocol,
    input_section: *const c_void,
    output_buffer: *mut *mut c_void,
    output_size: *mut UIntN,
    authentication_status: *mut u32,
) -> EfiStatus;

/// Low-level interface structure for `GUIDed` section extraction.
#[repr(C)]
pub struct EfiGuidedSectionExtractionProtocol {
    /// Extracts an encapsulated section stream from input buffer.
    pub extract_section: EfiExtractGuidedSection,
}

/// Extracted section buffer and authentication metadata.
#[derive(Debug, PartialEq, Eq)]
pub struct ExtractedSection {
    /// Pointer to extracted buffer in pool memory.
    pub buffer: *mut c_void,
    /// Size in bytes of extracted section stream.
    pub size: UIntN,
    /// Authentication flags resulting from verification.
    pub authentication_status: u32,
}

/// Safe client wrapper around [`EfiGuidedSectionExtractionProtocol`].
pub struct GuidedSectionExtraction<'a> {
    proto: &'a EfiGuidedSectionExtractionProtocol,
}

impl GuidedSectionExtraction<'_> {
    /// Wraps a raw pointer to an `EfiGuidedSectionExtractionProtocol`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `proto` points to a valid, live
    /// `EfiGuidedSectionExtractionProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiGuidedSectionExtractionProtocol) -> Self {
        unsafe { Self { proto: &*proto } }
    }

    /// Extracts encapsulated section stream from `input_section` slice.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if section processing or allocation fails.
    pub fn extract_section(&self, input_section: &[u8]) -> Result<ExtractedSection, EfiStatus> {
        let mut buffer: *mut c_void = core::ptr::null_mut();
        let mut size: UIntN = 0;
        let mut authentication_status: u32 = 0;

        let status = unsafe {
            (self.proto.extract_section)(
                self.proto,
                input_section.as_ptr().cast::<c_void>(),
                &raw mut buffer,
                &raw mut size,
                &raw mut authentication_status,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(ExtractedSection {
                buffer,
                size,
                authentication_status,
            })
        } else {
            Err(status)
        }
    }

    /// Extracts encapsulated section stream from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `input_section` points to valid, accessible memory
    /// of sufficient length for the encapsulated section header.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus` if extraction fails.
    pub unsafe fn extract_section_raw(
        &self,
        input_section: *const c_void,
    ) -> Result<ExtractedSection, EfiStatus> {
        let mut buffer: *mut c_void = core::ptr::null_mut();
        let mut size: UIntN = 0;
        let mut authentication_status: u32 = 0;

        let status = unsafe {
            (self.proto.extract_section)(
                self.proto,
                input_section,
                &raw mut buffer,
                &raw mut size,
                &raw mut authentication_status,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(ExtractedSection {
                buffer,
                size,
                authentication_status,
            })
        } else {
            Err(status)
        }
    }
}
