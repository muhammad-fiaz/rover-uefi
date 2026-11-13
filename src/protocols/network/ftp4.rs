//! # `FTPv4` Protocol
//!
//! Provides client-side File Transfer Protocol over `IPv4` (`FTPv4`) services.
//! Defined in UEFI Specification 2.2+.

use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use crate::protocols::network::types::Ipv4Address;
use core::ffi::c_void;

/// `FTPv4` Service Binding Protocol GUID: `{0faaecb1-226e-4782-aace-7db9bcbf4daf}`.
pub const EFI_FTP4_SERVICE_BINDING_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0FAA_ECB1,
    0x226E,
    0x4782,
    [0xAA, 0xCE, 0x7D, 0xB9, 0xBC, 0xBF, 0x4D, 0xAF],
);

/// `FTPv4` Protocol GUID: `{eb338826-681b-4295-b356-2b364c757b09}`.
pub const EFI_FTP4_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xEB33_8826,
    0x681B,
    0x4295,
    [0xB3, 0x56, 0x2B, 0x36, 0x4C, 0x75, 0x7B, 0x09],
);

/// Representation type: image/binary mode.
pub const REP_TYPE_IMAGE: u8 = 0x00;
/// Representation type: EBCDIC mode.
pub const REP_TYPE_EBCDIC: u8 = 0x01;
/// Representation type: ASCII mode.
pub const REP_TYPE_ASCII: u8 = 0x02;
/// Representation type: local byte size mode.
pub const REP_TYPE_LOCAL: u8 = 0x03;

/// File structure: continuous stream of bytes.
pub const FILE_STRUCT_FILE: u8 = 0x00;
/// File structure: record structure.
pub const FILE_STRUCT_RECORD: u8 = 0x01;
/// File structure: page structure.
pub const FILE_STRUCT_PAGE: u8 = 0x02;

/// Transfer mode: stream mode.
pub const TRANS_MODE_STREAM: u8 = 0x00;
/// Transfer mode: block mode.
pub const TRANS_MODE_BLOCK: u8 = 0x01;
/// Transfer mode: compressed mode.
pub const TRANS_MODE_COMPRESSED: u8 = 0x02;

/// Connection token used when opening or closing an `FTPv4` session.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiFtp4ConnectionToken {
    /// Event to signal after connection is established or closed.
    pub event: EfiEvent,
    /// Result status of the connection operation.
    pub status: EfiStatus,
}

/// Operational configuration parameters for an `FTPv4` session.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiFtp4ConfigData {
    /// Null-terminated ASCII username string pointer.
    pub username: *mut u8,
    /// Null-terminated ASCII password string pointer.
    pub password: *mut u8,
    /// `true` to initiate active data connection; `false` for passive.
    pub active: bool,
    /// `true` to use station default network configuration.
    pub use_default_setting: bool,
    /// Station `IPv4` address if `use_default_setting` is `false`.
    pub station_ip: Ipv4Address,
    /// Subnet mask if `use_default_setting` is `false`.
    pub subnet_mask: Ipv4Address,
    /// Gateway `IPv4` address if `use_default_setting` is `false`.
    pub gateway_ip: Ipv4Address,
    /// Server `IPv4` address.
    pub server_ip: Ipv4Address,
    /// Server control port (default 21).
    pub server_port: u16,
    /// Alternate data port (0 uses `server_port - 1`).
    pub alt_data_port: u16,
    /// Representation type code (`REP_TYPE_*`).
    pub rep_type: u8,
    /// File structure code (`FILE_STRUCT_*`).
    pub file_struct: u8,
    /// Transfer mode code (`TRANS_MODE_*`).
    pub trans_mode: u8,
}

/// Command token forward declaration.
#[repr(C)]
pub struct EfiFtp4CommandToken {
    /// Event signaled when the command completes.
    pub event: EfiEvent,
    /// Pathname on the server (null-terminated ASCII).
    pub pathname: *mut u8,
    /// Size of `data_buffer` in bytes.
    pub data_buffer_size: u64,
    /// Buffer receiving downloaded data or supplying upload payload.
    pub data_buffer: *mut c_void,
    /// Optional data callback function.
    pub data_callback: Option<EfiFtp4DataCallback>,
    /// Caller-defined context passed to `data_callback`.
    pub context: *mut c_void,
    /// Result status of the command operation.
    pub status: EfiStatus,
}

/// Function pointer for data callbacks during transfer.
pub type EfiFtp4DataCallback = unsafe extern "efiapi" fn(
    this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4CommandToken,
) -> EfiStatus;

/// Function pointer for `EFI_FTP4_GET_MODE_DATA`.
pub type EfiFtp4GetModeData = unsafe extern "efiapi" fn(
    this: *mut EfiFtp4Protocol,
    mode_data: *mut EfiFtp4ConfigData,
) -> EfiStatus;

/// Function pointer for `EFI_FTP4_CONNECT`.
pub type EfiFtp4Connect = unsafe extern "efiapi" fn(
    this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4ConnectionToken,
) -> EfiStatus;

/// Function pointer for `EFI_FTP4_CLOSE`.
pub type EfiFtp4Close = unsafe extern "efiapi" fn(
    this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4ConnectionToken,
) -> EfiStatus;

/// Function pointer for `EFI_FTP4_CONFIGURE`.
pub type EfiFtp4Configure = unsafe extern "efiapi" fn(
    this: *mut EfiFtp4Protocol,
    ftp_config_data: *const EfiFtp4ConfigData,
) -> EfiStatus;

/// Function pointer for `EFI_FTP4_READ_FILE`.
pub type EfiFtp4ReadFile = unsafe extern "efiapi" fn(
    this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4CommandToken,
) -> EfiStatus;

/// Function pointer for `EFI_FTP4_WRITE_FILE`.
pub type EfiFtp4WriteFile = unsafe extern "efiapi" fn(
    this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4CommandToken,
) -> EfiStatus;

/// Function pointer for `EFI_FTP4_READ_DIRECTORY`.
pub type EfiFtp4ReadDirectory = unsafe extern "efiapi" fn(
    this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4CommandToken,
) -> EfiStatus;

/// Function pointer for `EFI_FTP4_POLL`.
pub type EfiFtp4Poll = unsafe extern "efiapi" fn(this: *mut EfiFtp4Protocol) -> EfiStatus;

/// Raw ABI structure for `EFI_FTP4_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiFtp4Protocol {
    /// Reads the current operational configuration mode data.
    pub get_mode_data: EfiFtp4GetModeData,
    /// Initiates a control connection with an `FTPv4` server.
    pub connect: EfiFtp4Connect,
    /// Closes an active control connection gracefully.
    pub close: EfiFtp4Close,
    /// Configures or resets session parameters.
    pub configure: EfiFtp4Configure,
    /// Downloads a remote file.
    pub read_file: EfiFtp4ReadFile,
    /// Uploads a file to the server.
    pub write_file: EfiFtp4WriteFile,
    /// Downloads directory listings.
    pub read_directory: EfiFtp4ReadDirectory,
    /// Polls incoming/outgoing network packets.
    pub poll: EfiFtp4Poll,
}

/// Safe client wrapper around [`EfiFtp4Protocol`].
pub struct Ftp4<'a> {
    proto: &'a mut EfiFtp4Protocol,
}

impl<'a> Ftp4<'a> {
    /// Creates a new `Ftp4` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiFtp4Protocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiFtp4Protocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Configures or resets the `FTPv4` session.
    ///
    /// Passing `None` resets the instance to default configuration.
    ///
    /// # Errors
    ///
    /// Returns an error status if configuration fails.
    pub fn configure(&mut self, config: Option<&EfiFtp4ConfigData>) -> Result<(), EfiStatus> {
        let config_ptr = match config {
            Some(c) => core::ptr::from_ref(c),
            None => core::ptr::null(),
        };
        let status = unsafe { (self.proto.configure)(self.proto, config_ptr) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Initiates a connection request to the remote `FTPv4` server.
    ///
    /// # Errors
    ///
    /// Returns an error status if initiation fails.
    pub fn connect(&mut self, token: &mut EfiFtp4ConnectionToken) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.connect)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Initiates closing the control connection.
    ///
    /// # Errors
    ///
    /// Returns an error status if closing fails.
    pub fn close(&mut self, token: &mut EfiFtp4ConnectionToken) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.close)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Downloads a remote file described by `token`.
    ///
    /// # Errors
    ///
    /// Returns an error status if read request fails.
    pub fn read_file(&mut self, token: &mut EfiFtp4CommandToken) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.read_file)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Uploads a file described by `token`.
    ///
    /// # Errors
    ///
    /// Returns an error status if write request fails.
    pub fn write_file(&mut self, token: &mut EfiFtp4CommandToken) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.write_file)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads directory file listings.
    ///
    /// # Errors
    ///
    /// Returns an error status if directory listing fails.
    pub fn read_directory(&mut self, token: &mut EfiFtp4CommandToken) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.read_directory)(self.proto, token) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Polls for incoming/outgoing packets.
    ///
    /// # Errors
    ///
    /// Returns an error status if polling fails.
    pub fn poll(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.poll)(self.proto) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
