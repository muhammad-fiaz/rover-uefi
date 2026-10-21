//! # FAT file system
//!
//! Native parsing of the FAT12, FAT16, and FAT32 on-disk structures.
//!
//! The module reads the BIOS parameter block of a boot sector, decodes
//! file allocation table entries, walks cluster chains with loop and
//! length bounds, and parses directory entries including long names.
//!
//! The parser works on borrowed slices and never allocates. The caller
//! supplies the boot sector, the first file allocation table, and the
//! directory contents; reading them from a block device is the caller's
//! responsibility. The module is a firmware format parser, not a
//! firmware `Simple File System` implementation.

/// The size of one directory entry in bytes.
pub const DIR_ENTRY_SIZE: usize = 32;

/// The number of UTF-16 code units of one long-name chunk.
pub const LONG_NAME_CHUNK: usize = 13;

/// The offset of the boot signature in a boot sector.
const BOOT_SIGNATURE_OFFSET: usize = 510;

/// The end-of-chain marker of FAT12.
const FAT12_END_OF_CHAIN: u32 = 0x0FF8;

/// The bad-cluster marker of FAT12.
const FAT12_BAD_CLUSTER: u32 = 0x0FF7;

/// The end-of-chain marker of FAT16.
const FAT16_END_OF_CHAIN: u32 = 0xFFF8;

/// The bad-cluster marker of FAT16.
const FAT16_BAD_CLUSTER: u32 = 0xFFF7;

/// The end-of-chain marker of FAT32.
const FAT32_END_OF_CHAIN: u32 = 0x0FFF_FFF8;

/// The bad-cluster marker of FAT32.
const FAT32_BAD_CLUSTER: u32 = 0x0FFF_FFF7;

/// The mask that removes the reserved bits of a FAT32 entry.
const FAT32_ENTRY_MASK: u32 = 0x0FFF_FFFF;

/// The first cluster number that holds data.
pub const FIRST_DATA_CLUSTER: u32 = 2;

/// The attribute bit of a read-only entry.
pub const ATTR_READ_ONLY: u8 = 0x01;

/// The attribute bit of a hidden entry.
pub const ATTR_HIDDEN: u8 = 0x02;

/// The attribute bit of a system entry.
pub const ATTR_SYSTEM: u8 = 0x04;

/// The attribute bit of a volume label.
pub const ATTR_VOLUME_ID: u8 = 0x08;

/// The attribute bit of a directory.
pub const ATTR_DIRECTORY: u8 = 0x10;

/// The attribute bit of an archive.
pub const ATTR_ARCHIVE: u8 = 0x20;

/// The attribute value of a long-name entry.
pub const ATTR_LONG_NAME: u8 = 0x0F;

/// The variant of a FAT volume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FatKind {
    /// A FAT12 volume.
    Fat12,
    /// A FAT16 volume.
    Fat16,
    /// A FAT32 volume.
    Fat32,
}

impl FatKind {
    /// Returns the end-of-chain marker of the variant.
    #[must_use]
    pub const fn end_of_chain(self) -> u32 {
        match self {
            Self::Fat12 => FAT12_END_OF_CHAIN,
            Self::Fat16 => FAT16_END_OF_CHAIN,
            Self::Fat32 => FAT32_END_OF_CHAIN,
        }
    }

    /// Returns the bad-cluster marker of the variant.
    #[must_use]
    pub const fn bad_cluster(self) -> u32 {
        match self {
            Self::Fat12 => FAT12_BAD_CLUSTER,
            Self::Fat16 => FAT16_BAD_CLUSTER,
            Self::Fat32 => FAT32_BAD_CLUSTER,
        }
    }

    /// Returns whether `value` ends a cluster chain.
    #[must_use]
    pub const fn is_end_of_chain(self, value: u32) -> bool {
        value >= self.end_of_chain()
    }
}

/// The parsed BIOS parameter block of a FAT volume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FatVolume {
    kind: FatKind,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    fat_count: u8,
    root_entry_count: u16,
    total_sectors: u32,
    fat_size_sectors: u32,
    root_cluster: u32,
    first_data_sector: u32,
    cluster_count: u32,
}

impl FatVolume {
    /// Parses the boot sector of a FAT volume.
    ///
    /// Returns `None` when the sector is too short, the boot signature
    /// is missing, a geometry field is out of range, or the volume
    /// describes no data cluster.
    #[must_use]
    pub fn parse_boot_sector(sector: &[u8]) -> Option<Self> {
        let signature = sector.get(BOOT_SIGNATURE_OFFSET..BOOT_SIGNATURE_OFFSET + 2)?;
        if signature != [0x55, 0xAA] {
            return None;
        }
        let bytes_per_sector = read_u16(sector, 0x0B)?;
        if !matches!(bytes_per_sector, 512 | 1024 | 2048 | 4096) {
            return None;
        }
        let sectors_per_cluster = *sector.get(0x0D)?;
        if sectors_per_cluster == 0
            || !sectors_per_cluster.is_power_of_two()
            || sectors_per_cluster > 128
        {
            return None;
        }
        let reserved_sectors = read_u16(sector, 0x0E)?;
        if reserved_sectors == 0 {
            return None;
        }
        let fat_count = *sector.get(0x10)?;
        if fat_count == 0 {
            return None;
        }
        let root_entry_count = read_u16(sector, 0x11)?;
        let total_sectors = match read_u16(sector, 0x13)? {
            0 => read_u32(sector, 0x20)?,
            short => u32::from(short),
        };
        if total_sectors == 0 {
            return None;
        }
        let fat_size_16 = read_u16(sector, 0x16)?;
        let (fat_size_sectors, root_cluster) = if fat_size_16 == 0 {
            let fat_size_32 = read_u32(sector, 0x24)?;
            let root_cluster = read_u32(sector, 0x2C)?;
            if root_cluster < FIRST_DATA_CLUSTER {
                return None;
            }
            (fat_size_32, root_cluster)
        } else {
            // FAT12 and FAT16 keep a fixed-size root directory.
            if root_entry_count == 0 {
                return None;
            }
            (u32::from(fat_size_16), 0)
        };
        if fat_size_sectors == 0 {
            return None;
        }
        let root_dir_sectors = root_dir_sectors(root_entry_count, bytes_per_sector);
        let fats_size = u32::from(fat_count).checked_mul(fat_size_sectors)?;
        let first_data_sector = u32::from(reserved_sectors)
            .checked_add(fats_size)?
            .checked_add(root_dir_sectors)?;
        if first_data_sector >= total_sectors {
            return None;
        }
        let data_sectors = total_sectors.checked_sub(first_data_sector)?;
        let cluster_count = data_sectors / u32::from(sectors_per_cluster);
        if cluster_count == 0 {
            return None;
        }
        let kind = if fat_size_16 == 0 {
            FatKind::Fat32
        } else {
            // The cluster count thresholds are the ones the FAT
            // specification defines for the 12-bit and 16-bit variants.
            if cluster_count < 4085 {
                FatKind::Fat12
            } else if cluster_count < 65_525 {
                FatKind::Fat16
            } else {
                return None;
            }
        };
        Some(Self {
            kind,
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            fat_count,
            root_entry_count,
            total_sectors,
            fat_size_sectors,
            root_cluster,
            first_data_sector,
            cluster_count,
        })
    }

    /// Returns the variant of the volume.
    #[must_use]
    pub const fn kind(&self) -> FatKind {
        self.kind
    }

    /// Returns the size of a sector in bytes.
    #[must_use]
    pub const fn bytes_per_sector(&self) -> u16 {
        self.bytes_per_sector
    }

    /// Returns the number of sectors of one cluster.
    #[must_use]
    pub const fn sectors_per_cluster(&self) -> u8 {
        self.sectors_per_cluster
    }

    /// Returns the size of one cluster in bytes.
    #[must_use]
    pub fn cluster_size(&self) -> usize {
        usize::from(self.bytes_per_sector) * usize::from(self.sectors_per_cluster)
    }

    /// Returns the number of reserved sectors.
    #[must_use]
    pub const fn reserved_sectors(&self) -> u16 {
        self.reserved_sectors
    }

    /// Returns the number of file allocation tables.
    #[must_use]
    pub const fn fat_count(&self) -> u8 {
        self.fat_count
    }

    /// Returns the number of root directory entries.
    ///
    /// The value is zero on a FAT32 volume, which stores its root
    /// directory in a cluster chain.
    #[must_use]
    pub const fn root_entry_count(&self) -> u16 {
        self.root_entry_count
    }

    /// Returns the total number of sectors of the volume.
    #[must_use]
    pub const fn total_sectors(&self) -> u32 {
        self.total_sectors
    }

    /// Returns the size of one file allocation table in sectors.
    #[must_use]
    pub const fn fat_size_sectors(&self) -> u32 {
        self.fat_size_sectors
    }

    /// Returns the first cluster of the root directory.
    ///
    /// The value is zero on FAT12 and FAT16, whose root directory is
    /// a fixed region after the file allocation tables.
    #[must_use]
    pub const fn root_cluster(&self) -> u32 {
        self.root_cluster
    }

    /// Returns the first sector of the data region.
    #[must_use]
    pub const fn first_data_sector(&self) -> u32 {
        self.first_data_sector
    }

    /// Returns the number of data clusters of the volume.
    #[must_use]
    pub const fn cluster_count(&self) -> u32 {
        self.cluster_count
    }

    /// Returns the sector that holds the first file allocation table.
    #[must_use]
    pub fn first_fat_sector(&self) -> u32 {
        u32::from(self.reserved_sectors)
    }

    /// Returns the sector of the fixed root directory of FAT12 and
    /// FAT16, or `None` on FAT32.
    #[must_use]
    pub fn root_directory_sector(&self) -> Option<u32> {
        if self.kind == FatKind::Fat32 || self.root_entry_count == 0 {
            return None;
        }
        let fats = u32::from(self.fat_count).checked_mul(self.fat_size_sectors)?;
        u32::from(self.reserved_sectors).checked_add(fats)
    }

    /// Returns whether `cluster` is a data cluster of the volume.
    #[must_use]
    pub fn is_data_cluster(&self, cluster: u32) -> bool {
        cluster >= FIRST_DATA_CLUSTER && cluster < self.cluster_count + FIRST_DATA_CLUSTER
    }

    /// Returns the first sector of `cluster`.
    #[must_use]
    pub fn cluster_to_sector(&self, cluster: u32) -> Option<u32> {
        if !self.is_data_cluster(cluster) {
            return None;
        }
        let index = cluster.checked_sub(FIRST_DATA_CLUSTER)?;
        let offset = index.checked_mul(u32::from(self.sectors_per_cluster))?;
        self.first_data_sector.checked_add(offset)
    }

    /// Decodes the file allocation table entry of `cluster`.
    ///
    /// `fat` must hold at least the first entry of the table; it may be
    /// the whole table or a prefix that reaches the requested entry.
    /// Returns `None` for an out-of-range cluster or a table that is
    /// too short.
    #[must_use]
    pub fn fat_entry(&self, fat: &[u8], cluster: u32) -> Option<u32> {
        if !self.is_data_cluster(cluster) {
            return None;
        }
        match self.kind {
            FatKind::Fat12 => {
                let index = cluster.checked_add(cluster / 2)?;
                let entry = u32::from(read_u16(fat, usize::try_from(index).ok()?)?);
                if cluster & 1 == 0 {
                    Some(entry & 0x0FFF)
                } else {
                    Some(entry >> 4)
                }
            }
            FatKind::Fat16 => {
                let index = usize::try_from(cluster.checked_mul(2)?).ok()?;
                Some(u32::from(read_u16(fat, index)?))
            }
            FatKind::Fat32 => {
                let index = usize::try_from(cluster.checked_mul(4)?).ok()?;
                Some(read_u32(fat, index)? & FAT32_ENTRY_MASK)
            }
        }
    }

    /// Follows the cluster chain that starts at `start`.
    ///
    /// The cluster numbers are written to `out` and the number of
    /// clusters is returned. Returns `None` when the chain is invalid,
    /// revisits a cluster, contains a bad cluster, or does not end
    /// within `out`.
    #[must_use]
    pub fn cluster_chain(&self, fat: &[u8], start: u32, out: &mut [u32]) -> Option<usize> {
        if out.is_empty() {
            return None;
        }
        let mut cluster = start;
        let mut count = 0_usize;
        // A valid chain visits at most one cluster per data cluster of
        // the volume.
        let limit = usize::try_from(self.cluster_count).ok()?;
        while count < limit {
            if !self.is_data_cluster(cluster) {
                return None;
            }
            *out.get_mut(count)? = cluster;
            count = count.checked_add(1)?;
            let next = self.fat_entry(fat, cluster)?;
            if self.kind.is_end_of_chain(next) {
                return Some(count);
            }
            if next == self.kind.bad_cluster() || !self.is_data_cluster(next) {
                return None;
            }
            cluster = next;
        }
        None
    }

    /// Returns the size of the volume in bytes.
    ///
    /// Returns `None` when the size does not fit into a 64-bit value.
    #[must_use]
    pub fn size_bytes(&self) -> Option<u64> {
        u64::from(self.total_sectors).checked_mul(u64::from(self.bytes_per_sector))
    }

    /// Returns the number of root directory sectors of the volume.
    #[must_use]
    pub fn root_directory_sectors(&self) -> u32 {
        root_dir_sectors(self.root_entry_count, self.bytes_per_sector)
    }
}

/// Returns the number of sectors of a fixed root directory.
fn root_dir_sectors(entries: u16, bytes_per_sector: u16) -> u32 {
    if entries == 0 || bytes_per_sector == 0 {
        return 0;
    }
    let per_sector = u32::from(bytes_per_sector) / u32::try_from(DIR_ENTRY_SIZE).unwrap_or(32);
    if per_sector == 0 {
        return 0;
    }
    u32::from(entries).div_ceil(per_sector)
}

/// The kind of a directory entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryKind {
    /// A regular file.
    File,
    /// A directory.
    Directory,
    /// A volume label.
    VolumeLabel,
    /// A long-name fragment.
    LongName,
    /// A deleted entry.
    Deleted,
    /// The end of the directory.
    End,
}

/// A short directory entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DirectoryEntry {
    raw_name: [u8; 11],
    case_flags: u8,
    attributes: u8,
    first_cluster: u32,
    size: u32,
}

impl DirectoryEntry {
    /// Parses one 32-byte directory entry.
    ///
    /// Returns `None` when the slice is too short.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let chunk = bytes.get(..DIR_ENTRY_SIZE)?;
        let mut raw_name = [0_u8; 11];
        raw_name.copy_from_slice(chunk.get(..11)?);
        let case_flags = *chunk.get(0x0C)?;
        let attributes = *chunk.get(0x0B)?;
        let high = u32::from(read_u16(chunk, 0x14)?);
        let low = u32::from(read_u16(chunk, 0x1A)?);
        let first_cluster = (high << 16) | low;
        let size = read_u32(chunk, 0x1C)?;
        Some(Self {
            raw_name,
            case_flags,
            attributes,
            first_cluster,
            size,
        })
    }

    /// Returns the kind of the entry.
    #[must_use]
    pub fn kind(&self) -> EntryKind {
        match self.raw_name.first().copied().unwrap_or(0) {
            0x00 => EntryKind::End,
            0xE5 => EntryKind::Deleted,
            _ if self.is_long_name() => EntryKind::LongName,
            _ if self.is_volume_label() => EntryKind::VolumeLabel,
            _ if self.is_directory() => EntryKind::Directory,
            _ => EntryKind::File,
        }
    }

    /// Returns the attribute byte.
    #[must_use]
    pub const fn attributes(&self) -> u8 {
        self.attributes
    }

    /// Returns whether the entry is a long-name fragment.
    #[must_use]
    pub const fn is_long_name(&self) -> bool {
        self.attributes & 0x3F == ATTR_LONG_NAME
    }

    /// Returns whether the entry is a directory.
    #[must_use]
    pub const fn is_directory(&self) -> bool {
        self.attributes & ATTR_DIRECTORY != 0 && !self.is_long_name()
    }

    /// Returns whether the entry is a volume label.
    #[must_use]
    pub const fn is_volume_label(&self) -> bool {
        self.attributes & ATTR_VOLUME_ID != 0 && !self.is_long_name()
    }

    /// Returns whether the entry is read-only.
    #[must_use]
    pub const fn is_read_only(&self) -> bool {
        self.attributes & ATTR_READ_ONLY != 0
    }

    /// Returns whether the entry is hidden.
    #[must_use]
    pub const fn is_hidden(&self) -> bool {
        self.attributes & ATTR_HIDDEN != 0
    }

    /// Returns whether the entry is a system entry.
    #[must_use]
    pub const fn is_system(&self) -> bool {
        self.attributes & ATTR_SYSTEM != 0
    }

    /// Returns whether the entry is archived.
    #[must_use]
    pub const fn is_archive(&self) -> bool {
        self.attributes & ATTR_ARCHIVE != 0
    }

    /// Returns whether the entry starts a cluster chain.
    #[must_use]
    pub const fn first_cluster(&self) -> u32 {
        self.first_cluster
    }

    /// Returns the size of the file in bytes.
    #[must_use]
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Writes the 8.3 name into `out`.
    ///
    /// The name is written without a trailing NUL and the number of
    /// bytes is returned. Returns `None` when `out` is too small.
    #[must_use]
    pub fn short_name(&self, out: &mut [u8]) -> Option<usize> {
        let mut written = 0_usize;
        let mut base_len = 0_usize;
        while base_len < 8 && self.raw_name.get(base_len).copied().unwrap_or(b' ') != b' ' {
            base_len += 1;
        }
        let mut extension_len = 0_usize;
        while extension_len < 3
            && self
                .raw_name
                .get(8 + extension_len)
                .copied()
                .unwrap_or(b' ')
                != b' '
        {
            extension_len += 1;
        }
        let lower_base = self.case_flags & 0x08 != 0;
        let lower_extension = self.case_flags & 0x10 != 0;
        for index in 0..base_len {
            let mut byte = *self.raw_name.get(index)?;
            if index == 0 && byte == 0x05 {
                byte = 0xE5;
            }
            if lower_base {
                byte = byte.to_ascii_lowercase();
            }
            *out.get_mut(written)? = byte;
            written += 1;
        }
        if extension_len > 0 {
            *out.get_mut(written)? = b'.';
            written += 1;
            for index in 0..extension_len {
                let mut byte = *self.raw_name.get(8 + index)?;
                if lower_extension {
                    byte = byte.to_ascii_lowercase();
                }
                *out.get_mut(written)? = byte;
                written += 1;
            }
        }
        Some(written)
    }
}

/// A decoded long-name fragment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LongNameEntry {
    order: u8,
    last: bool,
    units: [u16; LONG_NAME_CHUNK],
}

impl LongNameEntry {
    /// Parses a long-name directory entry.
    ///
    /// Returns `None` when the slice is too short or the entry is not a
    /// long-name fragment.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let chunk = bytes.get(..DIR_ENTRY_SIZE)?;
        let attributes = *chunk.get(0x0B)?;
        if attributes & 0x3F != ATTR_LONG_NAME {
            return None;
        }
        let raw_order = *chunk.first()?;
        let order = raw_order & 0x1F;
        if order == 0 {
            return None;
        }
        let mut units = [0xFFFF_u16; LONG_NAME_CHUNK];
        let ranges = [0x01_usize, 0x0E, 0x1C];
        let lengths = [5_usize, 6, 2];
        let mut written = 0_usize;
        for (start, length) in ranges.into_iter().zip(lengths) {
            for index in 0..length {
                let unit = read_u16(chunk, start + index * 2)?;
                if let Some(slot) = units.get_mut(written) {
                    *slot = unit;
                }
                written += 1;
            }
        }
        Some(Self {
            order,
            last: raw_order & 0x40 != 0,
            units,
        })
    }

    /// Returns the one-based position of the fragment.
    #[must_use]
    pub const fn order(&self) -> u8 {
        self.order
    }

    /// Returns whether the fragment is the last of the name.
    #[must_use]
    pub const fn is_last(&self) -> bool {
        self.last
    }

    /// Returns the UTF-16 code units of the fragment.
    #[must_use]
    pub const fn units(&self) -> &[u16; LONG_NAME_CHUNK] {
        &self.units
    }

    /// Returns the number of meaningful code units of the fragment.
    ///
    /// The name is terminated by a NUL code unit or padded with
    /// `0xFFFF`; neither is counted.
    #[must_use]
    pub fn len(&self) -> usize {
        let mut length = 0_usize;
        for unit in self.units {
            if unit == 0 || unit == 0xFFFF {
                break;
            }
            length += 1;
        }
        length
    }

    /// Returns whether the fragment carries no name code unit.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Reads a little-endian 16-bit value at `offset`.
fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let slice = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([*slice.first()?, *slice.get(1)?]))
}

/// Reads a little-endian 32-bit value at `offset`.
fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([
        *slice.first()?,
        *slice.get(1)?,
        *slice.get(2)?,
        *slice.get(3)?,
    ]))
}
