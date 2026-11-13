//! # UEFI TLS Protocol and TLS Configuration Protocol
//!
//! Defined in UEFI Specification 2.5+ Section 29.8 / 29.9.
//!
//! Provides services to create, configure, and manage TLS sessions and
//! configure TLS keys, certificates, and certificate revocation lists (CRLs).

use core::ffi::c_void;

use crate::base::{EfiGuid, EfiStatus};

/// EFI TLS Service Binding Protocol `GUID`.
pub const EFI_TLS_SERVICE_BINDING_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x952C_B795,
    0xFF36,
    0x48CF,
    [0xA2, 0x49, 0x4D, 0xF4, 0x86, 0xD6, 0xAB, 0x8D],
);

/// EFI TLS Protocol `GUID`.
pub const EFI_TLS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x00CA_959F,
    0x6CFA,
    0x4DB1,
    [0x95, 0xBC, 0xE4, 0x6C, 0x47, 0x51, 0x43, 0x90],
);

/// EFI TLS Configuration Protocol `GUID`.
pub const EFI_TLS_CONFIGURATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1682_FE44,
    0xBD7A,
    0x4407,
    [0xB7, 0xC7, 0xDC, 0xA3, 0x7C, 0xA3, 0x92, 0x2D],
);

/// TLS Session Data Type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiTlsSessionDataType {
    /// TLS session Version (`EFI_TLS_VERSION`).
    Version = 0,
    /// TLS session end (client or server, `EFI_TLS_CONNECTION_END`).
    ConnectionEnd = 1,
    /// Priority of cipher suites (`EFI_TLS_CIPHER`).
    CipherList = 2,
    /// Compression method list.
    CompressionMethod = 3,
    /// Extension list.
    ExtensionList = 4,
    /// Verify method (`EFI_TLS_VERIFY`).
    VerifyMethod = 5,
    /// TLS session data (`EFI_TLS_SESSION_STATE`).
    SessionId = 6,
    /// TLS session state (`EFI_TLS_SESSION_STATE`).
    SessionState = 7,
    /// Client random bytes.
    ClientRandom = 8,
    /// Server random bytes.
    ServerRandom = 9,
    /// Key material.
    KeyMaterial = 10,
}

/// TLS Configuration Data Type.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiTlsConfigDataType {
    /// Local host public certificate (DER or PEM X.509).
    HostPublicCert = 0,
    /// Local host private key.
    HostPrivateKey = 1,
    /// CA certificate used to verify peer.
    CaCertificate = 2,
    /// CA-supplied Certificate Revocation List (CRL).
    CertRevocationList = 3,
}

/// Cryptographic direction for TLS application data processing.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EfiTlsCryptMode {
    /// Encrypt plaintext application data into ciphertext record.
    Encrypt = 0,
    /// Decrypt ciphertext record into plaintext application data.
    Decrypt = 1,
}

/// Fragment descriptor for fragmented TLS buffers.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiTlsFragmentData {
    /// Length of fragment in bytes.
    pub fragment_length: u32,
    /// Pointer to fragment memory buffer.
    pub fragment_buffer: *mut c_void,
}

/// Sets TLS session configuration or data.
pub type EfiTlsSetSessionData = unsafe extern "efiapi" fn(
    this: *mut EfiTlsProtocol,
    data_type: EfiTlsSessionDataType,
    data: *const c_void,
    data_size: usize,
) -> EfiStatus;

/// Gets TLS session configuration or data.
pub type EfiTlsGetSessionData = unsafe extern "efiapi" fn(
    this: *mut EfiTlsProtocol,
    data_type: EfiTlsSessionDataType,
    data: *mut c_void,
    data_size: *mut usize,
) -> EfiStatus;

/// Builds handshake response packet or error alert.
pub type EfiTlsBuildResponsePacket = unsafe extern "efiapi" fn(
    this: *mut EfiTlsProtocol,
    request_buffer: *const c_void,
    request_size: usize,
    buffer: *mut c_void,
    buffer_size: *mut usize,
) -> EfiStatus;

/// Decrypts or encrypts TLS application data packets.
pub type EfiTlsProcessPacket = unsafe extern "efiapi" fn(
    this: *mut EfiTlsProtocol,
    fragment_table: *mut *mut EfiTlsFragmentData,
    fragment_count: *mut u32,
    crypt_mode: EfiTlsCryptMode,
) -> EfiStatus;

/// EFI TLS Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiTlsProtocol {
    /// Sets TLS session parameters.
    pub set_session_data: EfiTlsSetSessionData,
    /// Queries TLS session parameters.
    pub get_session_data: EfiTlsGetSessionData,
    /// Processes inbound TLS handshake packets and generates responses.
    pub build_response_packet: EfiTlsBuildResponsePacket,
    /// Encrypts or decrypts application data packets.
    pub process_packet: EfiTlsProcessPacket,
}

/// Sets TLS configuration certificate/key data.
pub type EfiTlsConfigurationSetData = unsafe extern "efiapi" fn(
    this: *mut EfiTlsConfigurationProtocol,
    data_type: EfiTlsConfigDataType,
    data: *const c_void,
    data_size: usize,
) -> EfiStatus;

/// Gets TLS configuration certificate/key data.
pub type EfiTlsConfigurationGetData = unsafe extern "efiapi" fn(
    this: *mut EfiTlsConfigurationProtocol,
    data_type: EfiTlsConfigDataType,
    data: *mut c_void,
    data_size: *mut usize,
) -> EfiStatus;

/// EFI TLS Configuration Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiTlsConfigurationProtocol {
    /// Stores certificate or private key data into volatile/non-volatile storage.
    pub set_data: EfiTlsConfigurationSetData,
    /// Retrieves certificate or private key data.
    pub get_data: EfiTlsConfigurationGetData,
}
