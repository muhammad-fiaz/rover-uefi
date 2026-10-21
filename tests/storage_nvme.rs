//! Tests for `NvmePassThru` protocol structures, layouts, and GUID.

use rover_uefi::protocols::storage::guid::NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID;
use rover_uefi::protocols::storage::nvme_passthru::{
    EfiNvmExpressCommand, EfiNvmExpressCompletion, EfiNvmExpressPassThruCommandPacket,
    EfiNvmExpressPassThruMode, EfiNvmExpressPassThruProtocol, NVME_ADMIN_SUBMISSION_QUEUE,
    NVME_IO_SUBMISSION_QUEUE, NVME_PASS_THRU_ATTRIBUTES_PHYSICAL, NvmeCdw0,
};

#[test]
fn nvme_passthru_guid() {
    assert_eq!(
        NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID.to_bytes(),
        [
            0x12, 0x83, 0xC7, 0x52, 0xDC, 0x8E, 0x33, 0x42, 0x98, 0xF2, 0x1A, 0x1A, 0xA5, 0xE3,
            0x88, 0xA5
        ]
    );
    assert_eq!(NVME_PASS_THRU_ATTRIBUTES_PHYSICAL, 0x0001);
    assert_eq!(NVME_ADMIN_SUBMISSION_QUEUE, 0x00);
    assert_eq!(NVME_IO_SUBMISSION_QUEUE, 0x01);
}

#[test]
fn nvme_structures_layout() {
    assert_eq!(core::mem::size_of::<EfiNvmExpressPassThruMode>(), 12);
    assert_eq!(core::mem::size_of::<EfiNvmExpressCommand>(), 44);
    assert_eq!(core::mem::size_of::<EfiNvmExpressCompletion>(), 16);

    let cdw0 = NvmeCdw0::new(0x02, 0x01);
    assert_eq!(cdw0.opcode(), 0x02);
    assert_eq!(cdw0.fused_op(), 0x01);

    assert_eq!(
        core::mem::size_of::<EfiNvmExpressPassThruProtocol>(),
        5 * core::mem::size_of::<usize>()
    );

    assert!(core::mem::size_of::<EfiNvmExpressPassThruCommandPacket>() >= 48);
}
