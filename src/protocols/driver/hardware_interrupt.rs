//! # Hardware Interrupt and Interrupt Controller Protocols
//!
//! Provides hardware-based interrupt dispatching, handler registration, interrupt line
//! enabling/disabling, end-of-interrupt (`EOI`) signaling, and trigger edge/level configuration.

#![allow(clippy::doc_markdown)]

use crate::abi::types::{Bool, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// Hardware Interrupt Protocol `GUID` (v1).
///
/// `{2890b3ea-053d-1643-ad0c-d64808da3ff1}`
pub const HARDWARE_INTERRUPT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x2890_B3EA,
    0x053D,
    0x1643,
    [0xAD, 0x0C, 0xD6, 0x48, 0x08, 0xDA, 0x3F, 0xF1],
);

/// Hardware Interrupt 2 Protocol `GUID` (v2 with trigger configuration).
///
/// `{32898322-2da1-474a-baaa-f3f7cf569470}`
pub const HARDWARE_INTERRUPT2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x3289_8322,
    0x2DA1,
    0x474A,
    [0xBA, 0xAA, 0xF3, 0xF7, 0xCF, 0x56, 0x94, 0x70],
);

/// Represents an architectural hardware interrupt vector or IRQ line index.
pub type HardwareInterruptSource = UIntN;

/// Interrupt trigger modes supported by interrupt controllers (GIC, APIC, PLIC).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum HardwareInterruptTriggerType {
    /// Active low level-sensitive trigger.
    LevelLow = 0,
    /// Active high level-sensitive trigger.
    LevelHigh = 1,
    /// Falling-edge sensitive trigger.
    EdgeFalling = 2,
    /// Rising-edge sensitive trigger.
    EdgeRising = 3,
}

/// Raw C-ABI callback function invoked when an interrupt fires.
pub type HardwareInterruptHandler =
    unsafe extern "efiapi" fn(source: HardwareInterruptSource, system_context: *const c_void);

/// Function pointer to register an interrupt callback handler.
pub type HardwareInterruptRegister = unsafe extern "efiapi" fn(
    this: *mut HardwareInterruptProtocol,
    source: HardwareInterruptSource,
    handler: Option<HardwareInterruptHandler>,
) -> EfiStatus;

/// Function pointer to unmask/enable an interrupt line.
pub type HardwareInterruptEnable = unsafe extern "efiapi" fn(
    this: *mut HardwareInterruptProtocol,
    source: HardwareInterruptSource,
) -> EfiStatus;

/// Function pointer to mask/disable an interrupt line.
pub type HardwareInterruptDisable = unsafe extern "efiapi" fn(
    this: *mut HardwareInterruptProtocol,
    source: HardwareInterruptSource,
) -> EfiStatus;

/// Function pointer to query whether an interrupt line is enabled.
pub type HardwareInterruptGetState = unsafe extern "efiapi" fn(
    this: *mut HardwareInterruptProtocol,
    source: HardwareInterruptSource,
    interrupt_state: *mut Bool,
) -> EfiStatus;

/// Function pointer to signal End of Interrupt (`EOI`) to the controller.
pub type HardwareInterruptEndOfInterrupt = unsafe extern "efiapi" fn(
    this: *mut HardwareInterruptProtocol,
    source: HardwareInterruptSource,
) -> EfiStatus;

/// Raw Hardware Interrupt Protocol structure (v1).
#[derive(Debug)]
#[repr(C)]
pub struct HardwareInterruptProtocol {
    /// Registers or unregisters an interrupt handler callback.
    pub register_interrupt_source: HardwareInterruptRegister,
    /// Unmasks an interrupt line.
    pub enable_interrupt_source: HardwareInterruptEnable,
    /// Masks an interrupt line.
    pub disable_interrupt_source: HardwareInterruptDisable,
    /// Queries the mask status of an interrupt line.
    pub get_interrupt_source_state: HardwareInterruptGetState,
    /// Signals End of Interrupt (`EOI`).
    pub end_of_interrupt: HardwareInterruptEndOfInterrupt,
}

/// Function pointer for registering handlers on Hardware Interrupt 2.
pub type HardwareInterrupt2Register = unsafe extern "efiapi" fn(
    this: *mut HardwareInterrupt2Protocol,
    source: HardwareInterruptSource,
    handler: Option<HardwareInterruptHandler>,
) -> EfiStatus;

/// Function pointer for enabling interrupts on Hardware Interrupt 2.
pub type HardwareInterrupt2Enable = unsafe extern "efiapi" fn(
    this: *mut HardwareInterrupt2Protocol,
    source: HardwareInterruptSource,
) -> EfiStatus;

/// Function pointer for disabling interrupts on Hardware Interrupt 2.
pub type HardwareInterrupt2Disable = unsafe extern "efiapi" fn(
    this: *mut HardwareInterrupt2Protocol,
    source: HardwareInterruptSource,
) -> EfiStatus;

/// Function pointer for querying state on Hardware Interrupt 2.
pub type HardwareInterrupt2GetState = unsafe extern "efiapi" fn(
    this: *mut HardwareInterrupt2Protocol,
    source: HardwareInterruptSource,
    interrupt_state: *mut Bool,
) -> EfiStatus;

/// Function pointer for signaling EOI on Hardware Interrupt 2.
pub type HardwareInterrupt2EndOfInterrupt = unsafe extern "efiapi" fn(
    this: *mut HardwareInterrupt2Protocol,
    source: HardwareInterruptSource,
) -> EfiStatus;

/// Function pointer for querying trigger type.
pub type HardwareInterrupt2GetTriggerType = unsafe extern "efiapi" fn(
    this: *mut HardwareInterrupt2Protocol,
    source: HardwareInterruptSource,
    trigger_type: *mut HardwareInterruptTriggerType,
) -> EfiStatus;

/// Function pointer for configuring trigger type.
pub type HardwareInterrupt2SetTriggerType = unsafe extern "efiapi" fn(
    this: *mut HardwareInterrupt2Protocol,
    source: HardwareInterruptSource,
    trigger_type: HardwareInterruptTriggerType,
) -> EfiStatus;

/// Raw Hardware Interrupt 2 Protocol structure (v2).
#[derive(Debug)]
#[repr(C)]
pub struct HardwareInterrupt2Protocol {
    /// Registers or unregisters an interrupt handler callback.
    pub register_interrupt_source: HardwareInterrupt2Register,
    /// Unmasks an interrupt line.
    pub enable_interrupt_source: HardwareInterrupt2Enable,
    /// Masks an interrupt line.
    pub disable_interrupt_source: HardwareInterrupt2Disable,
    /// Queries the mask status of an interrupt line.
    pub get_interrupt_source_state: HardwareInterrupt2GetState,
    /// Signals End of Interrupt (`EOI`).
    pub end_of_interrupt: HardwareInterrupt2EndOfInterrupt,
    /// Retrieves current trigger mode (level/edge).
    pub get_trigger_type: HardwareInterrupt2GetTriggerType,
    /// Configures trigger mode (level/edge).
    pub set_trigger_type: HardwareInterrupt2SetTriggerType,
}

/// High-level safe wrapper around [`HardwareInterruptProtocol`].
pub struct HardwareInterrupt<'a> {
    proto: &'a mut HardwareInterruptProtocol,
}

impl<'a> HardwareInterrupt<'a> {
    /// Creates a safe wrapper around an active protocol instance.
    #[must_use]
    pub fn new(proto: &'a mut HardwareInterruptProtocol) -> Self {
        Self { proto }
    }

    /// Registers or unregisters an interrupt handler for `source`.
    ///
    /// # Errors
    ///
    /// Returns an error status if the interrupt controller fails to register the handler.
    pub fn register_interrupt_source(
        &mut self,
        source: HardwareInterruptSource,
        handler: Option<HardwareInterruptHandler>,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.register_interrupt_source)(self.proto, source, handler) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Enables (unmasks) `source`.
    ///
    /// # Errors
    ///
    /// Returns an error status if unmasking `source` fails.
    pub fn enable(&mut self, source: HardwareInterruptSource) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.enable_interrupt_source)(self.proto, source) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Disables (masks) `source`.
    ///
    /// # Errors
    ///
    /// Returns an error status if masking `source` fails.
    pub fn disable(&mut self, source: HardwareInterruptSource) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.disable_interrupt_source)(self.proto, source) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries whether `source` is currently unmasked.
    ///
    /// # Errors
    ///
    /// Returns an error status if querying mask state fails.
    pub fn is_enabled(&mut self, source: HardwareInterruptSource) -> Result<bool, EfiStatus> {
        let mut state: Bool = 0;
        let status =
            unsafe { (self.proto.get_interrupt_source_state)(self.proto, source, &raw mut state) };
        if status.is_success() {
            Ok(state != 0)
        } else {
            Err(status)
        }
    }

    /// Signals End of Interrupt (`EOI`).
    ///
    /// # Errors
    ///
    /// Returns an error status if signaling EOI fails on the controller.
    pub fn end_of_interrupt(&mut self, source: HardwareInterruptSource) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.end_of_interrupt)(self.proto, source) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// High-level safe wrapper around [`HardwareInterrupt2Protocol`].
pub struct HardwareInterrupt2<'a> {
    proto: &'a mut HardwareInterrupt2Protocol,
}

impl<'a> HardwareInterrupt2<'a> {
    /// Creates a safe wrapper around an active protocol instance.
    #[must_use]
    pub fn new(proto: &'a mut HardwareInterrupt2Protocol) -> Self {
        Self { proto }
    }

    /// Registers or unregisters an interrupt handler for `source`.
    ///
    /// # Errors
    ///
    /// Returns an error status if the interrupt controller fails to register the handler.
    pub fn register_interrupt_source(
        &mut self,
        source: HardwareInterruptSource,
        handler: Option<HardwareInterruptHandler>,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.register_interrupt_source)(self.proto, source, handler) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Enables (unmasks) `source`.
    ///
    /// # Errors
    ///
    /// Returns an error status if unmasking `source` fails.
    pub fn enable(&mut self, source: HardwareInterruptSource) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.enable_interrupt_source)(self.proto, source) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Disables (masks) `source`.
    ///
    /// # Errors
    ///
    /// Returns an error status if masking `source` fails.
    pub fn disable(&mut self, source: HardwareInterruptSource) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.disable_interrupt_source)(self.proto, source) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries whether `source` is currently unmasked.
    ///
    /// # Errors
    ///
    /// Returns an error status if querying mask state fails.
    pub fn is_enabled(&mut self, source: HardwareInterruptSource) -> Result<bool, EfiStatus> {
        let mut state: Bool = 0;
        let status =
            unsafe { (self.proto.get_interrupt_source_state)(self.proto, source, &raw mut state) };
        if status.is_success() {
            Ok(state != 0)
        } else {
            Err(status)
        }
    }

    /// Signals End of Interrupt (`EOI`).
    ///
    /// # Errors
    ///
    /// Returns an error status if signaling EOI fails on the controller.
    pub fn end_of_interrupt(&mut self, source: HardwareInterruptSource) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.end_of_interrupt)(self.proto, source) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries the configured trigger mode for `source`.
    ///
    /// # Errors
    ///
    /// Returns an error status if querying trigger mode fails.
    pub fn get_trigger_type(
        &mut self,
        source: HardwareInterruptSource,
    ) -> Result<HardwareInterruptTriggerType, EfiStatus> {
        let mut trigger = HardwareInterruptTriggerType::LevelHigh;
        let status = unsafe { (self.proto.get_trigger_type)(self.proto, source, &raw mut trigger) };
        if status.is_success() {
            Ok(trigger)
        } else {
            Err(status)
        }
    }

    /// Configures the trigger mode for `source`.
    ///
    /// # Errors
    ///
    /// Returns an error status if setting trigger mode fails or is unsupported.
    pub fn set_trigger_type(
        &mut self,
        source: HardwareInterruptSource,
        trigger: HardwareInterruptTriggerType,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set_trigger_type)(self.proto, source, trigger) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
