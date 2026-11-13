//! Integration tests for Dynamic Tables Configuration Manager Protocol.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation
)]

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::driver::config_manager::{
    CM_NAMESPACE_ID_MASK, CM_NAMESPACE_ID_SHIFT, CM_NULL_TOKEN, CM_OBJECT_ID_MASK,
    CONFIGURATION_MANAGER_PROTOCOL_GUID, CONFIGURATION_MANAGER_PROTOCOL_REVISION, CmObjDescriptor,
    CmStdObjAcpiTableInfo, CmStdObjConfigurationManagerInfo, CmStdObjSmbiosTableInfo,
    CmX64ObjIoApicInfo, CmX64ObjLocalApicInfo, ConfigurationManager, ConfigurationManagerProtocol,
    DYNAMIC_TABLE_FACTORY_PROTOCOL_GUID, DynamicTableFactory, DynamicTableFactoryProtocol,
    arm_object_id, get_cm_namespace_id, get_cm_object_id, make_cm_object_id, namespace,
    std_object_id, x64_object_id,
};

#[test]
fn test_config_manager_guid_and_constants() {
    assert_eq!(
        CONFIGURATION_MANAGER_PROTOCOL_GUID,
        EfiGuid::new(
            0xd85a_4835,
            0x5a82,
            0x4894,
            [0xac, 0x02, 0x70, 0x6f, 0x43, 0xd5, 0x97, 0x8e]
        )
    );
    assert_eq!(CONFIGURATION_MANAGER_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(CM_NULL_TOKEN, 0);
    assert_eq!(CM_OBJECT_ID_MASK, 0xFF);
    assert_eq!(CM_NAMESPACE_ID_MASK, 0xF);
    assert_eq!(CM_NAMESPACE_ID_SHIFT, 28);
}

#[test]
fn test_object_id_packing_and_unpacking() {
    let std_info_id = make_cm_object_id(namespace::STANDARD, std_object_id::CFG_MGR_INFO);
    assert_eq!(get_cm_namespace_id(std_info_id), namespace::STANDARD);
    assert_eq!(get_cm_object_id(std_info_id), std_object_id::CFG_MGR_INFO);

    let arm_obj_id = make_cm_object_id(namespace::ARM, 0x45);
    assert_eq!(get_cm_namespace_id(arm_obj_id), namespace::ARM);
    assert_eq!(get_cm_object_id(arm_obj_id), 0x45);

    let oem_obj_id = make_cm_object_id(namespace::OEM, 0xAB);
    assert_eq!(get_cm_namespace_id(oem_obj_id), namespace::OEM);
    assert_eq!(get_cm_object_id(oem_obj_id), 0xAB);
}

static mut MOCK_CFG_INFO: CmStdObjConfigurationManagerInfo = CmStdObjConfigurationManagerInfo {
    revision: CONFIGURATION_MANAGER_PROTOCOL_REVISION,
    oem_id: *b"ROVER ",
};

unsafe extern "efiapi" fn mock_get_object(
    _this: *const ConfigurationManagerProtocol,
    cm_object_id: u32,
    _token: usize,
    cm_object: *mut CmObjDescriptor,
) -> EfiStatus {
    if cm_object.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let ns = get_cm_namespace_id(cm_object_id);
    let id = get_cm_object_id(cm_object_id);

    if ns == namespace::STANDARD && id == std_object_id::CFG_MGR_INFO {
        unsafe {
            *cm_object = CmObjDescriptor {
                object_id: cm_object_id,
                size: core::mem::size_of::<CmStdObjConfigurationManagerInfo>() as u32,
                data: (&raw mut MOCK_CFG_INFO).cast(),
                count: 1,
            };
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_set_object(
    _this: *const ConfigurationManagerProtocol,
    _cm_object_id: u32,
    _token: usize,
    _cm_object: *const CmObjDescriptor,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_configuration_manager_wrapper() {
    let proto = ConfigurationManagerProtocol {
        revision: CONFIGURATION_MANAGER_PROTOCOL_REVISION,
        get_object: mock_get_object,
        set_object: mock_set_object,
        plat_repo_info: core::ptr::null_mut(),
    };

    let client = ConfigurationManager::new(&proto);
    assert_eq!(client.revision(), CONFIGURATION_MANAGER_PROTOCOL_REVISION);

    let info = client.get_cfg_mgr_info().unwrap();
    let rev = info.revision;
    assert_eq!(rev, CONFIGURATION_MANAGER_PROTOCOL_REVISION);
    assert_eq!(&info.oem_id, b"ROVER ");

    let desc = CmObjDescriptor {
        object_id: 1,
        size: 0,
        data: core::ptr::null_mut(),
        count: 0,
    };
    client.set_object(1, CM_NULL_TOKEN, &desc).unwrap();
}

#[test]
fn test_arch_object_ids_and_table_structures() {
    let arm_gicc = make_cm_object_id(namespace::ARM, arm_object_id::GICC_INFO);
    assert_eq!(get_cm_namespace_id(arm_gicc), namespace::ARM);
    assert_eq!(get_cm_object_id(arm_gicc), arm_object_id::GICC_INFO);

    let x64_lapic = make_cm_object_id(namespace::X64, x64_object_id::LOCAL_APIC_X2APIC_INFO);
    assert_eq!(get_cm_namespace_id(x64_lapic), namespace::X64);
    assert_eq!(
        get_cm_object_id(x64_lapic),
        x64_object_id::LOCAL_APIC_X2APIC_INFO
    );

    let acpi_info = CmStdObjAcpiTableInfo {
        acpi_table_signature: 0x4350_4146, // 'FACP'
        acpi_table_revision: 6,
        table_generator_id: 1,
        acpi_table_data: core::ptr::null_mut(),
        oem_table_id: 0,
        oem_revision: 1,
        minor_revision: 0,
    };
    assert_eq!(acpi_info.acpi_table_revision, 6);

    let smbios_info = CmStdObjSmbiosTableInfo {
        table_type: 1,
        table_generator_id: 1,
        smbios_table_data: core::ptr::null_mut(),
    };
    assert_eq!(smbios_info.table_type, 1);

    let lapic = CmX64ObjLocalApicInfo {
        acpi_processor_id: 0,
        apic_id: 0,
        flags: 1,
    };
    let lapic_flags = lapic.flags;
    assert_eq!(lapic_flags, 1);

    let ioapic = CmX64ObjIoApicInfo {
        io_apic_id: 2,
        io_apic_address: 0xFEC0_0000,
        global_system_interrupt_base: 0,
    };
    assert_eq!(ioapic.io_apic_id, 2);
}

unsafe extern "efiapi" fn mock_get_acpi_gen(
    _this: *const DynamicTableFactoryProtocol,
    generator_id: u32,
    generator: *mut *const core::ffi::c_void,
) -> EfiStatus {
    if !generator.is_null() && generator_id == 1 {
        unsafe {
            *generator = 0x1234 as *const core::ffi::c_void;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_register_gen(_gen: *const core::ffi::c_void) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_smbios_gen(
    _this: *const DynamicTableFactoryProtocol,
    _generator_id: u32,
    _generator: *mut *const core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::NOT_FOUND
}

#[test]
fn test_dynamic_table_factory_protocol_and_wrapper() {
    assert_eq!(
        DYNAMIC_TABLE_FACTORY_PROTOCOL_GUID.to_string(),
        "91d1e327-fe5a-49b8-ab65-0ece2ddb45ec"
    );

    let factory_proto = DynamicTableFactoryProtocol {
        revision: 0x0001_0000,
        get_acpi_table_generator: mock_get_acpi_gen,
        register_acpi_table_generator: mock_register_gen,
        deregister_acpi_table_generator: mock_register_gen,
        get_smbios_table_generator: mock_get_smbios_gen,
        register_smbios_table_generator: mock_register_gen,
        deregister_smbios_table_generator: mock_register_gen,
    };

    let factory = DynamicTableFactory::new(&factory_proto);
    let table_gen = factory.get_acpi_generator(1).expect("generator 1 found");
    assert_eq!(table_gen as usize, 0x1234);

    assert_eq!(factory.get_acpi_generator(999), Err(EfiStatus::NOT_FOUND));
    assert_eq!(factory.get_smbios_generator(1), Err(EfiStatus::NOT_FOUND));
}
