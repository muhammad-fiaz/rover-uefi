// SPDX-License-Identifier: Apache-2.0
//! Unit tests for Driver Health, Diagnostics, Override, and Supported Version protocols.

use rover_uefi::protocols::driver::{
    EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID, EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID,
    EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID, EFI_DRIVER_HEALTH_PROTOCOL_GUID,
    EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL_GUID, EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID,
    EfiDriverDiagnosticType, EfiDriverHealthStatus, EfiDriverSupportedEfiVersionProtocol,
};

#[test]
fn driver_model_protocols_guids_and_enums() {
    assert!(!EFI_DRIVER_HEALTH_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DRIVER_DIAGNOSTICS2_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DRIVER_DIAGNOSTICS_PROTOCOL_GUID.is_zero());
    assert!(!EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL_GUID.is_zero());
    assert!(!EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL_GUID.is_zero());
    assert!(!EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL_GUID.is_zero());

    assert_eq!(EfiDriverHealthStatus::Healthy as u32, 0);
    assert_eq!(EfiDriverHealthStatus::RepairRequired as u32, 1);
    assert_eq!(EfiDriverHealthStatus::ConfigurationRequired as u32, 2);
    assert_eq!(EfiDriverHealthStatus::Failed as u32, 3);
    assert_eq!(EfiDriverHealthStatus::ReconnectRequired as u32, 4);

    assert_eq!(EfiDriverDiagnosticType::Standard as u32, 0);
    assert_eq!(EfiDriverDiagnosticType::Extended as u32, 1);
    assert_eq!(EfiDriverDiagnosticType::Manufacturing as u32, 2);

    let supported_ver = EfiDriverSupportedEfiVersionProtocol {
        length: 8,
        firmware_version: 0x0002_00A0, // UEFI 2.10
    };
    assert_eq!(supported_ver.length, 8);
    assert_eq!(supported_ver.firmware_version, 0x0002_00A0);
}
