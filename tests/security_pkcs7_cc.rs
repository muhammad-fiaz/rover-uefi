// SPDX-License-Identifier: Apache-2.0
//! Unit tests for PKCS7 Verify and Confidential Computing measurement protocols.

use core::mem::size_of;
use rover_uefi::protocols::security::{
    CC_MEASUREMENT_PROTOCOL_GUID, EFI_PKCS7_VERIFY_PROTOCOL_GUID,
    EfiCcMeasurementBootServiceCapability, EfiCcMeasurementEvent, EfiCcMeasurementProtocol,
    EfiCcType, EfiPkcs7VerifyProtocol,
};

#[test]
fn pkcs7_and_cc_measurement_guids_and_types() {
    assert!(!EFI_PKCS7_VERIFY_PROTOCOL_GUID.is_zero());
    assert!(!CC_MEASUREMENT_PROTOCOL_GUID.is_zero());

    assert_eq!(EfiCcType::None as u8, 0);
    assert_eq!(EfiCcType::Sev as u8, 1);
    assert_eq!(EfiCcType::Tdx as u8, 2);
    assert_eq!(EfiCcType::ArmCca as u8, 3);

    assert!(size_of::<EfiPkcs7VerifyProtocol>() >= 16);
    assert!(size_of::<EfiCcMeasurementProtocol>() >= 32);
    assert!(size_of::<EfiCcMeasurementBootServiceCapability>() >= 8);
    assert!(size_of::<EfiCcMeasurementEvent>() >= 16);
}
