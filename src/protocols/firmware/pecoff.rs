//! # PE and COFF images
//!
//! Bounded `PE` and `COFF` parsing for firmware images. The parser
//! reads the `DOS` stub, `PE` signature, `COFF` header, optional
//! header (`PE32` or `PE32+`), section table, and data directories.
//!
//! Parsing describes bytes only. Loading or executing an image remains
//! a firmware operation (`BootServices::LoadImage`).

use crate::base::bytes::{read_u16_le, read_u32_le};

/// The DOS `MZ` magic.
pub const DOS_MAGIC: u16 = 0x5A4D;

/// The `PE` signature value (`PE\0\0`).
pub const PE_SIGNATURE: u32 = 0x0000_4550;

/// The `PE32` optional header magic.
pub const OPTIONAL_MAGIC_PE32: u16 = 0x10B;

/// The `PE32+` optional header magic.
pub const OPTIONAL_MAGIC_PE32_PLUS: u16 = 0x20B;

/// The maximum section count accepted by the parser.
pub const MAX_SECTIONS: u16 = 96;

/// A parsed `DOS` header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DosHeader {
    /// The `MZ` magic.
    pub magic: u16,
    /// The `PE` header offset.
    pub pe_offset: u32,
}

/// A parsed `COFF` header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CoffHeader {
    /// The machine type.
    pub machine: u16,
    /// The section count.
    pub section_count: u16,
    /// The optional header size.
    pub optional_header_size: u16,
    /// The characteristics.
    pub characteristics: u16,
}

/// The optional header kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OptionalKind {
    /// 32-bit image (`PE32`).
    Pe32,
    /// 64-bit image (`PE32+`).
    Pe32Plus,
}

/// A parsed optional header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OptionalHeader {
    /// The magic value.
    pub magic: u16,
    /// The image kind.
    pub kind: OptionalKind,
    /// The entry point relative virtual address.
    pub entry_point: u32,
    /// The image base (`u64` holds both `PE32` and `PE32+`).
    pub image_base: u64,
    /// The section alignment.
    pub section_alignment: u32,
    /// The file alignment.
    pub file_alignment: u32,
    /// The total image size.
    pub size_of_image: u32,
    /// The subsystem.
    pub subsystem: u16,
    /// The data directory count.
    pub directory_count: u32,
}

/// A data directory entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DataDirectory {
    /// The relative virtual address.
    pub virtual_address: u32,
    /// The size in bytes.
    pub size: u32,
}

/// A section header view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SectionHeader {
    /// The raw name bytes.
    pub name: [u8; 8],
    /// The virtual size.
    pub virtual_size: u32,
    /// The virtual address.
    pub virtual_address: u32,
    /// The raw data size.
    pub raw_size: u32,
    /// The raw data pointer.
    pub raw_pointer: u32,
    /// The characteristics.
    pub characteristics: u32,
}

/// A parsed `PE` image view. Name slices borrow from the input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeImage<'a> {
    dos: DosHeader,
    coff: CoffHeader,
    optional: OptionalHeader,
    directories: DataDirectoryView<'a>,
    sections: SectionView<'a>,
}

impl<'a> PeImage<'a> {
    /// Returns the `DOS` header.
    #[must_use]
    pub const fn dos(&self) -> &DosHeader {
        &self.dos
    }

    /// Returns the `COFF` header.
    #[must_use]
    pub const fn coff(&self) -> &CoffHeader {
        &self.coff
    }

    /// Returns the optional header.
    #[must_use]
    pub const fn optional(&self) -> &OptionalHeader {
        &self.optional
    }

    /// Returns the data directory view.
    #[must_use]
    pub const fn directories(&self) -> &DataDirectoryView<'a> {
        &self.directories
    }

    /// Returns the section view.
    #[must_use]
    pub const fn sections(&self) -> &SectionView<'a> {
        &self.sections
    }
}

/// A data directory view borrowing from the image bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataDirectoryView<'a> {
    bytes: &'a [u8],
    offset: usize,
    count: usize,
}

impl DataDirectoryView<'_> {
    /// Returns the directory count.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Returns the directory at `index`, or `None` out of range.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<DataDirectory> {
        if index >= self.count {
            return None;
        }
        let offset = self.offset + index.checked_mul(8)?;
        let va = read_u32_le(self.bytes, offset)?;
        let size = read_u32_le(self.bytes, offset.checked_add(4)?)?;
        Some(DataDirectory {
            virtual_address: va,
            size,
        })
    }
}

/// A section table view borrowing from the image bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectionView<'a> {
    bytes: &'a [u8],
    offset: usize,
    count: usize,
}

impl SectionView<'_> {
    /// Returns the section count.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Returns the section at `index`, or `None` out of range.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<SectionHeader> {
        if index >= self.count {
            return None;
        }
        let offset = self.offset + index.checked_mul(40)?;
        let mut name = [0_u8; 8];
        name.copy_from_slice(self.bytes.get(offset..offset.checked_add(8)?)?);
        Some(SectionHeader {
            name,
            virtual_size: read_u32_le(self.bytes, offset.checked_add(8)?)?,
            virtual_address: read_u32_le(self.bytes, offset.checked_add(12)?)?,
            raw_size: read_u32_le(self.bytes, offset.checked_add(16)?)?,
            raw_pointer: read_u32_le(self.bytes, offset.checked_add(20)?)?,
            characteristics: read_u32_le(self.bytes, offset.checked_add(36)?)?,
        })
    }
}

/// Parses a `PE` image from bytes.
///
/// Returns `None` when the `DOS` magic is wrong, the `PE` offset is
/// out of bounds, the signature mismatches, the `COFF` header is
/// truncated, the optional header magic is unknown, or the section or
/// directory tables exceed the buffer.
///
/// # Errors
///
/// Returns `None` for malformed or truncated images.
#[must_use]
pub fn parse_pe_image(input: &[u8]) -> Option<PeImage<'_>> {
    if input.len() < 64 {
        return None;
    }
    let magic = read_u16_le(input, 0)?;
    if magic != DOS_MAGIC {
        return None;
    }
    let pe_offset = read_u32_le(input, 60)? as usize;
    if pe_offset.checked_add(24)? > input.len() {
        return None;
    }
    if read_u32_le(input, pe_offset)? != PE_SIGNATURE {
        return None;
    }
    let coff_offset = pe_offset.checked_add(4)?;
    let machine = read_u16_le(input, coff_offset)?;
    let section_count = read_u16_le(input, coff_offset.checked_add(2)?)?;
    let optional_header_size = read_u16_le(input, coff_offset.checked_add(16)?)?;
    let characteristics = read_u16_le(input, coff_offset.checked_add(18)?)?;
    if section_count > MAX_SECTIONS {
        return None;
    }
    let coff = CoffHeader {
        machine,
        section_count,
        optional_header_size,
        characteristics,
    };
    let opt_offset = coff_offset.checked_add(20)?;
    if optional_header_size < 2
        || opt_offset.checked_add(usize::from(optional_header_size))? > input.len()
    {
        return None;
    }
    let opt_magic = read_u16_le(input, opt_offset)?;
    let kind = match opt_magic {
        OPTIONAL_MAGIC_PE32 => OptionalKind::Pe32,
        OPTIONAL_MAGIC_PE32_PLUS => OptionalKind::Pe32Plus,
        _ => return None,
    };
    let entry_point = read_u32_le(input, opt_offset.checked_add(16)?)?;
    let image_base = if kind == OptionalKind::Pe32Plus {
        if optional_header_size < 28 {
            return None;
        }
        let mut raw = [0_u8; 8];
        raw.copy_from_slice(input.get(opt_offset.checked_add(24)?..opt_offset.checked_add(32)?)?);
        u64::from_le_bytes(raw)
    } else {
        if optional_header_size < 32 {
            return None;
        }
        u64::from(read_u32_le(input, opt_offset.checked_add(28)?)?)
    };
    let (
        section_alignment,
        file_alignment,
        size_of_image,
        subsystem,
        directory_count,
        dir_offset,
        dir_count_usize,
    ) = parse_optional_extras(input, opt_offset, kind, optional_header_size)?;
    let sections_offset = opt_offset.checked_add(usize::from(optional_header_size))?;
    let total_sections =
        sections_offset.checked_add(usize::from(section_count).checked_mul(40)?)?;
    if total_sections > input.len() {
        return None;
    }
    let directories = DataDirectoryView {
        bytes: input,
        offset: dir_offset,
        count: dir_count_usize,
    };
    let sections = SectionView {
        bytes: input,
        offset: sections_offset,
        count: usize::from(section_count),
    };
    Some(PeImage {
        dos: DosHeader {
            magic,
            pe_offset: u32::try_from(pe_offset).unwrap_or(u32::MAX),
        },
        coff,
        optional: OptionalHeader {
            magic: opt_magic,
            kind,
            entry_point,
            image_base,
            section_alignment,
            file_alignment,
            size_of_image,
            subsystem,
            directory_count,
        },
        directories,
        sections,
    })
}

fn parse_optional_extras(
    input: &[u8],
    opt_offset: usize,
    kind: OptionalKind,
    optional_header_size: u16,
) -> Option<(u32, u32, u32, u16, u32, usize, usize)> {
    let dir_count_offset = if kind == OptionalKind::Pe32Plus {
        108
    } else {
        92
    };
    let dir_offset_rel = if kind == OptionalKind::Pe32Plus {
        112
    } else {
        96
    };
    if usize::from(optional_header_size) < dir_offset_rel {
        return None;
    }
    let sec_align = read_u32_le(input, opt_offset.checked_add(32)?)?;
    let file_align = read_u32_le(input, opt_offset.checked_add(36)?)?;
    let size_of_image = read_u32_le(input, opt_offset.checked_add(56)?)?;
    let subsystem = read_u16_le(input, opt_offset.checked_add(68)?)?;
    let directory_count =
        read_u32_le(input, opt_offset.checked_add(dir_count_offset)?).unwrap_or(16);
    let dir_offset = opt_offset.checked_add(dir_offset_rel)?;
    let clamped = if directory_count > 16 {
        16
    } else {
        directory_count
    } as usize;
    Some((
        sec_align,
        file_align,
        size_of_image,
        subsystem,
        u32::try_from(clamped).unwrap_or(0),
        dir_offset,
        clamped,
    ))
}
