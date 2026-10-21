// SPDX-License-Identifier: GPL-3.0-only
//! Unit tests for UEFI Extended SCSI Pass Thru protocol definitions.

use core::mem::size_of;
use rover_uefi::protocols::storage::{
    EXT_SCSI_PASS_THRU_PROTOCOL_GUID, EfiExtScsiPassThruMode, EfiExtScsiPassThruProtocol,
    EfiExtScsiPassThruScsiRequestPacket,
};

#[test]
fn scsi_passthru_guid() {
    let guid = EXT_SCSI_PASS_THRU_PROTOCOL_GUID;
    assert_eq!(guid.data1, 0x143b_7632);
    assert_eq!(guid.data2, 0xb81b);
    assert_eq!(guid.data3, 0x4cb7);
    assert_eq!(guid.data4, [0xab, 0xd3, 0xb6, 0x25, 0xa5, 0xb9, 0xbf, 0xfe]);
}

#[test]
fn scsi_structures_layout() {
    assert_eq!(size_of::<EfiExtScsiPassThruMode>(), 12);

    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(size_of::<EfiExtScsiPassThruProtocol>(), 64);
        assert_eq!(size_of::<EfiExtScsiPassThruScsiRequestPacket>(), 56);
    }

    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(size_of::<EfiExtScsiPassThruProtocol>(), 32);
        assert_eq!(size_of::<EfiExtScsiPassThruScsiRequestPacket>(), 40);
    }
}
