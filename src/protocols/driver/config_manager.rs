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

/// ARM Architecture Namespace Object IDs.
pub mod arm_object_id {
    /// Boot Architecture Info.
    pub const BOOT_ARCH_INFO: u32 = 1;
    /// GIC CPU Interface (GICC) Info.
    pub const GICC_INFO: u32 = 2;
    /// GIC Distributor (GICD) Info.
    pub const GICD_INFO: u32 = 3;
    /// GIC MSI Frame Info.
    pub const GIC_MSI_FRAME_INFO: u32 = 4;
    /// GIC Redistributor Info.
    pub const GIC_REDISTRIBUTOR_INFO: u32 = 5;
    /// GIC ITS Info.
    pub const GIC_ITS_INFO: u32 = 6;
    /// Generic Timer Info.
    pub const GENERIC_TIMER_INFO: u32 = 7;
    /// Platform GT Block Info.
    pub const PLATFORM_GT_BLOCK_INFO: u32 = 8;
    /// Generic Timer Block Timer Frame Info.
    pub const GT_BLOCK_TIMER_FRAME_INFO: u32 = 9;
    /// Platform Generic Watchdog.
    pub const PLATFORM_GENERIC_WATCHDOG_INFO: u32 = 10;
}

/// x86 / x64 Architecture Namespace Object IDs.
pub mod x64_object_id {
    /// FADT SCI Interrupt information.
    pub const FADT_SCI_INTERRUPT: u32 = 1;
    /// FADT SCI CMD information.
    pub const FADT_SCI_CMD_INFO: u32 = 2;
    /// FADT Power Management block info.
    pub const FADT_PM_BLOCK_INFO: u32 = 3;
    /// FADT GPE block info.
    pub const FADT_GPE_BLOCK_INFO: u32 = 4;
    /// HPET device info.
    pub const HPET_INFO: u32 = 11;
    /// MADT info.
    pub const MADT_INFO: u32 = 12;
    /// Local APIC and X2APIC info.
    pub const LOCAL_APIC_X2APIC_INFO: u32 = 13;
    /// IO APIC info.
    pub const IO_APIC_INFO: u32 = 14;
    /// Interrupt Source Override info.
    pub const INTR_SOURCE_OVERRIDE_INFO: u32 = 15;
    /// Local APIC and X2APIC NMI info.
    pub const LOCAL_APIC_X2APIC_NMI_INFO: u32 = 16;
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

/// Standard ACPI Table Information object.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CmStdObjAcpiTableInfo {
    /// Signature of the ACPI Table (e.g. `'FACP'`, `'APIC'`).
    pub acpi_table_signature: u32,
    /// ACPI table revision.
    pub acpi_table_revision: u8,
    /// ACPI table generator ID.
    pub table_generator_id: u32,
    /// Pointer to raw ACPI table data (if provided directly).
    pub acpi_table_data: *mut c_void,
    /// OEM table ID string as 64-bit value.
    pub oem_table_id: u64,
    /// OEM revision.
    pub oem_revision: u32,
    /// Minor table revision.
    pub minor_revision: u8,
}

/// Standard SMBIOS Table Information object.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CmStdObjSmbiosTableInfo {
    /// SMBIOS structure/table type (0-127 standard, 128-255 OEM).
    pub table_type: u8,
    /// SMBIOS table generator ID.
    pub table_generator_id: u32,
    /// Pointer to raw SMBIOS structure data.
    pub smbios_table_data: *mut c_void,
}

/// x64 Local APIC / x2APIC object descriptor.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CmX64ObjLocalApicInfo {
    /// ACPI processor ID.
    pub acpi_processor_id: u32,
    /// APIC ID.
    pub apic_id: u32,
    /// Flags (bit 0: Enabled).
    pub flags: u32,
}

/// x64 IO APIC object descriptor.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CmX64ObjIoApicInfo {
    /// IO APIC ID.
    pub io_apic_id: u8,
    /// Physical address of IO APIC.
    pub io_apic_address: u32,
    /// Global system interrupt base.
    pub global_system_interrupt_base: u32,
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

/// Dynamic Table Factory Protocol GUID.
pub const DYNAMIC_TABLE_FACTORY_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x91D1_E327,
    0xFE5A,
    0x49B8,
    [0xAB, 0x65, 0x0E, 0xCE, 0x2D, 0xDB, 0x45, 0xEC],
);

/// Dynamic Table Factory Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DynamicTableFactoryProtocol {
    /// Protocol revision.
    pub revision: u32,
    /// Requests an ACPI table generator by ID.
    pub get_acpi_table_generator: unsafe extern "efiapi" fn(
        this: *const DynamicTableFactoryProtocol,
        generator_id: u32,
        generator: *mut *const c_void,
    ) -> EfiStatus,
    /// Registers an ACPI table generator.
    pub register_acpi_table_generator:
        unsafe extern "efiapi" fn(generator: *const c_void) -> EfiStatus,
    /// Deregisters an ACPI table generator.
    pub deregister_acpi_table_generator:
        unsafe extern "efiapi" fn(generator: *const c_void) -> EfiStatus,
    /// Requests an SMBIOS table generator by ID.
    pub get_smbios_table_generator: unsafe extern "efiapi" fn(
        this: *const DynamicTableFactoryProtocol,
        generator_id: u32,
        generator: *mut *const c_void,
    ) -> EfiStatus,
    /// Registers an SMBIOS table generator.
    pub register_smbios_table_generator:
        unsafe extern "efiapi" fn(generator: *const c_void) -> EfiStatus,
    /// Deregisters an SMBIOS table generator.
    pub deregister_smbios_table_generator:
        unsafe extern "efiapi" fn(generator: *const c_void) -> EfiStatus,
}

/// High-level safe wrapper around [`DynamicTableFactoryProtocol`].
pub struct DynamicTableFactory<'a> {
    proto: &'a DynamicTableFactoryProtocol,
}

impl<'a> DynamicTableFactory<'a> {
    /// Wraps a raw [`DynamicTableFactoryProtocol`] reference.
    #[must_use]
    pub const fn new(proto: &'a DynamicTableFactoryProtocol) -> Self {
        Self { proto }
    }

    /// Queries an ACPI table generator by ID.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if not found or invalid parameter.
    pub fn get_acpi_generator(&self, generator_id: u32) -> Result<*const c_void, EfiStatus> {
        let mut generator = core::ptr::null();
        let status = unsafe {
            (self.proto.get_acpi_table_generator)(
                self.proto as *const _,
                generator_id,
                core::ptr::from_mut(&mut generator),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(generator)
        }
    }

    /// Queries an SMBIOS table generator by ID.
    ///
    /// # Errors
    /// Returns [`EfiStatus`] if not found or invalid parameter.
    pub fn get_smbios_generator(&self, generator_id: u32) -> Result<*const c_void, EfiStatus> {
        let mut generator = core::ptr::null();
        let status = unsafe {
            (self.proto.get_smbios_table_generator)(
                self.proto as *const _,
                generator_id,
                core::ptr::from_mut(&mut generator),
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(generator)
        }
    }
}
