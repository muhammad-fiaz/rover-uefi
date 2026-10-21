//! # Signatures
//!
//! Signature database structures and parsing.
//!
//! A signature database is a sequence of signature lists. Each
//! list has a 28-byte header followed by a type-specific header
//! and an array of signature entries. Each entry has a 16-byte
//! owner `GUID` followed by type-specific data. Unknown
//! signature types are preserved as unknown; parsing does not
//! assume a particular payload format.

use crate::base::EfiGuid;

/// X.509 certificate signature type `GUID`.
pub const EFI_CERT_X509_GUID: EfiGuid = EfiGuid::new(
    0xA5C0_59A1,
    0x94E4,
    0x4AA7,
    [0x87, 0xB5, 0xAB, 0x15, 0x5C, 0x2B, 0xF0, 0x72],
);

/// SHA-256 hash signature type `GUID`.
pub const EFI_CERT_SHA256_GUID: EfiGuid = EfiGuid::new(
    0xC1C4_1626,
    0x504C,
    0x4092,
    [0xAC, 0xA9, 0x41, 0xF9, 0x36, 0x93, 0x43, 0x28],
);

/// SHA-1 hash signature type `GUID`.
pub const EFI_CERT_SHA1_GUID: EfiGuid = EfiGuid::new(
    0x826C_A512,
    0xCF10,
    0x4AC9,
    [0xB1, 0x87, 0xBE, 0x01, 0x49, 0x66, 0x31, 0xBD],
);

/// SHA-224 hash signature type `GUID`.
pub const EFI_CERT_SHA224_GUID: EfiGuid = EfiGuid::new(
    0x0B6E_5233,
    0xA65C,
    0x44C9,
    [0x94, 0x07, 0xD9, 0xAB, 0x83, 0xBF, 0xC8, 0xBD],
);

/// SHA-384 hash signature type `GUID`.
pub const EFI_CERT_SHA384_GUID: EfiGuid = EfiGuid::new(
    0xFF3E_5307,
    0x9FD0,
    0x48C9,
    [0x85, 0xF1, 0x8A, 0xD5, 0x6C, 0x70, 0x1E, 0x01],
);

/// SHA-512 hash signature type `GUID`.
pub const EFI_CERT_SHA512_GUID: EfiGuid = EfiGuid::new(
    0x093E_0FAE,
    0xA6C4,
    0x4F50,
    [0x9F, 0x1B, 0xD4, 0x1E, 0x2B, 0x89, 0xC1, 0x9A],
);

/// RSA-2048 signature type `GUID`.
pub const EFI_CERT_RSA2048_GUID: EfiGuid = EfiGuid::new(
    0x3C57_66E8,
    0x269C,
    0x4E34,
    [0xAA, 0x14, 0xED, 0x77, 0x6E, 0x85, 0xB3, 0xB6],
);

/// RSA-2048 with SHA-256 signature type `GUID`.
pub const EFI_CERT_RSA2048_SHA256_GUID: EfiGuid = EfiGuid::new(
    0xE2B3_6190,
    0x879B,
    0x4A3D,
    [0xAD, 0x8D, 0xF2, 0xE7, 0xBB, 0xA3, 0x27, 0x84],
);

/// RSA-2048 with SHA-1 signature type `GUID`.
pub const EFI_CERT_RSA2048_SHA1_GUID: EfiGuid = EfiGuid::new(
    0x67F8_444F,
    0x8743,
    0x48F1,
    [0xA3, 0x28, 0x1E, 0xAA, 0xB8, 0x73, 0x60, 0x80],
);

/// SM3 hash signature type `GUID`.
pub const EFI_CERT_SM3_GUID: EfiGuid = EfiGuid::new(
    0x5734_7F87,
    0x7A9B,
    0x403A,
    [0xB9, 0x3C, 0xDC, 0x4A, 0xFB, 0x7A, 0x0E, 0xBC],
);

/// X.509 with SHA-256 revocation signature type `GUID`.
pub const EFI_CERT_X509_SHA256_GUID: EfiGuid = EfiGuid::new(
    0x3BD2_A492,
    0x96C0,
    0x4079,
    [0xB4, 0x20, 0xFC, 0xF9, 0x8E, 0xF1, 0x03, 0xED],
);

/// X.509 with SHA-384 revocation signature type `GUID`.
pub const EFI_CERT_X509_SHA384_GUID: EfiGuid = EfiGuid::new(
    0x7076_876E,
    0x80C2,
    0x4EE6,
    [0xAA, 0xD2, 0x28, 0xB3, 0x49, 0xA6, 0x86, 0x5B],
);

/// X.509 with SHA-512 revocation signature type `GUID`.
pub const EFI_CERT_X509_SHA512_GUID: EfiGuid = EfiGuid::new(
    0x446D_BF63,
    0x2502,
    0x4CDA,
    [0xBC, 0xFA, 0x24, 0x65, 0xD2, 0xB0, 0xFE, 0x9D],
);

/// X.509 with SM3 revocation signature type `GUID`.
pub const EFI_CERT_X509_SM3_GUID: EfiGuid = EfiGuid::new(
    0x60D8_07E5,
    0x10B4,
    0x49A9,
    [0x93, 0x31, 0xE4, 0x04, 0x37, 0x88, 0x8D, 0x37],
);

/// PKCS#7 signature type `GUID`.
pub const EFI_CERT_TYPE_PKCS7_GUID: EfiGuid = EfiGuid::new(
    0x4AAF_D29D,
    0x68DF,
    0x49EE,
    [0x8A, 0xA9, 0x34, 0x7D, 0x37, 0x56, 0x65, 0xA7],
);

/// A signature list header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EfiSignatureList {
    /// The signature type `GUID`.
    pub signature_type: EfiGuid,
    /// The list size in bytes.
    pub signature_list_size: u32,
    /// The header size, excluding the 28-byte list header.
    pub signature_header_size: u32,
    /// The size of each signature entry.
    pub signature_size: u32,
}

/// A parsed signature list view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SignatureListInfo<'a> {
    /// The signature type `GUID`.
    pub signature_type: EfiGuid,
    /// The list size in bytes.
    pub signature_list_size: u32,
    /// The header size, excluding the 28-byte list header.
    pub signature_header_size: u32,
    /// The size of each signature entry.
    pub signature_size: u32,
    /// The raw entry bytes after the list and type headers.
    pub signatures: &'a [u8],
    /// The total list size consumed.
    pub total_size: usize,
    /// The offset of the first entry within the input.
    pub entries_offset: usize,
}

/// Iterates signature lists.
pub struct SignatureListIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for SignatureListIter<'a> {
    type Item = SignatureListInfo<'a>;

    /// Returns the next list.
    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.bytes.get(self.offset..)?;
        if remaining.len() < 28 {
            return None;
        }
        let mut guid_bytes = [0_u8; 16];
        guid_bytes.copy_from_slice(remaining.get(0..16)?);
        let mut list_size_raw = [0_u8; 4];
        list_size_raw.copy_from_slice(remaining.get(16..20)?);
        let signature_list_size = u32::from_le_bytes(list_size_raw);
        let list_size = usize::try_from(signature_list_size).ok()?;
        if list_size < 28 || list_size > remaining.len() {
            return None;
        }
        let mut header_size_raw = [0_u8; 4];
        header_size_raw.copy_from_slice(remaining.get(20..24)?);
        let signature_header_size = u32::from_le_bytes(header_size_raw);
        let mut sig_size_raw = [0_u8; 4];
        sig_size_raw.copy_from_slice(remaining.get(24..28)?);
        let signature_size = u32::from_le_bytes(sig_size_raw);
        let header_len = usize::try_from(signature_header_size).ok()?;
        let entries_offset = self.offset.checked_add(28)?.checked_add(header_len)?;
        let list_end = self.offset.checked_add(list_size)?;
        if entries_offset > list_end {
            return None;
        }
        let signatures = self.bytes.get(entries_offset..list_end)?;
        let guid = EfiGuid::from_bytes(guid_bytes);
        self.offset = list_end;
        Some(SignatureListInfo {
            signature_type: guid,
            signature_list_size,
            signature_header_size,
            signature_size,
            signatures,
            total_size: list_size,
            entries_offset,
        })
    }
}

/// Creates a signature list iterator.
#[must_use]
pub fn signature_list_iter(bytes: &[u8]) -> SignatureListIter<'_> {
    SignatureListIter { bytes, offset: 0 }
}

/// Iterates individual signature entries within a list.
///
/// Each entry is `signature_size` bytes with a 16-byte owner
/// `GUID` followed by data. Iteration stops on malformed sizes.
pub struct SignatureDataIter<'a> {
    bytes: &'a [u8],
    offset: usize,
    entry_size: usize,
}

impl<'a> Iterator for SignatureDataIter<'a> {
    type Item = (EfiGuid, &'a [u8]);

    /// Returns the next entry as owner and data.
    fn next(&mut self) -> Option<Self::Item> {
        if self.entry_size < 16 {
            return None;
        }
        let entry = self
            .bytes
            .get(self.offset..self.offset.checked_add(self.entry_size)?)?;
        let mut owner_bytes = [0_u8; 16];
        owner_bytes.copy_from_slice(entry.get(0..16)?);
        let data = entry.get(16..)?;
        self.offset = self.offset.checked_add(self.entry_size)?;
        Some((EfiGuid::from_bytes(owner_bytes), data))
    }
}

/// Creates a signature data iterator.
#[must_use]
pub fn signature_data_iter(bytes: &[u8], signature_size: u32) -> Option<SignatureDataIter<'_>> {
    let size = usize::try_from(signature_size).ok()?;
    if size < 16 {
        return None;
    }
    Some(SignatureDataIter {
        bytes,
        offset: 0,
        entry_size: size,
    })
}

/// Returns the expected signature size for hash types.
#[must_use]
pub fn expected_signature_size(signature_type: &EfiGuid) -> Option<u32> {
    if *signature_type == EFI_CERT_SHA256_GUID {
        Some(16 + 32)
    } else if *signature_type == EFI_CERT_SHA1_GUID {
        Some(16 + 20)
    } else if *signature_type == EFI_CERT_SHA224_GUID {
        Some(16 + 28)
    } else if *signature_type == EFI_CERT_SHA384_GUID {
        Some(16 + 48)
    } else if *signature_type == EFI_CERT_SHA512_GUID {
        Some(16 + 64)
    } else if *signature_type == EFI_CERT_SM3_GUID {
        Some(16 + 32)
    } else if *signature_type == EFI_CERT_RSA2048_GUID
        || *signature_type == EFI_CERT_RSA2048_SHA256_GUID
        || *signature_type == EFI_CERT_RSA2048_SHA1_GUID
    {
        Some(16 + 256)
    } else if *signature_type == EFI_CERT_X509_SHA256_GUID {
        Some(16 + 48)
    } else if *signature_type == EFI_CERT_X509_SHA384_GUID {
        Some(16 + 64)
    } else if *signature_type == EFI_CERT_X509_SHA512_GUID {
        Some(16 + 80)
    } else if *signature_type == EFI_CERT_X509_SM3_GUID {
        Some(16 + 48)
    } else {
        None
    }
}
