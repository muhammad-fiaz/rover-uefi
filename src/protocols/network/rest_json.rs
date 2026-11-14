//! # REST JSON Structure Protocol
//!
//! Provides services to convert between JSON text and native C/Rust structures
//! for REST and Redfish services in UEFI. Defined in UEFI Specification 2.8+.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// REST JSON Structure Protocol GUID: `{a9a048f6-48a0-4714-b7da-a9ad87d4dac9}`.
pub const EFI_REST_JSON_STRUCTURE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA9A0_48F6,
    0x48A0,
    0x4714,
    [0xB7, 0xDA, 0xA9, 0xAD, 0x87, 0xD4, 0xDA, 0xC9],
);

/// REST JSON resource namespace specification.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiRestJsonResourceTypeNamespace {
    /// Resource type name (null-terminated ASCII).
    pub resource_type_name: *const u8,
    /// Resource major version string.
    pub major_version: *const u8,
    /// Resource minor version string.
    pub minor_version: *const u8,
    /// Resource errata version string.
    pub errata_version: *const u8,
}

/// REST JSON resource type identifier combining namespace and datatype.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiRestJsonResourceTypeIdentifier {
    /// Namespace of the resource type.
    pub name_space: EfiRestJsonResourceTypeNamespace,
    /// Name of datatype declared in this resource type (null-terminated ASCII).
    pub data_type: *const u8,
}

/// Header for parsed REST JSON structures.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiRestJsonStructureHeader {
    /// Resource type identifier.
    pub json_rsrc_identifier: EfiRestJsonResourceTypeIdentifier,
    /// Pointer to native payload structure.
    pub json_structure_pointer: *mut c_void,
}

/// Linked entry indicating supported JSON conversions.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiRestJsonStructureSupported {
    /// Forward and backward list links (`LIST_ENTRY`).
    pub next_supported_rsrc_interp: [*mut c_void; 2],
    /// JSON resource type this converter supports.
    pub rest_resource_interp: EfiRestJsonResourceTypeIdentifier,
}

/// Function pointer for `EFI_REST_JSON_STRUCTURE_TO_STRUCTURE`.
pub type EfiRestJsonStructureToStructure = unsafe extern "efiapi" fn(
    this: *mut EfiRestJsonStructureProtocol,
    json_rsrc_identifier: *const EfiRestJsonResourceTypeIdentifier,
    resource_json_text: *const u8,
    json_structure: *mut *mut EfiRestJsonStructureHeader,
) -> EfiStatus;

/// Function pointer for `EFI_REST_JSON_STRUCTURE_TO_JSON`.
pub type EfiRestJsonStructureToJson = unsafe extern "efiapi" fn(
    this: *mut EfiRestJsonStructureProtocol,
    json_structure_header: *mut EfiRestJsonStructureHeader,
    resource_json_text: *mut *mut u8,
) -> EfiStatus;

/// Function pointer for `EFI_REST_JSON_STRUCTURE_DESTORY_STRUCTURE`.
pub type EfiRestJsonStructureDestroyStructure = unsafe extern "efiapi" fn(
    this: *mut EfiRestJsonStructureProtocol,
    json_structure_header: *mut EfiRestJsonStructureHeader,
) -> EfiStatus;

/// Function pointer for `EFI_REST_JSON_STRUCTURE_REGISTER`.
pub type EfiRestJsonStructureRegister = unsafe extern "efiapi" fn(
    this: *mut EfiRestJsonStructureProtocol,
    json_structure_supported: *mut EfiRestJsonStructureSupported,
    to_structure: EfiRestJsonStructureToStructure,
    to_json: EfiRestJsonStructureToJson,
    destroy_structure: EfiRestJsonStructureDestroyStructure,
) -> EfiStatus;

/// Raw ABI structure for `EFI_REST_JSON_STRUCTURE_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiRestJsonStructureProtocol {
    /// Registers a JSON-to-structure converter.
    pub register: EfiRestJsonStructureRegister,
    /// Converts JSON text into a native structure.
    pub to_structure: EfiRestJsonStructureToStructure,
    /// Converts a native structure into JSON text.
    pub to_json: EfiRestJsonStructureToJson,
    /// Destroys and deallocates a native structure.
    pub destory_structure: EfiRestJsonStructureDestroyStructure,
}

/// Safe client wrapper around [`EfiRestJsonStructureProtocol`].
pub struct RestJsonStructure<'a> {
    proto: &'a mut EfiRestJsonStructureProtocol,
}

impl<'a> RestJsonStructure<'a> {
    /// Creates a new `RestJsonStructure` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiRestJsonStructureProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiRestJsonStructureProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Converts JSON text into a native structure.
    ///
    /// # Safety
    ///
    /// `json_text` must be null-terminated ASCII.
    ///
    /// # Errors
    ///
    /// Returns an error status if conversion fails.
    pub unsafe fn to_structure_raw(
        &mut self,
        identifier: Option<&EfiRestJsonResourceTypeIdentifier>,
        json_text: *const u8,
    ) -> Result<*mut EfiRestJsonStructureHeader, EfiStatus> {
        let mut out: *mut EfiRestJsonStructureHeader = core::ptr::null_mut();
        let id_ptr = match identifier {
            Some(id) => core::ptr::from_ref(id),
            None => core::ptr::null(),
        };
        let status = unsafe {
            (self.proto.to_structure)(self.proto, id_ptr, json_text, core::ptr::from_mut(&mut out))
        };
        if status == EfiStatus::SUCCESS {
            Ok(out)
        } else {
            Err(status)
        }
    }

    /// Converts a structure header into JSON text.
    ///
    /// # Safety
    ///
    /// `header` must be a valid structure header previously produced or initialized.
    ///
    /// # Errors
    ///
    /// Returns an error status if conversion fails.
    pub unsafe fn to_json_raw(
        &mut self,
        header: *mut EfiRestJsonStructureHeader,
    ) -> Result<*mut u8, EfiStatus> {
        let mut out: *mut u8 = core::ptr::null_mut();
        let status =
            unsafe { (self.proto.to_json)(self.proto, header, core::ptr::from_mut(&mut out)) };
        if status == EfiStatus::SUCCESS {
            Ok(out)
        } else {
            Err(status)
        }
    }

    /// Destroys a parsed structure.
    ///
    /// # Safety
    ///
    /// `header` must be a valid structure allocated by this protocol.
    ///
    /// # Errors
    ///
    /// Returns an error status if destruction fails.
    pub unsafe fn destroy_structure(
        &mut self,
        header: *mut EfiRestJsonStructureHeader,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.destory_structure)(self.proto, header) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
