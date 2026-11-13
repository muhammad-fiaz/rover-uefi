//! Integration tests for UEFI TLS and HTTP Boot Callback protocols.

use rover_uefi::protocols::network::http_boot_callback::*;
use rover_uefi::protocols::network::tls_protocol::*;

#[test]
fn test_tls_and_http_boot_protocol_guids() {
    assert_ne!(EFI_TLS_SERVICE_BINDING_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_TLS_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_TLS_CONFIGURATION_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_HTTP_BOOT_CALLBACK_PROTOCOL_GUID.data1, 0);
}

#[test]
fn test_tls_protocol_struct_layouts() {
    assert_eq!(
        core::mem::size_of::<EfiTlsProtocol>(),
        4 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiTlsConfigurationProtocol>(),
        2 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiHttpBootCallbackProtocol>(),
        core::mem::size_of::<usize>()
    );
}

#[test]
fn test_tls_and_http_boot_enums() {
    assert_eq!(EfiTlsSessionDataType::Version as u32, 0);
    assert_eq!(EfiTlsSessionDataType::KeyMaterial as u32, 10);
    assert_eq!(EfiTlsConfigDataType::HostPublicCert as u32, 0);
    assert_eq!(EfiTlsConfigDataType::CertRevocationList as u32, 3);
    assert_eq!(EfiTlsCryptMode::Encrypt as u32, 0);
    assert_eq!(EfiTlsCryptMode::Decrypt as u32, 1);
    assert_eq!(EfiHttpBootCallbackDataType::HttpBootDhcp4 as u32, 0);
    assert_eq!(
        EfiHttpBootCallbackDataType::HttpBootHttpEntityBody as u32,
        4
    );
}
