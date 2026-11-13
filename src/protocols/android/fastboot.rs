//! # Android Fastboot Protocols and Client API
//!
//! Provides platform hooks and transport abstractions for the Android Fastboot
//! protocol, facilitating partition flashing, erasing, OEM command processing,
//! variable retrieval, and Google Bootloader (`GBL`) Fastboot and transport extensions.

use crate::base::{EfiEvent, EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Android Fastboot Platform Protocol GUID.
pub const ANDROID_FASTBOOT_PLATFORM_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5246_85A0,
    0x89A0,
    0x11E3,
    [0x9D, 0x4D, 0xBF, 0xA9, 0xF6, 0xA4, 0x03, 0x08],
);

/// Android Fastboot Transport Protocol GUID.
pub const ANDROID_FASTBOOT_TRANSPORT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x74BD_9FE0,
    0x8902,
    0x11E3,
    [0xB9, 0xD3, 0xF7, 0x22, 0x38, 0xFC, 0x9A, 0x31],
);

/// GBL EFI Fastboot Protocol GUID (`{c67e48a0-5eb8-4127-be89-df2ed93d8a9a}`).
pub const GBL_EFI_FASTBOOT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xC67E_48A0,
    0x5EB8,
    0x4127,
    [0xBE, 0x89, 0xDF, 0x2E, 0xD9, 0x3D, 0x8A, 0x9A],
);

/// GBL EFI Fastboot Protocol Revision (1.0).
pub const GBL_EFI_FASTBOOT_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// GBL EFI Fastboot Transport Protocol GUID (`{edade92c-5c48-440d-849c-e2a0c7e55143}`).
pub const GBL_EFI_FASTBOOT_TRANSPORT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xEDAD_E92C,
    0x5C48,
    0x440D,
    [0x84, 0x9C, 0xE2, 0xA0, 0xC7, 0xE5, 0x51, 0x43],
);

/// GBL EFI Fastboot Transport Protocol Revision (1.0).
pub const GBL_EFI_FASTBOOT_TRANSPORT_PROTOCOL_REVISION: u64 = 0x0001_0000;

/// Maximum length of Fastboot serial number in UTF-8 (32 bytes).
pub const GBL_EFI_FASTBOOT_SERIAL_NUMBER_MAX_LEN_UTF8: usize = 32;

/// Buffer length for Fastboot partition type string (56 bytes).
pub const GBL_EFI_FASTBOOT_PARTITION_TYPE_BUF_LEN: usize = 56;

/// GBL Fastboot message type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GblEfiFastbootMessageType {
    /// Success response (`"OKAY"`).
    Okay = 0,
    /// Failure response (`"FAIL"`).
    Fail = 1,
    /// Information message (`"INFO"`).
    Info = 2,
}

/// GBL Fastboot command execution result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GblEfiFastbootCommandExecResult {
    /// Command is prohibited on this device.
    Prohibited = 0,
    /// Command handled by default bootloader implementation.
    DefaultImpl = 1,
    /// Command handled by custom firmware implementation.
    CustomImpl = 2,
}

/// GBL Fastboot transport receive mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GblEfiFastbootRxMode {
    /// Single packet receive mode.
    SinglePacket = 0,
    /// Fixed-length receive mode.
    FixedLength = 1,
}

/// Initialise platform resources for fastboot.
pub type FastbootPlatformInit = unsafe extern "efiapi" fn() -> EfiStatus;

/// Uninitialise platform resources when exiting fastboot.
pub type FastbootPlatformUnInit = unsafe extern "efiapi" fn();

/// Flash a named partition with image data.
pub type FastbootPlatformFlash = unsafe extern "efiapi" fn(
    partition_name: *const u8,
    buffer_size: usize,
    buffer: *const c_void,
) -> EfiStatus;

/// Erase a named partition.
pub type FastbootPlatformErase = unsafe extern "efiapi" fn(partition_name: *const u8) -> EfiStatus;

/// Retrieve a fastboot variable string.
pub type FastbootPlatformGetVar =
    unsafe extern "efiapi" fn(name: *const u8, value: *mut u8) -> EfiStatus;

/// Execute an OEM-specific fastboot command.
pub type FastbootPlatformDoOemCommand = unsafe extern "efiapi" fn(command: *const u8) -> EfiStatus;

/// Android Fastboot Platform Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FastbootPlatformProtocol {
    /// Initialise platform for fastboot mode.
    pub init: Option<FastbootPlatformInit>,
    /// Uninitialise platform resources.
    pub un_init: Option<FastbootPlatformUnInit>,
    /// Flash a designated partition.
    pub flash_partition: Option<FastbootPlatformFlash>,
    /// Erase a designated partition.
    pub erase_partition: Option<FastbootPlatformErase>,
    /// Get variable value.
    pub get_var: Option<FastbootPlatformGetVar>,
    /// Execute OEM command.
    pub do_oem_command: Option<FastbootPlatformDoOemCommand>,
}

/// Send data over the fastboot transport.
pub type FastbootTransportSend = unsafe extern "efiapi" fn(
    buffer_size: usize,
    buffer: *const c_void,
    callback_event: EfiEvent,
) -> EfiStatus;

/// Receive data from the fastboot transport.
pub type FastbootTransportReceive = unsafe extern "efiapi" fn(
    buffer_size: usize,
    buffer: *mut c_void,
    callback_event: EfiEvent,
) -> EfiStatus;

/// Android Fastboot Transport Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FastbootTransportProtocol {
    /// Transmit data over transport channel.
    pub send: Option<FastbootTransportSend>,
    /// Receive incoming data from transport channel.
    pub receive: Option<FastbootTransportReceive>,
}

/// Safe client wrapper for Android Fastboot platform operations.
#[derive(Debug)]
pub struct FastbootPlatform<'a> {
    protocol: &'a FastbootPlatformProtocol,
}

impl<'a> FastbootPlatform<'a> {
    /// Create a new safe view of the Fastboot Platform protocol.
    #[must_use]
    pub fn new(protocol: &'a FastbootPlatformProtocol) -> Self {
        Self { protocol }
    }

    /// Initialize fastboot platform resources.
    ///
    /// # Errors
    /// Returns an error if the platform initialization hook fails.
    pub fn init(&self) -> Result<(), EfiStatus> {
        if let Some(init_fn) = self.protocol.init {
            let status = unsafe { init_fn() };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Ok(())
        }
    }

    /// Flash an image buffer to the named partition.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn flash_partition(&self, name: &[u8], buffer: &[u8]) -> Result<(), EfiStatus> {
        if let Some(flash_fn) = self.protocol.flash_partition {
            let status = unsafe {
                flash_fn(
                    name.as_ptr(),
                    buffer.len(),
                    buffer.as_ptr().cast::<c_void>(),
                )
            };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Erase the named partition.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or the status returned by firmware.
    pub fn erase_partition(&self, name: &[u8]) -> Result<(), EfiStatus> {
        if let Some(erase_fn) = self.protocol.erase_partition {
            let status = unsafe { erase_fn(name.as_ptr()) };
            if status == EfiStatus::SUCCESS {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }
}

// =========================================================================
// GBL EFI Fastboot Protocol & Transport
// =========================================================================

/// Callback invoked for each variable returned by `GetVarAll`.
pub type GblEfiFastbootGetVarAllCallback = unsafe extern "efiapi" fn(
    context: *mut c_void,
    num_args: usize,
    args: *const *const u8,
    value: *const u8,
);

/// Sender callback for fastboot message responses.
pub type GblEfiFastbootMessageSender = unsafe extern "efiapi" fn(
    context: *mut c_void,
    msg_type: u32,
    msg_len: usize,
    msg: *const u8,
) -> EfiStatus;

/// Query variable value in GBL fastboot.
pub type GblEfiFastbootGetVar = unsafe extern "efiapi" fn(
    this: *mut GblEfiFastbootProtocol,
    num_args: usize,
    args: *const *const u8,
    buffer_size: *mut usize,
    buffer: *mut u8,
) -> EfiStatus;

/// Enumerate all fastboot variables.
pub type GblEfiFastbootGetVarAll = unsafe extern "efiapi" fn(
    this: *mut GblEfiFastbootProtocol,
    context: *mut c_void,
    callback: GblEfiFastbootGetVarAllCallback,
) -> EfiStatus;

/// Retrieve staged data buffer.
pub type GblEfiFastbootGetStaged = unsafe extern "efiapi" fn(
    this: *mut GblEfiFastbootProtocol,
    buffer_size: *mut usize,
    buffer_remains: *mut usize,
    buffer: *mut u8,
) -> EfiStatus;

/// Execute a fastboot command.
pub type GblEfiFastbootCommandExec = unsafe extern "efiapi" fn(
    this: *mut GblEfiFastbootProtocol,
    num_args: usize,
    args: *const *const u8,
    download_buffer_size: usize,
    download_buffer_used_size: usize,
    download_buffer: *mut u8,
    implementation: *mut u32,
    sender: GblEfiFastbootMessageSender,
    context: *mut c_void,
) -> EfiStatus;

/// Query the filesystem type of a partition.
pub type GblEfiFastbootGetPartitionType = unsafe extern "efiapi" fn(
    this: *mut GblEfiFastbootProtocol,
    part_name: *const u8,
    part_type_len: *mut usize,
    part_type: *mut u8,
) -> EfiStatus;

/// GBL EFI Fastboot Protocol structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GblEfiFastbootProtocol {
    /// Protocol revision (`GBL_EFI_FASTBOOT_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Null-terminated UTF-8 serial number string.
    pub serial_number: [u8; GBL_EFI_FASTBOOT_SERIAL_NUMBER_MAX_LEN_UTF8],
    /// Get fastboot variable value.
    pub get_var: Option<GblEfiFastbootGetVar>,
    /// Enumerate all fastboot variables.
    pub get_var_all: Option<GblEfiFastbootGetVarAll>,
    /// Get staged download buffer.
    pub get_staged: Option<GblEfiFastbootGetStaged>,
    /// Execute fastboot command.
    pub command_exec: Option<GblEfiFastbootCommandExec>,
    /// Get partition filesystem type.
    pub get_partition_type: Option<GblEfiFastbootGetPartitionType>,
}

/// GBL EFI Fastboot Transport start hook.
pub type GblEfiFastbootTransportStart =
    unsafe extern "efiapi" fn(this: *mut GblEfiFastbootTransportProtocol) -> EfiStatus;

/// GBL EFI Fastboot Transport stop hook.
pub type GblEfiFastbootTransportStop =
    unsafe extern "efiapi" fn(this: *mut GblEfiFastbootTransportProtocol) -> EfiStatus;

/// GBL EFI Fastboot Transport receive hook.
pub type GblEfiFastbootTransportReceive = unsafe extern "efiapi" fn(
    this: *mut GblEfiFastbootTransportProtocol,
    buffer_size: *mut usize,
    buffer: *mut c_void,
    mode: u32,
) -> EfiStatus;

/// GBL EFI Fastboot Transport send hook.
pub type GblEfiFastbootTransportSend = unsafe extern "efiapi" fn(
    this: *mut GblEfiFastbootTransportProtocol,
    buffer_size: *mut usize,
    buffer: *const c_void,
) -> EfiStatus;

/// GBL EFI Fastboot Transport flush hook.
pub type GblEfiFastbootTransportFlush =
    unsafe extern "efiapi" fn(this: *mut GblEfiFastbootTransportProtocol) -> EfiStatus;

/// GBL EFI Fastboot Transport Protocol structure.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GblEfiFastbootTransportProtocol {
    /// Protocol revision (`GBL_EFI_FASTBOOT_TRANSPORT_PROTOCOL_REVISION`).
    pub revision: u64,
    /// Transport description string (e.g. `"USB"` or `"TCP"`).
    pub description: *const u8,
    /// Start transport listener.
    pub start: Option<GblEfiFastbootTransportStart>,
    /// Stop transport listener.
    pub stop: Option<GblEfiFastbootTransportStop>,
    /// Receive packet from host.
    pub receive: Option<GblEfiFastbootTransportReceive>,
    /// Send packet to host.
    pub send: Option<GblEfiFastbootTransportSend>,
    /// Flush pending outgoing buffers.
    pub flush: Option<GblEfiFastbootTransportFlush>,
}

/// Safe client wrapper for `GblEfiFastbootProtocol`.
#[derive(Debug)]
pub struct GblFastboot<'a> {
    protocol: &'a mut GblEfiFastbootProtocol,
}

impl<'a> GblFastboot<'a> {
    /// Create a new safe view of the GBL Fastboot protocol.
    #[must_use]
    pub fn new(protocol: &'a mut GblEfiFastbootProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the protocol revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.protocol.revision
    }

    /// Returns the device serial number as a byte slice.
    #[must_use]
    pub fn serial_number(&self) -> &[u8] {
        let raw = &self.protocol.serial_number;
        let len = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        &raw[..len]
    }

    /// Retrieve a fastboot variable.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or status returned by firmware.
    pub fn get_var(&mut self, args: &[&[u8]], out_buf: &mut [u8]) -> Result<usize, EfiStatus> {
        let Some(get_var_fn) = self.protocol.get_var else {
            return Err(EfiStatus::UNSUPPORTED);
        };

        let mut arg_ptrs = [core::ptr::null::<u8>(); 16];
        let count = args.len().min(16);
        for i in 0..count {
            arg_ptrs[i] = args[i].as_ptr();
        }

        let mut buf_size = out_buf.len();
        let status = unsafe {
            get_var_fn(
                self.protocol,
                count,
                arg_ptrs.as_ptr(),
                &raw mut buf_size,
                out_buf.as_mut_ptr(),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(buf_size)
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper for `GblEfiFastbootTransportProtocol`.
#[derive(Debug)]
pub struct GblFastbootTransport<'a> {
    protocol: &'a mut GblEfiFastbootTransportProtocol,
}

impl<'a> GblFastbootTransport<'a> {
    /// Create a new safe view of the GBL Fastboot Transport protocol.
    #[must_use]
    pub fn new(protocol: &'a mut GblEfiFastbootTransportProtocol) -> Self {
        Self { protocol }
    }

    /// Start the fastboot transport.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or status returned by firmware.
    pub fn start(&mut self) -> Result<(), EfiStatus> {
        let Some(start_fn) = self.protocol.start else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { start_fn(self.protocol) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Stop the fastboot transport.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or status returned by firmware.
    pub fn stop(&mut self) -> Result<(), EfiStatus> {
        let Some(stop_fn) = self.protocol.stop else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { stop_fn(self.protocol) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Send a data packet over the transport.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or status returned by firmware.
    pub fn send(&mut self, data: &[u8]) -> Result<usize, EfiStatus> {
        let Some(send_fn) = self.protocol.send else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let mut len = data.len();
        let status =
            unsafe { send_fn(self.protocol, &raw mut len, data.as_ptr().cast::<c_void>()) };
        if status == EfiStatus::SUCCESS {
            Ok(len)
        } else {
            Err(status)
        }
    }

    /// Flush the transport stream.
    ///
    /// # Errors
    /// Returns `EfiStatus::UNSUPPORTED` if not implemented, or status returned by firmware.
    pub fn flush(&mut self) -> Result<(), EfiStatus> {
        let Some(flush_fn) = self.protocol.flush else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { flush_fn(self.protocol) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
