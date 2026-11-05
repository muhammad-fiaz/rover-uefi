//! # Integration tests for SD/MMC Pass Thru protocol
//!
//! Verifies SD/MMC protocol structures, GUIDs, and mock operations.

#![no_std]

extern crate alloc;

use core::ffi::c_void;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::storage::{
    EFI_SD_MMC_PASS_THRU_PROTOCOL_GUID, EfiSdMmcCommandBlock, EfiSdMmcPassThruCommandPacket,
    EfiSdMmcPassThruProtocol, EfiSdMmcStatusBlock, SdMmcCommandType, SdMmcPassThruProtocol,
    SdMmcResponseType,
};

#[test]
fn test_sd_mmc_guid() {
    assert_eq!(
        EFI_SD_MMC_PASS_THRU_PROTOCOL_GUID.to_u128_le(),
        0x708e_9ad3_8b51_e981_4f69_ff83_716e_f0d9
    );
}

unsafe extern "efiapi" fn mock_pass_thru(
    _this: *mut EfiSdMmcPassThruProtocol,
    _slot: u8,
    packet: *mut EfiSdMmcPassThruCommandPacket,
    _event: *mut c_void,
) -> EfiStatus {
    unsafe {
        (*packet).transaction_status = EfiStatus::SUCCESS;
        if !(*packet).sd_mmc_status_blk.is_null() {
            (*(*packet).sd_mmc_status_blk).resp0 = 0x0000_0900; // Ready state
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_next_slot(
    _this: *mut EfiSdMmcPassThruProtocol,
    slot: *mut u8,
) -> EfiStatus {
    unsafe {
        if *slot == 0xFF {
            *slot = 0;
            EfiStatus::SUCCESS
        } else if *slot == 0 {
            *slot = 1;
            EfiStatus::SUCCESS
        } else {
            EfiStatus::NOT_FOUND
        }
    }
}

unsafe extern "efiapi" fn mock_build_device_path(
    _this: *mut EfiSdMmcPassThruProtocol,
    _slot: u8,
    _device_path: *mut *mut c_void,
) -> EfiStatus {
    EfiStatus::UNSUPPORTED
}

unsafe extern "efiapi" fn mock_get_slot_number(
    _this: *mut EfiSdMmcPassThruProtocol,
    _device_path: *const c_void,
    slot: *mut u8,
) -> EfiStatus {
    unsafe {
        *slot = 0;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_reset_device(
    _this: *mut EfiSdMmcPassThruProtocol,
    _slot: u8,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_sd_mmc_operations() {
    let mut raw = EfiSdMmcPassThruProtocol {
        io_align: 4,
        pass_thru: mock_pass_thru,
        get_next_slot: mock_get_next_slot,
        build_device_path: mock_build_device_path,
        get_slot_number: mock_get_slot_number,
        reset_device: mock_reset_device,
    };

    let mut protocol = SdMmcPassThruProtocol::new(&mut raw);
    assert_eq!(protocol.io_align(), 4);

    let slots: alloc::vec::Vec<u8> = protocol.slots().collect();
    assert_eq!(slots, alloc::vec![0, 1]);

    assert!(protocol.reset_device(0).is_ok());

    let mut cmd_blk = EfiSdMmcCommandBlock {
        command_index: 13, // SEND_STATUS
        command_argument: 0x0001_0000,
        command_type: SdMmcCommandType::Ac as u32,
        response_type: SdMmcResponseType::R1 as u32,
    };
    let mut status_blk = EfiSdMmcStatusBlock::default();
    let mut packet = EfiSdMmcPassThruCommandPacket {
        timeout: 1000,
        sd_mmc_cmd_blk: &raw mut cmd_blk,
        sd_mmc_status_blk: &raw mut status_blk,
        in_data_buffer: core::ptr::null_mut(),
        out_data_buffer: core::ptr::null(),
        in_transfer_length: 0,
        out_transfer_length: 0,
        transaction_status: EfiStatus::NOT_READY,
    };

    assert!(unsafe { protocol.pass_thru_sync(0, &mut packet) }.is_ok());
    assert_eq!(status_blk.resp0, 0x0000_0900);
}
