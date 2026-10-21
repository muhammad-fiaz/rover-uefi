//! Tests for `AtaPassThru` protocol structures, layouts, and GUID.

use rover_uefi::protocols::storage::ata_passthru::{
    ATA_PASS_THRU_ATTRIBUTES_PHYSICAL, EfiAtaCommandBlock, EfiAtaPassThruCommandPacket,
    EfiAtaPassThruMode, EfiAtaPassThruProtocol, EfiAtaStatusBlock,
};
use rover_uefi::protocols::storage::guid::ATA_PASS_THRU_PROTOCOL_GUID;

#[test]
fn ata_passthru_guid() {
    assert_eq!(
        ATA_PASS_THRU_PROTOCOL_GUID.to_bytes(),
        [
            0xF0, 0xE7, 0x3D, 0x1D, 0x07, 0x08, 0x4F, 0x42, 0xAA, 0x69, 0x11, 0xA5, 0x4E, 0x19,
            0xA4, 0x6F
        ]
    );
    assert_eq!(ATA_PASS_THRU_ATTRIBUTES_PHYSICAL, 0x0001);
}

#[test]
fn ata_structures_layout() {
    assert_eq!(core::mem::size_of::<EfiAtaPassThruMode>(), 8);
    assert_eq!(core::mem::size_of::<EfiAtaCommandBlock>(), 20);
    assert_eq!(core::mem::size_of::<EfiAtaStatusBlock>(), 20);

    assert_eq!(
        core::mem::size_of::<EfiAtaPassThruProtocol>(),
        8 * core::mem::size_of::<usize>()
    );

    assert!(core::mem::size_of::<EfiAtaPassThruCommandPacket>() >= 40);
}
