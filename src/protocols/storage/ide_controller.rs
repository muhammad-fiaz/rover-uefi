//! # IDE Controller Initialization Protocol
//!
//! Provides the chipset-specific hardware configuration and timing mode negotiation
//! interface between IDE bus/device drivers and host controller silicon, defined by
//! the UEFI Platform Initialization (PI) Specification Volume 5.

use crate::base::{EfiGuid, EfiStatus};

/// The `EFI_IDE_CONTROLLER_INIT_PROTOCOL_GUID`.
pub const EFI_IDE_CONTROLLER_INIT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA1E3_7052,
    0x80D9,
    0x4E65,
    [0xA3, 0x17, 0x3E, 0x9A, 0x55, 0xC4, 0x3E, 0xC9],
);

/// Automatically detects the optimum SATA speed.
pub const EFI_SATA_AUTO_SPEED: u32 = 0;
/// Indicates a first-generation (Gen1, 1.5 Gbps) SATA speed.
pub const EFI_SATA_GEN1_SPEED: u32 = 1;
/// Indicates a second-generation (Gen2, 3.0 Gbps) SATA speed.
pub const EFI_SATA_GEN2_SPEED: u32 = 2;
/// Indicates a third-generation (Gen3, 6.0 Gbps) SATA speed.
pub const EFI_SATA_GEN3_SPEED: u32 = 3;

/// Flag indicating that IDENTIFY data is from an ATAPI device.
pub const EFI_ATAPI_DEVICE_IDENTIFY_DATA: u16 = 0x8000;

/// Enumeration phase during IDE controller channel enumeration.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EfiIdeControllerEnumPhase {
    /// Driver is about to begin enumerating devices behind the channel.
    BeforeChannelEnumeration = 0,
    /// Driver has completed enumerating devices behind the channel.
    AfterChannelEnumeration = 1,
    /// Driver is about to reset devices behind the channel.
    BeforeChannelReset = 2,
    /// Driver has completed resetting devices behind the channel.
    AfterChannelReset = 3,
    /// Driver is about to detect presence of devices behind the channel.
    BusBeforeDevicePresenceDetection = 4,
    /// Driver is done detecting presence of devices behind the channel.
    BusAfterDevicePresenceDetection = 5,
    /// IDE bus requests controller to reprogram and reset mode/timing settings.
    ResetMode = 6,
    /// Maximum enumeration phase boundary.
    BusPhaseMaximum = 7,
}

/// Extended transfer protocol types beyond legacy PIO and DMA.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EfiAtaExtTransferProtocol {
    /// Serial ATA (SATA) transfer protocol.
    Sata = 0,
}

/// ATA timing or speed mode entry.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EfiAtaMode {
    /// True if this mode entry is valid.
    pub valid: bool,
    /// The actual ATA mode value.
    pub mode: u32,
}

/// Extended ATA mode entry for SATA and future interfaces.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfiAtaExtendedMode {
    /// The transfer protocol identifier.
    pub transfer_protocol: EfiAtaExtTransferProtocol,
    /// The mode value for operating the transfer protocol.
    pub mode: u32,
}

/// Collective modes supported by or requested for an IDE device.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfiAtaCollectiveMode {
    /// Programmed Input/Output (PIO) mode.
    pub pio_mode: EfiAtaMode,
    /// Single word DMA mode (legacy/obsolete).
    pub single_word_dma_mode: EfiAtaMode,
    /// Multiword DMA mode.
    pub multi_word_dma_mode: EfiAtaMode,
    /// Ultra DMA (UDMA) mode.
    pub udma_mode: EfiAtaMode,
    /// Number of extended-mode entries.
    pub ext_mode_count: u32,
    /// Extended transfer protocol entries (e.g., SATA speeds).
    pub ext_mode: [EfiAtaExtendedMode; 1],
}

impl Default for EfiAtaCollectiveMode {
    fn default() -> Self {
        Self {
            pio_mode: EfiAtaMode::default(),
            single_word_dma_mode: EfiAtaMode::default(),
            multi_word_dma_mode: EfiAtaMode::default(),
            udma_mode: EfiAtaMode::default(),
            ext_mode_count: 0,
            ext_mode: [EfiAtaExtendedMode {
                transfer_protocol: EfiAtaExtTransferProtocol::Sata,
                mode: EFI_SATA_AUTO_SPEED,
            }],
        }
    }
}

/// Raw 512-byte buffer containing ATA or ATAPI device identification data.
#[repr(C)]
#[derive(Clone, Copy)]
pub union EfiIdeIdentifyData {
    /// Raw byte array.
    pub raw: [u8; 512],
    /// Configuration word at offset 0 (bit 15 indicates ATAPI device if set).
    pub config: u16,
}

impl core::fmt::Debug for EfiIdeIdentifyData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EfiIdeIdentifyData")
            .field("config", &unsafe { self.config })
            .finish()
    }
}

/// Function pointer for `EFI_IDE_CONTROLLER_GET_CHANNEL_INFO`.
pub type EfiIdeControllerGetChannelInfo = unsafe extern "efiapi" fn(
    this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    enabled: *mut bool,
    max_devices: *mut u8,
) -> EfiStatus;

/// Function pointer for `EFI_IDE_CONTROLLER_NOTIFY_PHASE`.
pub type EfiIdeControllerNotifyPhase = unsafe extern "efiapi" fn(
    this: *const EfiIdeControllerInitProtocol,
    phase: EfiIdeControllerEnumPhase,
    channel: u8,
) -> EfiStatus;

/// Function pointer for `EFI_IDE_CONTROLLER_SUBMIT_DATA`.
pub type EfiIdeControllerSubmitData = unsafe extern "efiapi" fn(
    this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    device: u8,
    identify_data: *const EfiIdeIdentifyData,
) -> EfiStatus;

/// Function pointer for `EFI_IDE_CONTROLLER_DISQUALIFY_MODE`.
pub type EfiIdeControllerDisqualifyMode = unsafe extern "efiapi" fn(
    this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    device: u8,
    bad_modes: *const EfiAtaCollectiveMode,
) -> EfiStatus;

/// Function pointer for `EFI_IDE_CONTROLLER_CALCULATE_MODE`.
pub type EfiIdeControllerCalculateMode = unsafe extern "efiapi" fn(
    this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    device: u8,
    supported_modes: *mut *mut EfiAtaCollectiveMode,
) -> EfiStatus;

/// Function pointer for `EFI_IDE_CONTROLLER_SET_TIMING`.
pub type EfiIdeControllerSetTiming = unsafe extern "efiapi" fn(
    this: *const EfiIdeControllerInitProtocol,
    channel: u8,
    device: u8,
    modes: *const EfiAtaCollectiveMode,
) -> EfiStatus;

/// Interface structure for the IDE Controller Initialization Protocol (`EFI_IDE_CONTROLLER_INIT_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiIdeControllerInitProtocol {
    /// Returns information about a specific channel.
    pub get_channel_info: EfiIdeControllerGetChannelInfo,
    /// Notifies the controller driver of an enumeration phase change.
    pub notify_phase: EfiIdeControllerNotifyPhase,
    /// Submits drive identify data to the controller driver.
    pub submit_data: EfiIdeControllerSubmitData,
    /// Submits modes that must be disqualified for a device.
    pub disqualify_mode: EfiIdeControllerDisqualifyMode,
    /// Calculates the optimum operating modes for a device.
    pub calculate_mode: EfiIdeControllerCalculateMode,
    /// Programs hardware timings according to specified modes.
    pub set_timing: EfiIdeControllerSetTiming,
    /// True if enumeration group includes all channels; false if each channel is independent.
    pub enum_all: bool,
    /// Total number of channels produced by this controller.
    pub channel_count: u8,
}

/// Safe wrapper around [`EfiIdeControllerInitProtocol`].
pub struct IdeControllerInit<'a> {
    proto: &'a EfiIdeControllerInitProtocol,
}

impl<'a> IdeControllerInit<'a> {
    /// Wraps a raw reference to [`EfiIdeControllerInitProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiIdeControllerInitProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiIdeControllerInitProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to a valid, initialized [`EfiIdeControllerInitProtocol`].
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiIdeControllerInitProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns whether the enumeration group covers all channels together.
    #[must_use]
    pub const fn enum_all(&self) -> bool {
        self.proto.enum_all
    }

    /// Returns the total number of channels supported by the IDE controller.
    #[must_use]
    pub const fn channel_count(&self) -> u8 {
        self.proto.channel_count
    }

    /// Retrieves status and capacity information for a specific channel.
    ///
    /// Returns `(enabled, max_devices)`.
    ///
    /// # Errors
    ///
    /// Returns an error status if `channel` is invalid or cannot be queried.
    pub fn channel_info(&self, channel: u8) -> Result<(bool, u8), EfiStatus> {
        let mut enabled = false;
        let mut max_devices = 0u8;
        let status = unsafe {
            (self.proto.get_channel_info)(
                self.proto,
                channel,
                core::ptr::from_mut(&mut enabled),
                core::ptr::from_mut(&mut max_devices),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok((enabled, max_devices))
        } else {
            Err(status)
        }
    }

    /// Notifies the controller driver that the bus driver is entering a particular enumeration phase.
    ///
    /// # Errors
    ///
    /// Returns an error status if `channel` is invalid or `phase` is not accepted.
    pub fn notify_phase(
        &self,
        phase: EfiIdeControllerEnumPhase,
        channel: u8,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.notify_phase)(self.proto, phase, channel) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Submits drive identification data to the controller driver.
    ///
    /// Pass `None` if the device is not present on the channel.
    ///
    /// # Errors
    ///
    /// Returns an error status if channel/device index is invalid.
    pub fn submit_data(
        &self,
        channel: u8,
        device: u8,
        identify_data: Option<&EfiIdeIdentifyData>,
    ) -> Result<(), EfiStatus> {
        let data_ptr = identify_data.map_or(core::ptr::null(), core::ptr::from_ref);
        let status = unsafe { (self.proto.submit_data)(self.proto, channel, device, data_ptr) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Disqualifies specific timing modes for a device on a channel.
    ///
    /// # Errors
    ///
    /// Returns an error status if channel/device index is invalid.
    pub fn disqualify_mode(
        &self,
        channel: u8,
        device: u8,
        bad_modes: &EfiAtaCollectiveMode,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.disqualify_mode)(
                self.proto,
                channel,
                device,
                core::ptr::from_ref(bad_modes),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Calculates optimum timing modes for a device.
    ///
    /// # Safety
    ///
    /// The caller is responsible for deallocating the returned pool buffer when no longer needed.
    ///
    /// # Errors
    ///
    /// Returns an error status if channel/device index is invalid or mode calculation fails.
    pub unsafe fn calculate_mode(
        &self,
        channel: u8,
        device: u8,
    ) -> Result<*mut EfiAtaCollectiveMode, EfiStatus> {
        let mut supported_modes = core::ptr::null_mut();
        let status = unsafe {
            (self.proto.calculate_mode)(
                self.proto,
                channel,
                device,
                core::ptr::from_mut(&mut supported_modes),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(supported_modes)
        } else {
            Err(status)
        }
    }

    /// Commands the controller driver to program hardware timing per specified modes.
    ///
    /// # Errors
    ///
    /// Returns an error status if channel/device index is invalid or hardware programming fails.
    pub fn set_timing(
        &self,
        channel: u8,
        device: u8,
        modes: &EfiAtaCollectiveMode,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.proto.set_timing)(self.proto, channel, device, core::ptr::from_ref(modes))
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
