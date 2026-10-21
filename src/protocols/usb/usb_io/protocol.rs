//! # USB I/O protocol
//!
//! The `EFI_USB_IO_PROTOCOL`.
//!
//! Synchronous transfers use caller slices.
//! Asynchronous transfers are unsafe because
//! buffers must remain alive until the
//! callback.

use core::ffi::c_void;

use crate::abi::types::{Char16, UIntN};
use crate::base::EfiStatus;
use crate::protocols::usb::descriptor::{
    ConfigDescriptor, DeviceDescriptor, EndpointDescriptor, InterfaceDescriptor,
};
use crate::protocols::usb::types::{UsbDeviceRequest, UsbDirection};

/// The async callback.
pub type EfiAsyncUsbTransferCallback =
    unsafe extern "efiapi" fn(*mut c_void, UIntN, *mut c_void, u32) -> EfiStatus;

/// Transfers control.
pub type EfiUsbIoControlTransfer = unsafe extern "efiapi" fn(
    *mut EfiUsbIoProtocol,
    *mut UsbDeviceRequest,
    UsbDirection,
    u32,
    *mut c_void,
    UIntN,
    *mut u32,
) -> EfiStatus;

/// Transfers bulk.
pub type EfiUsbIoBulkTransfer = unsafe extern "efiapi" fn(
    *mut EfiUsbIoProtocol,
    u8,
    *mut c_void,
    *mut UIntN,
    UIntN,
    *mut u32,
) -> EfiStatus;

/// Transfers sync interrupt.
pub type EfiUsbIoSyncInterruptTransfer = unsafe extern "efiapi" fn(
    *mut EfiUsbIoProtocol,
    u8,
    *mut c_void,
    *mut UIntN,
    UIntN,
    *mut u32,
) -> EfiStatus;

/// Transfers isochronous.
pub type EfiUsbIoIsochronousTransfer =
    unsafe extern "efiapi" fn(*mut EfiUsbIoProtocol, u8, *mut c_void, UIntN, *mut u32) -> EfiStatus;

/// The USB I/O protocol.
///
/// The struct is `#[repr(C)]`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiUsbIoProtocol {
    /// Control transfer.
    pub control_transfer: EfiUsbIoControlTransfer,
    /// Bulk transfer.
    pub bulk_transfer: EfiUsbIoBulkTransfer,
    /// Async interrupt transfer.
    pub async_interrupt_transfer: unsafe extern "efiapi" fn(
        *mut EfiUsbIoProtocol,
        u8,
        crate::abi::types::Bool,
        UIntN,
        UIntN,
        Option<EfiAsyncUsbTransferCallback>,
        *mut c_void,
    ) -> EfiStatus,
    /// Sync interrupt transfer.
    pub sync_interrupt_transfer: EfiUsbIoSyncInterruptTransfer,
    /// Isochronous transfer.
    pub isochronous_transfer: EfiUsbIoIsochronousTransfer,
    /// Async isochronous transfer.
    pub async_isochronous_transfer: unsafe extern "efiapi" fn(
        *mut EfiUsbIoProtocol,
        u8,
        *mut c_void,
        UIntN,
        EfiAsyncUsbTransferCallback,
        *mut c_void,
    ) -> EfiStatus,
    /// Get device descriptor.
    pub get_device_descriptor:
        unsafe extern "efiapi" fn(*mut EfiUsbIoProtocol, *mut DeviceDescriptor) -> EfiStatus,
    /// Get config descriptor.
    pub get_config_descriptor:
        unsafe extern "efiapi" fn(*mut EfiUsbIoProtocol, *mut ConfigDescriptor) -> EfiStatus,
    /// Get interface descriptor.
    pub get_interface_descriptor:
        unsafe extern "efiapi" fn(*mut EfiUsbIoProtocol, *mut InterfaceDescriptor) -> EfiStatus,
    /// Get endpoint descriptor.
    pub get_endpoint_descriptor:
        unsafe extern "efiapi" fn(*mut EfiUsbIoProtocol, u8, *mut EndpointDescriptor) -> EfiStatus,
    /// Get string descriptor.
    pub get_string_descriptor:
        unsafe extern "efiapi" fn(*mut EfiUsbIoProtocol, u16, u8, *mut *mut Char16) -> EfiStatus,
    /// Get supported languages.
    pub get_supported_languages:
        unsafe extern "efiapi" fn(*mut EfiUsbIoProtocol, *mut *mut u16, *mut u16) -> EfiStatus,
    /// Resets the port.
    pub port_reset: unsafe extern "efiapi" fn(*mut EfiUsbIoProtocol) -> EfiStatus,
}

const _: () = {
    assert!(core::mem::size_of::<EfiUsbIoProtocol>() == 13 * core::mem::size_of::<usize>());
};

/// A typed USB I/O view.
pub struct UsbIo<'a> {
    protocol: &'a EfiUsbIoProtocol,
}

impl<'a> UsbIo<'a> {
    /// Creates a view.
    #[must_use]
    pub const fn new(protocol: &'a EfiUsbIoProtocol) -> Self {
        Self { protocol }
    }

    /// Returns the underlying protocol.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiUsbIoProtocol {
        self.protocol
    }

    /// Performs a control transfer.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// transfer fails.
    pub fn control_transfer(
        &self,
        request: &mut UsbDeviceRequest,
        direction: UsbDirection,
        timeout: u32,
        data: &mut [u8],
    ) -> Result<u32, EfiStatus> {
        let mut status: u32 = 0;
        // SAFETY: Request and data pointers are valid for the call.
        let result = unsafe {
            (self.protocol.control_transfer)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                request,
                direction,
                timeout,
                data.as_mut_ptr().cast(),
                data.len(),
                &raw mut status,
            )
        };
        if result.is_success() {
            Ok(status)
        } else {
            Err(result)
        }
    }

    /// Performs a bulk transfer.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// transfer fails.
    pub fn bulk_transfer(
        &self,
        endpoint: u8,
        data: &mut [u8],
        timeout: UIntN,
    ) -> Result<(UIntN, u32), EfiStatus> {
        let mut length = data.len();
        let mut status: u32 = 0;
        // SAFETY: The slice pointer is valid for `length` bytes.
        let result = unsafe {
            (self.protocol.bulk_transfer)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                endpoint,
                data.as_mut_ptr().cast(),
                &raw mut length,
                timeout,
                &raw mut status,
            )
        };
        if result.is_success() {
            Ok((length, status))
        } else {
            Err(result)
        }
    }

    /// Performs a sync interrupt transfer.
    ///
    /// # Errors
    ///
    /// Returns the firmware status when the
    /// transfer fails.
    pub fn sync_interrupt_transfer(
        &self,
        endpoint: u8,
        data: &mut [u8],
        timeout: UIntN,
    ) -> Result<(UIntN, u32), EfiStatus> {
        let mut length = data.len();
        let mut status: u32 = 0;
        // SAFETY: The slice pointer is valid for `length` bytes.
        let result = unsafe {
            (self.protocol.sync_interrupt_transfer)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                endpoint,
                data.as_mut_ptr().cast(),
                &raw mut length,
                timeout,
                &raw mut status,
            )
        };
        if result.is_success() {
            Ok((length, status))
        } else {
            Err(result)
        }
    }

    /// Resets the port.
    #[must_use]
    pub fn port_reset(&self) -> EfiStatus {
        // SAFETY: The borrowed protocol is valid for the call.
        unsafe { (self.protocol.port_reset)(core::ptr::from_ref(self.protocol).cast_mut()) }
    }

    /// Submits an async interrupt transfer.
    ///
    /// # Safety
    ///
    /// The callback must follow `EFIAPI`, the
    /// data buffer must remain alive until the
    /// callback, and the endpoint must be
    /// valid.
    #[must_use]
    pub unsafe fn async_interrupt_transfer(
        &self,
        endpoint: u8,
        is_new: crate::abi::types::Bool,
        polling_interval: UIntN,
        data_length: UIntN,
        callback: Option<EfiAsyncUsbTransferCallback>,
        context: *mut c_void,
    ) -> EfiStatus {
        // SAFETY: The caller guarantees callback and lifetime requirements.
        unsafe {
            (self.protocol.async_interrupt_transfer)(
                core::ptr::from_ref(self.protocol).cast_mut(),
                endpoint,
                is_new,
                polling_interval,
                data_length,
                callback,
                context,
            )
        }
    }
}

impl core::fmt::Debug for UsbIo<'_> {
    /// Formats without dereferencing firmware
    /// state.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("UsbIo")
            .field("protocol", &core::ptr::from_ref(self.protocol))
            .finish()
    }
}
