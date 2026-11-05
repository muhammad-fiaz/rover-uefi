// SPDX-License-Identifier: Apache-2.0
//! Integration tests for ACPI Table protocol.

use core::ffi::c_void;

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::acpi::{
    AcpiTableProtocol, EFI_ACPI_TABLE_PROTOCOL_GUID, EfiAcpiTableProtocol,
};

static mut INSTALLED_SIZE: usize = 0;
static mut UNINSTALLED_KEY: UIntN = 0;

unsafe extern "efiapi" fn mock_install(
    _this: *mut EfiAcpiTableProtocol,
    _buffer: *const c_void,
    size: UIntN,
    table_key: *mut UIntN,
) -> EfiStatus {
    if table_key.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        INSTALLED_SIZE = size;
        *table_key = 42;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_uninstall(
    _this: *mut EfiAcpiTableProtocol,
    table_key: UIntN,
) -> EfiStatus {
    unsafe {
        UNINSTALLED_KEY = table_key;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_acpi_table_protocol() {
    assert_eq!(
        EFI_ACPI_TABLE_PROTOCOL_GUID.to_u128_le(),
        0x5c27_c57e_9c5a_b27b_46a6_6107_ffe0_6bdd
    );

    let raw = EfiAcpiTableProtocol {
        install_acpi_table: mock_install,
        uninstall_acpi_table: mock_uninstall,
    };
    let protocol = AcpiTableProtocol::new(&raw);

    let fake_table = [0xAA_u8; 64];
    let install_res = protocol.install_acpi_table(&fake_table);
    let Ok(key) = install_res else {
        panic!("expected success");
    };
    assert_eq!(key, 42);
    assert_eq!(unsafe { INSTALLED_SIZE }, 64);

    let uninstall_res = protocol.uninstall_acpi_table(42);
    assert!(uninstall_res.is_ok());
    assert_eq!(unsafe { UNINSTALLED_KEY }, 42);

    // Empty buffer check
    assert_eq!(
        protocol.install_acpi_table(&[]),
        Err(EfiStatus::INVALID_PARAMETER)
    );
}
