//! # UEFI Image Execution Information Table
//!
//! Provides data structures, execution action flags, and zero-copy parsers for the
//! UEFI Image Execution Information Table (`EFI_IMAGE_EXECUTION_INFO_TABLE`) installed
//! in the EFI System Configuration Table to audit image signature validation and
//! policy decisions taken before image dispatch.

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::security::types::IMAGE_SECURITY_DATABASE_GUID;

/// Standard GUID for the Image Execution Information Table in the System Configuration Table.
pub const EFI_IMAGE_EXECUTION_INFO_TABLE_GUID: EfiGuid = EfiGuid::new(
    0x58FB_BED1,
    0x9E24,
    0x4FA6,
    [0x9D, 0x0F, 0x6A, 0x0B, 0x17, 0x43, 0xA0, 0xC8],
);

/// Image Security Database GUID under which the table is also published in configuration tables.
pub const EFI_IMAGE_SECURITY_DATABASE_GUID: EfiGuid = IMAGE_SECURITY_DATABASE_GUID;

/// Mask for extracting the authentication result code from an execution action bitfield.
pub const EFI_IMAGE_EXECUTION_AUTHENTICATION: u32 = 0x0000_0007;

/// Authentication untested or unverified.
pub const EFI_IMAGE_EXECUTION_AUTH_UNTESTED: u32 = 0x0000_0000;

/// Image signature verification failed.
pub const EFI_IMAGE_EXECUTION_AUTH_SIG_FAILED: u32 = 0x0000_0001;

/// Image signature successfully verified against authorized database (`db`).
pub const EFI_IMAGE_EXECUTION_AUTH_SIG_PASSED: u32 = 0x0000_0002;

/// Signature was not found on the image.
pub const EFI_IMAGE_EXECUTION_AUTH_SIG_NOT_FOUND: u32 = 0x0000_0003;

/// Signature was found in forbidden database (`dbx`).
pub const EFI_IMAGE_EXECUTION_AUTH_SIG_FOUND: u32 = 0x0000_0004;

/// Image execution was denied by platform security policy.
pub const EFI_IMAGE_EXECUTION_POLICY_FAILED: u32 = 0x0000_0005;

/// Image execution was initialized / executed by the firmware.
pub const EFI_IMAGE_EXECUTION_INITIALIZED: u32 = 0x0000_0008;

/// Errors encountered while parsing or building Image Execution Information Tables.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageExecutionError {
    /// Provided buffer is smaller than the required header size.
    BufferTooSmall,
    /// Specified item size is corrupted or exceeds remaining buffer.
    InvalidInfoSize,
    /// Null-terminated UTF-16 image name string is missing terminator.
    UnterminatedName,
    /// Name string is improperly aligned or invalid.
    InvalidName,
    /// Device path length is invalid or exceeds available bytes.
    InvalidDevicePath,
    /// Destination buffer is too small to receive serialized bytes.
    DestinationBufferTooSmall,
    /// Image counter is inconsistent with packed entry count.
    InconsistentImageCount,
}

impl From<ImageExecutionError> for EfiStatus {
    fn from(err: ImageExecutionError) -> Self {
        match err {
            ImageExecutionError::BufferTooSmall
            | ImageExecutionError::DestinationBufferTooSmall
            | ImageExecutionError::InvalidInfoSize => Self::BUFFER_TOO_SMALL,
            ImageExecutionError::UnterminatedName
            | ImageExecutionError::InvalidName
            | ImageExecutionError::InvalidDevicePath
            | ImageExecutionError::InconsistentImageCount => Self::INVALID_PARAMETER,
        }
    }
}

/// Fixed-length header of a serialized `EFI_IMAGE_EXECUTION_INFO` record.
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct EfiImageExecutionInfoHeader {
    /// Action taken by the firmware regarding this image (`EFI_IMAGE_EXECUTION_*`).
    pub action: u32,
    /// Total byte size of the entire `EFI_IMAGE_EXECUTION_INFO` record.
    pub info_size: u32,
}

/// Header of the `EFI_IMAGE_EXECUTION_INFO_TABLE` in system configuration tables.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct EfiImageExecutionInfoTableHeader {
    /// Number of image instances of `EFI_IMAGE_EXECUTION_INFO` structures.
    pub number_of_images: usize,
}

/// Zero-copy parsed view over a single `EFI_IMAGE_EXECUTION_INFO` record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageExecutionInfoView<'a> {
    /// Action taken by firmware (`EFI_IMAGE_EXECUTION_*`).
    pub action: u32,
    /// Null-terminated UTF-16 user-friendly image or device name without trailing null.
    pub name: &'a [u16],
    /// Binary device path specifying location of the evaluated image.
    pub device_path: &'a [u8],
    /// Optional signature list or certificate data attached to the image.
    pub signature: &'a [u8],
}

impl<'a> ImageExecutionInfoView<'a> {
    /// Parses an `EFI_IMAGE_EXECUTION_INFO` record from raw byte slice.
    ///
    /// # Errors
    ///
    /// Returns [`ImageExecutionError`] if bytes are truncated or corrupted.
    #[allow(clippy::cast_ptr_alignment)]
    pub fn parse(bytes: &'a [u8]) -> Result<Self, ImageExecutionError> {
        let hdr_size = core::mem::size_of::<EfiImageExecutionInfoHeader>();
        if bytes.len() < hdr_size {
            return Err(ImageExecutionError::BufferTooSmall);
        }

        let action = u32::from_le_bytes(
            bytes[0..4]
                .try_into()
                .map_err(|_| ImageExecutionError::BufferTooSmall)?,
        );
        let info_size = u32::from_le_bytes(
            bytes[4..8]
                .try_into()
                .map_err(|_| ImageExecutionError::BufferTooSmall)?,
        ) as usize;

        if info_size < hdr_size || bytes.len() < info_size {
            return Err(ImageExecutionError::InvalidInfoSize);
        }

        let payload = &bytes[hdr_size..info_size];

        // Parse null-terminated UTF-16 Name
        let mut name_u16_len = 0;
        let mut found_null = false;
        let max_pairs = payload.len() / 2;

        for i in 0..max_pairs {
            let ch = u16::from_le_bytes([payload[2 * i], payload[2 * i + 1]]);
            if ch == 0 {
                name_u16_len = i;
                found_null = true;
                break;
            }
        }

        if !found_null {
            return Err(ImageExecutionError::UnterminatedName);
        }

        let name_bytes_len = name_u16_len * 2;
        let name_bytes = &payload[..name_bytes_len];
        let name_ptr = name_bytes.as_ptr();

        if name_ptr as usize % core::mem::align_of::<u16>() != 0 {
            return Err(ImageExecutionError::InvalidName);
        }

        let name = unsafe { core::slice::from_raw_parts(name_ptr.cast::<u16>(), name_u16_len) };

        // After null terminator (2 bytes)
        let dp_offset = name_bytes_len + 2;
        if payload.len() < dp_offset + 4 {
            // Must have at least 4 bytes for EndEntireDevicePath node (0x7F, 0xFF, 0x04, 0x00)
            return Err(ImageExecutionError::InvalidDevicePath);
        }

        let after_name = &payload[dp_offset..];
        let dp_len =
            measure_device_path(after_name).ok_or(ImageExecutionError::InvalidDevicePath)?;

        if after_name.len() < dp_len {
            return Err(ImageExecutionError::InvalidDevicePath);
        }

        let device_path = &after_name[..dp_len];
        let signature = &after_name[dp_len..];

        Ok(Self {
            action,
            name,
            device_path,
            signature,
        })
    }

    /// Returns the authentication state portion of the action flags.
    #[must_use]
    pub const fn auth_state(&self) -> u32 {
        self.action & EFI_IMAGE_EXECUTION_AUTHENTICATION
    }

    /// Returns `true` if image signature verified successfully against `db`.
    #[must_use]
    pub const fn is_sig_passed(&self) -> bool {
        self.auth_state() == EFI_IMAGE_EXECUTION_AUTH_SIG_PASSED
    }

    /// Returns `true` if image signature validation failed.
    #[must_use]
    pub const fn is_sig_failed(&self) -> bool {
        self.auth_state() == EFI_IMAGE_EXECUTION_AUTH_SIG_FAILED
    }

    /// Returns `true` if platform policy denied execution.
    #[must_use]
    pub const fn is_policy_failed(&self) -> bool {
        self.auth_state() == EFI_IMAGE_EXECUTION_POLICY_FAILED
    }

    /// Returns `true` if image execution was allowed and initialized.
    #[must_use]
    pub const fn is_initialized(&self) -> bool {
        (self.action & EFI_IMAGE_EXECUTION_INITIALIZED) != 0
    }

    /// Returns total serialized byte size of this record.
    #[must_use]
    pub fn serialized_size(&self) -> usize {
        core::mem::size_of::<EfiImageExecutionInfoHeader>()
            + (self.name.len() + 1) * 2
            + self.device_path.len()
            + self.signature.len()
    }

    /// Serializes this record into destination buffer.
    ///
    /// # Errors
    ///
    /// Returns [`ImageExecutionError::DestinationBufferTooSmall`] if buffer is insufficient.
    pub fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ImageExecutionError> {
        let total = self.serialized_size();
        if buffer.len() < total {
            return Err(ImageExecutionError::DestinationBufferTooSmall);
        }

        let info_size =
            u32::try_from(total).map_err(|_| ImageExecutionError::DestinationBufferTooSmall)?;

        buffer[0..4].copy_from_slice(&self.action.to_le_bytes());
        buffer[4..8].copy_from_slice(&info_size.to_le_bytes());

        let mut offset = 8;
        for &ch in self.name {
            buffer[offset..offset + 2].copy_from_slice(&ch.to_le_bytes());
            offset += 2;
        }
        buffer[offset..offset + 2].copy_from_slice(&0u16.to_le_bytes());
        offset += 2;

        buffer[offset..offset + self.device_path.len()].copy_from_slice(self.device_path);
        offset += self.device_path.len();

        buffer[offset..offset + self.signature.len()].copy_from_slice(self.signature);
        offset += self.signature.len();

        Ok(offset)
    }
}

/// Measures the length of an EFI device path structure by traversing nodes until `EndEntire`.
fn measure_device_path(bytes: &[u8]) -> Option<usize> {
    let mut offset = 0;
    while offset + 4 <= bytes.len() {
        let node_type = bytes[offset];
        let sub_type = bytes[offset + 1];
        let len = u16::from_le_bytes([bytes[offset + 2], bytes[offset + 3]]) as usize;

        if len < 4 || offset + len > bytes.len() {
            return None;
        }

        offset += len;
        // End Entire Device Path is type 0x7F, sub_type 0xFF
        if node_type == 0x7F && sub_type == 0xFF {
            return Some(offset);
        }
    }
    None
}

/// Zero-copy parsed view over the full `EFI_IMAGE_EXECUTION_INFO_TABLE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageExecutionInfoTableView<'a> {
    /// Number of image execution info records recorded in the table.
    pub number_of_images: usize,
    raw_payload: &'a [u8],
}

impl<'a> ImageExecutionInfoTableView<'a> {
    /// Parses an `EFI_IMAGE_EXECUTION_INFO_TABLE` from raw configuration table memory.
    ///
    /// # Errors
    ///
    /// Returns [`ImageExecutionError`] if the table header or records are invalid.
    pub fn parse(bytes: &'a [u8]) -> Result<Self, ImageExecutionError> {
        let hdr_size = core::mem::size_of::<EfiImageExecutionInfoTableHeader>();
        if bytes.len() < hdr_size {
            return Err(ImageExecutionError::BufferTooSmall);
        }

        let num_bytes: [u8; core::mem::size_of::<usize>()] = bytes[..hdr_size]
            .try_into()
            .map_err(|_| ImageExecutionError::BufferTooSmall)?;
        let number_of_images = usize::from_ne_bytes(num_bytes);
        let raw_payload = &bytes[hdr_size..];

        Ok(Self {
            number_of_images,
            raw_payload,
        })
    }

    /// Returns an iterator yielding each `ImageExecutionInfoView` in this table.
    #[must_use]
    pub const fn iter(&self) -> ImageExecutionInfoIter<'a> {
        ImageExecutionInfoIter {
            remaining: self.raw_payload,
            expected: self.number_of_images,
            yielded: 0,
        }
    }
}

impl<'a> IntoIterator for ImageExecutionInfoTableView<'a> {
    type Item = ImageExecutionInfoView<'a>;
    type IntoIter = ImageExecutionInfoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &ImageExecutionInfoTableView<'a> {
    type Item = ImageExecutionInfoView<'a>;
    type IntoIter = ImageExecutionInfoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over image execution info records in an `EFI_IMAGE_EXECUTION_INFO_TABLE`.
#[derive(Clone, Debug)]
pub struct ImageExecutionInfoIter<'a> {
    remaining: &'a [u8],
    expected: usize,
    yielded: usize,
}

impl<'a> Iterator for ImageExecutionInfoIter<'a> {
    type Item = ImageExecutionInfoView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded >= self.expected || self.remaining.is_empty() {
            return None;
        }

        let view = ImageExecutionInfoView::parse(self.remaining).ok()?;
        let info_size = view.serialized_size();
        self.remaining = &self.remaining[info_size..];
        self.yielded += 1;
        Some(view)
    }
}
