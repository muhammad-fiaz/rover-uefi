//! # Platform Configuration Database (PCD) Protocols and PPIs
//!
//! Defined in the Platform Initialization (PI) Specification Volume 3.
//!
//! The Platform Configuration Database provides a unified abstraction for
//! configuration settings, directives, and platform options across PEI and DXE
//! execution phases. It supports typed datum extraction (8, 16, 32, 64-bit,
//! boolean, and pointer buffers) with SKU-based parameterization and change
//! notification callbacks.

#![allow(clippy::unreadable_literal)]

use crate::abi::Bool;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// EFI PCD Protocol GUID (`EFI_PCD_PROTOCOL_GUID`).
pub const EFI_PCD_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x13a3_f0f6,
    0x264a,
    0x3ef0,
    [0xf2, 0xe0, 0xde, 0xc5, 0x12, 0x34, 0x2f, 0x34],
);

/// EFI Get PCD Info Protocol GUID (`EFI_GET_PCD_INFO_PROTOCOL_GUID`).
pub const EFI_GET_PCD_INFO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xfd0f_4478,
    0x0efd,
    0x461d,
    [0xba, 0x2d, 0xe5, 0x8c, 0x45, 0xfd, 0x5f, 0x5e],
);

/// EFI PEI PCD PPI GUID (`EFI_PEI_PCD_PPI_GUID`).
pub const EFI_PEI_PCD_PPI_GUID: EfiGuid = EfiGuid::new(
    0x01f3_4d25,
    0x4de2,
    0x23ad,
    [0x3f, 0xf3, 0x36, 0x35, 0x3f, 0xf3, 0x23, 0xf1],
);

/// EFI PEI Get PCD Info PPI GUID (`EFI_PEI_GET_PCD_INFO_PPI_GUID`).
pub const EFI_PEI_GET_PCD_INFO_PPI_GUID: EfiGuid = EfiGuid::new(
    0xa60c_be59,
    0xd4a1,
    0x4386,
    [0x92, 0x40, 0xe1, 0x86, 0x49, 0x28, 0x58, 0x44],
);

/// Special token number value indicating an invalid PCD token.
pub const EFI_PCD_INVALID_TOKEN_NUMBER: usize = 0;

/// Default SKU identifier (0 is reserved as the universal default SKU).
pub const EFI_PCD_DEFAULT_SKU_ID: usize = 0;

/// PCD Datum Types as defined in the PI specification.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EfiPcdType {
    /// 8-bit unsigned integer.
    Type8 = 0,
    /// 16-bit unsigned integer.
    Type16 = 1,
    /// 32-bit unsigned integer.
    Type32 = 2,
    /// 64-bit unsigned integer.
    Type64 = 3,
    /// Boolean value.
    TypeBool = 4,
    /// Pointer to variable-length binary buffer or string.
    TypePtr = 5,
}

/// Information record describing a PCD token.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EfiPcdInfo {
    /// Datum type associated with this token.
    pub pcd_type: EfiPcdType,
    /// Size of the datum in bytes.
    pub pcd_size: usize,
    /// Null-terminated ASCII name of the token or token space.
    pub pcd_name: *const u8,
}

/// Function signature for PCD modification callback notifications.
pub type EfiPcdProtocolCallback = unsafe extern "efiapi" fn(
    guid: *const EfiGuid,
    callback_token: usize,
    token_data: *mut c_void,
    token_data_size: usize,
);

/// Function pointer to set the active platform SKU.
pub type EfiPcdSetSku = unsafe extern "efiapi" fn(sku_id: usize);
/// Function pointer to retrieve an 8-bit PCD value.
pub type EfiPcdGet8 = unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize) -> u8;
/// Function pointer to retrieve a 16-bit PCD value.
pub type EfiPcdGet16 = unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize) -> u16;
/// Function pointer to retrieve a 32-bit PCD value.
pub type EfiPcdGet32 = unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize) -> u32;
/// Function pointer to retrieve a 64-bit PCD value.
pub type EfiPcdGet64 = unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize) -> u64;
/// Function pointer to retrieve a pointer to a PCD buffer value.
pub type EfiPcdGetPtr =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize) -> *mut c_void;
/// Function pointer to retrieve a boolean PCD value.
pub type EfiPcdGetBool =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize) -> Bool;
/// Function pointer to retrieve the byte size of a PCD value.
pub type EfiPcdGetSize =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize) -> usize;

/// Function pointer to set an 8-bit PCD value.
pub type EfiPcdSet8 =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize, value: u8) -> EfiStatus;
/// Function pointer to set a 16-bit PCD value.
pub type EfiPcdSet16 =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize, value: u16) -> EfiStatus;
/// Function pointer to set a 32-bit PCD value.
pub type EfiPcdSet32 =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize, value: u32) -> EfiStatus;
/// Function pointer to set a 64-bit PCD value.
pub type EfiPcdSet64 =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize, value: u64) -> EfiStatus;
/// Function pointer to set a buffer PCD value.
pub type EfiPcdSetPtr = unsafe extern "efiapi" fn(
    guid: *const EfiGuid,
    token_number: usize,
    size_of_value: *mut usize,
    buffer: *mut c_void,
) -> EfiStatus;
/// Function pointer to set a boolean PCD value.
pub type EfiPcdSetBool =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: usize, value: Bool) -> EfiStatus;

/// Function pointer to register a modification callback.
pub type EfiPcdCallbackOnSet = unsafe extern "efiapi" fn(
    guid: *const EfiGuid,
    callback_token: usize,
    callback_function: EfiPcdProtocolCallback,
) -> EfiStatus;

/// Function pointer to cancel a modification callback.
pub type EfiPcdCancelCallback = unsafe extern "efiapi" fn(
    guid: *const EfiGuid,
    callback_token: usize,
    callback_function: EfiPcdProtocolCallback,
) -> EfiStatus;

/// Function pointer to enumerate the next valid PCD token number.
pub type EfiPcdGetNextToken =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, token_number: *mut usize) -> EfiStatus;

/// Function pointer to enumerate the next valid PCD token namespace.
pub type EfiPcdGetNextTokenSpace =
    unsafe extern "efiapi" fn(guid: *mut *const EfiGuid) -> EfiStatus;

/// EFI PCD Protocol (`EFI_PCD_PROTOCOL`).
///
/// Implemented by the DXE PCD driver to provide dynamic platform configuration access.
#[repr(C)]
pub struct EfiPcdProtocol {
    /// Sets the active platform SKU.
    pub set_sku: EfiPcdSetSku,
    /// Retrieves an 8-bit PCD value.
    pub get_8: EfiPcdGet8,
    /// Retrieves a 16-bit PCD value.
    pub get_16: EfiPcdGet16,
    /// Retrieves a 32-bit PCD value.
    pub get_32: EfiPcdGet32,
    /// Retrieves a 64-bit PCD value.
    pub get_64: EfiPcdGet64,
    /// Retrieves a pointer PCD value.
    pub get_ptr: EfiPcdGetPtr,
    /// Retrieves a boolean PCD value.
    pub get_bool: EfiPcdGetBool,
    /// Retrieves the byte size of a PCD value.
    pub get_size: EfiPcdGetSize,
    /// Sets an 8-bit PCD value.
    pub set_8: EfiPcdSet8,
    /// Sets a 16-bit PCD value.
    pub set_16: EfiPcdSet16,
    /// Sets a 32-bit PCD value.
    pub set_32: EfiPcdSet32,
    /// Sets a 64-bit PCD value.
    pub set_64: EfiPcdSet64,
    /// Sets a pointer/buffer PCD value.
    pub set_ptr: EfiPcdSetPtr,
    /// Sets a boolean PCD value.
    pub set_bool: EfiPcdSetBool,
    /// Registers a change notification callback.
    pub callback_on_set: EfiPcdCallbackOnSet,
    /// Cancels a change notification callback.
    pub cancel_callback: EfiPcdCancelCallback,
    /// Enumerates the next token number.
    pub get_next_token: EfiPcdGetNextToken,
    /// Enumerates the next token namespace GUID.
    pub get_next_token_space: EfiPcdGetNextTokenSpace,
}

/// Function to retrieve metadata associated with a PCD token.
pub type EfiGetPcdInfoGetInfo = unsafe extern "efiapi" fn(
    guid: *const EfiGuid,
    token_number: usize,
    pcd_info: *mut EfiPcdInfo,
) -> EfiStatus;

/// Function to retrieve the active SKU identifier.
pub type EfiGetPcdInfoGetSku = unsafe extern "efiapi" fn() -> usize;

/// EFI Get PCD Info Protocol (`EFI_GET_PCD_INFO_PROTOCOL`).
#[repr(C)]
pub struct EfiGetPcdInfoProtocol {
    /// Retrieves information and metadata for a PCD token.
    pub get_info: EfiGetPcdInfoGetInfo,
    /// Retrieves the currently active SKU identifier.
    pub get_sku: EfiGetPcdInfoGetSku,
}

/// EFI PEI PCD PPI (`EFI_PEI_PCD_PPI`).
///
/// Implemented by the PEI PCD PEIM to provide platform configuration access during PEI.
#[repr(C)]
pub struct EfiPeiPcdPpi {
    /// Sets the active platform SKU.
    pub set_sku: EfiPcdSetSku,
    /// Retrieves an 8-bit PCD value.
    pub get_8: EfiPcdGet8,
    /// Retrieves a 16-bit PCD value.
    pub get_16: EfiPcdGet16,
    /// Retrieves a 32-bit PCD value.
    pub get_32: EfiPcdGet32,
    /// Retrieves a 64-bit PCD value.
    pub get_64: EfiPcdGet64,
    /// Retrieves a pointer PCD value.
    pub get_ptr: EfiPcdGetPtr,
    /// Retrieves a boolean PCD value.
    pub get_bool: EfiPcdGetBool,
    /// Retrieves the byte size of a PCD value.
    pub get_size: EfiPcdGetSize,
    /// Sets an 8-bit PCD value.
    pub set_8: EfiPcdSet8,
    /// Sets a 16-bit PCD value.
    pub set_16: EfiPcdSet16,
    /// Sets a 32-bit PCD value.
    pub set_32: EfiPcdSet32,
    /// Sets a 64-bit PCD value.
    pub set_64: EfiPcdSet64,
    /// Sets a pointer/buffer PCD value.
    pub set_ptr: EfiPcdSetPtr,
    /// Sets a boolean PCD value.
    pub set_bool: EfiPcdSetBool,
    /// Registers a change notification callback.
    pub callback_on_set: EfiPcdCallbackOnSet,
    /// Cancels a change notification callback.
    pub cancel_callback: EfiPcdCancelCallback,
    /// Enumerates the next token number.
    pub get_next_token: EfiPcdGetNextToken,
    /// Enumerates the next token namespace GUID.
    pub get_next_token_space: EfiPcdGetNextTokenSpace,
}

/// EFI PEI Get PCD Info PPI (`EFI_PEI_GET_PCD_INFO_PPI`).
#[repr(C)]
pub struct EfiPeiGetPcdInfoPpi {
    /// Retrieves information and metadata for a PCD token.
    pub get_info: EfiGetPcdInfoGetInfo,
    /// Retrieves the currently active SKU identifier.
    pub get_sku: EfiGetPcdInfoGetSku,
}

// -----------------------------------------------------------------------------
// Safe Native In-Memory PCD Database Implementation
// -----------------------------------------------------------------------------

/// Safe representation of a dynamic PCD datum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcdValue<'a> {
    /// 8-bit unsigned integer.
    U8(u8),
    /// 16-bit unsigned integer.
    U16(u16),
    /// 32-bit unsigned integer.
    U32(u32),
    /// 64-bit unsigned integer.
    U64(u64),
    /// Boolean flag.
    Bool(bool),
    /// Variable-length byte slice.
    Bytes(&'a [u8]),
}

impl PcdValue<'_> {
    /// Returns the corresponding `EfiPcdType`.
    #[must_use]
    pub const fn pcd_type(&self) -> EfiPcdType {
        match self {
            Self::U8(_) => EfiPcdType::Type8,
            Self::U16(_) => EfiPcdType::Type16,
            Self::U32(_) => EfiPcdType::Type32,
            Self::U64(_) => EfiPcdType::Type64,
            Self::Bool(_) => EfiPcdType::TypeBool,
            Self::Bytes(_) => EfiPcdType::TypePtr,
        }
    }

    /// Returns the length of the datum in bytes.
    #[must_use]
    pub const fn size(&self) -> usize {
        match self {
            Self::U16(_) => 2,
            Self::U32(_) => 4,
            Self::U64(_) => 8,
            Self::U8(_) | Self::Bool(_) => 1,
            Self::Bytes(slice) => slice.len(),
        }
    }
}

/// An entry in the Platform Configuration Database.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PcdEntry<'a> {
    /// Token space namespace GUID.
    pub token_space: EfiGuid,
    /// Token number within the namespace.
    pub token_number: u32,
    /// Human-readable token name.
    pub name: &'static str,
    /// SKU identifier (0 = universal default).
    pub sku_id: usize,
    /// Stored value.
    pub value: PcdValue<'a>,
}

/// Safe in-memory Platform Configuration Database with fixed capacity.
pub struct PcdDatabase<'a, const CAPACITY: usize> {
    entries: [Option<PcdEntry<'a>>; CAPACITY],
    count: usize,
    active_sku: usize,
}

impl<'a, const CAPACITY: usize> PcdDatabase<'a, CAPACITY> {
    /// Creates an empty database.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: [None; CAPACITY],
            count: 0,
            active_sku: EFI_PCD_DEFAULT_SKU_ID,
        }
    }

    /// Sets the active platform SKU identifier.
    pub fn set_sku(&mut self, sku_id: usize) {
        self.active_sku = sku_id;
    }

    /// Returns the current active SKU identifier.
    #[must_use]
    pub const fn sku(&self) -> usize {
        self.active_sku
    }

    /// Returns the total number of registered PCD entries.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    /// Registers a new PCD entry in the database.
    ///
    /// # Errors
    /// Returns `Status::INVALID_PARAMETER` if `token_number` is 0.
    /// Returns `Status::OUT_OF_RESOURCES` if the database is full.
    pub fn register(&mut self, entry: PcdEntry<'a>) -> Result<(), EfiStatus> {
        // Reject invalid token number
        if entry.token_number == 0 {
            return Err(EfiStatus::INVALID_PARAMETER);
        }

        // Check if matching entry already exists to update
        for existing in self.entries.iter_mut().take(self.count).flatten() {
            if existing.token_space == entry.token_space
                && existing.token_number == entry.token_number
                && existing.sku_id == entry.sku_id
            {
                *existing = entry;
                return Ok(());
            }
        }

        if self.count >= CAPACITY {
            return Err(EfiStatus::OUT_OF_RESOURCES);
        }

        self.entries[self.count] = Some(entry);
        self.count += 1;
        Ok(())
    }

    /// Looks up a PCD entry matching the token and active SKU (or falling back to default SKU).
    fn find_entry(&self, token_space: &EfiGuid, token_number: u32) -> Option<&PcdEntry<'a>> {
        let mut fallback: Option<&PcdEntry<'a>> = None;

        for entry in self.entries.iter().take(self.count).flatten() {
            if entry.token_space == *token_space && entry.token_number == token_number {
                if entry.sku_id == self.active_sku {
                    return Some(entry);
                }
                if entry.sku_id == EFI_PCD_DEFAULT_SKU_ID {
                    fallback = Some(entry);
                }
            }
        }

        fallback
    }

    /// Retrieves an 8-bit unsigned integer PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn get_u8(&self, token_space: &EfiGuid, token_number: u32) -> Result<u8, EfiStatus> {
        let entry = self
            .find_entry(token_space, token_number)
            .ok_or(EfiStatus::NOT_FOUND)?;
        match entry.value {
            PcdValue::U8(val) => Ok(val),
            _ => Err(EfiStatus::INVALID_PARAMETER),
        }
    }

    /// Retrieves a 16-bit unsigned integer PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn get_u16(&self, token_space: &EfiGuid, token_number: u32) -> Result<u16, EfiStatus> {
        let entry = self
            .find_entry(token_space, token_number)
            .ok_or(EfiStatus::NOT_FOUND)?;
        match entry.value {
            PcdValue::U16(val) => Ok(val),
            _ => Err(EfiStatus::INVALID_PARAMETER),
        }
    }

    /// Retrieves a 32-bit unsigned integer PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn get_u32(&self, token_space: &EfiGuid, token_number: u32) -> Result<u32, EfiStatus> {
        let entry = self
            .find_entry(token_space, token_number)
            .ok_or(EfiStatus::NOT_FOUND)?;
        match entry.value {
            PcdValue::U32(val) => Ok(val),
            _ => Err(EfiStatus::INVALID_PARAMETER),
        }
    }

    /// Retrieves a 64-bit unsigned integer PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn get_u64(&self, token_space: &EfiGuid, token_number: u32) -> Result<u64, EfiStatus> {
        let entry = self
            .find_entry(token_space, token_number)
            .ok_or(EfiStatus::NOT_FOUND)?;
        match entry.value {
            PcdValue::U64(val) => Ok(val),
            _ => Err(EfiStatus::INVALID_PARAMETER),
        }
    }

    /// Retrieves a Boolean PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn get_bool(&self, token_space: &EfiGuid, token_number: u32) -> Result<bool, EfiStatus> {
        let entry = self
            .find_entry(token_space, token_number)
            .ok_or(EfiStatus::NOT_FOUND)?;
        match entry.value {
            PcdValue::Bool(val) => Ok(val),
            _ => Err(EfiStatus::INVALID_PARAMETER),
        }
    }

    /// Retrieves a byte slice PCD buffer.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn get_bytes(
        &self,
        token_space: &EfiGuid,
        token_number: u32,
    ) -> Result<&'a [u8], EfiStatus> {
        let entry = self
            .find_entry(token_space, token_number)
            .ok_or(EfiStatus::NOT_FOUND)?;
        match entry.value {
            PcdValue::Bytes(slice) => Ok(slice),
            _ => Err(EfiStatus::INVALID_PARAMETER),
        }
    }

    /// Sets an 8-bit unsigned integer PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn set_u8(
        &mut self,
        token_space: &EfiGuid,
        token_number: u32,
        val: u8,
    ) -> Result<(), EfiStatus> {
        for entry in self.entries.iter_mut().take(self.count).flatten() {
            if entry.token_space == *token_space
                && entry.token_number == token_number
                && (entry.sku_id == self.active_sku || entry.sku_id == EFI_PCD_DEFAULT_SKU_ID)
            {
                match entry.value {
                    PcdValue::U8(_) => {
                        entry.value = PcdValue::U8(val);
                        return Ok(());
                    }
                    _ => return Err(EfiStatus::INVALID_PARAMETER),
                }
            }
        }
        Err(EfiStatus::NOT_FOUND)
    }

    /// Sets a 16-bit unsigned integer PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn set_u16(
        &mut self,
        token_space: &EfiGuid,
        token_number: u32,
        val: u16,
    ) -> Result<(), EfiStatus> {
        for entry in self.entries.iter_mut().take(self.count).flatten() {
            if entry.token_space == *token_space
                && entry.token_number == token_number
                && (entry.sku_id == self.active_sku || entry.sku_id == EFI_PCD_DEFAULT_SKU_ID)
            {
                match entry.value {
                    PcdValue::U16(_) => {
                        entry.value = PcdValue::U16(val);
                        return Ok(());
                    }
                    _ => return Err(EfiStatus::INVALID_PARAMETER),
                }
            }
        }
        Err(EfiStatus::NOT_FOUND)
    }

    /// Sets a 32-bit unsigned integer PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn set_u32(
        &mut self,
        token_space: &EfiGuid,
        token_number: u32,
        val: u32,
    ) -> Result<(), EfiStatus> {
        for entry in self.entries.iter_mut().take(self.count).flatten() {
            if entry.token_space == *token_space
                && entry.token_number == token_number
                && (entry.sku_id == self.active_sku || entry.sku_id == EFI_PCD_DEFAULT_SKU_ID)
            {
                match entry.value {
                    PcdValue::U32(_) => {
                        entry.value = PcdValue::U32(val);
                        return Ok(());
                    }
                    _ => return Err(EfiStatus::INVALID_PARAMETER),
                }
            }
        }
        Err(EfiStatus::NOT_FOUND)
    }

    /// Sets a 64-bit unsigned integer PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn set_u64(
        &mut self,
        token_space: &EfiGuid,
        token_number: u32,
        val: u64,
    ) -> Result<(), EfiStatus> {
        for entry in self.entries.iter_mut().take(self.count).flatten() {
            if entry.token_space == *token_space
                && entry.token_number == token_number
                && (entry.sku_id == self.active_sku || entry.sku_id == EFI_PCD_DEFAULT_SKU_ID)
            {
                match entry.value {
                    PcdValue::U64(_) => {
                        entry.value = PcdValue::U64(val);
                        return Ok(());
                    }
                    _ => return Err(EfiStatus::INVALID_PARAMETER),
                }
            }
        }
        Err(EfiStatus::NOT_FOUND)
    }

    /// Sets a Boolean PCD value.
    ///
    /// # Errors
    /// Returns `NOT_FOUND` if token doesn't exist.
    /// Returns `INVALID_PARAMETER` if type mismatch.
    pub fn set_bool(
        &mut self,
        token_space: &EfiGuid,
        token_number: u32,
        val: bool,
    ) -> Result<(), EfiStatus> {
        for entry in self.entries.iter_mut().take(self.count).flatten() {
            if entry.token_space == *token_space
                && entry.token_number == token_number
                && (entry.sku_id == self.active_sku || entry.sku_id == EFI_PCD_DEFAULT_SKU_ID)
            {
                match entry.value {
                    PcdValue::Bool(_) => {
                        entry.value = PcdValue::Bool(val);
                        return Ok(());
                    }
                    _ => return Err(EfiStatus::INVALID_PARAMETER),
                }
            }
        }
        Err(EfiStatus::NOT_FOUND)
    }

    /// Searches for a PCD token by human-readable name.
    #[must_use]
    pub fn find_token_by_name(&self, name: &str) -> Option<(EfiGuid, u32)> {
        for entry in self.entries.iter().take(self.count).flatten() {
            if entry.name == name {
                return Some((entry.token_space, entry.token_number));
            }
        }
        None
    }
}

impl<const CAPACITY: usize> Default for PcdDatabase<'_, CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}
