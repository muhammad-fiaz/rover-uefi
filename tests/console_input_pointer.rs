// SPDX-License-Identifier: GPL-3.0-only
//! Unit tests for pointer and extended input protocols.

use core::mem::size_of;
use rover_uefi::protocols::console::{
    ABSOLUTE_POINTER_PROTOCOL_GUID, EfiAbsolutePointerMode, EfiAbsolutePointerProtocol,
    EfiAbsolutePointerState, EfiKeyData, EfiKeyState, EfiSimplePointerMode,
    EfiSimplePointerProtocol, EfiSimplePointerState, EfiSimpleTextInputExProtocol,
    SIMPLE_POINTER_PROTOCOL_GUID, SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID,
};

#[test]
fn protocol_guids() {
    let simple_ptr = SIMPLE_POINTER_PROTOCOL_GUID;
    assert_eq!(simple_ptr.data1, 0x3187_8c87);
    assert_eq!(simple_ptr.data2, 0x0b75);
    assert_eq!(simple_ptr.data3, 0x11d5);
    assert_eq!(
        simple_ptr.data4,
        [0x9a, 0x4f, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]
    );

    let abs_ptr = ABSOLUTE_POINTER_PROTOCOL_GUID;
    assert_eq!(abs_ptr.data1, 0x8d59_d32b);
    assert_eq!(abs_ptr.data2, 0xc655);
    assert_eq!(abs_ptr.data3, 0x4ae9);
    assert_eq!(
        abs_ptr.data4,
        [0x9b, 0x15, 0xf2, 0x59, 0x04, 0x99, 0x2a, 0x43]
    );

    let text_in_ex = SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID;
    assert_eq!(text_in_ex.data1, 0xdd9e_7534);
    assert_eq!(text_in_ex.data2, 0x7762);
    assert_eq!(text_in_ex.data3, 0x4698);
    assert_eq!(
        text_in_ex.data4,
        [0x8c, 0x14, 0xf5, 0x85, 0x17, 0xa6, 0x25, 0xaa]
    );
}

#[test]
fn simple_pointer_layouts() {
    assert_eq!(size_of::<EfiSimplePointerState>(), 16);
    assert_eq!(size_of::<EfiSimplePointerMode>(), 32);

    #[cfg(target_pointer_width = "64")]
    assert_eq!(size_of::<EfiSimplePointerProtocol>(), 32);

    #[cfg(target_pointer_width = "32")]
    assert_eq!(size_of::<EfiSimplePointerProtocol>(), 16);
}

#[test]
fn absolute_pointer_layouts() {
    assert_eq!(size_of::<EfiAbsolutePointerState>(), 32);
    assert_eq!(size_of::<EfiAbsolutePointerMode>(), 56);

    #[cfg(target_pointer_width = "64")]
    assert_eq!(size_of::<EfiAbsolutePointerProtocol>(), 32);

    #[cfg(target_pointer_width = "32")]
    assert_eq!(size_of::<EfiAbsolutePointerProtocol>(), 16);
}

#[test]
fn input_ex_layouts() {
    assert_eq!(size_of::<EfiKeyState>(), 8);
    assert_eq!(size_of::<EfiKeyData>(), 12);

    #[cfg(target_pointer_width = "64")]
    assert_eq!(size_of::<EfiSimpleTextInputExProtocol>(), 48);

    #[cfg(target_pointer_width = "32")]
    assert_eq!(size_of::<EfiSimpleTextInputExProtocol>(), 24);
}
