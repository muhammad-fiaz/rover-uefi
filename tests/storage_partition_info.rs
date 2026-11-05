// SPDX-License-Identifier: Apache-2.0
//! Unit tests for UEFI 2.7 Partition Information Protocol.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::storage::partition::{
    EFI_PARTITION_INFO_PROTOCOL_REVISION, EfiPartitionInfoData, EfiPartitionInfoProtocol,
    GptPartitionRecord, MbrPartitionRecord, PARTITION_INFO_PROTOCOL_GUID, PARTITION_TYPE_GPT,
    PARTITION_TYPE_MBR, PARTITION_TYPE_OTHER,
};

#[test]
fn guid_and_constants() {
    assert_eq!(
        PARTITION_INFO_PROTOCOL_GUID.to_bytes(),
        [
            0x2c, 0xf6, 0xf2, 0x8c, 0x9b, 0xbc, 0x21, 0x48, 0x80, 0x8d, 0xec, 0x9e, 0xc4, 0x21,
            0xa1, 0xa0
        ]
    );
    assert_eq!(EFI_PARTITION_INFO_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(PARTITION_TYPE_OTHER, 0x00);
    assert_eq!(PARTITION_TYPE_MBR, 0x01);
    assert_eq!(PARTITION_TYPE_GPT, 0x02);
}

#[test]
fn mbr_partition_record() {
    let mbr = MbrPartitionRecord {
        boot_indicator: 0x80,
        start_head: 0,
        start_sector: 1,
        start_track: 0,
        os_indicator: 0xef,
        end_head: 254,
        end_sector: 63,
        end_track: 255,
        starting_lba: 2048u32.to_le_bytes(),
        size_in_lba: 204_800_u32.to_le_bytes(),
    };

    assert_eq!(mbr.starting_lba(), 2048);
    assert_eq!(mbr.sector_count(), 204_800);

    let info = EfiPartitionInfoProtocol {
        revision: EFI_PARTITION_INFO_PROTOCOL_REVISION,
        partition_type: PARTITION_TYPE_MBR,
        system: 1,
        reserved: [0; 7],
        info: EfiPartitionInfoData { mbr },
    };

    assert!(info.is_system_partition());
    assert_eq!(info.partition_type(), PARTITION_TYPE_MBR);
    let Some(retrieved_mbr) = info.as_mbr() else {
        panic!("expected MBR record");
    };
    let indicator = retrieved_mbr.boot_indicator;
    assert_eq!(indicator, 0x80);
    assert_eq!(retrieved_mbr.starting_lba(), 2048);
    assert!(info.as_gpt().is_none());
}

#[test]
fn gpt_partition_record() {
    let gpt = GptPartitionRecord {
        partition_type_guid: EfiGuid::new(0x1234, 0x5678, 0x9abc, [1, 2, 3, 4, 5, 6, 7, 8]),
        unique_partition_guid: EfiGuid::new(0xabcd, 0xef01, 0x2345, [8, 7, 6, 5, 4, 3, 2, 1]),
        starting_lba: 100_000,
        ending_lba: 200_000,
        attributes: 0,
        partition_name: [0; 36],
    };

    let info = EfiPartitionInfoProtocol {
        revision: EFI_PARTITION_INFO_PROTOCOL_REVISION,
        partition_type: PARTITION_TYPE_GPT,
        system: 0,
        reserved: [0; 7],
        info: EfiPartitionInfoData { gpt },
    };

    assert!(!info.is_system_partition());
    assert_eq!(info.partition_type(), PARTITION_TYPE_GPT);
    let Some(retrieved_gpt) = info.as_gpt() else {
        panic!("expected GPT record");
    };
    let start_lba = { retrieved_gpt.starting_lba };
    let end_lba = { retrieved_gpt.ending_lba };
    assert_eq!(start_lba, 100_000);
    assert_eq!(end_lba, 200_000);
    assert!(info.as_mbr().is_none());
}
