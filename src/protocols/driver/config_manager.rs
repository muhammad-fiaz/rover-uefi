//! # Dynamic Tables Configuration Manager Protocol
//!
//! Provides the Configuration Manager protocol interface used for dynamically
//! discovering, querying, and updating hardware descriptions (ACPI tables, SMBIOS
//! structures, Device Trees) across standard, architecture-common, and architecture-specific
//! namespaces.

#![allow(
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::ref_as_ptr,
    clippy::borrow_as_ptr,
    clippy::ptr_as_ptr
)]

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Configuration Manager Protocol GUID.
///
/// `{d85a4835-5a82-4894-ac02-706f43d5978e}`
pub const CONFIGURATION_MANAGER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xd85a_4835,
    0x5a82,
    0x4894,
    [0xac, 0x02, 0x70, 0x6f, 0x43, 0xd5, 0x97, 0x8e],
);

/// Configuration Manager Protocol Revision 1.0.
pub const CONFIGURATION_MANAGER_PROTOCOL_REVISION: u32 = 0x0001_0000;

/// Mask for Configuration Manager Object ID (bits 7:0).
pub const CM_OBJECT_ID_MASK: u32 = 0x0000_00FF;

/// Mask for Configuration Manager Namespace ID (bits 31:28).
pub const CM_NAMESPACE_ID_MASK: u32 = 0x0000_000F;

/// Bit shift for Namespace ID.
pub const CM_NAMESPACE_ID_SHIFT: u32 = 28;

/// Null/empty object token.
pub const CM_NULL_TOKEN: usize = 0;

/// Configuration Manager Namespaces.
pub mod namespace {
    /// Standard objects namespace (e.g., table lists, info).
    pub const STANDARD: u32 = 0;
    /// Architecture common objects namespace.
    pub const ARCH_COMMON: u32 = 1;
    /// ARM architecture objects namespace.
    pub const ARM: u32 = 2;
    /// x86 / x64 architecture objects namespace.
    pub const X64: u32 = 3;
    /// RISC-V architecture objects namespace.
    pub const RISCV: u32 = 4;
    /// LoongArch64 architecture objects namespace.
    pub const LOONGARCH64: u32 = 5;
    /// Custom / OEM objects namespace.
    pub const OEM: u32 = 0x0F;
}

/// Standard Namespace Object IDs.
pub mod std_object_id {
    /// Configuration Manager Information (revision, OEM ID).
    pub const CFG_MGR_INFO: u32 = 0;
    /// ACPI Table Info List.
    pub const ACPI_TABLE_LIST: u32 = 1;
    /// SMBIOS Table Info List.
    pub const SMBIOS_TABLE_LIST: u32 = 2;
}

/// Constructs a 32-bit Configuration Manager Object ID from namespace and object numbers.
#[must_use]
pub const fn make_cm_object_id(namespace_id: u32, object_id: u32) -> u32 {
    ((namespace_id & CM_NAMESPACE_ID_MASK) << CM_NAMESPACE_ID_SHIFT)
        | (object_id & CM_OBJECT_ID_MASK)
}

/// Extracts the namespace ID from a 32-bit Configuration Manager Object ID.
#[must_use]
pub const fn get_cm_namespace_id(cm_object_id: u32) -> u32 {
    (cm_object_id >> CM_NAMESPACE_ID_SHIFT) & CM_NAMESPACE_ID_MASK
}

/// Extracts the object ID from a 32-bit Configuration Manager Object ID.
#[must_use]
pub const fn get_cm_object_id(cm_object_id: u32) -> u32 {
    cm_object_id & CM_OBJECT_ID_MASK
}

/// Descriptor for Configuration Manager Objects.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CmObjDescriptor {
    /// Configuration Manager Object ID.
    pub object_id: u32,
    /// Size in bytes of the described object or object list.
    pub size: u32,
    /// Pointer to the described object or object list.
    pub data: *mut c_void,
    /// Count of objects in the list.
    pub count: u32,
}

impl Default for CmObjDescriptor {
    fn default() -> Self {
        Self {
            object_id: 0,
            size: 0,
            data: core::ptr::null_mut(),
            count: 0,
        }
    }
}

/// Standard Configuration Manager Information.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CmStdObjConfigurationManagerInfo {
    /// Configuration Manager Revision.
    pub revision: u32,
    /// 6-byte OEM ID used to populate ACPI/SMBIOS table headers.
    pub oem_id: [u8; 6],
}

/// Raw function pointer for querying a Configuration Manager object.
pub type ConfigurationManagerGetObject = unsafe extern "efiapi" fn(
    this: *const ConfigurationManagerProtocol,
    cm_object_id: u32,
    token: usize,
    cm_object: *mut CmObjDescriptor,
) -> EfiStatus;

/// Raw function pointer for updating a Configuration Manager object.
pub type ConfigurationManagerSetObject = unsafe extern "efiapi" fn(
    this: *const ConfigurationManagerProtocol,
    cm_object_id: u32,
    token: usize,
    cm_object: *const CmObjDescriptor,
) -> EfiStatus;

/// Dynamic Tables Configuration Manager Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConfigurationManagerProtocol {
    /// Configuration Manager Protocol revision.
    pub revision: u32,
    /// Interface used to request information about Configuration Manager objects.
    pub get_object: ConfigurationManagerGetObject,
    /// Interface used to update information stored in the repository.
    pub set_object: ConfigurationManagerSetObject,
    /// Pointer to implementation-defined abstract platform repository.
    pub plat_repo_info: *mut c_void,
}

/// High-level safe wrapper around [`ConfigurationManagerProtocol`].
pub struct ConfigurationManager<'a> {
    proto: &'a ConfigurationManagerProtocol,
}

impl<'a> ConfigurationManager<'a> {
    /// Wraps a raw [`ConfigurationManagerProtocol`] reference.
    #[must_use]
    pub const fn new(proto: &'a ConfigurationManagerProtocol) -> Self {
        Self { proto }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u32 {
        self.proto.revision
    }

    /// Queries a Configuration Manager object descriptor.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if the object is not found, buffer size is invalid,
    /// or on hardware error.
    pub fn get_object(
        &self,
        cm_object_id: u32,
        token: usize,
    ) -> Result<CmObjDescriptor, EfiStatus> {
        let mut desc = CmObjDescriptor::default();
        let status = unsafe {
            (self.proto.get_object)(self.proto as *const _, cm_object_id, token, &mut desc)
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(desc)
        }
    }

    /// Updates a Configuration Manager object descriptor.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if unsupported, invalid parameter, or on repository failure.
    pub fn set_object(
        &self,
        cm_object_id: u32,
        token: usize,
        descriptor: &CmObjDescriptor,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.set_object)(self.proto as *const _, cm_object_id, token, descriptor)
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Convenience method to retrieve standard Configuration Manager info.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if query fails or descriptor size is smaller than expected.
    pub fn get_cfg_mgr_info(&self) -> Result<CmStdObjConfigurationManagerInfo, EfiStatus> {
        let obj_id = make_cm_object_id(namespace::STANDARD, std_object_id::CFG_MGR_INFO);
        let desc = self.get_object(obj_id, CM_NULL_TOKEN)?;
        if desc.size < core::mem::size_of::<CmStdObjConfigurationManagerInfo>() as u32
            || desc.data.is_null()
        {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        }
        let info_ptr = desc.data.cast::<CmStdObjConfigurationManagerInfo>();
        let info = unsafe { core::ptr::read_unaligned(info_ptr) };
        Ok(info)
    }
}
