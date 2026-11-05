// SPDX-License-Identifier: Apache-2.0
//! Unit tests for Unicode Collation protocols.

use core::mem::size_of;
use rover_uefi::protocols::console::{
    EFI_UNICODE_COLLATION_PROTOCOL_GUID, EFI_UNICODE_COLLATION2_PROTOCOL_GUID,
    EfiUnicodeCollation2Protocol, EfiUnicodeCollationProtocol,
};

#[test]
fn unicode_collation_guids_and_layout() {
    assert!(!EFI_UNICODE_COLLATION2_PROTOCOL_GUID.is_zero());
    assert!(!EFI_UNICODE_COLLATION_PROTOCOL_GUID.is_zero());

    assert!(size_of::<EfiUnicodeCollation2Protocol>() > 32);
    assert!(size_of::<EfiUnicodeCollationProtocol>() > 32);
}
