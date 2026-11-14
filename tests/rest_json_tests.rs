//! # REST JSON Structure Protocol Tests
//!
//! Unit tests for REST JSON structure protocol and client wrapper.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::network::rest_json::{
    EFI_REST_JSON_STRUCTURE_PROTOCOL_GUID, EfiRestJsonResourceTypeIdentifier,
    EfiRestJsonResourceTypeNamespace, EfiRestJsonStructureHeader, EfiRestJsonStructureProtocol,
    EfiRestJsonStructureSupported, RestJsonStructure,
};

#[test]
fn test_rest_json_guid_and_structs() {
    assert_ne!(
        EFI_REST_JSON_STRUCTURE_PROTOCOL_GUID,
        rover_uefi::base::EfiGuid::ZERO
    );

    let ns = EfiRestJsonResourceTypeNamespace {
        resource_type_name: c"ComputerSystem".as_ptr().cast::<u8>(),
        major_version: c"1".as_ptr().cast::<u8>(),
        minor_version: c"0".as_ptr().cast::<u8>(),
        errata_version: c"0".as_ptr().cast::<u8>(),
    };
    let ident = EfiRestJsonResourceTypeIdentifier {
        name_space: ns,
        data_type: c"ComputerSystem".as_ptr().cast::<u8>(),
    };
    let hdr = EfiRestJsonStructureHeader {
        json_rsrc_identifier: ident,
        json_structure_pointer: core::ptr::null_mut(),
    };
    assert!(hdr.json_structure_pointer.is_null());

    let supp = EfiRestJsonStructureSupported {
        next_supported_rsrc_interp: [core::ptr::null_mut(), core::ptr::null_mut()],
        rest_resource_interp: ident,
    };
    assert!(supp.next_supported_rsrc_interp[0].is_null());
}

static TO_STRUCT_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static TO_JSON_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static DESTROY_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static mut DUMMY_HEADER: EfiRestJsonStructureHeader = EfiRestJsonStructureHeader {
    json_rsrc_identifier: EfiRestJsonResourceTypeIdentifier {
        name_space: EfiRestJsonResourceTypeNamespace {
            resource_type_name: core::ptr::null(),
            major_version: core::ptr::null(),
            minor_version: core::ptr::null(),
            errata_version: core::ptr::null(),
        },
        data_type: core::ptr::null(),
    },
    json_structure_pointer: core::ptr::null_mut(),
};

static mut DUMMY_JSON: [u8; 8] = *b"{\"id\":1}";

unsafe extern "efiapi" fn mock_register(
    _this: *mut EfiRestJsonStructureProtocol,
    _json_structure_supported: *mut EfiRestJsonStructureSupported,
    _to_structure: rover_uefi::protocols::network::rest_json::EfiRestJsonStructureToStructure,
    _to_json: rover_uefi::protocols::network::rest_json::EfiRestJsonStructureToJson,
    _destroy_structure: rover_uefi::protocols::network::rest_json::EfiRestJsonStructureDestroyStructure,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_to_structure(
    _this: *mut EfiRestJsonStructureProtocol,
    _json_rsrc_identifier: *const EfiRestJsonResourceTypeIdentifier,
    _resource_json_text: *const u8,
    json_structure: *mut *mut EfiRestJsonStructureHeader,
) -> EfiStatus {
    TO_STRUCT_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !json_structure.is_null() {
        unsafe {
            *json_structure = core::ptr::addr_of_mut!(DUMMY_HEADER);
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_to_json(
    _this: *mut EfiRestJsonStructureProtocol,
    _json_structure_header: *mut EfiRestJsonStructureHeader,
    resource_json_text: *mut *mut u8,
) -> EfiStatus {
    TO_JSON_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !resource_json_text.is_null() {
        unsafe {
            *resource_json_text = core::ptr::addr_of_mut!(DUMMY_JSON).cast::<u8>();
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_destroy(
    _this: *mut EfiRestJsonStructureProtocol,
    _json_structure_header: *mut EfiRestJsonStructureHeader,
) -> EfiStatus {
    DESTROY_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    EfiStatus::SUCCESS
}

#[test]
fn test_rest_json_client_wrapper() {
    let mut raw = EfiRestJsonStructureProtocol {
        register: mock_register,
        to_structure: mock_to_structure,
        to_json: mock_to_json,
        destory_structure: mock_destroy,
    };

    let mut client = RestJsonStructure::new(&mut raw);
    assert!(!client.raw().is_null());

    let text = c"{\"hello\": \"world\"}";
    let struct_ptr = match unsafe { client.to_structure_raw(None, text.as_ptr().cast::<u8>()) } {
        Ok(ptr) => ptr,
        Err(_) => core::ptr::null_mut(),
    };
    assert_eq!(struct_ptr, core::ptr::addr_of_mut!(DUMMY_HEADER));
    assert!(TO_STRUCT_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    let json_ptr = match unsafe { client.to_json_raw(struct_ptr) } {
        Ok(ptr) => ptr,
        Err(_) => core::ptr::null_mut(),
    };
    assert_eq!(json_ptr, core::ptr::addr_of_mut!(DUMMY_JSON).cast::<u8>());
    assert!(TO_JSON_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    assert!(unsafe { client.destroy_structure(struct_ptr) }.is_ok());
    assert!(DESTROY_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}
