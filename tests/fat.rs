//! Tests for the native FAT12/16/32 parser.
use rover_uefi::protocols::storage::filesystem::fat::{
    ATTR_ARCHIVE, ATTR_DIRECTORY, ATTR_LONG_NAME, ATTR_VOLUME_ID, DirectoryEntry, EntryKind,
    FIRST_DATA_CLUSTER, FatKind, FatVolume, LongNameEntry,
};

/// Builds a boot sector with the given geometry.
///
/// The positional parameters mirror the fields of the BIOS parameter
/// block so each test can spell out the geometry it exercises.
#[allow(clippy::too_many_arguments)]
fn boot_sector(
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved: u16,
    fat_count: u8,
    root_entries: u16,
    total_sectors_16: u16,
    fat_size_16: u16,
    total_sectors_32: u32,
    fat_size_32: u32,
    root_cluster: u32,
) -> [u8; 512] {
    let mut sector = [0_u8; 512];
    sector[0x0B..0x0D].copy_from_slice(&bytes_per_sector.to_le_bytes());
    sector[0x0D] = sectors_per_cluster;
    sector[0x0E..0x10].copy_from_slice(&reserved.to_le_bytes());
    sector[0x10] = fat_count;
    sector[0x11..0x13].copy_from_slice(&root_entries.to_le_bytes());
    sector[0x13..0x15].copy_from_slice(&total_sectors_16.to_le_bytes());
    sector[0x16..0x18].copy_from_slice(&fat_size_16.to_le_bytes());
    sector[0x20..0x24].copy_from_slice(&total_sectors_32.to_le_bytes());
    if fat_size_16 == 0 {
        sector[0x24..0x28].copy_from_slice(&fat_size_32.to_le_bytes());
        sector[0x2C..0x30].copy_from_slice(&root_cluster.to_le_bytes());
    }
    sector[510] = 0x55;
    sector[511] = 0xAA;
    sector
}

fn fat32_volume() -> FatVolume {
    // 32 reserved sectors, two 512-sector tables, 2048 data clusters.
    let sector = boot_sector(512, 1, 32, 2, 0, 0, 0, 3104, 512, 2);
    let Some(volume) = FatVolume::parse_boot_sector(&sector) else {
        panic!("the FAT32 boot sector must parse");
    };
    volume
}

fn fat16_volume() -> FatVolume {
    // 1 reserved sector, two 20-sector tables, a 512-entry root
    // directory, and 4085 data clusters.
    let sector = boot_sector(512, 1, 1, 2, 512, 0, 20, 4158, 0, 0);
    let Some(volume) = FatVolume::parse_boot_sector(&sector) else {
        panic!("the FAT16 boot sector must parse");
    };
    volume
}

fn fat12_volume() -> FatVolume {
    // 1 reserved sector, two 9-sector tables, a 224-entry root
    // directory, and 100 data clusters.
    let sector = boot_sector(512, 1, 1, 2, 224, 0, 9, 133, 0, 0);
    let Some(volume) = FatVolume::parse_boot_sector(&sector) else {
        panic!("the FAT12 boot sector must parse");
    };
    volume
}

#[test]
fn fat32_geometry() {
    let volume = fat32_volume();
    assert_eq!(volume.kind(), FatKind::Fat32);
    assert_eq!(volume.bytes_per_sector(), 512);
    assert_eq!(volume.sectors_per_cluster(), 1);
    assert_eq!(volume.cluster_size(), 512);
    assert_eq!(volume.fat_count(), 2);
    assert_eq!(volume.fat_size_sectors(), 512);
    assert_eq!(volume.root_cluster(), 2);
    assert_eq!(volume.first_fat_sector(), 32);
    assert_eq!(volume.root_directory_sector(), None);
    assert_eq!(volume.first_data_sector(), 1056);
    assert_eq!(volume.cluster_count(), 2048);
    assert_eq!(volume.size_bytes(), Some(3104 * 512));
}

#[test]
fn fat16_geometry() {
    let volume = fat16_volume();
    assert_eq!(volume.kind(), FatKind::Fat16);
    assert_eq!(volume.root_entry_count(), 512);
    assert_eq!(volume.root_directory_sector(), Some(41));
    assert_eq!(volume.root_directory_sectors(), 32);
    assert_eq!(volume.first_data_sector(), 73);
    assert_eq!(volume.cluster_count(), 4085);
    assert_eq!(volume.root_cluster(), 0);
}

#[test]
fn fat12_geometry() {
    let volume = fat12_volume();
    assert_eq!(volume.kind(), FatKind::Fat12);
    assert_eq!(volume.root_directory_sectors(), 14);
    assert_eq!(volume.first_data_sector(), 33);
    assert_eq!(volume.cluster_count(), 100);
}

#[test]
fn cluster_to_sector_is_checked() {
    let volume = fat32_volume();
    assert_eq!(volume.cluster_to_sector(0), None);
    assert_eq!(volume.cluster_to_sector(1), None);
    assert_eq!(volume.cluster_to_sector(FIRST_DATA_CLUSTER), Some(1056));
    assert_eq!(volume.cluster_to_sector(3), Some(1057));
    assert_eq!(volume.cluster_to_sector(2049), Some(1056 + 2047));
    assert_eq!(volume.cluster_to_sector(2050), None);
    assert!(!volume.is_data_cluster(1));
    assert!(volume.is_data_cluster(2));
}

#[test]
fn boot_sector_rejections() {
    let mut sector = boot_sector(512, 1, 1, 1, 16, 0, 1, 100, 0, 0);
    // Missing signature.
    sector[510] = 0;
    assert!(FatVolume::parse_boot_sector(&sector).is_none());
    // Truncated sector.
    assert!(FatVolume::parse_boot_sector(&[0_u8; 510]).is_none());
    // Invalid sector size.
    let mut sector = boot_sector(300, 1, 1, 1, 16, 0, 1, 100, 0, 0);
    assert!(FatVolume::parse_boot_sector(&sector).is_none());
    // Non-power-of-two cluster size.
    sector = boot_sector(512, 3, 1, 1, 16, 0, 1, 100, 0, 0);
    assert!(FatVolume::parse_boot_sector(&sector).is_none());
    // Zero reserved sectors.
    sector = boot_sector(512, 1, 0, 1, 16, 0, 1, 100, 0, 0);
    assert!(FatVolume::parse_boot_sector(&sector).is_none());
    // Zero tables.
    sector = boot_sector(512, 1, 1, 0, 16, 0, 1, 100, 0, 0);
    assert!(FatVolume::parse_boot_sector(&sector).is_none());
    // Zero table size.
    sector = boot_sector(512, 1, 1, 1, 16, 0, 0, 100, 0, 2);
    assert!(FatVolume::parse_boot_sector(&sector).is_none());
    // Root cluster below the first data cluster on FAT32.
    sector = boot_sector(512, 1, 1, 1, 0, 0, 0, 100, 1, 1);
    assert!(FatVolume::parse_boot_sector(&sector).is_none());
    // No data clusters at all: the data region starts at sector 2 and
    // the volume ends there.
    sector = boot_sector(512, 1, 1, 1, 0, 0, 0, 2, 1, 2);
    assert!(FatVolume::parse_boot_sector(&sector).is_none());
}

#[test]
fn fat16_entry_decoding() {
    let volume = fat16_volume();
    let mut fat = [0_u8; 32];
    fat[4..6].copy_from_slice(&5_u16.to_le_bytes());
    fat[6..8].copy_from_slice(&0xFFF8_u16.to_le_bytes());
    assert_eq!(volume.fat_entry(&fat, 2), Some(5));
    assert_eq!(volume.fat_entry(&fat, 3), Some(0xFFF8));
    assert!(volume.kind().is_end_of_chain(0xFFF8));
    assert!(!volume.kind().is_end_of_chain(5));
    // Out-of-range clusters and short tables.
    assert_eq!(volume.fat_entry(&fat, 0), None);
    assert_eq!(volume.fat_entry(&fat, 20), None);
    assert_eq!(volume.fat_entry(&[0_u8; 3], 2), None);
}

#[test]
fn fat12_entry_decoding() {
    let volume = fat12_volume();
    let mut fat = [0_u8; 64];
    // Cluster 2 = 0x123, cluster 3 = 0x456, packed as two entries.
    fat[3] = 0x23;
    fat[4] = 0x61;
    fat[5] = 0x45;
    assert_eq!(volume.fat_entry(&fat, 2), Some(0x123));
    assert_eq!(volume.fat_entry(&fat, 3), Some(0x456));
}

#[test]
fn cluster_chain_follows_links() {
    let volume = fat32_volume();
    let mut fat = [0_u8; 64];
    // Cluster 2 -> 5 -> 6 -> end of chain.
    fat[8..12].copy_from_slice(&5_u32.to_le_bytes());
    fat[20..24].copy_from_slice(&6_u32.to_le_bytes());
    fat[24..28].copy_from_slice(&0x0FFF_FFFF_u32.to_le_bytes());

    let mut chain = [0_u32; 8];
    assert_eq!(volume.cluster_chain(&fat, 2, &mut chain), Some(3));
    assert_eq!(&chain[..3], &[2, 5, 6]);

    // A single-cluster chain ends immediately.
    let mut single = [0_u32; 4];
    assert_eq!(volume.cluster_chain(&fat, 6, &mut single), Some(1));
    assert_eq!(single[0], 6);
}

#[test]
fn cluster_chain_rejects_bad_chains() {
    let volume = fat32_volume();
    let mut fat = [0_u8; 64];
    // A self loop must not be followed forever.
    fat[8..12].copy_from_slice(&2_u32.to_le_bytes());
    let mut chain = [0_u32; 8];
    assert_eq!(volume.cluster_chain(&fat, 2, &mut chain), None);

    // A bad cluster ends the traversal with an error.
    fat[8..12].copy_from_slice(&0x0FFF_FFF7_u32.to_le_bytes());
    assert_eq!(volume.cluster_chain(&fat, 2, &mut chain), None);

    // A chain that does not fit into the output buffer is rejected.
    fat[8..12].copy_from_slice(&5_u32.to_le_bytes());
    fat[20..24].copy_from_slice(&6_u32.to_le_bytes());
    fat[24..28].copy_from_slice(&7_u32.to_le_bytes());
    fat[28..32].copy_from_slice(&0x0FFF_FFFF_u32.to_le_bytes());
    let mut small = [0_u32; 2];
    assert_eq!(volume.cluster_chain(&fat, 2, &mut small), None);
    // An empty output buffer is rejected as well.
    assert_eq!(volume.cluster_chain(&fat, 2, &mut []), None);
    // Starting outside the data region is rejected.
    assert_eq!(volume.cluster_chain(&fat, 1, &mut chain), None);
}

/// Builds a short directory entry with an 8.3 name.
fn directory_entry(name: &[u8; 11], attributes: u8) -> [u8; 32] {
    let mut entry = [0_u8; 32];
    entry[..11].copy_from_slice(name);
    entry[0x0B] = attributes;
    entry
}

#[test]
fn directory_entry_name_and_attributes() {
    let raw = directory_entry(b"README  TXT", ATTR_ARCHIVE);
    let Some(entry) = DirectoryEntry::parse(&raw) else {
        panic!("a full entry must parse");
    };
    assert_eq!(entry.kind(), EntryKind::File);
    assert!(entry.is_archive());
    assert!(!entry.is_directory());
    assert!(!entry.is_read_only());
    assert!(!entry.is_hidden());
    assert!(!entry.is_system());
    let mut name = [0_u8; 13];
    assert_eq!(entry.short_name(&mut name), Some(10));
    assert_eq!(&name[..10], b"README.TXT");
}

#[test]
fn directory_entry_rejects_short_slices() {
    assert!(DirectoryEntry::parse(&[]).is_none());
    assert!(DirectoryEntry::parse(&[0_u8; 31]).is_none());
    let Some(entry) = DirectoryEntry::parse(&[0_u8; 32]) else {
        panic!("a full entry must parse");
    };
    // A name buffer that cannot hold the name is rejected.
    assert!(entry.short_name(&mut [0_u8; 2]).is_none());
}

#[test]
fn directory_entry_kinds() {
    let end = directory_entry(b"\0          ", 0);
    let Some(entry) = DirectoryEntry::parse(&end) else {
        panic!("a full entry must parse");
    };
    assert_eq!(entry.kind(), EntryKind::End);

    let mut deleted = directory_entry(b"GONE    TXT", 0);
    deleted[0] = 0xE5;
    let Some(entry) = DirectoryEntry::parse(&deleted) else {
        panic!("a full entry must parse");
    };
    assert_eq!(entry.kind(), EntryKind::Deleted);

    let volume = directory_entry(b"ROVER      ", ATTR_VOLUME_ID);
    let Some(entry) = DirectoryEntry::parse(&volume) else {
        panic!("a full entry must parse");
    };
    assert_eq!(entry.kind(), EntryKind::VolumeLabel);

    let directory = directory_entry(b"EFI        ", ATTR_DIRECTORY);
    let Some(entry) = DirectoryEntry::parse(&directory) else {
        panic!("a full entry must parse");
    };
    assert_eq!(entry.kind(), EntryKind::Directory);
    assert!(entry.is_directory());
}

#[test]
fn directory_entry_cluster_and_size() {
    let mut raw = directory_entry(b"KERNEL  ELF", ATTR_ARCHIVE);
    raw[0x14..0x16].copy_from_slice(&0x0001_u16.to_le_bytes());
    raw[0x1A..0x1C].copy_from_slice(&0x0020_u16.to_le_bytes());
    raw[0x1C..0x20].copy_from_slice(&0x0001_0000_u32.to_le_bytes());
    let Some(entry) = DirectoryEntry::parse(&raw) else {
        panic!("a full entry must parse");
    };
    assert_eq!(entry.first_cluster(), 0x0001_0020);
    assert_eq!(entry.size(), 0x0001_0000);
}

#[test]
fn directory_entry_lowercase_flags() {
    let mut raw = directory_entry(b"README  TXT", ATTR_ARCHIVE);
    raw[0x0C] = 0x18;
    let Some(entry) = DirectoryEntry::parse(&raw) else {
        panic!("a full entry must parse");
    };
    let mut name = [0_u8; 13];
    assert_eq!(entry.short_name(&mut name), Some(10));
    assert_eq!(&name[..10], b"readme.txt");

    // Only the base name is flagged lowercase.
    raw[0x0C] = 0x08;
    let Some(entry) = DirectoryEntry::parse(&raw) else {
        panic!("a full entry must parse");
    };
    assert_eq!(entry.short_name(&mut name), Some(10));
    assert_eq!(&name[..10], b"readme.TXT");
}

#[test]
fn directory_entry_without_extension() {
    let Some(entry) = DirectoryEntry::parse(&directory_entry(b"EFI        ", ATTR_DIRECTORY))
    else {
        panic!("a full entry must parse");
    };
    let mut name = [0_u8; 13];
    assert_eq!(entry.short_name(&mut name), Some(3));
    assert_eq!(&name[..3], b"EFI");
}

#[test]
fn long_name_entry_decoding() {
    let mut raw = [0xFF_u8; 32];
    raw[0] = 0x41; // Last fragment, order one.
    raw[0x0B] = ATTR_LONG_NAME;
    raw[0x0D] = 0; // Checksum of the short name.
    let units: [u16; 13] = [
        u16::from(b'l'),
        u16::from(b'o'),
        u16::from(b'n'),
        u16::from(b'g'),
        u16::from(b'.'),
        u16::from(b't'),
        u16::from(b'x'),
        u16::from(b't'),
        0,
        0xFFFF,
        0xFFFF,
        0xFFFF,
        0xFFFF,
    ];
    let ranges = [0x01_usize, 0x0E, 0x1C];
    let lengths = [5_usize, 6, 2];
    let mut index = 0_usize;
    for (start, length) in ranges.into_iter().zip(lengths) {
        for offset in 0..length {
            let unit = units[index];
            raw[start + offset * 2..start + offset * 2 + 2].copy_from_slice(&unit.to_le_bytes());
            index += 1;
        }
    }
    let Some(entry) = LongNameEntry::parse(&raw) else {
        panic!("a long-name entry must parse");
    };
    assert_eq!(entry.order(), 1);
    assert!(entry.is_last());
    assert_eq!(entry.len(), 8);
    assert!(!entry.is_empty());
    let units = entry.units();
    assert_eq!(units[0], u16::from(b'l'));
    assert_eq!(units[7], u16::from(b't'));
}

#[test]
fn long_name_entry_rejections() {
    let mut raw = [0_u8; 32];
    raw[0] = 0x41;
    raw[0x0B] = ATTR_ARCHIVE;
    // A plain short entry is not a long-name fragment.
    assert!(LongNameEntry::parse(&raw).is_none());
    // A zero order is invalid.
    raw[0x0B] = ATTR_LONG_NAME;
    raw[0] = 0x40;
    assert!(LongNameEntry::parse(&raw).is_none());
    // A truncated entry is rejected.
    assert!(LongNameEntry::parse(&[0_u8; 31]).is_none());
}

#[test]
fn directory_entry_marks_long_names() {
    let mut raw = [0_u8; 32];
    raw[0] = 0x41;
    raw[0x0B] = ATTR_LONG_NAME;
    let Some(entry) = DirectoryEntry::parse(&raw) else {
        panic!("a full entry must parse");
    };
    assert!(entry.is_long_name());
    assert_eq!(entry.kind(), EntryKind::LongName);
    assert!(!entry.is_directory());
}

#[test]
fn fat32_entry_decoding_masks_reserved_bits() {
    let volume = fat32_volume();
    let mut fat = [0_u8; 32];
    fat[8..12].copy_from_slice(&0xF000_0005_u32.to_le_bytes());
    fat[12..16].copy_from_slice(&0x0FFF_FFFF_u32.to_le_bytes());
    assert_eq!(volume.fat_entry(&fat, 2), Some(5));
    assert_eq!(volume.fat_entry(&fat, 3), Some(0x0FFF_FFFF));
    assert!(volume.kind().is_end_of_chain(0x0FFF_FFFF));
}
