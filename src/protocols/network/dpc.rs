//! # Deferred Procedure Call (DPC) Protocol
//!
//! Provides asynchronous work queuing and dispatch mechanisms across task priority
//! levels (TPL) for network drivers and asynchronous completion routines.

use core::ffi::c_void;

use crate::base::{EfiGuid, EfiStatus};

/// Deferred Procedure Call Protocol GUID: `{480f8ae9-0c46-4aa9-bc89-db9fba619806}`.
pub const DPC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x480F_8AE9,
    0x0C46,
    0x4AA9,
    [0xBC, 0x89, 0xDB, 0x9F, 0xBA, 0x61, 0x98, 0x06],
);

/// Task Priority Level for normal application execution.
pub const TPL_APPLICATION: usize = 4;
/// Task Priority Level for asynchronous event and network callbacks.
pub const TPL_CALLBACK: usize = 8;
/// Task Priority Level for timer and notification handlers.
pub const TPL_NOTIFY: usize = 16;
/// Task Priority Level for highest priority firmware execution (interrupts masked).
pub const TPL_HIGH_LEVEL: usize = 31;

/// Deferred Procedure Call execution callback.
pub type DpcProcedure = unsafe extern "efiapi" fn(dpc_context: *mut c_void);

/// Function pointer type for enqueuing a deferred procedure call.
pub type QueueDpcFn = unsafe extern "efiapi" fn(
    this: *mut DpcProtocol,
    dpc_tpl: usize,
    dpc_procedure: DpcProcedure,
    dpc_context: *mut c_void,
) -> EfiStatus;

/// Function pointer type for dispatching enqueued deferred procedure calls.
pub type DispatchDpcFn = unsafe extern "efiapi" fn(this: *mut DpcProtocol) -> EfiStatus;

/// Raw Deferred Procedure Call Protocol interface table.
#[repr(C)]
pub struct DpcProtocol {
    /// Enqueues a procedure call to be dispatched at the specified TPL.
    pub queue_dpc: QueueDpcFn,
    /// Dispatches all pending deferred procedure calls in the queue.
    pub dispatch_dpc: DispatchDpcFn,
}

/// Safe client wrapper around [`DpcProtocol`].
pub struct Dpc<'a> {
    raw: &'a mut DpcProtocol,
}

impl Dpc<'_> {
    /// Wraps a raw [`DpcProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut DpcProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Adds a deferred procedure call to the DPC queue at the designated TPL.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `procedure` and `context` remain valid until dispatched.
    ///
    /// # Errors
    ///
    /// Returns an error status if the TPL is invalid, procedure is null, or insufficient memory.
    pub unsafe fn queue(
        &mut self,
        tpl: usize,
        procedure: DpcProcedure,
        context: *mut c_void,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.queue_dpc)(self.raw, tpl, procedure, context) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Dispatches queued deferred procedure calls.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if no DPCs were pending execution, or an error status code.
    pub fn dispatch(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.dispatch_dpc)(self.raw) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
