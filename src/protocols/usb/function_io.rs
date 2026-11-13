//! # USB Function I/O Protocol
//!
//! Provides an I/O abstraction for a USB Controller operating in Function / Device / Peripheral mode,
//! enabling data transactions and controller management between the device and the host.
//! Standard UEFI 2.5+ protocol used for USB Fastboot, recovery flashing, DFU, and peripheral communications.

use crate::abi::types::{Bool, EFI_FALSE, EFI_TRUE, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::usb::types::UsbDeviceRequest;
use core::ffi::c_void;

/// The USB Function I/O Protocol `GUID`.
pub const USBFN_IO_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x32D2_963A,
    0xFE5D,
    0x4F30,
    [0xB6, 0x33, 0x6E, 0x5D, 0xC5, 0x58, 0x03, 0xCC],
);

/// Current revision of the USB Function I/O Protocol (`0x00010001`).
pub const USBFN_IO_PROTOCOL_REVISION: u32 = 0x0001_0001;

/// The type of attached USB port.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbFnPortType {
    /// Port type is unknown.
    Unknown = 0,
    /// Standard downstream port (SDP).
    StandardDownstreamPort = 1,
    /// Charging downstream port (CDP).
    ChargingDownstreamPort = 2,
    /// Dedicated charging port (DCP).
    DedicatedChargingPort = 3,
    /// Invalid dedicated charging port.
    InvalidDedicatedChargingPort = 4,
}

/// USB Endpoint type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbFnEndpointType {
    /// Control endpoint.
    Control = 0,
    /// Bulk endpoint.
    Bulk = 2,
}

/// Device information string identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbFnDeviceInfoId {
    /// Unknown device information ID.
    Unknown = 0,
    /// Device serial number string.
    SerialNumber = 1,
    /// Manufacturer name string.
    ManufacturerName = 2,
    /// Product name string.
    ProductName = 3,
}

/// Transfer direction of a USB function endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbFnEndpointDirection {
    /// Host to device transfer (Device receive / Out).
    HostOut = 0,
    /// Device to host transfer (Device transmit / In).
    HostIn = 1,
}

/// Transfer completion status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbFnTransferStatus {
    /// Status unknown.
    Unknown = 0,
    /// Transfer completed successfully.
    Complete = 1,
    /// Transfer was aborted.
    Aborted = 2,
    /// Transfer is active / ongoing.
    Active = 3,
    /// No transfer has been queued.
    None = 4,
}

/// Standard USB Interface descriptor layout for function mode configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct UsbInterfaceDescriptor {
    /// Length of this descriptor in bytes.
    pub length: u8,
    /// Descriptor type (`0x04`).
    pub descriptor_type: u8,
    /// Number of this interface.
    pub interface_number: u8,
    /// Value used to select this alternate setting.
    pub alternate_setting: u8,
    /// Number of endpoints used by this interface.
    pub num_endpoints: u8,
    /// Class code.
    pub interface_class: u8,
    /// Subclass code.
    pub interface_sub_class: u8,
    /// Protocol code.
    pub interface_protocol: u8,
    /// Index of string descriptor describing this interface.
    pub interface: u8,
}

/// Standard USB Endpoint descriptor layout for function mode configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct UsbEndpointDescriptor {
    /// Length of this descriptor in bytes.
    pub length: u8,
    /// Descriptor type (`0x05`).
    pub descriptor_type: u8,
    /// Address of endpoint.
    pub endpoint_address: u8,
    /// Endpoint attributes.
    pub attributes: u8,
    /// Maximum packet size.
    pub max_packet_size: u16,
    /// Polling interval.
    pub interval: u8,
}

/// Standard USB Configuration descriptor layout for function mode configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct UsbConfigDescriptor {
    /// Length of this descriptor in bytes.
    pub length: u8,
    /// Descriptor type (`0x02`).
    pub descriptor_type: u8,
    /// Total length of data returned for this configuration.
    pub total_length: u16,
    /// Number of interfaces supported by this configuration.
    pub num_interfaces: u8,
    /// Value to use as an argument to Set Configuration to select this configuration.
    pub configuration_value: u8,
    /// Index of string descriptor describing this configuration.
    pub configuration: u8,
    /// Configuration characteristics.
    pub attributes: u8,
    /// Maximum power consumption in 2 mA units.
    pub max_power: u8,
}

/// Standard USB Device descriptor layout for function mode configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct UsbDeviceDescriptor {
    /// Length of this descriptor in bytes.
    pub length: u8,
    /// Descriptor type (`0x01`).
    pub descriptor_type: u8,
    /// USB Specification Release Number in BCD.
    pub bcd_usb: u16,
    /// Class code.
    pub device_class: u8,
    /// Subclass code.
    pub device_sub_class: u8,
    /// Protocol code.
    pub device_protocol: u8,
    /// Maximum packet size for endpoint zero.
    pub max_packet_size0: u8,
    /// Vendor ID.
    pub id_vendor: u16,
    /// Product ID.
    pub id_product: u16,
    /// Device release number in BCD.
    pub bcd_device: u16,
    /// Index of manufacturer string descriptor.
    pub manufacturer: u8,
    /// Index of product string descriptor.
    pub product: u8,
    /// Index of serial number string descriptor.
    pub serial_number: u8,
    /// Number of possible configurations.
    pub num_configurations: u8,
}

/// Interface information containing descriptors and endpoint table.
#[derive(Debug)]
#[repr(C)]
pub struct UsbInterfaceInfo {
    /// Interface descriptor pointer.
    pub interface_descriptor: *mut UsbInterfaceDescriptor,
    /// Table of pointers to endpoint descriptors.
    pub endpoint_descriptor_table: *mut *mut UsbEndpointDescriptor,
}

/// Configuration information containing descriptors and interface table.
#[derive(Debug)]
#[repr(C)]
pub struct UsbConfigInfo {
    /// Configuration descriptor pointer.
    pub config_descriptor: *mut UsbConfigDescriptor,
    /// Table of pointers to interface info structures.
    pub interface_info_table: *mut *mut UsbInterfaceInfo,
}

/// Device information hierarchy for configuring function endpoints.
#[derive(Debug)]
#[repr(C)]
pub struct UsbDeviceInfo {
    /// Device descriptor pointer.
    pub device_descriptor: *mut UsbDeviceDescriptor,
    /// Table of pointers to configuration info structures.
    pub config_info_table: *mut *mut UsbConfigInfo,
}

/// Details of a completed or updated transfer.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct UsbFnTransferResult {
    /// Number of bytes transferred.
    pub bytes_transferred: UIntN,
    /// Transfer completion status.
    pub transfer_status: UsbFnTransferStatus,
    /// Endpoint index.
    pub endpoint_index: u8,
    /// Endpoint transfer direction.
    pub direction: UsbFnEndpointDirection,
    /// Associated transfer buffer.
    pub buffer: *mut c_void,
}

/// USB Bus Speed in function mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbFnBusSpeed {
    /// Unknown bus speed.
    Unknown = 0,
    /// Low speed (1.5 Mbps).
    Low = 1,
    /// Full speed (12 Mbps).
    Full = 2,
    /// High speed (480 Mbps).
    High = 3,
    /// Super speed (5 Gbps).
    Super = 4,
}

/// Notification event message generated by the USB controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbFnMessage {
    /// No event.
    None = 0,
    /// SETUP packet received on endpoint 0.
    SetupPacket = 1,
    /// RX transfer completed or updated on an endpoint.
    EndpointStatusChangedRx = 2,
    /// TX transfer completed or updated on an endpoint.
    EndpointStatusChangedTx = 3,
    /// Bus detach event detected.
    BusEventDetach = 4,
    /// Bus attach event detected.
    BusEventAttach = 5,
    /// Bus reset event detected.
    BusEventReset = 6,
    /// Bus suspend event detected.
    BusEventSuspend = 7,
    /// Bus resume event detected.
    BusEventResume = 8,
    /// Bus operating speed changed.
    BusEventSpeed = 9,
}

/// Payload associated with a USB Function notification message.
#[derive(Copy, Clone)]
#[repr(C)]
pub union UsbFnMessagePayload {
    /// Setup packet device request (when message is `SetupPacket`).
    pub udr: UsbDeviceRequest,
    /// Transfer result details (when message is `EndpointStatusChangedRx` or `EndpointStatusChangedTx`).
    pub utr: UsbFnTransferResult,
    /// Bus speed information (when message is `BusEventSpeed`).
    pub ubs: UsbFnBusSpeed,
}

/// Policy types for non-control endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum UsbFnPolicyType {
    /// Undefined policy.
    Undefined = 0,
    /// Maximum transaction size.
    MaxTransactionSize = 1,
    /// Whether zero-length termination is supported.
    ZeroLengthTerminationSupport = 2,
    /// Zero-length termination behavior.
    ZeroLengthTermination = 3,
}

/// EFI USB Function I/O Protocol structure layout.
#[derive(Debug)]
#[repr(C)]
pub struct EfiUsbFnIoProtocol {
    /// Protocol revision.
    pub revision: u32,
    /// Detects attached port type.
    pub detect_port:
        unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol, *mut UsbFnPortType) -> EfiStatus,
    /// Configures and enables endpoints using descriptor tables.
    pub configure_enable_endpoints:
        unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol, *mut UsbDeviceInfo) -> EfiStatus,
    /// Retrieves maximum packet size for specified endpoint type and speed.
    pub get_endpoint_max_packet_size: unsafe extern "efiapi" fn(
        *mut EfiUsbFnIoProtocol,
        UsbFnEndpointType,
        UsbFnBusSpeed,
        *mut u16,
    ) -> EfiStatus,
    /// Retrieves device info Unicode string.
    pub get_device_info: unsafe extern "efiapi" fn(
        *mut EfiUsbFnIoProtocol,
        UsbFnDeviceInfoId,
        *mut UIntN,
        *mut c_void,
    ) -> EfiStatus,
    /// Retrieves device Vendor ID and Product ID.
    pub get_vendor_id_product_id:
        unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol, *mut u16, *mut u16) -> EfiStatus,
    /// Aborts an active transfer on an endpoint.
    pub abort_transfer:
        unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol, u8, UsbFnEndpointDirection) -> EfiStatus,
    /// Gets endpoint stall state.
    pub get_endpoint_stall_state: unsafe extern "efiapi" fn(
        *mut EfiUsbFnIoProtocol,
        u8,
        UsbFnEndpointDirection,
        *mut Bool,
    ) -> EfiStatus,
    /// Sets endpoint stall state.
    pub set_endpoint_stall_state: unsafe extern "efiapi" fn(
        *mut EfiUsbFnIoProtocol,
        u8,
        UsbFnEndpointDirection,
        *mut Bool,
    ) -> EfiStatus,
    /// Polls or handles USB controller events.
    pub event_handler: unsafe extern "efiapi" fn(
        *mut EfiUsbFnIoProtocol,
        *mut UsbFnMessage,
        *mut UIntN,
        *mut UsbFnMessagePayload,
    ) -> EfiStatus,
    /// Submits or receives data transfer.
    pub transfer: unsafe extern "efiapi" fn(
        *mut EfiUsbFnIoProtocol,
        u8,
        UsbFnEndpointDirection,
        *mut UIntN,
        *mut c_void,
    ) -> EfiStatus,
    /// Returns maximum single transfer size in bytes.
    pub get_max_transfer_size:
        unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol, *mut UIntN) -> EfiStatus,
    /// Allocates controller-aligned transfer buffer.
    pub allocate_transfer_buffer:
        unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol, UIntN, *mut *mut c_void) -> EfiStatus,
    /// Frees controller-aligned transfer buffer.
    pub free_transfer_buffer:
        unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol, *mut c_void) -> EfiStatus,
    /// Powers and initializes the USB function controller.
    pub start_controller: unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol) -> EfiStatus,
    /// Stops the USB function hardware.
    pub stop_controller: unsafe extern "efiapi" fn(*mut EfiUsbFnIoProtocol) -> EfiStatus,
    /// Sets configuration policy for non-control endpoint.
    pub set_endpoint_policy: unsafe extern "efiapi" fn(
        *mut EfiUsbFnIoProtocol,
        u8,
        UsbFnEndpointDirection,
        UsbFnPolicyType,
        UIntN,
        *mut c_void,
    ) -> EfiStatus,
    /// Retrieves configuration policy for non-control endpoint.
    pub get_endpoint_policy: unsafe extern "efiapi" fn(
        *mut EfiUsbFnIoProtocol,
        u8,
        UsbFnEndpointDirection,
        UsbFnPolicyType,
        *mut UIntN,
        *mut c_void,
    ) -> EfiStatus,
}

/// Safe idiomatic Rust client wrapper for the USB Function I/O Protocol.
#[derive(Debug)]
pub struct UsbFnIo<'a> {
    protocol: &'a mut EfiUsbFnIoProtocol,
}

impl<'a> UsbFnIo<'a> {
    /// USB Function I/O Protocol GUID.
    pub const GUID: EfiGuid = USBFN_IO_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a mutable reference to the raw protocol.
    #[must_use]
    pub fn new(protocol: &'a mut EfiUsbFnIoProtocol) -> Self {
        Self { protocol }
    }

    /// Detects what type of USB port is attached.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` or other hardware failure codes on error.
    pub fn detect_port(&mut self) -> Result<UsbFnPortType, EfiStatus> {
        let mut port_type = UsbFnPortType::Unknown;
        let status = unsafe { (self.protocol.detect_port)(self.protocol, &raw mut port_type) };
        if status.is_success() {
            Ok(port_type)
        } else {
            Err(status)
        }
    }

    /// Configures endpoints based on supplied device and configuration descriptors.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if descriptor structures are invalid.
    pub fn configure_enable_endpoints(
        &mut self,
        info: &mut UsbDeviceInfo,
    ) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.protocol.configure_enable_endpoints)(self.protocol, &raw mut *info) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the maximum packet size of the specified endpoint type for the given bus speed.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if endpoint type or bus speed is unsupported.
    pub fn get_endpoint_max_packet_size(
        &mut self,
        endpoint_type: UsbFnEndpointType,
        bus_speed: UsbFnBusSpeed,
    ) -> Result<u16, EfiStatus> {
        let mut size: u16 = 0;
        let status = unsafe {
            (self.protocol.get_endpoint_max_packet_size)(
                self.protocol,
                endpoint_type,
                bus_speed,
                &raw mut size,
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Returns the Vendor ID and Product ID of the device.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if IDs cannot be retrieved.
    pub fn get_vendor_id_product_id(&mut self) -> Result<(u16, u16), EfiStatus> {
        let mut vid: u16 = 0;
        let mut pid: u16 = 0;
        let status = unsafe {
            (self.protocol.get_vendor_id_product_id)(self.protocol, &raw mut vid, &raw mut pid)
        };
        if status.is_success() {
            Ok((vid, pid))
        } else {
            Err(status)
        }
    }

    /// Aborts any pending transfer on the specified endpoint.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if endpoint or direction is invalid.
    pub fn abort_transfer(
        &mut self,
        endpoint: u8,
        direction: UsbFnEndpointDirection,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.abort_transfer)(self.protocol, endpoint, direction) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries whether the specified endpoint is in a stalled state.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if endpoint or direction is invalid.
    pub fn get_endpoint_stall_state(
        &mut self,
        endpoint: u8,
        direction: UsbFnEndpointDirection,
    ) -> Result<bool, EfiStatus> {
        let mut state = EFI_FALSE;
        let status = unsafe {
            (self.protocol.get_endpoint_stall_state)(
                self.protocol,
                endpoint,
                direction,
                &raw mut state,
            )
        };
        if status.is_success() {
            Ok(state != EFI_FALSE)
        } else {
            Err(status)
        }
    }

    /// Sets or clears the stall state on the specified endpoint.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if endpoint or direction is invalid.
    pub fn set_endpoint_stall_state(
        &mut self,
        endpoint: u8,
        direction: UsbFnEndpointDirection,
        stall: bool,
    ) -> Result<(), EfiStatus> {
        let mut state = if stall { EFI_TRUE } else { EFI_FALSE };
        let status = unsafe {
            (self.protocol.set_endpoint_stall_state)(
                self.protocol,
                endpoint,
                direction,
                &raw mut state,
            )
        };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Polls for incoming events, setup packets, bus state changes, or transfer status changes.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if the supplied payload buffer is insufficient.
    pub fn event_handler(
        &mut self,
        payload: &mut UsbFnMessagePayload,
    ) -> Result<UsbFnMessage, EfiStatus> {
        let mut msg = UsbFnMessage::None;
        let mut size = core::mem::size_of::<UsbFnMessagePayload>();
        let status = unsafe {
            (self.protocol.event_handler)(
                self.protocol,
                &raw mut msg,
                &raw mut size,
                &raw mut *payload,
            )
        };
        if status.is_success() {
            Ok(msg)
        } else {
            Err(status)
        }
    }

    /// Submits a transfer on the specified endpoint.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if parameters or direction are invalid.
    pub fn transfer(
        &mut self,
        endpoint: u8,
        direction: UsbFnEndpointDirection,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut size = buffer.len();
        let status = unsafe {
            (self.protocol.transfer)(
                self.protocol,
                endpoint,
                direction,
                &raw mut size,
                buffer.as_mut_ptr().cast::<c_void>(),
            )
        };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Returns the maximum transfer size supported by the underlying controller.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` on controller failure.
    pub fn get_max_transfer_size(&mut self) -> Result<usize, EfiStatus> {
        let mut size: usize = 0;
        let status = unsafe { (self.protocol.get_max_transfer_size)(self.protocol, &raw mut size) };
        if status.is_success() {
            Ok(size)
        } else {
            Err(status)
        }
    }

    /// Allocates a transfer buffer meeting controller alignment requirements.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    pub fn allocate_transfer_buffer(&mut self, size: usize) -> Result<*mut c_void, EfiStatus> {
        let mut ptr = core::ptr::null_mut();
        let status =
            unsafe { (self.protocol.allocate_transfer_buffer)(self.protocol, size, &raw mut ptr) };
        if status.is_success() {
            Ok(ptr)
        } else {
            Err(status)
        }
    }

    /// Frees a transfer buffer previously allocated with `allocate_transfer_buffer`.
    ///
    /// # Safety
    ///
    /// `buffer` must have been returned by a previous call to `allocate_transfer_buffer`.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if buffer address is invalid.
    pub unsafe fn free_transfer_buffer(&mut self, buffer: *mut c_void) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.free_transfer_buffer)(self.protocol, buffer) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Initializes and starts the USB function controller.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` if the controller cannot be started.
    pub fn start_controller(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.start_controller)(self.protocol) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Stops the USB function controller.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::DEVICE_ERROR` if the controller cannot be stopped.
    pub fn stop_controller(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.stop_controller)(self.protocol) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
