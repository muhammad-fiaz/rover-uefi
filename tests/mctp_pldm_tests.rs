//! Integration tests for MCTP and PLDM manageability protocols and client wrappers.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation
)]

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::manageability::{
    EfiMctpProtocol, EfiPldmProtocol, MCTP_PROTOCOL_GUID, MctpProtocol, PLDM_PROTOCOL_GUID,
    PldmProtocol, mctp_type, pldm_base_cmd, pldm_type,
};

static mut MOCK_MCTP_RESP: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x00, 0x00, 0x00, 0x00];

unsafe extern "efiapi" fn mock_mctp_submit(
    _this: *mut EfiMctpProtocol,
    mctp_type: u8,
    _source_eid: *const u8,
    _dest_eid: *const u8,
    _req_integrity: bool,
    _req_data: *const u8,
    _req_size: u32,
    _req_timeout: u32,
    resp_data: *mut u8,
    resp_size: *mut u32,
    _resp_timeout: u32,
    additional_error: *mut u32,
) -> EfiStatus {
    if mctp_type == mctp_type::PLDM {
        if !resp_data.is_null() && !resp_size.is_null() {
            unsafe {
                let copy_len = (*resp_size as usize).min(4);
                core::ptr::copy_nonoverlapping(
                    (&raw const MOCK_MCTP_RESP).cast::<u8>(),
                    resp_data,
                    copy_len,
                );
                *resp_size = copy_len as u32;
            }
        }
        if !additional_error.is_null() {
            unsafe {
                *additional_error = 0;
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::UNSUPPORTED
    }
}

static mut MOCK_PLDM_RESP: [u8; 4] = [0x00, 0x05, 0x00, 0x00]; // CompletionCode 0, TID 5

unsafe extern "efiapi" fn mock_pldm_submit(
    _this: *mut EfiPldmProtocol,
    pldm_type: u8,
    command: u8,
    _src_tid: u8,
    _dst_tid: u8,
    _req_data: *const u8,
    _req_size: u32,
    resp_data: *mut u8,
    resp_size: *mut u32,
) -> EfiStatus {
    if pldm_type == pldm_type::BASE && command == pldm_base_cmd::GET_TID {
        if !resp_data.is_null() && !resp_size.is_null() {
            unsafe {
                let copy_len = (*resp_size as usize).min(4);
                core::ptr::copy_nonoverlapping(
                    (&raw const MOCK_PLDM_RESP).cast::<u8>(),
                    resp_data,
                    copy_len,
                );
                *resp_size = copy_len as u32;
            }
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

#[test]
fn test_mctp_protocol_guids_and_client_wrapper() {
    assert_eq!(
        MCTP_PROTOCOL_GUID.to_string(),
        "e93465c1-9a31-4c96-9256-220ae180b41b"
    );

    let mut raw_proto = EfiMctpProtocol {
        submit_command: mock_mctp_submit,
    };
    let mut client = MctpProtocol::new(&mut raw_proto);

    let req = [0x01, 0x02];
    let mut resp = [0_u8; 16];
    let res = client.submit_message(
        mctp_type::PLDM,
        Some(10),
        Some(20),
        false,
        &req,
        1000,
        &mut resp,
        1000,
    );

    assert!(res.is_ok());
    let (resp_len, add_err) = res.unwrap();
    assert_eq!(resp_len, 4);
    assert_eq!(add_err, 0);
    assert_eq!(&resp[0..4], &[0x00, 0x01, 0x02, 0x03]);

    // Unsupported type
    let err_res = client.submit_message(
        mctp_type::NVME_MI,
        None,
        None,
        false,
        &req,
        1000,
        &mut resp,
        1000,
    );
    assert_eq!(err_res.unwrap_err(), EfiStatus::UNSUPPORTED);
}

#[test]
fn test_pldm_protocol_guids_and_client_wrapper() {
    assert_eq!(
        PLDM_PROTOCOL_GUID.to_string(),
        "60997616-db70-4b5f-86a4-0958a37147b4"
    );

    let mut raw_proto = EfiPldmProtocol {
        submit_command: mock_pldm_submit,
    };
    let mut client = PldmProtocol::new(&mut raw_proto);

    let req = [];
    let mut resp = [0_u8; 8];
    let res = client.submit_command(
        pldm_type::BASE,
        pldm_base_cmd::GET_TID,
        1,
        2,
        &req,
        &mut resp,
    );

    assert!(res.is_ok());
    let resp_len = res.unwrap();
    assert_eq!(resp_len, 4);
    assert_eq!(resp[0], 0x00); // SUCCESS CC
    assert_eq!(resp[1], 0x05); // TID 5
}
