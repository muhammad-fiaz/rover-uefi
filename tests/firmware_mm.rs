// SPDX-License-Identifier: Apache-2.0
//! Unit tests for Management Mode communication protocols.

use core::mem::size_of;
use rover_uefi::protocols::firmware::{
    EfiMmCommunicateHeader, EfiMmCommunication2Protocol, EfiMmCommunicationProtocol,
    MM_COMMUNICATION_PROTOCOL_GUID, MM_COMMUNICATION2_PROTOCOL_GUID,
};

#[test]
fn mm_guids() {
    let mm1 = MM_COMMUNICATION_PROTOCOL_GUID;
    assert_eq!(mm1.data1, 0xc68e_d8e2);
    assert_eq!(mm1.data2, 0x9dc6);
    assert_eq!(mm1.data3, 0x4cbd);
    assert_eq!(mm1.data4, [0x9d, 0x94, 0xdb, 0x65, 0xac, 0xc5, 0xc3, 0x32]);

    let mm2 = MM_COMMUNICATION2_PROTOCOL_GUID;
    assert_eq!(mm2.data1, 0x378d_aedc);
    assert_eq!(mm2.data2, 0xf06b);
    assert_eq!(mm2.data3, 0x4446);
    assert_eq!(mm2.data4, [0x83, 0x14, 0x40, 0xab, 0x93, 0x3c, 0x87, 0xa3]);
}

#[test]
fn mm_structures_layout() {
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(size_of::<EfiMmCommunicateHeader>(), 24);
        assert_eq!(size_of::<EfiMmCommunicationProtocol>(), 8);
        assert_eq!(size_of::<EfiMmCommunication2Protocol>(), 8);
    }

    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(size_of::<EfiMmCommunicateHeader>(), 20);
        assert_eq!(size_of::<EfiMmCommunicationProtocol>(), 4);
        assert_eq!(size_of::<EfiMmCommunication2Protocol>(), 4);
    }
}
