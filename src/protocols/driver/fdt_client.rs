//! # Flattened Device Tree (FDT) Client Protocol
//!
//! Provides Device Tree (`DTB`) inspection, node querying, compatible device discovery,
//! register/memory bank extraction, property mutation, and chosen node manipulation.

#![allow(clippy::doc_markdown)]

use crate::abi::types::{Char8, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// FDT Client Protocol `GUID`.
///
/// `{e11faca0-4710-4c8e-a7a2-01baa2591b4c}`
pub const FDT_CLIENT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE11F_ACA0,
    0x4710,
    0x4C8E,
    [0xA7, 0xA2, 0x01, 0xBA, 0xA2, 0x59, 0x1B, 0x4C],
);

/// Function pointer to query a node property.
pub type FdtClientGetNodeProperty = unsafe extern "efiapi" fn(
    this: *mut FdtClientProtocol,
    node: i32,
    property_name: *const Char8,
    prop: *mut *const c_void,
    prop_size: *mut u32,
) -> EfiStatus;

/// Function pointer to mutate/add a node property.
pub type FdtClientSetNodeProperty = unsafe extern "efiapi" fn(
    this: *mut FdtClientProtocol,
    node: i32,
    property_name: *const Char8,
    prop: *const c_void,
    prop_size: u32,
) -> EfiStatus;

/// Function pointer to find the first node matching `compatible`.
pub type FdtClientFindCompatibleNode = unsafe extern "efiapi" fn(
    this: *mut FdtClientProtocol,
    compatible_string: *const Char8,
    node: *mut i32,
) -> EfiStatus;

/// Function pointer to find the next node matching `compatible`.
pub type FdtClientFindNextCompatibleNode = unsafe extern "efiapi" fn(
    this: *mut FdtClientProtocol,
    compatible_string: *const Char8,
    prev_node: i32,
    node: *mut i32,
) -> EfiStatus;

/// Function pointer to find a property on a compatible node.
pub type FdtClientFindCompatibleNodeProperty = unsafe extern "efiapi" fn(
    this: *mut FdtClientProtocol,
    compatible_string: *const Char8,
    property_name: *const Char8,
    prop: *mut *const c_void,
    prop_size: *mut u32,
) -> EfiStatus;

/// Function pointer to query register regions for a compatible node.
pub type FdtClientFindCompatibleNodeReg = unsafe extern "efiapi" fn(
    this: *mut FdtClientProtocol,
    compatible_string: *const Char8,
    reg: *mut *const c_void,
    address_cells: *mut UIntN,
    size_cells: *mut UIntN,
    reg_size: *mut u32,
) -> EfiStatus;

/// Function pointer to query memory bank register regions.
pub type FdtClientFindMemoryNodeReg = unsafe extern "efiapi" fn(
    this: *mut FdtClientProtocol,
    node: *mut i32,
    reg: *mut *const c_void,
    address_cells: *mut UIntN,
    size_cells: *mut UIntN,
    reg_size: *mut u32,
) -> EfiStatus;

/// Function pointer to query next memory bank register regions.
pub type FdtClientFindNextMemoryNodeReg = unsafe extern "efiapi" fn(
    this: *mut FdtClientProtocol,
    prev_node: i32,
    node: *mut i32,
    reg: *mut *const c_void,
    address_cells: *mut UIntN,
    size_cells: *mut UIntN,
    reg_size: *mut u32,
) -> EfiStatus;

/// Function pointer to locate or create the `/chosen` node in the device tree.
pub type FdtClientGetOrInsertChosenNode =
    unsafe extern "efiapi" fn(this: *mut FdtClientProtocol, node: *mut i32) -> EfiStatus;

/// Raw FDT Client Protocol structure.
#[derive(Debug)]
#[repr(C)]
pub struct FdtClientProtocol {
    /// Reads property bytes.
    pub get_node_property: FdtClientGetNodeProperty,
    /// Writes property bytes.
    pub set_node_property: FdtClientSetNodeProperty,
    /// Finds first matching compatible node.
    pub find_compatible_node: FdtClientFindCompatibleNode,
    /// Finds next matching compatible node.
    pub find_next_compatible_node: FdtClientFindNextCompatibleNode,
    /// Reads property from compatible node.
    pub find_compatible_node_property: FdtClientFindCompatibleNodeProperty,
    /// Reads `reg` regions from compatible node.
    pub find_compatible_node_reg: FdtClientFindCompatibleNodeReg,
    /// Finds first memory node.
    pub find_memory_node_reg: FdtClientFindMemoryNodeReg,
    /// Finds next memory node.
    pub find_next_memory_node_reg: FdtClientFindNextMemoryNodeReg,
    /// Locates or creates `/chosen`.
    pub get_or_insert_chosen_node: FdtClientGetOrInsertChosenNode,
}

/// High-level safe wrapper around [`FdtClientProtocol`].
pub struct FdtClient<'a> {
    proto: &'a mut FdtClientProtocol,
}

impl<'a> FdtClient<'a> {
    /// Creates a safe wrapper around an active protocol instance.
    #[must_use]
    pub fn new(proto: &'a mut FdtClientProtocol) -> Self {
        Self { proto }
    }

    /// Queries the raw bytes of `property_name` on `node`.
    ///
    /// # Errors
    ///
    /// Returns an error status if `node` or `property_name` is not found, or access fails.
    pub fn get_node_property(
        &mut self,
        node: i32,
        property_name: &[u8],
    ) -> Result<&'a [u8], EfiStatus> {
        let mut prop: *const c_void = core::ptr::null();
        let mut size: u32 = 0;

        let status = unsafe {
            (self.proto.get_node_property)(
                self.proto,
                node,
                property_name.as_ptr(),
                &raw mut prop,
                &raw mut size,
            )
        };

        if status.is_success() && !prop.is_null() {
            unsafe {
                Ok(core::slice::from_raw_parts(
                    prop.cast::<u8>(),
                    size as usize,
                ))
            }
        } else {
            Err(status)
        }
    }

    /// Sets or updates `property_name` on `node`.
    ///
    /// # Errors
    ///
    /// Returns an error status if `value` length exceeds 4 GiB or device tree modification fails.
    pub fn set_node_property(
        &mut self,
        node: i32,
        property_name: &[u8],
        value: &[u8],
    ) -> Result<(), EfiStatus> {
        let Ok(val_len) = u32::try_from(value.len()) else {
            return Err(EfiStatus::BAD_BUFFER_SIZE);
        };
        let status = unsafe {
            (self.proto.set_node_property)(
                self.proto,
                node,
                property_name.as_ptr(),
                value.as_ptr().cast::<c_void>(),
                val_len,
            )
        };

        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Finds the first device tree node matching `compatible_string`.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if no matching device tree node exists.
    pub fn find_compatible_node(&mut self, compatible_string: &[u8]) -> Result<i32, EfiStatus> {
        let mut node: i32 = -1;
        let status = unsafe {
            (self.proto.find_compatible_node)(self.proto, compatible_string.as_ptr(), &raw mut node)
        };

        if status.is_success() {
            Ok(node)
        } else {
            Err(status)
        }
    }

    /// Finds the next device tree node matching `compatible_string` after `prev_node`.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if no further matching nodes exist.
    pub fn find_next_compatible_node(
        &mut self,
        compatible_string: &[u8],
        prev_node: i32,
    ) -> Result<i32, EfiStatus> {
        let mut node: i32 = -1;
        let status = unsafe {
            (self.proto.find_next_compatible_node)(
                self.proto,
                compatible_string.as_ptr(),
                prev_node,
                &raw mut node,
            )
        };

        if status.is_success() {
            Ok(node)
        } else {
            Err(status)
        }
    }

    /// Retrieves or creates the `/chosen` node offset in the device tree.
    ///
    /// # Errors
    ///
    /// Returns an error status if `/chosen` cannot be created or accessed.
    pub fn get_or_insert_chosen_node(&mut self) -> Result<i32, EfiStatus> {
        let mut node: i32 = -1;
        let status = unsafe { (self.proto.get_or_insert_chosen_node)(self.proto, &raw mut node) };
        if status.is_success() {
            Ok(node)
        } else {
            Err(status)
        }
    }
}
