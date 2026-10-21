//! Tests for PRM and FSP firmware structures.

use core::ffi::c_void;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::fsp::{
    FSP_IA32, FSP_INFO_HEADER_OFFSET, FSP_INFO_HEADER_SIGNATURE, FSP_X64, FspInfoHeader,
    FspNotifyParams, FspPhase,
};
use rover_uefi::protocols::firmware::prm::{
    PRM_ACPI_PARAMETER_DATA_BUFFER_HEADER_SIGNATURE, PRM_CONFIG_PROTOCOL_GUID,
    PRM_CONFIG_PROTOCOL_SIGNATURE, PRM_CONFIG_PROTOCOL_VERSION,
    PRM_CONTEXT_BUFFER_INTERFACE_VERSION, PRM_CONTEXT_BUFFER_SIGNATURE,
    PRM_DATA_BUFFER_HEADER_SIGNATURE, PrmConfigProtocol, PrmContextBuffer, PrmModuleContextBuffers,
};

#[test]
fn test_prm_constants_and_guids() {
    assert_eq!(PRM_CONFIG_PROTOCOL_GUID.data1, 0x4E5B_4FEA);
    assert_eq!(PRM_CONFIG_PROTOCOL_SIGNATURE, 0x504D_4350);
    assert_eq!(PRM_CONFIG_PROTOCOL_VERSION, 1);
    assert_eq!(PRM_CONTEXT_BUFFER_SIGNATURE, 0x434D_5250);
    assert_eq!(PRM_CONTEXT_BUFFER_INTERFACE_VERSION, 1);
    assert_eq!(PRM_DATA_BUFFER_HEADER_SIGNATURE, 0x534D_5250);
    assert_eq!(PRM_ACPI_PARAMETER_DATA_BUFFER_HEADER_SIGNATURE, 0x504D_5250);
}

unsafe extern "efiapi" fn dummy_prm_handler(
    _parameter_buffer: *mut c_void,
    _context_buffer: *mut PrmContextBuffer,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_prm_structures() {
    let handler_guid = EfiGuid::new(1, 2, 3, [4, 5, 6, 7, 8, 9, 10, 11]);
    let mut context_buf = PrmContextBuffer {
        signature: PRM_CONTEXT_BUFFER_SIGNATURE,
        version: PRM_CONTEXT_BUFFER_INTERFACE_VERSION,
        reserved: 0,
        handler_guid,
        static_data_buffer: core::ptr::null_mut(),
        runtime_mmio_ranges: core::ptr::null_mut(),
    };

    let status = unsafe { dummy_prm_handler(core::ptr::null_mut(), &raw mut context_buf) };
    assert_eq!(status, EfiStatus::SUCCESS);

    let module_buffers = PrmModuleContextBuffers {
        module_guid: handler_guid,
        buffer_count: 1,
        buffer: &raw mut context_buf,
        runtime_mmio_ranges: core::ptr::null_mut(),
        acpi_parameter_buffer_descriptor_count: 0,
        acpi_parameter_buffer_descriptors: core::ptr::null_mut(),
    };

    let prm_config = PrmConfigProtocol {
        module_context_buffers: module_buffers,
    };
    let count = prm_config.module_context_buffers.buffer_count;
    assert_eq!(count, 1);
}

#[test]
fn test_fsp_constants_and_header() {
    assert_eq!(FSP_INFO_HEADER_OFFSET, 0x94);
    assert_eq!(FSP_INFO_HEADER_SIGNATURE, 0x4850_5346);
    assert_eq!(FSP_IA32, 0);
    assert_eq!(FSP_X64, 1);

    let header = FspInfoHeader {
        signature: FSP_INFO_HEADER_SIGNATURE,
        header_length: u32::try_from(core::mem::size_of::<FspInfoHeader>()).unwrap_or_default(),
        reserved1: [0; 2],
        spec_version: 0x24,
        header_revision: 7,
        image_revision: 0x0100_0000,
        image_id: *b"FSP-TEST",
        image_size: 0x0010_0000,
        image_base: 0xFFF0_0000,
        image_attribute: 0,
        component_attribute: 0,
        cfg_region_offset: 0,
        cfg_region_size: 0,
        reserved2: [0; 4],
        temp_ram_init_entry_offset: 0x1000,
        reserved3: [0; 4],
        notify_phase_entry_offset: 0x2000,
        fsp_memory_init_entry_offset: 0x3000,
        temp_ram_exit_entry_offset: 0x4000,
        fsp_silicon_init_entry_offset: 0x5000,
        fsp_multi_phase_si_init_entry_offset: 0x6000,
    };

    let sig = header.signature;
    let spec = header.spec_version;
    let img_id = header.image_id;
    assert_eq!(sig, 0x4850_5346);
    assert_eq!(spec, 0x24);
    assert_eq!(img_id, *b"FSP-TEST");

    let params = FspNotifyParams {
        phase: FspPhase::ReadyToBoot as u32,
    };
    let phase = params.phase;
    assert_eq!(phase, 0x40);
}
