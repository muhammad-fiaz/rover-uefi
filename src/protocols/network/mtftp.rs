//! # Multicast Trivial File Transfer Protocols
//!
//! Defined by the UEFI Specification Section 29.7: `MTFTPv4` Protocol and
//! Section 29.8: `MTFTPv6` Protocol.
//!
//! Used by network boot implementations (PXE, HTTP boot) to retrieve boot configurations,
//! kernels, and initrd images over UDP.

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::network::types::{Ipv4Address, Ipv6Address};
use core::ffi::c_void;

/// The `EFI_MTFTP4_PROTOCOL_GUID`.
pub const EFI_MTFTP4_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x7824_7C57,
    0x63DB,
    0x4708,
    [0x99, 0xC2, 0xA8, 0xB4, 0xA9, 0xA6, 0x1F, 0x65],
);

/// The `EFI_MTFTP6_PROTOCOL_GUID`.
pub const EFI_MTFTP6_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xBF34_2110,
    0x0C2B,
    0x4996,
    [0x8C, 0x6B, 0x15, 0x1E, 0x0F, 0xBC, 0x6E, 0xB8],
);

/// Operational mode configuration for `MTFTPv4`.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiMtftp4ModeData {
    /// Station IPv4 address.
    pub station_address: Ipv4Address,
    /// Subnet mask.
    pub subnet_mask: Ipv4Address,
    /// Server IPv4 address.
    pub server_address: Ipv4Address,
    /// Initial server port (default 69).
    pub initial_server_port: u16,
}

/// Token representing an `MTFTPv4` transaction.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiMtftp4Token {
    /// Transaction return status code.
    pub status: EfiStatus,
    /// Pointer to filename ASCII string.
    pub filename: *const u8,
    /// Destination or source memory buffer size.
    pub buffer_size: u64,
    /// Destination or source memory buffer.
    pub buffer: *mut c_void,
}

/// The raw ABI structure for `EFI_MTFTP4_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiMtftp4Protocol {
    /// Retrieves current operational mode.
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *const EfiMtftp4Protocol,
        mode_data: *mut EfiMtftp4ModeData,
    ) -> EfiStatus,
    /// Configures or resets `MTFTPv4` session parameters.
    pub configure: unsafe extern "efiapi" fn(
        this: *const EfiMtftp4Protocol,
        mtftp4_config_data: *const c_void,
    ) -> EfiStatus,
    /// Reads file metadata and options without transferring payload.
    pub get_info: unsafe extern "efiapi" fn(
        this: *const EfiMtftp4Protocol,
        override_data: *const c_void,
        filename: *const u8,
        mode_str: *const u8,
        option_count: u8,
        option_list: *const c_void,
        packet_length: *mut u32,
        packet: *mut *mut c_void,
    ) -> EfiStatus,
    /// Parses options returned in an OACK packet.
    pub parse_options: unsafe extern "efiapi" fn(
        this: *const EfiMtftp4Protocol,
        packet_len: u32,
        packet: *const c_void,
        option_count: *mut u32,
        option_list: *mut *mut c_void,
    ) -> EfiStatus,
    /// Downloads a file from the `MTFTPv4` server into memory.
    pub read_file: unsafe extern "efiapi" fn(
        this: *const EfiMtftp4Protocol,
        token: *mut EfiMtftp4Token,
    ) -> EfiStatus,
    /// Uploads a file from memory to the `MTFTPv4` server.
    pub write_file: unsafe extern "efiapi" fn(
        this: *const EfiMtftp4Protocol,
        token: *mut EfiMtftp4Token,
    ) -> EfiStatus,
    /// Reads directory listing from the `MTFTPv4` server.
    pub read_directory: unsafe extern "efiapi" fn(
        this: *const EfiMtftp4Protocol,
        token: *mut EfiMtftp4Token,
    ) -> EfiStatus,
}

/// Operational mode configuration for `MTFTPv6`.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiMtftp6ModeData {
    /// Station IPv6 address.
    pub station_address: Ipv6Address,
    /// Server IPv6 address.
    pub server_address: Ipv6Address,
    /// Initial server port (default 69).
    pub initial_server_port: u16,
}

/// The raw ABI structure for `EFI_MTFTP6_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiMtftp6Protocol {
    /// Retrieves current operational mode.
    pub get_mode_data: unsafe extern "efiapi" fn(
        this: *const EfiMtftp6Protocol,
        mode_data: *mut EfiMtftp6ModeData,
    ) -> EfiStatus,
    /// Configures or resets `MTFTPv6` session parameters.
    pub configure: unsafe extern "efiapi" fn(
        this: *const EfiMtftp6Protocol,
        mtftp6_config_data: *const c_void,
    ) -> EfiStatus,
    /// Downloads a file from the `MTFTPv6` server into memory.
    pub read_file:
        unsafe extern "efiapi" fn(this: *const EfiMtftp6Protocol, token: *mut c_void) -> EfiStatus,
    /// Uploads a file from memory to the `MTFTPv6` server.
    pub write_file:
        unsafe extern "efiapi" fn(this: *const EfiMtftp6Protocol, token: *mut c_void) -> EfiStatus,
    /// Reads directory listing from the `MTFTPv6` server.
    pub read_directory:
        unsafe extern "efiapi" fn(this: *const EfiMtftp6Protocol, token: *mut c_void) -> EfiStatus,
}
