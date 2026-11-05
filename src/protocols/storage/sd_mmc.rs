//! # SD/MMC Pass Thru Protocol
//!
//! UEFI 2.5+ `EFI_SD_MMC_PASS_THRU_PROTOCOL` allows sending raw SD/MMC commands
//! to cards attached to an SD compatible host controller.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// The `EFI_SD_MMC_PASS_THRU_PROTOCOL` GUID.
pub const EFI_SD_MMC_PASS_THRU_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x716E_F0D9,
    0xFF83,
    0x4F69,
    [0x81, 0xE9, 0x51, 0x8B, 0xD3, 0x9A, 0x8E, 0x70],
);

/// SD/MMC Command Types.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SdMmcCommandType {
    /// Broadcast commands, no response.
    Bc = 0,
    /// Broadcast commands with response.
    Bcr = 1,
    /// Addressed (point-to-point) commands.
    Ac = 2,
    /// Addressed (point-to-point) data transfer commands.
    Adtc = 3,
}

/// SD/MMC Response Types.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SdMmcResponseType {
    /// R1 response.
    R1 = 0,
    /// R1b response (with busy).
    R1b = 1,
    /// R2 response.
    R2 = 2,
    /// R3 response.
    R3 = 3,
    /// R4 response.
    R4 = 4,
    /// R5 response.
    R5 = 5,
    /// R5b response.
    R5b = 6,
    /// R6 response.
    R6 = 7,
    /// R7 response.
    R7 = 8,
}

/// Command block describing an SD/MMC command.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EfiSdMmcCommandBlock {
    /// Command index (e.g. 0 to 63).
    pub command_index: u16,
    /// Command argument.
    pub command_argument: u32,
    /// Command type (`SdMmcCommandType`).
    pub command_type: u32,
    /// Expected response type (`SdMmcResponseType`).
    pub response_type: u32,
}

/// Status block containing the hardware response words.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct EfiSdMmcStatusBlock {
    /// Response word 0.
    pub resp0: u32,
    /// Response word 1.
    pub resp1: u32,
    /// Response word 2.
    pub resp2: u32,
    /// Response word 3.
    pub resp3: u32,
}

/// Command packet passed to `PassThru`.
#[repr(C)]
pub struct EfiSdMmcPassThruCommandPacket {
    /// Timeout in 100ns units (0 for infinite).
    pub timeout: u64,
    /// Pointer to command block.
    pub sd_mmc_cmd_blk: *mut EfiSdMmcCommandBlock,
    /// Pointer to status block.
    pub sd_mmc_status_blk: *mut EfiSdMmcStatusBlock,
    /// Input data buffer (read from card).
    pub in_data_buffer: *mut c_void,
    /// Output data buffer (written to card).
    pub out_data_buffer: *const c_void,
    /// In transfer length in bytes.
    pub in_transfer_length: u32,
    /// Out transfer length in bytes.
    pub out_transfer_length: u32,
    /// Completion transaction status.
    pub transaction_status: EfiStatus,
}

/// Raw UEFI `EFI_SD_MMC_PASS_THRU_PROTOCOL`.
#[repr(C)]
pub struct EfiSdMmcPassThruProtocol {
    /// I/O alignment requirement.
    pub io_align: u32,
    /// Send SD command to card.
    pub pass_thru: unsafe extern "efiapi" fn(
        this: *mut EfiSdMmcPassThruProtocol,
        slot: u8,
        packet: *mut EfiSdMmcPassThruCommandPacket,
        event: *mut c_void,
    ) -> EfiStatus,
    /// Retrieve next slot number.
    pub get_next_slot:
        unsafe extern "efiapi" fn(this: *mut EfiSdMmcPassThruProtocol, slot: *mut u8) -> EfiStatus,
    /// Build device path node for card.
    pub build_device_path: unsafe extern "efiapi" fn(
        this: *mut EfiSdMmcPassThruProtocol,
        slot: u8,
        device_path: *mut *mut c_void,
    ) -> EfiStatus,
    /// Retrieve slot number from device path.
    pub get_slot_number: unsafe extern "efiapi" fn(
        this: *mut EfiSdMmcPassThruProtocol,
        device_path: *const c_void,
        slot: *mut u8,
    ) -> EfiStatus,
    /// Reset the SD card.
    pub reset_device:
        unsafe extern "efiapi" fn(this: *mut EfiSdMmcPassThruProtocol, slot: u8) -> EfiStatus,
}

/// Safe wrapper around `EfiSdMmcPassThruProtocol`.
pub struct SdMmcPassThruProtocol<'a> {
    raw: &'a mut EfiSdMmcPassThruProtocol,
}

impl<'a> SdMmcPassThruProtocol<'a> {
    /// The protocol GUID.
    pub const GUID: EfiGuid = EFI_SD_MMC_PASS_THRU_PROTOCOL_GUID;

    /// Creates a safe wrapper from a mutable raw protocol pointer.
    #[must_use]
    pub fn new(raw: &'a mut EfiSdMmcPassThruProtocol) -> Self {
        Self { raw }
    }

    /// Returns the required I/O alignment.
    #[must_use]
    pub const fn io_align(&self) -> u32 {
        self.raw.io_align
    }

    /// Returns an iterator over available slot numbers.
    #[must_use]
    pub fn slots(&mut self) -> SdMmcSlotIterator<'_> {
        SdMmcSlotIterator {
            protocol: self.raw,
            current_slot: 0xFF,
            done: false,
        }
    }

    /// Resets the card in the given slot.
    ///
    /// # Errors
    /// Returns an error if device reset fails.
    pub fn reset_device(&mut self, slot: u8) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.reset_device)(self.raw, slot) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Synchronously executes an SD/MMC command.
    ///
    /// # Safety
    /// `packet` must be properly initialized with valid buffers and pointers.
    ///
    /// # Errors
    /// Returns an error if the transaction fails on the host controller or card.
    pub unsafe fn pass_thru_sync(
        &mut self,
        slot: u8,
        packet: &mut EfiSdMmcPassThruCommandPacket,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.raw.pass_thru)(
                self.raw,
                slot,
                core::ptr::from_mut::<EfiSdMmcPassThruCommandPacket>(packet),
                core::ptr::null_mut(),
            )
        };
        if status.is_success() {
            if packet.transaction_status.is_success() {
                Ok(())
            } else {
                Err(packet.transaction_status)
            }
        } else {
            Err(status)
        }
    }
}

/// Iterator over available slot numbers on an SD/MMC controller.
pub struct SdMmcSlotIterator<'a> {
    protocol: &'a mut EfiSdMmcPassThruProtocol,
    current_slot: u8,
    done: bool,
}

impl Iterator for SdMmcSlotIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut slot = self.current_slot;
        let status = unsafe { (self.protocol.get_next_slot)(self.protocol, &raw mut slot) };
        if status.is_success() {
            self.current_slot = slot;
            Some(slot)
        } else {
            self.done = true;
            None
        }
    }
}
