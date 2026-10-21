//! # Protocols
//!
//! Protocol handle and interface operations.
//!
//! Methods in this module expose the exact binary
//! interface because correctness depends on
//! caller-provided pointers and on the lifetime
//! of the returned interfaces. Each unsafe
//! method documents its pointer requirements.
//! The caller owns the contract for opening and
//! closing a protocol: an interface obtained
//! with open must be closed with the same agent
//! and controller handles.
//!
//! Variadic install and uninstall services are
//! exposed only through the raw table because
//! Rust cannot forward variadic arguments
//! generically. Callers who need them create
//! them through the raw table pointer. The
//! single-interface install and uninstall
//! methods cover the same operations without
//! variadic arguments.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiGuid, EfiHandle, EfiStatus, InterfaceType};
use crate::services::boot::BootServices;
use crate::tables::boot::{EfiDevicePath, EfiOpenProtocolInformationEntry};

impl BootServices<'_> {
    /// Installs a protocol interface on a handle.
    ///
    /// # Safety
    ///
    /// The handle pointer must be valid, the
    /// protocol pointer must point to a valid
    /// `GUID` for the call, and the interface
    /// pointer must point to the protocol
    /// implementation for the duration of the
    /// installation.
    #[must_use]
    pub unsafe fn install_protocol_interface(
        &self,
        handle: *mut EfiHandle,
        protocol: *mut EfiGuid,
        interface_type: InterfaceType,
        interface: *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe {
            (self.table.install_protocol_interface)(handle, protocol, interface_type, interface)
        }
    }

    /// Reinstalls a protocol interface.
    ///
    /// # Safety
    ///
    /// The handle must name a device, the
    /// protocol pointer must be valid, and both
    /// interface pointers must be valid for the
    /// call.
    #[must_use]
    pub unsafe fn reinstall_protocol_interface(
        &self,
        handle: EfiHandle,
        protocol: *mut EfiGuid,
        old_interface: *mut c_void,
        new_interface: *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees handle and pointer validity for the call.
        unsafe {
            (self.table.reinstall_protocol_interface)(
                handle,
                protocol,
                old_interface,
                new_interface,
            )
        }
    }

    /// Uninstalls a protocol interface.
    ///
    /// # Safety
    ///
    /// The handle, protocol, and interface
    /// pointers must identify the installed
    /// interface for the call.
    #[must_use]
    pub unsafe fn uninstall_protocol_interface(
        &self,
        handle: EfiHandle,
        protocol: *mut EfiGuid,
        interface: *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees the handle and pointers identify the installed interface.
        unsafe { (self.table.uninstall_protocol_interface)(handle, protocol, interface) }
    }

    /// Queries a handle for a protocol.
    ///
    /// # Safety
    ///
    /// The protocol pointer must be valid, and
    /// the interface pointer must be valid for
    /// writing a pointer. The returned interface
    /// must be used according to the protocol
    /// lifetime and open rules.
    #[must_use]
    pub unsafe fn handle_protocol(
        &self,
        handle: EfiHandle,
        protocol: *mut EfiGuid,
        interface: *mut *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe { (self.table.handle_protocol)(handle, protocol, interface) }
    }

    /// Registers a protocol notification.
    ///
    /// # Safety
    ///
    /// The protocol pointer must be valid, the
    /// event must be a valid firmware event, and
    /// the registration pointer must be valid
    /// for writing.
    #[must_use]
    pub unsafe fn register_protocol_notify(
        &self,
        protocol: *mut EfiGuid,
        event: EfiEvent,
        registration: *mut *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer and event validity for the call.
        unsafe { (self.table.register_protocol_notify)(protocol, event, registration) }
    }

    /// Locates a device by device path.
    ///
    /// # Safety
    ///
    /// The protocol pointer must be valid, the
    /// device path double pointer must be valid,
    /// and the device pointer must be valid for
    /// writing.
    #[must_use]
    pub unsafe fn locate_device_path(
        &self,
        protocol: *mut EfiGuid,
        device_path: *mut *mut EfiDevicePath,
        device: *mut EfiHandle,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe { (self.table.locate_device_path)(protocol, device_path, device) }
    }

    /// Installs a configuration table.
    ///
    /// # Safety
    ///
    /// The GUID pointer must be valid. The table
    /// pointer must be valid for the call and,
    /// when non-null, must remain valid for the
    /// firmware lifetime of the entry.
    #[must_use]
    pub unsafe fn install_configuration_table(
        &self,
        guid: *mut EfiGuid,
        table: *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe { (self.table.install_configuration_table)(guid, table) }
    }

    /// Opens a protocol on a handle.
    ///
    /// # Safety
    ///
    /// The protocol and interface pointers must
    /// be valid, the handles must be valid, and
    /// the attributes must use the defined
    /// `OPEN_PROTOCOL` values. The interface
    /// obtained must be closed with `close_protocol`.
    #[must_use]
    pub unsafe fn open_protocol(
        &self,
        handle: EfiHandle,
        protocol: *mut EfiGuid,
        interface: *mut *mut c_void,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer, handle, and attribute validity.
        unsafe {
            (self.table.open_protocol)(
                handle,
                protocol,
                interface,
                agent_handle,
                controller_handle,
                attributes,
            )
        }
    }

    /// Closes a protocol on a handle.
    ///
    /// # Safety
    ///
    /// The protocol pointer must be valid, and
    /// the handles must identify the open that
    /// was created by `open_protocol`.
    #[must_use]
    pub unsafe fn close_protocol(
        &self,
        handle: EfiHandle,
        protocol: *mut EfiGuid,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer and handle validity for the call.
        unsafe { (self.table.close_protocol)(handle, protocol, agent_handle, controller_handle) }
    }

    /// Retrieves open protocol information.
    ///
    /// # Safety
    ///
    /// The protocol pointer must be valid and
    /// both out pointers must be valid for
    /// writing. The returned buffer is owned by
    /// firmware and must be freed according to
    /// the pool ownership rules.
    #[must_use]
    pub unsafe fn open_protocol_information(
        &self,
        handle: EfiHandle,
        protocol: *mut EfiGuid,
        entry_buffer: *mut *mut EfiOpenProtocolInformationEntry,
        entry_count: *mut UIntN,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe {
            (self.table.open_protocol_information)(handle, protocol, entry_buffer, entry_count)
        }
    }

    /// Retrieves protocols for a handle.
    ///
    /// # Safety
    ///
    /// The protocol buffer and count pointers
    /// must be valid for writing. The returned
    /// buffer is owned by firmware and must be
    /// freed according to the pool ownership rules.
    #[must_use]
    pub unsafe fn protocols_per_handle(
        &self,
        handle: EfiHandle,
        protocol_buffer: *mut *mut *mut EfiGuid,
        protocol_buffer_count: *mut UIntN,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe { (self.table.protocols_per_handle)(handle, protocol_buffer, protocol_buffer_count) }
    }

    /// Locates a protocol.
    ///
    /// # Safety
    ///
    /// The protocol and interface pointers must
    /// be valid. The registration pointer may be
    /// null and is passed through unchanged.
    #[must_use]
    pub unsafe fn locate_protocol(
        &self,
        protocol: *mut EfiGuid,
        registration: *mut c_void,
        interface: *mut *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees pointer validity for the call.
        unsafe { (self.table.locate_protocol)(protocol, registration, interface) }
    }
}
