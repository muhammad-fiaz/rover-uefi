//! # Embedded MMC Host Protocol
//!
//! Provides hardware abstraction for MultiMediaCard (MMC) and SD card host
//! controllers on embedded and SoC platforms, managing command dispatch, responses,
//! bus clock frequencies, bus widths, and block I/O transactions.

#![allow(clippy::doc_markdown)]

use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::device_path::EfiDevicePath;

/// Embedded MMC Host Protocol GUID: `{3e591c00-9e4a-11df-9244-0002a5d5c51b}`.
pub const EMBEDDED_MMC_HOST_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3E59_1C00,
    0x9E4A,
    0x11DF,
    [0x92, 0x44, 0x00, 0x02, 0xA5, 0xD5, 0xC5, 0x1B],
);

/// MMC Host Protocol revision: 1.2 (`0x00010002`).
pub const MMC_HOST_PROTOCOL_REVISION: u32 = 0x0001_0002;

/// Response flag: command expects a response from the card.
pub const MMC_CMD_WAIT_RESPONSE: u32 = 1 << 16;
/// Response flag: command expects a 136-bit long response (R2).
pub const MMC_CMD_LONG_RESPONSE: u32 = 1 << 17;
/// Response flag: command response does not include a CRC check (e.g. R3).
pub const MMC_CMD_NO_CRC_RESPONSE: u32 = 1 << 18;

/// Helper macro to form MMC command bitfields.
#[must_use]
pub const fn mmc_cmd(idx: u32, flags: u32) -> u32 {
    (idx & 0xFFFF) | flags
}

/// CMD0: GO_IDLE_STATE.
pub const MMC_CMD0: u32 = mmc_cmd(0, MMC_CMD_NO_CRC_RESPONSE);
/// CMD1: SEND_OP_COND.
pub const MMC_CMD1: u32 = mmc_cmd(1, MMC_CMD_WAIT_RESPONSE | MMC_CMD_NO_CRC_RESPONSE);
/// CMD2: ALL_SEND_CID.
pub const MMC_CMD2: u32 = mmc_cmd(2, MMC_CMD_WAIT_RESPONSE | MMC_CMD_LONG_RESPONSE);
/// CMD3: SET_RELATIVE_ADDR.
pub const MMC_CMD3: u32 = mmc_cmd(3, MMC_CMD_WAIT_RESPONSE);
/// CMD5: SLEEP_AWAKE.
pub const MMC_CMD5: u32 = mmc_cmd(5, MMC_CMD_WAIT_RESPONSE | MMC_CMD_NO_CRC_RESPONSE);
/// CMD6: SWITCH.
pub const MMC_CMD6: u32 = mmc_cmd(6, MMC_CMD_WAIT_RESPONSE);
/// CMD7: SELECT_DESELECT_CARD.
pub const MMC_CMD7: u32 = mmc_cmd(7, MMC_CMD_WAIT_RESPONSE);
/// CMD8: SEND_EXT_CSD / SEND_IF_COND.
pub const MMC_CMD8: u32 = mmc_cmd(8, MMC_CMD_WAIT_RESPONSE);
/// CMD9: SEND_CSD.
pub const MMC_CMD9: u32 = mmc_cmd(9, MMC_CMD_WAIT_RESPONSE | MMC_CMD_LONG_RESPONSE);
/// CMD11: VOLTAGE_SWITCH.
pub const MMC_CMD11: u32 = mmc_cmd(11, MMC_CMD_WAIT_RESPONSE);
/// CMD12: STOP_TRANSMISSION.
pub const MMC_CMD12: u32 = mmc_cmd(12, MMC_CMD_WAIT_RESPONSE);
/// CMD13: SEND_STATUS.
pub const MMC_CMD13: u32 = mmc_cmd(13, MMC_CMD_WAIT_RESPONSE);
/// CMD16: SET_BLOCKLEN.
pub const MMC_CMD16: u32 = mmc_cmd(16, MMC_CMD_WAIT_RESPONSE);
/// CMD17: READ_SINGLE_BLOCK.
pub const MMC_CMD17: u32 = mmc_cmd(17, MMC_CMD_WAIT_RESPONSE);
/// CMD18: READ_MULTIPLE_BLOCK.
pub const MMC_CMD18: u32 = mmc_cmd(18, MMC_CMD_WAIT_RESPONSE);
/// CMD20: BUSTEST_R.
pub const MMC_CMD20: u32 = mmc_cmd(20, MMC_CMD_WAIT_RESPONSE);
/// CMD23: SET_BLOCK_COUNT.
pub const MMC_CMD23: u32 = mmc_cmd(23, MMC_CMD_WAIT_RESPONSE);
/// CMD24: WRITE_BLOCK.
pub const MMC_CMD24: u32 = mmc_cmd(24, MMC_CMD_WAIT_RESPONSE);
/// CMD25: WRITE_MULTIPLE_BLOCK.
pub const MMC_CMD25: u32 = mmc_cmd(25, MMC_CMD_WAIT_RESPONSE);
/// CMD55: APP_CMD.
pub const MMC_CMD55: u32 = mmc_cmd(55, MMC_CMD_WAIT_RESPONSE);
/// ACMD41: SD_SEND_OP_COND.
pub const MMC_ACMD41: u32 = mmc_cmd(41, MMC_CMD_WAIT_RESPONSE | MMC_CMD_NO_CRC_RESPONSE);
/// ACMD51: SEND_SCR.
pub const MMC_ACMD51: u32 = mmc_cmd(51, MMC_CMD_WAIT_RESPONSE);

/// MMC Response format: R1 (standard normal response).
pub const MMC_RESPONSE_TYPE_R1: u32 = 0;
/// MMC Response format: R1b (busy signaling response).
pub const MMC_RESPONSE_TYPE_R1B: u32 = 0;
/// MMC Response format: R2 (136-bit CID/CSD register response).
pub const MMC_RESPONSE_TYPE_R2: u32 = 1;
/// MMC Response format: R3 (OCR register response).
pub const MMC_RESPONSE_TYPE_R3: u32 = 0;
/// MMC Response format: R6 (Published RCA response).
pub const MMC_RESPONSE_TYPE_R6: u32 = 0;
/// MMC Response format: R7 (Card interface condition response).
pub const MMC_RESPONSE_TYPE_R7: u32 = 0;

/// Timing modes for eMMC high speed transactions: backward compatible default speed.
pub const EMMC_TIMING_BACKWARD: u32 = 0;
/// Timing mode: High-Speed at 26 MHz.
pub const EMMC_TIMING_HS26: u32 = 1 << 0;
/// Timing mode: High-Speed at 52 MHz.
pub const EMMC_TIMING_HS52: u32 = 1 << 1;
/// Timing mode: High-Speed Dual Data Rate at 52 MHz, 1.8V or 3.0V I/O.
pub const EMMC_TIMING_HS52_DDR_1V8: u32 = 1 << 2;
/// Timing mode: High-Speed Dual Data Rate at 52 MHz, 1.2V I/O.
pub const EMMC_TIMING_HS52_DDR_1V2: u32 = 1 << 3;
/// Timing mode: HS200 Single Data Rate at 200 MHz, 1.8V I/O.
pub const EMMC_TIMING_HS200_SDR_1V8: u32 = 1 << 4;
/// Timing mode: HS200 Single Data Rate at 200 MHz, 1.2V I/O.
pub const EMMC_TIMING_HS200_SDR_1V2: u32 = 1 << 5;
/// Timing mode: HS400 Dual Data Rate at 400 MHz, 1.8V I/O.
pub const EMMC_TIMING_HS400_DDR_1V8: u32 = 1 << 6;
/// Timing mode: HS400 Dual Data Rate at 400 MHz, 1.2V I/O.
pub const EMMC_TIMING_HS400_DDR_1V2: u32 = 1 << 7;

/// Card operating states.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MmcState {
    /// Invalid card state.
    Invalid = 0,
    /// Hardware initialization state.
    HwInitialization = 1,
    /// Idle state following reset.
    Idle = 2,
    /// Ready state after operating condition negotiation.
    Ready = 3,
    /// Card identification state.
    Identification = 4,
    /// Standby state awaiting selection.
    StandBy = 5,
    /// Active data transfer state.
    Transfer = 6,
    /// Transmitting data to host.
    SendingData = 7,
    /// Receiving data from host.
    ReceiveData = 8,
    /// Internal programming / write cycle.
    Programming = 9,
    /// Card disconnected or powered down.
    Disconnect = 10,
}

/// Function type: Check if an MMC/SD card is physically inserted.
pub type MmcIsCardPresent = unsafe extern "efiapi" fn(this: *mut MmcHostProtocol) -> bool;

/// Function type: Check if the card write protect notch is active.
pub type MmcIsReadOnly = unsafe extern "efiapi" fn(this: *mut MmcHostProtocol) -> bool;

/// Function type: Construct a device path protocol for the card device.
pub type MmcBuildDevicePath = unsafe extern "efiapi" fn(
    this: *mut MmcHostProtocol,
    device_path: *mut *mut EfiDevicePath,
) -> EfiStatus;

/// Function type: Notify the host controller of state transitions.
pub type MmcNotifyState =
    unsafe extern "efiapi" fn(this: *mut MmcHostProtocol, state: MmcState) -> EfiStatus;

/// Function type: Dispatch a command and argument to the card.
pub type MmcSendCommand =
    unsafe extern "efiapi" fn(this: *mut MmcHostProtocol, cmd: u32, argument: u32) -> EfiStatus;

/// Function type: Read response registers from the card.
pub type MmcReceiveResponse = unsafe extern "efiapi" fn(
    this: *mut MmcHostProtocol,
    resp_type: u32,
    buffer: *mut u32,
) -> EfiStatus;

/// Function type: Read block data from the card.
pub type MmcReadBlockData = unsafe extern "efiapi" fn(
    this: *mut MmcHostProtocol,
    lba: u64,
    length: usize,
    buffer: *mut u32,
) -> EfiStatus;

/// Function type: Write block data to the card.
pub type MmcWriteBlockData = unsafe extern "efiapi" fn(
    this: *mut MmcHostProtocol,
    lba: u64,
    length: usize,
    buffer: *mut u32,
) -> EfiStatus;

/// Function type: Configure bus clock frequency, bus width, and timing mode.
pub type MmcSetIos = unsafe extern "efiapi" fn(
    this: *mut MmcHostProtocol,
    bus_clock_freq: u32,
    bus_width: u32,
    timing_mode: u32,
) -> EfiStatus;

/// Function type: Check if multi-block transactions are supported.
pub type MmcIsMultiBlock = unsafe extern "efiapi" fn(this: *mut MmcHostProtocol) -> bool;

/// Embedded MMC Host Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MmcHostProtocol {
    /// Revision of the protocol interface (`MMC_HOST_PROTOCOL_REVISION`).
    pub revision: u32,
    /// Checks if a card is present.
    pub is_card_present: MmcIsCardPresent,
    /// Checks write protection.
    pub is_read_only: MmcIsReadOnly,
    /// Allocates and builds the device path.
    pub build_device_path: MmcBuildDevicePath,
    /// Notifies the controller of state changes.
    pub notify_state: MmcNotifyState,
    /// Transmits a command to the card.
    pub send_command: MmcSendCommand,
    /// Receives command response data.
    pub receive_response: MmcReceiveResponse,
    /// Reads block data from the card.
    pub read_block_data: MmcReadBlockData,
    /// Writes block data to the card.
    pub write_block_data: MmcWriteBlockData,
    /// Configures clock, bus width, and speed timing.
    pub set_ios: Option<MmcSetIos>,
    /// Returns true if controller handles multi-block transfers.
    pub is_multi_block: Option<MmcIsMultiBlock>,
}

/// High-level client wrapper providing safe access to the MMC host controller.
#[derive(Debug)]
pub struct MmcHost<'a> {
    protocol: &'a mut MmcHostProtocol,
}

impl<'a> MmcHost<'a> {
    /// Wraps a mutable reference to an `MmcHostProtocol`.
    pub fn new(protocol: &'a mut MmcHostProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the protocol interface revision.
    #[must_use]
    pub const fn revision(&self) -> u32 {
        self.protocol.revision
    }

    /// Checks if an MMC or SD card is inserted.
    pub fn is_card_present(&mut self) -> bool {
        unsafe { (self.protocol.is_card_present)(&raw mut *self.protocol) }
    }

    /// Checks if the inserted card is write-protected.
    pub fn is_read_only(&mut self) -> bool {
        unsafe { (self.protocol.is_read_only)(&raw mut *self.protocol) }
    }

    /// Builds the device path associated with the host controller and card.
    ///
    /// # Errors
    ///
    /// Returns an error status if the host controller cannot generate a device path.
    pub fn build_device_path(&mut self) -> Result<*mut EfiDevicePath, EfiStatus> {
        let mut path: *mut EfiDevicePath = core::ptr::null_mut();
        let status =
            unsafe { (self.protocol.build_device_path)(&raw mut *self.protocol, &raw mut path) };
        if status.is_success() && !path.is_null() {
            Ok(path)
        } else {
            Err(status)
        }
    }

    /// Notifies the host controller of a state transition.
    ///
    /// # Errors
    ///
    /// Returns an error status if the controller state transition is rejected.
    pub fn notify_state(&mut self, state: MmcState) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.notify_state)(&raw mut *self.protocol, state) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Sends an MMC/SD command and 32-bit argument to the card.
    ///
    /// # Errors
    ///
    /// Returns an error status if command transmission fails or times out.
    pub fn send_command(&mut self, cmd: u32, argument: u32) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.protocol.send_command)(&raw mut *self.protocol, cmd, argument) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Receives command response data into the provided buffer.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if `buffer` is empty, or an error status if response retrieval fails.
    pub fn receive_response(
        &mut self,
        resp_type: u32,
        buffer: &mut [u32],
    ) -> Result<(), EfiStatus> {
        if buffer.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let status = unsafe {
            (self.protocol.receive_response)(
                &raw mut *self.protocol,
                resp_type,
                buffer.as_mut_ptr(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads block data into a 32-bit aligned buffer slice.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if `buffer` capacity in bytes is less than `length`,
    /// or an error status if the block read fails.
    pub fn read_block_data(
        &mut self,
        lba: u64,
        length: usize,
        buffer: &mut [u32],
    ) -> Result<(), EfiStatus> {
        if core::mem::size_of_val(buffer) < length {
            return Err(EfiStatus::BUFFER_TOO_SMALL);
        }
        let status = unsafe {
            (self.protocol.read_block_data)(
                &raw mut *self.protocol,
                lba,
                length,
                buffer.as_mut_ptr(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Writes block data from a 32-bit aligned buffer slice.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if `buffer` capacity in bytes is less than `length`,
    /// or an error status if the block write fails.
    pub fn write_block_data(
        &mut self,
        lba: u64,
        length: usize,
        buffer: &[u32],
    ) -> Result<(), EfiStatus> {
        if core::mem::size_of_val(buffer) < length {
            return Err(EfiStatus::BUFFER_TOO_SMALL);
        }
        let status = unsafe {
            (self.protocol.write_block_data)(
                &raw mut *self.protocol,
                lba,
                length,
                buffer.as_ptr().cast_mut(),
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Configures bus clock frequency, bus width, and timing mode if supported.
    ///
    /// # Errors
    ///
    /// Returns `UNSUPPORTED` if `set_ios` is not implemented by the host controller,
    /// or an error status if clock/bus reconfiguration fails.
    pub fn set_ios(
        &mut self,
        bus_clock_freq: u32,
        bus_width: u32,
        timing_mode: u32,
    ) -> Result<(), EfiStatus> {
        if let Some(set_ios) = self.protocol.set_ios {
            let status = unsafe {
                set_ios(
                    &raw mut *self.protocol,
                    bus_clock_freq,
                    bus_width,
                    timing_mode,
                )
            };
            if status.is_success() {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            Err(EfiStatus::UNSUPPORTED)
        }
    }

    /// Queries whether the host controller supports multi-block transfers.
    #[must_use]
    pub fn is_multi_block(&mut self) -> bool {
        if let Some(is_multi) = self.protocol.is_multi_block {
            unsafe { is_multi(&raw mut *self.protocol) }
        } else {
            false
        }
    }
}
