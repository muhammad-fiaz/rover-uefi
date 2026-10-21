//! Tests for MCTP and PLDM manageability protocols.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::manageability::mctp::{EfiMctpProtocol, MCTP_PROTOCOL_GUID, mctp_type};
use rover_uefi::protocols::manageability::pldm::{
    EfiPldmProtocol, PLDM_PROTOCOL_GUID, base_cmd as pldm_base_cmd, pldm_type,
};

#[test]
fn test_mctp_pldm_guids_and_types() {
    assert_eq!(MCTP_PROTOCOL_GUID.data1, 0xE934_65C1);
    assert_eq!(PLDM_PROTOCOL_GUID.data1, 0x6099_7616);

    assert_eq!(mctp_type::CONTROL, 0x00);
    assert_eq!(mctp_type::PLDM, 0x01);
    assert_eq!(mctp_type::SPDM, 0x05);

    assert_eq!(pldm_type::BASE, 0x00);
    assert_eq!(pldm_type::PLATFORM_MONITORING, 0x02);
    assert_eq!(pldm_type::BIOS_CONTROL, 0x03);

    assert_eq!(pldm_base_cmd::GET_TID, 0x02);
    assert_eq!(pldm_base_cmd::GET_PLDM_VERSION, 0x03);
}

unsafe extern "efiapi" fn dummy_mctp_submit(
    _this: *mut EfiMctpProtocol,
    _mctp_type: u8,
    _source_eid: *const u8,
    _dest_eid: *const u8,
    _req_integrity_check: bool,
    _req_data: *const u8,
    _req_size: u32,
    _req_timeout: u32,
    _resp_data: *mut u8,
    _resp_size: *mut u32,
    _resp_timeout: u32,
    _additional_error: *mut u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn dummy_pldm_submit(
    _this: *mut EfiPldmProtocol,
    _pldm_type: u8,
    _command: u8,
    _src_tid: u8,
    _dest_tid: u8,
    _req_data: *const u8,
    _req_size: u32,
    _resp_data: *mut u8,
    _resp_size: *mut u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_mctp_pldm_protocols() {
    let mut mctp = EfiMctpProtocol {
        submit_command: dummy_mctp_submit,
    };

    let status = unsafe {
        (mctp.submit_command)(
            &raw mut mctp,
            mctp_type::PLDM,
            core::ptr::null(),
            core::ptr::null(),
            false,
            core::ptr::null(),
            0,
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
        )
    };
    assert_eq!(status, EfiStatus::SUCCESS);

    let mut pldm = EfiPldmProtocol {
        submit_command: dummy_pldm_submit,
    };

    let status = unsafe {
        (pldm.submit_command)(
            &raw mut pldm,
            pldm_type::BASE,
            pldm_base_cmd::GET_TID,
            0,
            1,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };
    assert_eq!(status, EfiStatus::SUCCESS);
}
