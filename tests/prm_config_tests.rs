//! Integration tests for Platform Runtime Mechanism (PRM) configuration.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::firmware::prm::{
    AcpiParameterBufferDescriptor, PRM_ACPI_PARAMETER_DATA_BUFFER_HEADER_SIGNATURE,
    PRM_CONFIG_PROTOCOL_GUID, PRM_CONFIG_PROTOCOL_SIGNATURE, PRM_CONFIG_PROTOCOL_VERSION,
    PRM_CONTEXT_BUFFER_INTERFACE_VERSION, PRM_CONTEXT_BUFFER_SIGNATURE,
    PRM_DATA_BUFFER_HEADER_SIGNATURE, PrmConfig, PrmConfigProtocol, PrmContextBuffer,
    PrmDataBufferHeader, PrmModuleContextBuffers,
};

#[test]
fn test_prm_constants_and_guids() {
    assert_eq!(
        PRM_CONFIG_PROTOCOL_GUID,
        EfiGuid::new(
            0x4E5B_4FEA,
            0x936A,
            0x45BC,
            [0xAC, 0x6A, 0x2F, 0x8F, 0x14, 0xA6, 0xC2, 0x9E],
        )
    );
    assert_eq!(PRM_CONFIG_PROTOCOL_SIGNATURE, 0x504D_4350);
    assert_eq!(PRM_CONFIG_PROTOCOL_VERSION, 1);
    assert_eq!(PRM_CONTEXT_BUFFER_SIGNATURE, 0x434D_5250);
    assert_eq!(PRM_CONTEXT_BUFFER_INTERFACE_VERSION, 1);
}

#[test]
fn test_prm_config_wrapper_and_validation() {
    let module_guid = EfiGuid::new(0x1111_2222, 0x3333, 0x4444, [1, 2, 3, 4, 5, 6, 7, 8]);
    let handler_guid = EfiGuid::new(0xAAAA_BBBB, 0xCCCC, 0xDDDD, [8, 7, 6, 5, 4, 3, 2, 1]);

    let mut acpi_desc = AcpiParameterBufferDescriptor {
        handler_guid,
        acpi_parameter_buffer_address: 0x8000_1000,
    };

    let proto = PrmConfigProtocol {
        module_context_buffers: PrmModuleContextBuffers {
            module_guid,
            buffer_count: 2,
            buffer: core::ptr::null_mut(),
            runtime_mmio_ranges: core::ptr::null_mut(),
            acpi_parameter_buffer_descriptor_count: 1,
            acpi_parameter_buffer_descriptors: &raw mut acpi_desc,
        },
    };

    let prm = PrmConfig::new(&proto);
    assert_eq!(prm.module_guid(), module_guid);
    assert_eq!(prm.buffer_count(), 2);
    assert_eq!(prm.acpi_descriptor_count(), 1);

    let found_addr = prm.find_acpi_buffer(&handler_guid);
    assert_eq!(found_addr, Some(0x8000_1000));

    let unknown_guid = EfiGuid::new(0, 0, 0, [0; 8]);
    assert_eq!(prm.find_acpi_buffer(&unknown_guid), None);

    let valid_ctx = PrmContextBuffer {
        signature: PRM_CONTEXT_BUFFER_SIGNATURE,
        version: PRM_CONTEXT_BUFFER_INTERFACE_VERSION,
        reserved: 0,
        handler_guid,
        static_data_buffer: core::ptr::null_mut(),
        runtime_mmio_ranges: core::ptr::null_mut(),
    };
    assert!(PrmConfig::validate_context_buffer(&valid_ctx));

    let invalid_ctx = PrmContextBuffer {
        signature: 0x1234,
        version: 0,
        reserved: 0,
        handler_guid,
        static_data_buffer: core::ptr::null_mut(),
        runtime_mmio_ranges: core::ptr::null_mut(),
    };
    assert!(!PrmConfig::validate_context_buffer(&invalid_ctx));

    let valid_hdr = PrmDataBufferHeader {
        signature: PRM_DATA_BUFFER_HEADER_SIGNATURE,
        length: 64,
    };
    assert!(PrmConfig::validate_data_buffer_header(&valid_hdr));

    let valid_acpi_hdr = PrmDataBufferHeader {
        signature: PRM_ACPI_PARAMETER_DATA_BUFFER_HEADER_SIGNATURE,
        length: 32,
    };
    assert!(PrmConfig::validate_data_buffer_header(&valid_acpi_hdr));
}
