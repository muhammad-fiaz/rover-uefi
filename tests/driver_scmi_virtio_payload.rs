//! Tests for SCMI, `VirtIO`, and Universal Payload definitions.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::driver::scmi::{
    ARM_SCMI_BASE_PROTOCOL_GUID, ARM_SCMI_CLOCK_PROTOCOL_GUID, ARM_SCMI_PERFORMANCE_PROTOCOL_GUID,
    EfiScmiBaseProtocol,
};
use rover_uefi::protocols::driver::virtio::{
    EfiVirtioDeviceProtocol, VIRTIO_DEVICE_PROTOCOL_GUID, virtio_device_id, virtio_status,
};
use rover_uefi::protocols::firmware::universal_payload::{
    UNIVERSAL_PAYLOAD_IDENTIFIER, UniversalPayloadGenericHeader, UniversalPayloadInfoHeader,
};

#[test]
fn test_scmi_and_virtio_guids() {
    assert_eq!(ARM_SCMI_BASE_PROTOCOL_GUID.data1, 0xD7E5_ABE9);
    assert_eq!(ARM_SCMI_CLOCK_PROTOCOL_GUID.data1, 0x91CE_67A8);
    assert_eq!(ARM_SCMI_PERFORMANCE_PROTOCOL_GUID.data1, 0x09B8_BA84);
    assert_eq!(VIRTIO_DEVICE_PROTOCOL_GUID.data1, 0xFA92_0010);
}

#[test]
fn test_virtio_constants() {
    assert_eq!(virtio_device_id::NETWORK, 1);
    assert_eq!(virtio_device_id::BLOCK, 2);
    assert_eq!(virtio_device_id::CONSOLE, 3);
    assert_eq!(virtio_device_id::ENTROPY, 4);
    assert_eq!(virtio_device_id::SCSI, 8);
    assert_eq!(virtio_device_id::GPU, 16);
    assert_eq!(virtio_device_id::FS, 26);

    assert_eq!(virtio_status::ACKNOWLEDGE, 1);
    assert_eq!(virtio_status::DRIVER, 2);
    assert_eq!(virtio_status::DRIVER_OK, 4);
    assert_eq!(virtio_status::FEATURES_OK, 8);
    assert_eq!(virtio_status::FAILED, 128);
}

unsafe extern "efiapi" fn dummy_scmi_base_version(
    _this: *mut EfiScmiBaseProtocol,
    version: *mut u32,
) -> EfiStatus {
    if !version.is_null() {
        unsafe { *version = 0x20000 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_scmi_base_total(
    _this: *mut EfiScmiBaseProtocol,
    num: *mut u32,
) -> EfiStatus {
    if !num.is_null() {
        unsafe { *num = 4 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_scmi_discover_vendor(
    _this: *mut EfiScmiBaseProtocol,
    _vendor_identifier: *mut [u8; 16],
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_scmi_discover_sub_vendor(
    _this: *mut EfiScmiBaseProtocol,
    _vendor_identifier: *mut [u8; 16],
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_scmi_discover_impl_version(
    _this: *mut EfiScmiBaseProtocol,
    _implementation_version: *mut u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_scmi_discover_list_protocols(
    _this: *mut EfiScmiBaseProtocol,
    _protocol_list_size: *mut u32,
    _protocol_list: *mut u8,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_scmi_base_protocol() {
    let mut scmi = EfiScmiBaseProtocol {
        get_version: dummy_scmi_base_version,
        get_total_protocols: dummy_scmi_base_total,
        discover_vendor: dummy_scmi_discover_vendor,
        discover_sub_vendor: dummy_scmi_discover_sub_vendor,
        discover_implementation_version: dummy_scmi_discover_impl_version,
        discover_list_protocols: dummy_scmi_discover_list_protocols,
    };

    let mut ver = 0u32;
    let status = unsafe { (scmi.get_version)(&raw mut scmi, &raw mut ver) };
    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(ver, 0x20000);
}

unsafe extern "efiapi" fn dummy_virtio_get_features(
    _this: *mut EfiVirtioDeviceProtocol,
    features: *mut u64,
) -> EfiStatus {
    if !features.is_null() {
        unsafe { *features = 0x1 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_set_features(
    _this: *mut EfiVirtioDeviceProtocol,
    _features: u64,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_get_status(
    _this: *mut EfiVirtioDeviceProtocol,
    status: *mut u8,
) -> EfiStatus {
    if !status.is_null() {
        unsafe { *status = virtio_status::DRIVER_OK };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_set_status(
    _this: *mut EfiVirtioDeviceProtocol,
    _status: u8,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_set_queue_address(
    _this: *mut EfiVirtioDeviceProtocol,
    _ring: *const core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_set_queue_sel(
    _this: *mut EfiVirtioDeviceProtocol,
    _sel: u16,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_set_queue_notify(
    _this: *mut EfiVirtioDeviceProtocol,
    _sel: u16,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_set_queue_align(
    _this: *mut EfiVirtioDeviceProtocol,
    _align: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_set_page_size(
    _this: *mut EfiVirtioDeviceProtocol,
    _page_size: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_get_queue_num_max(
    _this: *mut EfiVirtioDeviceProtocol,
    _queue_num_max: *mut u16,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_set_queue_num(
    _this: *mut EfiVirtioDeviceProtocol,
    _queue_num: u16,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_write_device(
    _this: *mut EfiVirtioDeviceProtocol,
    _field_offset: usize,
    _field_size: usize,
    _buffer_size: usize,
    _buffer: *const core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_read_device(
    _this: *mut EfiVirtioDeviceProtocol,
    _field_offset: usize,
    _field_size: usize,
    _buffer_size: usize,
    _buffer: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_allocate_shared_pages(
    _this: *mut EfiVirtioDeviceProtocol,
    _num_pages: usize,
    _host_address: *mut *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_free_shared_pages(
    _this: *mut EfiVirtioDeviceProtocol,
    _num_pages: usize,
    _host_address: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_map_shared(
    _this: *mut EfiVirtioDeviceProtocol,
    _operation: rover_uefi::protocols::driver::virtio::VirtioMapOperation,
    _host_address: *mut core::ffi::c_void,
    _number_of_bytes: *mut usize,
    _device_address: *mut u64,
    _mapping: *mut *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_virtio_unmap_shared(
    _this: *mut EfiVirtioDeviceProtocol,
    _mapping: *mut core::ffi::c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_virtio_device_protocol() {
    let mut virtio = EfiVirtioDeviceProtocol {
        revision: 0x10000,
        subsystem_device_id: i32::from(virtio_device_id::BLOCK),
        get_device_features: dummy_virtio_get_features,
        set_guest_features: dummy_virtio_set_features,
        set_queue_address: dummy_virtio_set_queue_address,
        set_queue_sel: dummy_virtio_set_queue_sel,
        set_queue_notify: dummy_virtio_set_queue_notify,
        set_queue_align: dummy_virtio_set_queue_align,
        set_page_size: dummy_virtio_set_page_size,
        get_queue_num_max: dummy_virtio_get_queue_num_max,
        set_queue_num: dummy_virtio_set_queue_num,
        get_device_status: dummy_virtio_get_status,
        set_device_status: dummy_virtio_set_status,
        write_device: dummy_virtio_write_device,
        read_device: dummy_virtio_read_device,
        allocate_shared_pages: dummy_virtio_allocate_shared_pages,
        free_shared_pages: dummy_virtio_free_shared_pages,
        map_shared_buffer: dummy_virtio_map_shared,
        unmap_shared_buffer: dummy_virtio_unmap_shared,
    };

    let mut feat = 0u64;
    let status = unsafe { (virtio.get_device_features)(&raw mut virtio, &raw mut feat) };
    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(feat, 0x1);

    let mut dev_status = 0u8;
    let status = unsafe { (virtio.get_device_status)(&raw mut virtio, &raw mut dev_status) };
    assert_eq!(status, EfiStatus::SUCCESS);
    assert_eq!(dev_status, virtio_status::DRIVER_OK);
}

#[test]
fn test_universal_payload_structures() {
    assert_eq!(UNIVERSAL_PAYLOAD_IDENTIFIER, 0x4844_4C50);

    let info = UniversalPayloadInfoHeader {
        identifier: UNIVERSAL_PAYLOAD_IDENTIFIER,
        header_length: u32::try_from(core::mem::size_of::<UniversalPayloadInfoHeader>())
            .unwrap_or_default(),
        spec_revision: 0x0075,
        reserved: [0; 2],
        revision: 0x0001_0000,
        attribute: 0,
        capability: 0,
        producer_id: *b"ROVER-BOOTLOADER",
        image_id: *b"ROVER-UPLD-IMAGE",
    };

    let id = info.identifier;
    let spec = info.spec_revision;
    assert_eq!(id, 0x4844_4C50);
    assert_eq!(spec, 0x0075);

    let generic = UniversalPayloadGenericHeader {
        revision: 1,
        reserved: 0,
        length: u16::try_from(core::mem::size_of::<UniversalPayloadGenericHeader>())
            .unwrap_or_default(),
    };
    let rev = generic.revision;
    assert_eq!(rev, 1);
}
