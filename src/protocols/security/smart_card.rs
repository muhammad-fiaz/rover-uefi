//! # Smart Card Reader and Smart Card Edge Protocols
//!
//! Provides PC/SC workgroup compliant smart card reader abstraction and UEFI 2.5
//! Smart Card Edge services for authentication, PIN verification, credential access,
//! and PKI cryptographic operations.

use crate::abi::types::{Char16, UIntN};
use crate::base::{EfiGuid, EfiHandle, EfiStatus};

/// The `EFI_SMART_CARD_READER_PROTOCOL_GUID`.
pub const EFI_SMART_CARD_READER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x2A4D_1ADF,
    0x21DC,
    0x4B81,
    [0xA4, 0x2F, 0x8B, 0x8E, 0xE2, 0x38, 0x00, 0x60],
);

/// The `EFI_SMART_CARD_EDGE_PROTOCOL_GUID`.
pub const EFI_SMART_CARD_EDGE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xD317_F29B,
    0xA325,
    0x4712,
    [0x9B, 0xF1, 0xC6, 0x19, 0x54, 0xDC, 0x19, 0x8C],
);

/// Smart Card reader access mode: exclusive access to reader.
pub const SCARD_AM_READER: u32 = 0x0001;
/// Smart Card reader access mode: exclusive access to card.
pub const SCARD_AM_CARD: u32 = 0x0002;

/// Smart Card card action: do not reset card.
pub const SCARD_CA_NORESET: u32 = 0x0000;
/// Smart Card card action: perform a cold reset.
pub const SCARD_CA_COLDRESET: u32 = 0x0001;
/// Smart Card card action: perform a warm reset.
pub const SCARD_CA_WARMRESET: u32 = 0x0002;
/// Smart Card card action: power off the card.
pub const SCARD_CA_UNPOWER: u32 = 0x0003;
/// Smart Card card action: eject the card.
pub const SCARD_CA_EJECT: u32 = 0x0004;

/// Smart Card protocol undefined.
pub const SCARD_PROTOCOL_UNDEFINED: u32 = 0x0000;
/// Smart Card protocol T=0 (character oriented).
pub const SCARD_PROTOCOL_T0: u32 = 0x0001;
/// Smart Card protocol T=1 (block oriented).
pub const SCARD_PROTOCOL_T1: u32 = 0x0002;
/// Smart Card protocol RAW.
pub const SCARD_PROTOCOL_RAW: u32 = 0x0004;

/// Smart Card reader state: state is unknown.
pub const SCARD_UNKNOWN: u32 = 0x0000;
/// Smart Card reader state: card is absent.
pub const SCARD_ABSENT: u32 = 0x0001;
/// Smart Card reader state: card is present but not powered.
pub const SCARD_INACTIVE: u32 = 0x0002;
/// Smart Card reader state: card is present and powered.
pub const SCARD_ACTIVE: u32 = 0x0003;

/// Maximum size for a Smart Card Application Identifier (AID).
pub const SCARD_AID_MAXSIZE: usize = 16;
/// Size of Card Serial Number (CSN) in bytes.
pub const SCARD_CSN_SIZE: usize = 16;
/// Smart Card Edge Protocol specification version 1.0.
pub const SMART_CARD_EDGE_PROTOCOL_VERSION_1: u32 = 0x0000_0100;

/// Smart Card Edge TLV tag for header.
pub const SC_EDGE_TAG_HEADER: u16 = 0x0000;
/// Smart Card Edge TLV tag for certificate.
pub const SC_EDGE_TAG_CERT: u16 = 0x0001;
/// Smart Card Edge TLV tag for key index.
pub const SC_EDGE_TAG_KEY_ID: u16 = 0x0002;
/// Smart Card Edge TLV tag for key type.
pub const SC_EDGE_TAG_KEY_TYPE: u16 = 0x0003;
/// Smart Card Edge TLV tag for key size.
pub const SC_EDGE_TAG_KEY_SIZE: u16 = 0x0004;

// ---------------------------------------------------------------------------
// Smart Card Reader Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_SMART_CARD_READER_CONNECT`.
pub type EfiSmartCardReaderConnect = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardReaderProtocol,
    access_mode: u32,
    card_action: u32,
    preferred_protocols: u32,
    active_protocol: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_READER_DISCONNECT`.
pub type EfiSmartCardReaderDisconnect = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardReaderProtocol,
    card_action: u32,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_READER_STATUS`.
pub type EfiSmartCardReaderStatus = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardReaderProtocol,
    reader_name: *mut Char16,
    reader_name_length: *mut UIntN,
    state: *mut u32,
    card_protocol: *mut u32,
    atr: *mut u8,
    atr_length: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_READER_TRANSMIT`.
pub type EfiSmartCardReaderTransmit = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardReaderProtocol,
    capdu: *const u8,
    capdu_length: UIntN,
    rapdu: *mut u8,
    rapdu_length: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_READER_CONTROL`.
pub type EfiSmartCardReaderControl = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardReaderProtocol,
    control_code: u32,
    in_buffer: *const u8,
    in_buffer_length: UIntN,
    out_buffer: *mut u8,
    out_buffer_length: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_READER_GET_ATTRIB`.
pub type EfiSmartCardReaderGetAttrib = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardReaderProtocol,
    attrib: u32,
    out_buffer: *mut u8,
    out_buffer_length: *mut UIntN,
) -> EfiStatus;

/// Raw ABI structure for Smart Card Reader Protocol (`EFI_SMART_CARD_READER_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiSmartCardReaderProtocol {
    /// Requests connection to smart card or reader.
    pub scard_connect: EfiSmartCardReaderConnect,
    /// Releases previously acquired reader connection.
    pub scard_disconnect: EfiSmartCardReaderDisconnect,
    /// Retrieves status, ATR, and reader name.
    pub scard_status: EfiSmartCardReaderStatus,
    /// Transmits Command APDU and receives Response APDU.
    pub scard_transmit: EfiSmartCardReaderTransmit,
    /// Sends direct control commands to reader driver.
    pub scard_control: EfiSmartCardReaderControl,
    /// Retrieves reader or card attributes.
    pub scard_get_attrib: EfiSmartCardReaderGetAttrib,
}

/// Safe wrapper around [`EfiSmartCardReaderProtocol`].
pub struct SmartCardReader<'a> {
    proto: &'a EfiSmartCardReaderProtocol,
}

impl<'a> SmartCardReader<'a> {
    /// Wraps a reference to [`EfiSmartCardReaderProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiSmartCardReaderProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiSmartCardReaderProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiSmartCardReaderProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Connects to the smart card or reader.
    ///
    /// Returns the negotiated active protocol.
    ///
    /// # Errors
    ///
    /// Returns an error status if connection negotiation fails.
    pub fn connect(
        &self,
        access_mode: u32,
        card_action: u32,
        preferred_protocols: u32,
    ) -> Result<u32, EfiStatus> {
        let mut active_proto = 0u32;
        let status = unsafe {
            (self.proto.scard_connect)(
                self.proto,
                access_mode,
                card_action,
                preferred_protocols,
                core::ptr::from_mut(&mut active_proto),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(active_proto)
        } else {
            Err(status)
        }
    }

    /// Disconnects from the smart card reader.
    ///
    /// # Errors
    ///
    /// Returns an error status if disconnect fails.
    pub fn disconnect(&self, card_action: u32) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.scard_disconnect)(self.proto, card_action) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Transmits a Command APDU and receives a Response APDU.
    ///
    /// Returns the number of bytes written into `rapdu`.
    ///
    /// # Errors
    ///
    /// Returns an error status if transmission or card communication fails.
    pub fn transmit(&self, capdu: &[u8], rapdu: &mut [u8]) -> Result<usize, EfiStatus> {
        let mut rapdu_len = rapdu.len();
        let status = unsafe {
            (self.proto.scard_transmit)(
                self.proto,
                capdu.as_ptr(),
                capdu.len(),
                rapdu.as_mut_ptr(),
                core::ptr::from_mut(&mut rapdu_len),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(rapdu_len)
        } else {
            Err(status)
        }
    }

    /// Sends a direct driver control code.
    ///
    /// Returns the number of bytes written into `out_buffer` if provided.
    ///
    /// # Errors
    ///
    /// Returns an error status if control operation fails.
    pub fn control(
        &self,
        control_code: u32,
        in_buffer: Option<&[u8]>,
        out_buffer: Option<&mut [u8]>,
    ) -> Result<usize, EfiStatus> {
        let in_ptr = in_buffer.map_or(core::ptr::null(), <[u8]>::as_ptr);
        let in_len = in_buffer.map_or(0, <[u8]>::len);
        let mut out_len = out_buffer.as_ref().map_or(0, |b| b.len());
        let out_ptr = out_buffer.map_or(core::ptr::null_mut(), <[u8]>::as_mut_ptr);

        let status = unsafe {
            (self.proto.scard_control)(
                self.proto,
                control_code,
                in_ptr,
                in_len,
                out_ptr,
                core::ptr::from_mut(&mut out_len),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(out_len)
        } else {
            Err(status)
        }
    }

    /// Retrieves an attribute from the smart card or reader.
    ///
    /// Returns the number of bytes written into `out_buffer`.
    ///
    /// # Errors
    ///
    /// Returns an error status if attribute retrieval fails.
    pub fn get_attrib(&self, attrib: u32, out_buffer: &mut [u8]) -> Result<usize, EfiStatus> {
        let mut out_len = out_buffer.len();
        let status = unsafe {
            (self.proto.scard_get_attrib)(
                self.proto,
                attrib,
                out_buffer.as_mut_ptr(),
                core::ptr::from_mut(&mut out_len),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(out_len)
        } else {
            Err(status)
        }
    }
}

// ---------------------------------------------------------------------------
// Smart Card Edge Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer for `EFI_SMART_CARD_EDGE_GET_CONTEXT`.
pub type EfiSmartCardEdgeGetContext = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_context: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_CONNECT`.
pub type EfiSmartCardEdgeConnect = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    reader_name: *const Char16,
    scard_handle: *mut EfiHandle,
    active_protocol: *mut u32,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_DISCONNECT`.
pub type EfiSmartCardEdgeDisconnect = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    card_action: u32,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_GET_CSN`.
pub type EfiSmartCardEdgeGetCsn = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    csn: *mut [u8; SCARD_CSN_SIZE],
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_GET_READER_NAME`.
pub type EfiSmartCardEdgeGetReaderName = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    reader_name: *mut Char16,
    reader_name_length: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_VERIFY_PIN`.
pub type EfiSmartCardEdgeVerifyPin = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    pin: *const u8,
    pin_length: UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_GET_PIN_REMAINING`.
pub type EfiSmartCardEdgeGetPinRemaining = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    remaining: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_GET_DATA`.
pub type EfiSmartCardEdgeGetData = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    tag: u16,
    data: *mut u8,
    data_length: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_GET_CREDENTIAL`.
pub type EfiSmartCardEdgeGetCredential = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    cred_list: *mut *mut u8,
    cred_list_length: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_SIGN_DATA`.
pub type EfiSmartCardEdgeSignData = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    key_id: UIntN,
    data: *const u8,
    data_length: UIntN,
    signature: *mut u8,
    signature_length: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_DECRYPT_DATA`.
pub type EfiSmartCardEdgeDecryptData = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    key_id: UIntN,
    cipher: *const u8,
    cipher_length: UIntN,
    plain: *mut u8,
    plain_length: *mut UIntN,
) -> EfiStatus;

/// Function pointer for `EFI_SMART_CARD_EDGE_BUILD_DH_AGREEMENT`.
pub type EfiSmartCardEdgeBuildDhAgreement = unsafe extern "efiapi" fn(
    this: *const EfiSmartCardEdgeProtocol,
    scard_handle: EfiHandle,
    key_id: UIntN,
    data_qx: *const u8,
    data_qy: *const u8,
    dh_agreement: *mut u8,
) -> EfiStatus;

/// Interface structure for Smart Card Edge Protocol (`EFI_SMART_CARD_EDGE_PROTOCOL`).
#[repr(C)]
#[derive(Debug)]
pub struct EfiSmartCardEdgeProtocol {
    /// Retrieves smart card context handle.
    pub get_context: EfiSmartCardEdgeGetContext,
    /// Connects to a smart card in the specified reader.
    pub connect: EfiSmartCardEdgeConnect,
    /// Disconnects from the smart card.
    pub disconnect: EfiSmartCardEdgeDisconnect,
    /// Retrieves Card Serial Number (CSN).
    pub get_csn: EfiSmartCardEdgeGetCsn,
    /// Retrieves reader name string.
    pub get_reader_name: EfiSmartCardEdgeGetReaderName,
    /// Verifies user PIN against smart card.
    pub verify_pin: EfiSmartCardEdgeVerifyPin,
    /// Returns remaining PIN verification attempts before lockout.
    pub get_pin_remaining: EfiSmartCardEdgeGetPinRemaining,
    /// Retrieves raw data object by tag.
    pub get_data: EfiSmartCardEdgeGetData,
    /// Retrieves credentials list TLV structure.
    pub get_credential: EfiSmartCardEdgeGetCredential,
    /// Signs data using a private key residing in smart card.
    pub sign_data: EfiSmartCardEdgeSignData,
    /// Decrypts ciphertext using a private key residing in smart card.
    pub decrypt_data: EfiSmartCardEdgeDecryptData,
    /// Computes Diffie-Hellman agreement with smart card key container.
    pub build_dh_agreement: EfiSmartCardEdgeBuildDhAgreement,
}

/// Safe wrapper around [`EfiSmartCardEdgeProtocol`].
pub struct SmartCardEdge<'a> {
    proto: &'a EfiSmartCardEdgeProtocol,
}

impl<'a> SmartCardEdge<'a> {
    /// Wraps a reference to [`EfiSmartCardEdgeProtocol`].
    #[must_use]
    pub const fn new(proto: &'a EfiSmartCardEdgeProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiSmartCardEdgeProtocol`].
    ///
    /// # Safety
    ///
    /// If non-null, `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiSmartCardEdgeProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Retrieves the Smart Card context identifier.
    ///
    /// # Errors
    ///
    /// Returns an error status if context retrieval fails.
    pub fn get_context(&self) -> Result<UIntN, EfiStatus> {
        let mut ctx = 0 as UIntN;
        let status = unsafe { (self.proto.get_context)(self.proto, core::ptr::from_mut(&mut ctx)) };
        if status == EfiStatus::SUCCESS {
            Ok(ctx)
        } else {
            Err(status)
        }
    }

    /// Disconnects from smart card connection `scard_handle`.
    ///
    /// # Errors
    ///
    /// Returns an error status if disconnect fails.
    pub fn disconnect(&self, scard_handle: EfiHandle, card_action: u32) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.disconnect)(self.proto, scard_handle, card_action) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves Card Serial Number (CSN) from smart card.
    ///
    /// # Errors
    ///
    /// Returns an error status if CSN query fails.
    pub fn get_csn(&self, scard_handle: EfiHandle) -> Result<[u8; SCARD_CSN_SIZE], EfiStatus> {
        let mut csn = [0u8; SCARD_CSN_SIZE];
        let status = unsafe {
            (self.proto.get_csn)(self.proto, scard_handle, core::ptr::from_mut(&mut csn))
        };
        if status == EfiStatus::SUCCESS {
            Ok(csn)
        } else {
            Err(status)
        }
    }

    /// Verifies user PIN against the card.
    ///
    /// # Errors
    ///
    /// Returns an error status if PIN verification fails.
    pub fn verify_pin(&self, scard_handle: EfiHandle, pin: &[u8]) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.proto.verify_pin)(self.proto, scard_handle, pin.as_ptr(), pin.len()) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Retrieves remaining PIN attempts before lockout.
    ///
    /// # Errors
    ///
    /// Returns an error status if attempt query fails.
    pub fn get_pin_remaining(&self, scard_handle: EfiHandle) -> Result<UIntN, EfiStatus> {
        let mut rem = 0 as UIntN;
        let status = unsafe {
            (self.proto.get_pin_remaining)(self.proto, scard_handle, core::ptr::from_mut(&mut rem))
        };
        if status == EfiStatus::SUCCESS {
            Ok(rem)
        } else {
            Err(status)
        }
    }

    /// Signs data using a private key identified by `key_id`.
    ///
    /// Returns the number of bytes written to `signature`.
    ///
    /// # Errors
    ///
    /// Returns an error status if signing fails.
    pub fn sign_data(
        &self,
        scard_handle: EfiHandle,
        key_id: UIntN,
        data: &[u8],
        signature: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut sig_len = signature.len();
        let status = unsafe {
            (self.proto.sign_data)(
                self.proto,
                scard_handle,
                key_id,
                data.as_ptr(),
                data.len(),
                signature.as_mut_ptr(),
                core::ptr::from_mut(&mut sig_len),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(sig_len)
        } else {
            Err(status)
        }
    }

    /// Decrypts ciphertext using a private key identified by `key_id`.
    ///
    /// Returns the number of bytes written to `plain`.
    ///
    /// # Errors
    ///
    /// Returns an error status if decryption fails.
    pub fn decrypt_data(
        &self,
        scard_handle: EfiHandle,
        key_id: UIntN,
        cipher: &[u8],
        plain: &mut [u8],
    ) -> Result<usize, EfiStatus> {
        let mut plain_len = plain.len();
        let status = unsafe {
            (self.proto.decrypt_data)(
                self.proto,
                scard_handle,
                key_id,
                cipher.as_ptr(),
                cipher.len(),
                plain.as_mut_ptr(),
                core::ptr::from_mut(&mut plain_len),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(plain_len)
        } else {
            Err(status)
        }
    }
}
