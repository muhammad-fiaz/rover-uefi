//! Integration tests for Realm Aperture Management Protocol.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use rover_uefi::base::{EfiHandle, EfiStatus};
use rover_uefi::protocols::driver::realm_aperture::{
    REALM_APERTURE_MANAGEMENT_PROTOCOL_GUID, REALM_APERTURE_MANAGEMENT_PROTOCOL_REVISION,
    RealmApertureManagement, RealmApertureManagementProtocol,
};

static mut MOCK_APERTURE_HANDLE: usize = 0x9999;

unsafe extern "efiapi" fn mock_open_aperture(
    memory: u64,
    pages: usize,
    aperture_reference: *mut EfiHandle,
) -> EfiStatus {
    if aperture_reference.is_null() || memory == 0 || pages == 0 {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *aperture_reference = EfiHandle::from_ptr((&raw mut MOCK_APERTURE_HANDLE).cast());
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_close_aperture(
    aperture_reference: EfiHandle,
    _memory_map_locked: bool,
) -> EfiStatus {
    if aperture_reference.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_realm_aperture_guid_and_revision() {
    assert_eq!(
        REALM_APERTURE_MANAGEMENT_PROTOCOL_GUID.to_string(),
        "585c00be-cf7c-4db8-8aa2-490d67f5f6e6"
    );
    assert_eq!(REALM_APERTURE_MANAGEMENT_PROTOCOL_REVISION, 0x0001_0000);
}

#[test]
fn test_realm_aperture_wrapper() {
    let proto = RealmApertureManagementProtocol {
        revision: REALM_APERTURE_MANAGEMENT_PROTOCOL_REVISION,
        open_aperture: mock_open_aperture,
        close_aperture: mock_close_aperture,
    };

    let client = RealmApertureManagement::new(&proto);
    assert_eq!(client.revision(), 0x0001_0000);

    let handle = client.open_aperture(0x1000_0000, 4).expect("open failed");
    assert!(!handle.is_null());

    assert!(client.close_aperture(handle, false).is_ok());

    assert_eq!(
        client.open_aperture(0, 4),
        Err(EfiStatus::INVALID_PARAMETER)
    );
    assert_eq!(
        client.close_aperture(EfiHandle::NULL, false),
        Err(EfiStatus::INVALID_PARAMETER)
    );
}
