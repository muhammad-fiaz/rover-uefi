//! # Events
//!
//! Event creation, timers, and signaling.
//!
//! Safe methods cover events without a
//! notification callback. The unsafe method
//! covers callbacks because firmware invokes
//! the callback with the supplied context at
//! a task priority level above the application.
//! The caller must guarantee that the callback
//! follows `EFIAPI`, handles the context
//! correctly, and does not call boot services
//! that are forbidden at that priority.
//!
//! Event handles are firmware owned. An event
//! must be closed exactly once.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiGuid, EfiStatus, EfiTpl, TimerDelay};
use crate::services::boot::BootServices;
use crate::tables::boot::EfiEventNotify;

/// Timer event flag.
pub const EVT_TIMER: u32 = 0x8000_0000;

/// Runtime event flag.
pub const EVT_RUNTIME: u32 = 0x4000_0000;

/// Notify on wait flag.
pub const EVT_NOTIFY_WAIT: u32 = 0x0000_0100;

/// Notify on signal flag.
pub const EVT_NOTIFY_SIGNAL: u32 = 0x0000_0200;

/// Signal on exit boot services.
pub const EVT_SIGNAL_EXIT_BOOT_SERVICES: u32 = 0x0000_0201;

/// Signal on virtual address change.
pub const EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE: u32 = 0x6000_0202;

/// Runtime context flag.
pub const EVT_RUNTIME_CONTEXT: u32 = 0x2000_0000;

impl BootServices<'_> {
    /// Creates an event without a callback.
    ///
    /// The event is owned by firmware and must
    /// be closed with `close_event`.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when creation
    /// fails.
    pub fn create_event(&self, event_type: u32, notify_tpl: EfiTpl) -> Result<EfiEvent, EfiStatus> {
        let mut event = EfiEvent::NULL;
        // SAFETY: The stack local lives for the call and its address is valid for the out parameter.
        let status = unsafe {
            (self.table.create_event)(
                event_type,
                notify_tpl,
                None,
                core::ptr::null_mut(),
                &raw mut event,
            )
        };
        if status.is_success() {
            Ok(event)
        } else {
            Err(status)
        }
    }

    /// Creates an event with a callback.
    ///
    /// The callback is invoked by firmware with
    /// the context pointer.
    ///
    /// # Safety
    ///
    /// The callback must use the `EFIAPI` calling
    /// convention, must handle the context
    /// pointer correctly, and must obey the
    /// priority restrictions of the event. The
    /// context must remain valid until the event
    /// is closed.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when creation
    /// fails.
    pub unsafe fn create_event_with_notify(
        &self,
        event_type: u32,
        notify_tpl: EfiTpl,
        notify_function: EfiEventNotify,
        notify_context: *mut c_void,
    ) -> Result<EfiEvent, EfiStatus> {
        let mut event = EfiEvent::NULL;
        // SAFETY: The caller guarantees the callback and context meet the safety requirements and the out local is valid.
        let status = unsafe {
            (self.table.create_event)(
                event_type,
                notify_tpl,
                Some(notify_function),
                notify_context,
                &raw mut event,
            )
        };
        if status.is_success() {
            Ok(event)
        } else {
            Err(status)
        }
    }

    /// Creates an event in a group without a
    /// callback.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when creation
    /// fails.
    pub fn create_event_ex(
        &self,
        event_type: u32,
        notify_tpl: EfiTpl,
        event_group: Option<&EfiGuid>,
    ) -> Result<EfiEvent, EfiStatus> {
        let mut event = EfiEvent::NULL;
        let group_ptr = match event_group {
            Some(guid) => core::ptr::from_ref(guid).cast(),
            None => core::ptr::null(),
        };
        // SAFETY: The reference (when present) is valid for the call and the out local is valid.
        let status = unsafe {
            (self.table.create_event_ex)(
                event_type,
                notify_tpl,
                None,
                core::ptr::null(),
                group_ptr,
                &raw mut event,
            )
        };
        if status.is_success() {
            Ok(event)
        } else {
            Err(status)
        }
    }

    /// Creates an event in a group with a
    /// callback.
    ///
    /// # Safety
    ///
    /// The callback and context must meet the
    /// same requirements as `create_event_with_notify`.
    /// The group pointer must be valid for the call.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when creation
    /// fails.
    pub unsafe fn create_event_ex_with_notify(
        &self,
        event_type: u32,
        notify_tpl: EfiTpl,
        notify_function: EfiEventNotify,
        notify_context: *const c_void,
        event_group: *const EfiGuid,
    ) -> Result<EfiEvent, EfiStatus> {
        let mut event = EfiEvent::NULL;
        // SAFETY: The caller guarantees the callback, context, and group pointer meet the safety requirements and the out local is valid.
        let status = unsafe {
            (self.table.create_event_ex)(
                event_type,
                notify_tpl,
                Some(notify_function),
                notify_context,
                event_group,
                &raw mut event,
            )
        };
        if status.is_success() {
            Ok(event)
        } else {
            Err(status)
        }
    }

    /// Sets the timer for an event.
    #[must_use]
    pub fn set_timer(&self, event: EfiEvent, delay: TimerDelay, trigger_time: u64) -> EfiStatus {
        // SAFETY: The event value is a firmware token and the arguments are copied values.
        unsafe { (self.table.set_timer)(event, delay, trigger_time) }
    }

    /// Signals an event.
    #[must_use]
    pub fn signal_event(&self, event: EfiEvent) -> EfiStatus {
        // SAFETY: Firmware returns an error for an invalid event instead of undefined behavior.
        unsafe { (self.table.signal_event)(event) }
    }

    /// Waits for one of the events.
    ///
    /// The slice must be non-empty. The returned
    /// index identifies the signalled event.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the wait
    /// fails.
    pub fn wait_for_event(&self, events: &[EfiEvent]) -> Result<UIntN, EfiStatus> {
        let mut index: UIntN = 0;
        // SAFETY: The slice pointer is valid for `events.len()` handles and the stack local is valid for the out parameter.
        let status = unsafe {
            (self.table.wait_for_event)(events.len(), events.as_ptr().cast_mut(), &raw mut index)
        };
        if status.is_success() {
            Ok(index)
        } else {
            Err(status)
        }
    }

    /// Closes an event.
    #[must_use]
    pub fn close_event(&self, event: EfiEvent) -> EfiStatus {
        // SAFETY: Firmware returns an error for an invalid event.
        unsafe { (self.table.close_event)(event) }
    }

    /// Checks whether an event is signalled.
    #[must_use]
    pub fn check_event(&self, event: EfiEvent) -> EfiStatus {
        // SAFETY: Firmware returns an error for an invalid event.
        unsafe { (self.table.check_event)(event) }
    }
}
