//! Integration tests for Smart Card Reader and Smart Card Edge protocols.

use core::ffi::c_void;
use rover_uefi::abi::types::{Char16, UIntN};
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::security::{
    EFI_SMART_CARD_EDGE_PROTOCOL_GUID, EFI_SMART_CARD_READER_PROTOCOL_GUID,
    EfiSmartCardEdgeProtocol, EfiSmartCardReaderProtocol, SCARD_ACTIVE, SCARD_AM_CARD,
    SCARD_CA_NORESET, SCARD_CSN_SIZE, SCARD_PROTOCOL_T1, SMART_CARD_EDGE_PROTOCOL_VERSION_1,
    SmartCardEdge, SmartCardReader,
};

// Mock functions for Smart Card Reader
unsafe extern "efiapi" fn mock_scard_connect(
    _this: *const EfiSmartCardReaderProtocol,
    _access_mode: u32,
    _card_action: u32,
    _preferred_protocols: u32,
    active_protocol: *mut u32,
) -> EfiStatus {
    if !active_protocol.is_null() {
        unsafe { *active_protocol = SCARD_PROTOCOL_T1 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_scard_disconnect(
    _this: *const EfiSmartCardReaderProtocol,
    _card_action: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_scard_status(
    _this: *const EfiSmartCardReaderProtocol,
    _reader_name: *mut Char16,
    reader_name_length: *mut UIntN,
    state: *mut u32,
    card_protocol: *mut u32,
    _atr: *mut u8,
    atr_length: *mut UIntN,
) -> EfiStatus {
    if !reader_name_length.is_null() {
        unsafe { *reader_name_length = 0 };
    }
    if !state.is_null() {
        unsafe { *state = SCARD_ACTIVE };
    }
    if !card_protocol.is_null() {
        unsafe { *card_protocol = SCARD_PROTOCOL_T1 };
    }
    if !atr_length.is_null() {
        unsafe { *atr_length = 0 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_scard_transmit(
    _this: *const EfiSmartCardReaderProtocol,
    capdu: *const u8,
    capdu_length: UIntN,
    rapdu: *mut u8,
    rapdu_length: *mut UIntN,
) -> EfiStatus {
    if capdu.is_null() || rapdu.is_null() || rapdu_length.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if capdu_length == 5 {
        // Echo back 2 status bytes 0x90 0x00
        unsafe {
            if *rapdu_length >= 2 {
                *rapdu = 0x90;
                *rapdu.add(1) = 0x00;
                *rapdu_length = 2;
                return EfiStatus::SUCCESS;
            }
        }
    }
    EfiStatus::BAD_BUFFER_SIZE
}

unsafe extern "efiapi" fn mock_scard_control(
    _this: *const EfiSmartCardReaderProtocol,
    control_code: u32,
    _in_buf: *const u8,
    _in_len: UIntN,
    out_buf: *mut u8,
    out_len: *mut UIntN,
) -> EfiStatus {
    if control_code == 0x42 && !out_buf.is_null() && !out_len.is_null() {
        unsafe {
            if *out_len >= 1 {
                *out_buf = 0xAA;
                *out_len = 1;
                return EfiStatus::SUCCESS;
            }
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_scard_get_attrib(
    _this: *const EfiSmartCardReaderProtocol,
    attrib: u32,
    out_buf: *mut u8,
    out_len: *mut UIntN,
) -> EfiStatus {
    if attrib == 0x01 && !out_buf.is_null() && !out_len.is_null() {
        unsafe {
            if *out_len >= 4 {
                let slice = core::slice::from_raw_parts_mut(out_buf, 4);
                slice.copy_from_slice(&[1, 2, 3, 4]);
                *out_len = 4;
                return EfiStatus::SUCCESS;
            }
        }
    }
    EfiStatus::UNSUPPORTED
}

// Mock functions for Smart Card Edge
unsafe extern "efiapi" fn mock_edge_get_context(
    _this: *const EfiSmartCardEdgeProtocol,
    scard_context: *mut UIntN,
) -> EfiStatus {
    if !scard_context.is_null() {
        unsafe { *scard_context = 0x1234 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_edge_connect(
    _this: *const EfiSmartCardEdgeProtocol,
    _reader_name: *const Char16,
    scard_handle: *mut EfiHandle,
    active_protocol: *mut u32,
) -> EfiStatus {
    if !scard_handle.is_null() {
        unsafe { *scard_handle = EfiHandle::from_ptr(0x5678 as *mut c_void) };
    }
    if !active_protocol.is_null() {
        unsafe { *active_protocol = SCARD_PROTOCOL_T1 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_edge_disconnect(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    _card_action: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_edge_get_csn(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    csn: *mut [u8; SCARD_CSN_SIZE],
) -> EfiStatus {
    if !csn.is_null() {
        unsafe { (*csn).copy_from_slice(&[0x11; SCARD_CSN_SIZE]) };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_edge_get_reader_name(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    _reader_name: *mut Char16,
    reader_name_length: *mut UIntN,
) -> EfiStatus {
    if !reader_name_length.is_null() {
        unsafe { *reader_name_length = 0 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_edge_verify_pin(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    pin: *const u8,
    pin_length: UIntN,
) -> EfiStatus {
    if pin_length == 4 && !pin.is_null() {
        let slice = unsafe { core::slice::from_raw_parts(pin, pin_length) };
        if slice == b"1234" {
            return EfiStatus::SUCCESS;
        }
    }
    EfiStatus::ACCESS_DENIED
}

unsafe extern "efiapi" fn mock_edge_get_pin_remaining(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    remaining: *mut UIntN,
) -> EfiStatus {
    if !remaining.is_null() {
        unsafe { *remaining = 3 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_edge_get_data(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    _tag: u16,
    _data: *mut u8,
    data_length: *mut UIntN,
) -> EfiStatus {
    if !data_length.is_null() {
        unsafe { *data_length = 0 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_edge_get_credential(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    _cred_list: *mut *mut u8,
    cred_list_length: *mut UIntN,
) -> EfiStatus {
    if !cred_list_length.is_null() {
        unsafe { *cred_list_length = 0 };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_edge_sign_data(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    _key_id: UIntN,
    _data: *const u8,
    _data_length: UIntN,
    signature: *mut u8,
    signature_length: *mut UIntN,
) -> EfiStatus {
    if !signature.is_null() && !signature_length.is_null() {
        unsafe {
            if *signature_length >= 4 {
                let slice = core::slice::from_raw_parts_mut(signature, 4);
                slice.copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
                *signature_length = 4;
                return EfiStatus::SUCCESS;
            }
        }
    }
    EfiStatus::BAD_BUFFER_SIZE
}

unsafe extern "efiapi" fn mock_edge_decrypt_data(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    _key_id: UIntN,
    _cipher: *const u8,
    _cipher_length: UIntN,
    plain: *mut u8,
    plain_length: *mut UIntN,
) -> EfiStatus {
    if !plain.is_null() && !plain_length.is_null() {
        unsafe {
            if *plain_length >= 2 {
                let slice = core::slice::from_raw_parts_mut(plain, 2);
                slice.copy_from_slice(&[0xCA, 0xFE]);
                *plain_length = 2;
                return EfiStatus::SUCCESS;
            }
        }
    }
    EfiStatus::BAD_BUFFER_SIZE
}

unsafe extern "efiapi" fn mock_edge_build_dh_agreement(
    _this: *const EfiSmartCardEdgeProtocol,
    _scard_handle: EfiHandle,
    _key_id: UIntN,
    _data_qx: *const u8,
    _data_qy: *const u8,
    _dh_agreement: *mut u8,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

#[test]
fn test_smart_card_reader_guid_and_constants() {
    assert_eq!(
        EFI_SMART_CARD_READER_PROTOCOL_GUID,
        EfiGuid::new(
            0x2A4D_1ADF,
            0x21DC,
            0x4B81,
            [0xA4, 0x2F, 0x8B, 0x8E, 0xE2, 0x38, 0x00, 0x60],
        )
    );
    assert_eq!(
        EFI_SMART_CARD_EDGE_PROTOCOL_GUID,
        EfiGuid::new(
            0xD317_F29B,
            0xA325,
            0x4712,
            [0x9B, 0xF1, 0xC6, 0x19, 0x54, 0xDC, 0x19, 0x8C],
        )
    );
    assert_eq!(SMART_CARD_EDGE_PROTOCOL_VERSION_1, 0x0100);
}

#[test]
fn test_smart_card_reader_client() {
    let mock_proto = EfiSmartCardReaderProtocol {
        scard_connect: mock_scard_connect,
        scard_disconnect: mock_scard_disconnect,
        scard_status: mock_scard_status,
        scard_transmit: mock_scard_transmit,
        scard_control: mock_scard_control,
        scard_get_attrib: mock_scard_get_attrib,
    };

    let client = SmartCardReader::new(&mock_proto);

    let active = client
        .connect(SCARD_AM_CARD, SCARD_CA_NORESET, SCARD_PROTOCOL_T1)
        .unwrap_or(0);
    assert_eq!(active, SCARD_PROTOCOL_T1);

    assert!(client.disconnect(SCARD_CA_NORESET).is_ok());

    let apdu_cmd = [0x00, 0xA4, 0x04, 0x00, 0x00];
    let mut resp = [0u8; 16];
    let len = client.transmit(&apdu_cmd, &mut resp).unwrap_or(0);
    assert_eq!(len, 2);
    assert_eq!(&resp[0..2], &[0x90, 0x00]);

    let mut out = [0u8; 8];
    let attr_len = client.get_attrib(0x01, &mut out).unwrap_or(0);
    assert_eq!(attr_len, 4);
    assert_eq!(&out[0..4], &[1, 2, 3, 4]);
}

#[test]
fn test_smart_card_edge_client() {
    let mock_proto = EfiSmartCardEdgeProtocol {
        get_context: mock_edge_get_context,
        connect: mock_edge_connect,
        disconnect: mock_edge_disconnect,
        get_csn: mock_edge_get_csn,
        get_reader_name: mock_edge_get_reader_name,
        verify_pin: mock_edge_verify_pin,
        get_pin_remaining: mock_edge_get_pin_remaining,
        get_data: mock_edge_get_data,
        get_credential: mock_edge_get_credential,
        sign_data: mock_edge_sign_data,
        decrypt_data: mock_edge_decrypt_data,
        build_dh_agreement: mock_edge_build_dh_agreement,
    };

    let edge = SmartCardEdge::new(&mock_proto);

    let ctx = edge.get_context().unwrap_or(0);
    assert_eq!(ctx, 0x1234);

    let dummy_handle = EfiHandle::from_ptr(0x5678 as *mut c_void);
    let csn = edge.get_csn(dummy_handle).unwrap_or([0u8; SCARD_CSN_SIZE]);
    assert_eq!(csn, [0x11; SCARD_CSN_SIZE]);

    assert!(edge.verify_pin(dummy_handle, b"1234").is_ok());
    assert!(edge.verify_pin(dummy_handle, b"0000").is_err());

    let remaining = edge.get_pin_remaining(dummy_handle).unwrap_or(0);
    assert_eq!(remaining, 3);

    let mut sig = [0u8; 8];
    let sig_len = edge
        .sign_data(dummy_handle, 1, b"test", &mut sig)
        .unwrap_or(0);
    assert_eq!(sig_len, 4);
    assert_eq!(&sig[0..4], &[0xDE, 0xAD, 0xBE, 0xEF]);

    let mut plain = [0u8; 8];
    let plain_len = edge
        .decrypt_data(dummy_handle, 1, b"cipher", &mut plain)
        .unwrap_or(0);
    assert_eq!(plain_len, 2);
    assert_eq!(&plain[0..2], &[0xCA, 0xFE]);
}
