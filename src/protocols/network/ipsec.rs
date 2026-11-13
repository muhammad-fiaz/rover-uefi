//! # `IPsec` and `IPsec` Configuration Protocols
//!
//! Provides services for packet-level security (authentication, encryption, SPD/SAD/PAD configuration)
//! across IPv4 and IPv6 network stacks. Defined in UEFI Specification 2.2 through 2.5+.

use crate::abi::types::UIntN;
use crate::base::{EfiEvent, EfiGuid, EfiHandle, EfiStatus};
use core::ffi::c_void;

/// `IPsec` Protocol GUID: `{dfb386f7-e100-43ad-9c9a-ed90d08a5e12}`.
pub const EFI_IPSEC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xDFB3_86F7,
    0xE100,
    0x43AD,
    [0x9C, 0x9A, 0xED, 0x90, 0xD0, 0x8A, 0x5E, 0x12],
);

/// `IPsec2` Protocol GUID: `{a3979e64-ace8-4ddc-bc07-4d66b8fd0977}`.
pub const EFI_IPSEC2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xA397_9E64,
    0xACE8,
    0x4DDC,
    [0xBC, 0x07, 0x4D, 0x66, 0xB8, 0xFD, 0x09, 0x77],
);

/// `IPsec` Configuration Protocol GUID: `{ce5e5929-c7a3-4602-ad9e-c9daf94ebfcf}`.
pub const EFI_IPSEC_CONFIG_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xCE5E_5929,
    0xC7A3,
    0x4602,
    [0xAD, 0x9E, 0xC9, 0xDA, 0xF9, 0x4E, 0xBF, 0xCF],
);

/// Traffic direction in `IPsec` packet filtering and processing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiIpSecTrafficDir {
    /// Inbound network traffic.
    Inbound = 0,
    /// Outbound network traffic.
    Outbound = 1,
}

/// Action to apply to matching network packets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiIpSecAction {
    /// Discard and drop matching traffic.
    Discard = 0,
    /// Allow traffic to traverse the `IPsec` boundary without encryption or authentication.
    Bypass = 1,
    /// Protect traffic using negotiated `IPsec` security associations (AH / ESP).
    Protect = 2,
}

/// Configuration data categories for `IPsec` policies and keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiIpSecConfigDataType {
    /// Security Policy Database (SPD).
    Spd = 0,
    /// Security Association Database (SAD).
    Sad = 1,
    /// Peer Authorization Database (PAD).
    Pad = 2,
    /// Maximum configuration type index.
    Maximum = 3,
}

/// Fragment descriptor for IP packet buffers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiIpSecFragmentData {
    /// Length of this fragment in bytes.
    pub fragment_length: u32,
    /// Pointer to fragment payload buffer.
    pub fragment_buffer: *mut c_void,
}

/// IP address and network prefix info.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiIpAddressInfo {
    /// IPv4 (first 4 bytes) or IPv6 address.
    pub address: [u8; 16],
    /// Prefix length in bits.
    pub prefix_length: u8,
}

/// Security Policy Database (SPD) selector definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiIpSecSpdSelector {
    /// Number of local address entries.
    pub local_address_count: u32,
    /// Pointer to local address ranges.
    pub local_address: *mut EfiIpAddressInfo,
    /// Number of remote peer address entries.
    pub remote_address_count: u32,
    /// Pointer to remote peer address ranges.
    pub remote_address: *mut EfiIpAddressInfo,
    /// Next layer protocol (TCP=6, UDP=17, 0=wildcard).
    pub next_layer_protocol: u16,
    /// Local port number (0=wildcard).
    pub local_port: u16,
    /// Local port range size.
    pub local_port_range: u16,
    /// Remote port number (0=wildcard).
    pub remote_port: u16,
    /// Remote port range size.
    pub remote_port_range: u16,
}

/// Security Association (SA) lifetime constraints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiIpSecSaLifetime {
    /// Maximum byte count before key refresh.
    pub byte_count: u64,
    /// Soft lifetime in seconds.
    pub soft_lifetime: u64,
    /// Hard lifetime in seconds before termination.
    pub hard_lifetime: u64,
}

/// Function pointer for `EFI_IPSEC_PROCESS`.
pub type EfiIpSecProcess = unsafe extern "efiapi" fn(
    this: *mut EfiIpSecProtocol,
    nic_handle: EfiHandle,
    ip_ver: u8,
    ip_head: *mut c_void,
    last_head: *mut u8,
    options_buffer: *mut c_void,
    options_length: u32,
    fragment_table: *mut *mut EfiIpSecFragmentData,
    fragment_count: *mut u32,
    traffic_direction: EfiIpSecTrafficDir,
    recycle_signal: *mut EfiEvent,
) -> EfiStatus;

/// Raw ABI structure for `EFI_IPSEC_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiIpSecProtocol {
    /// Processes inbound and outbound IP packets.
    pub process: EfiIpSecProcess,
    /// Event signaled when the `IPsec` interface is disabled.
    pub disabled_event: EfiEvent,
    /// Flag indicating whether `IPsec` is currently disabled.
    pub disabled_flag: bool,
}

/// Function pointer for `EFI_IPSEC_PROCESSEXT`.
pub type EfiIpSecProcessExt = unsafe extern "efiapi" fn(
    this: *mut EfiIpSec2Protocol,
    nic_handle: EfiHandle,
    ip_ver: u8,
    ip_head: *mut c_void,
    last_head: *mut u8,
    options_buffer: *mut *mut c_void,
    options_length: *mut u32,
    fragment_table: *mut *mut EfiIpSecFragmentData,
    fragment_count: *mut u32,
    traffic_direction: EfiIpSecTrafficDir,
    recycle_signal: *mut EfiEvent,
) -> EfiStatus;

/// Raw ABI structure for `EFI_IPSEC2_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiIpSec2Protocol {
    /// Extended `IPsec` packet processor supporting extension headers and tunnel mode.
    pub process_ext: EfiIpSecProcessExt,
    /// Event signaled when the interface is disabled.
    pub disabled_event: EfiEvent,
    /// Flag indicating whether `IPsec2` is disabled.
    pub disabled_flag: bool,
}

/// Function pointer for `EFI_IPSEC_CONFIG_SET_DATA`.
pub type EfiIpSecConfigSetData = unsafe extern "efiapi" fn(
    this: *mut EfiIpSecConfigProtocol,
    data_type: EfiIpSecConfigDataType,
    selector: *const c_void,
    data: *const c_void,
    selector_for_insert: *const c_void,
) -> EfiStatus;

/// Function pointer for `EFI_IPSEC_CONFIG_GET_DATA`.
pub type EfiIpSecConfigGetData = unsafe extern "efiapi" fn(
    this: *mut EfiIpSecConfigProtocol,
    data_type: EfiIpSecConfigDataType,
    selector: *const c_void,
    data_size: *mut UIntN,
    data: *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_IPSEC_CONFIG_GET_NEXT_SELECTOR`.
pub type EfiIpSecConfigGetNextSelector = unsafe extern "efiapi" fn(
    this: *mut EfiIpSecConfigProtocol,
    data_type: EfiIpSecConfigDataType,
    selector_size: *mut UIntN,
    selector: *mut c_void,
) -> EfiStatus;

/// Function pointer for `EFI_IPSEC_CONFIG_REGISTER_NOTIFY`.
pub type EfiIpSecConfigRegisterNotify = unsafe extern "efiapi" fn(
    this: *mut EfiIpSecConfigProtocol,
    data_type: EfiIpSecConfigDataType,
    event: EfiEvent,
) -> EfiStatus;

/// Function pointer for `EFI_IPSEC_CONFIG_UNREGISTER_NOTIFY`.
pub type EfiIpSecConfigUnregisterNotify = unsafe extern "efiapi" fn(
    this: *mut EfiIpSecConfigProtocol,
    data_type: EfiIpSecConfigDataType,
    event: EfiEvent,
) -> EfiStatus;

/// Raw ABI structure for `EFI_IPSEC_CONFIG_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiIpSecConfigProtocol {
    /// Sets an entry in the specified `IPsec` database (SPD, SAD, or PAD).
    pub set_data: EfiIpSecConfigSetData,
    /// Retrieves an entry from the specified database.
    pub get_data: EfiIpSecConfigGetData,
    /// Enumerates selectors sequentially.
    pub get_next_selector: EfiIpSecConfigGetNextSelector,
    /// Registers a notification event when policy configuration is updated.
    pub register_data_notify: EfiIpSecConfigRegisterNotify,
    /// Unregisters a policy notification event.
    pub unregister_data_notify: EfiIpSecConfigUnregisterNotify,
}

/// Safe client wrapper around [`EfiIpSecProtocol`].
pub struct IpSec<'a> {
    proto: &'a mut EfiIpSecProtocol,
}

impl<'a> IpSec<'a> {
    /// Creates a new `IpSec` client wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiIpSecProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiIpSecProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Returns whether the `IPsec` interface is disabled.
    #[must_use]
    pub fn is_disabled(&self) -> bool {
        self.proto.disabled_flag
    }
}

/// Safe client wrapper around [`EfiIpSec2Protocol`].
pub struct IpSec2<'a> {
    proto: &'a mut EfiIpSec2Protocol,
}

impl<'a> IpSec2<'a> {
    /// Creates a new `IpSec2` client wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiIpSec2Protocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiIpSec2Protocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Returns whether the `IPsec2` interface is disabled.
    #[must_use]
    pub fn is_disabled(&self) -> bool {
        self.proto.disabled_flag
    }
}

/// Safe client wrapper around [`EfiIpSecConfigProtocol`].
pub struct IpSecConfig<'a> {
    proto: &'a mut EfiIpSecConfigProtocol,
}

impl<'a> IpSecConfig<'a> {
    /// Creates a new `IpSecConfig` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiIpSecConfigProtocol) -> Self {
        Self { proto }
    }

    /// Returns a raw pointer to the protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiIpSecConfigProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Retrieves an entry from the specified `IPsec` database into `buffer`.
    ///
    /// # Safety
    ///
    /// `selector` must point to a valid selector matching `data_type`.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if buffer is too small, or an error status.
    pub unsafe fn get_data_raw(
        &mut self,
        data_type: EfiIpSecConfigDataType,
        selector: *const c_void,
        buffer: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut size = buffer.len() as UIntN;
        let status = unsafe {
            (self.proto.get_data)(
                self.proto,
                data_type,
                selector,
                core::ptr::from_mut(&mut size),
                buffer.as_mut_ptr().cast::<c_void>(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(size as usize)
        } else {
            Err(status)
        }
    }

    /// Registers a notification event when policy configuration is updated.
    ///
    /// # Errors
    ///
    /// Returns an error status if registration fails.
    pub fn register_notify(
        &mut self,
        data_type: EfiIpSecConfigDataType,
        event: EfiEvent,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.register_data_notify)(self.proto, data_type, event) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unregisters a policy notification event.
    ///
    /// # Errors
    ///
    /// Returns an error status if unregistration fails.
    pub fn unregister_notify(
        &mut self,
        data_type: EfiIpSecConfigDataType,
        event: EfiEvent,
    ) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.unregister_data_notify)(self.proto, data_type, event) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
