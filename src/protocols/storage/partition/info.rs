//! # Partition Information Protocol
//!
//! UEFI 2.7 Partition Information Protocol (`EFI_PARTITION_INFO_PROTOCOL`).
//! Provides partition information without reading block devices directly.

use crate::base::EfiGuid;

/// Partition Information Protocol GUID: `8cf2f62c-bc9b-4821-808d-ec9ec421a1a0`
pub const PARTITION_INFO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x8cf2_f62c,
    0xbc9b,
    0x4821,
    [0x80, 0x8d, 0xec, 0x9e, 0xc4, 0x21, 0xa1, 0xa0],
);

/// Partition Information Protocol Revision.
pub const EFI_PARTITION_INFO_PROTOCOL_REVISION: u32 = 0x0001_0000;

/// Unknown or other partition type.
pub const PARTITION_TYPE_OTHER: u32 = 0x00;

/// MBR partition type.
pub const PARTITION_TYPE_MBR: u32 = 0x01;

/// GPT partition type.
pub const PARTITION_TYPE_GPT: u32 = 0x02;

/// Raw 16-byte MBR partition record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C, packed)]
pub struct MbrPartitionRecord {
    /// Boot indicator (0x80 = bootable/active).
    pub boot_indicator: u8,
    /// Starting head.
    pub start_head: u8,
    /// Starting sector (bits 0-5) and high cylinder bits (bits 6-7).
    pub start_sector: u8,
    /// Starting cylinder low bits.
    pub start_track: u8,
    /// OS indicator / partition type.
    pub os_indicator: u8,
    /// Ending head.
    pub end_head: u8,
    /// Ending sector.
    pub end_sector: u8,
    /// Ending cylinder.
    pub end_track: u8,
    /// Starting LBA as little-endian 4-byte array.
    pub starting_lba: [u8; 4],
    /// Size in LBA sectors as little-endian 4-byte array.
    pub size_in_lba: [u8; 4],
}

impl MbrPartitionRecord {
    /// Returns the starting LBA as a `u32`.
    #[must_use]
    pub fn starting_lba(&self) -> u32 {
        u32::from_le_bytes(self.starting_lba)
    }

    /// Returns the sector count as a `u32`.
    #[must_use]
    pub fn sector_count(&self) -> u32 {
        u32::from_le_bytes(self.size_in_lba)
    }
}

/// Raw 128-byte GPT partition entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C, packed)]
pub struct GptPartitionRecord {
    /// Partition type GUID.
    pub partition_type_guid: EfiGuid,
    /// Unique partition GUID.
    pub unique_partition_guid: EfiGuid,
    /// Starting LBA.
    pub starting_lba: u64,
    /// Ending LBA.
    pub ending_lba: u64,
    /// Attributes mask.
    pub attributes: u64,
    /// Partition name as 36 UTF-16 characters.
    pub partition_name: [u16; 36],
}

/// Union of partition record types in Partition Information Protocol.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub union EfiPartitionInfoData {
    /// MBR partition record.
    pub mbr: MbrPartitionRecord,
    /// GPT partition entry.
    pub gpt: GptPartitionRecord,
}

/// Raw UEFI Partition Information Protocol interface (`EFI_PARTITION_INFO_PROTOCOL`).
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct EfiPartitionInfoProtocol {
    /// Revision number (`EFI_PARTITION_INFO_PROTOCOL_REVISION`).
    pub revision: u32,
    /// Partition type: `PARTITION_TYPE_OTHER`, `PARTITION_TYPE_MBR`, or `PARTITION_TYPE_GPT`.
    pub partition_type: u32,
    /// Non-zero if partition describes an EFI System Partition.
    pub system: u8,
    /// Reserved bytes.
    pub reserved: [u8; 7],
    /// Partition details union.
    pub info: EfiPartitionInfoData,
}

impl EfiPartitionInfoProtocol {
    /// Returns whether this partition is an EFI System Partition (ESP).
    #[must_use]
    pub fn is_system_partition(&self) -> bool {
        self.system != 0
    }

    /// Returns the partition style.
    #[must_use]
    pub fn partition_type(&self) -> u32 {
        self.partition_type
    }

    /// Returns the MBR partition record if this partition is MBR.
    #[must_use]
    pub fn as_mbr(&self) -> Option<&MbrPartitionRecord> {
        if self.partition_type == PARTITION_TYPE_MBR {
            Some(unsafe { &self.info.mbr })
        } else {
            None
        }
    }

    /// Returns the GPT partition entry if this partition is GPT.
    #[must_use]
    pub fn as_gpt(&self) -> Option<&GptPartitionRecord> {
        if self.partition_type == PARTITION_TYPE_GPT {
            Some(unsafe { &self.info.gpt })
        } else {
            None
        }
    }
}
