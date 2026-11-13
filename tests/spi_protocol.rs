//! # Integration tests for SPI protocols
//!
//! Verifies SPI protocol structures, GUIDs, and mock operations.

#![no_std]

extern crate alloc;

use core::ffi::c_void;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::spi::{
    EFI_SPI_CONFIGURATION_GUID, EFI_SPI_HC_PROTOCOL_GUID, EFI_SPI_NOR_FLASH_PROTOCOL_GUID,
    EfiSpiBusTransaction, EfiSpiHcProtocol, EfiSpiNorFlashProtocol, EfiSpiNorFlashProtocolEx,
    SPI_NOR_FLASH_PROTOCOL_EX_GUID, SpiHcProtocol, SpiNorFlashProtocol, SpiNorFlashProtocolEx,
    SpiTransactionType, hc_attributes,
};

#[test]
fn test_spi_guids() {
    assert_eq!(
        EFI_SPI_CONFIGURATION_GUID.to_u128_le(),
        0xc8dd_f64a_d5c6_8fb3_4afc_b65b_85a6_d3e6
    );
    assert_eq!(
        EFI_SPI_HC_PROTOCOL_GUID.to_u128_le(),
        0x2d00_e37f_9715_99b3_4ae2_fa96_c74e_5db2
    );
    assert_eq!(
        EFI_SPI_NOR_FLASH_PROTOCOL_GUID.to_u128_le(),
        0x4b44_876a_7d2a_7885_4ba6_f833_b57e_c3fe
    );
    assert_eq!(
        SPI_NOR_FLASH_PROTOCOL_EX_GUID.to_u128_le(),
        0x9ba9_5bfe_8e56_5284_46b5_7ff9_99de_f701
    );
}

unsafe extern "efiapi" fn mock_get_flash_id(
    _this: *const EfiSpiNorFlashProtocol,
    buffer: *mut u8,
) -> EfiStatus {
    unsafe {
        *buffer = 0xEF;
        *buffer.add(1) = 0x40;
        *buffer.add(2) = 0x18;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_read_data(
    _this: *const EfiSpiNorFlashProtocol,
    flash_address: u32,
    length_in_bytes: u32,
    buffer: *mut u8,
) -> EfiStatus {
    for i in 0..length_in_bytes {
        unsafe {
            *buffer.add(i as usize) = ((flash_address + i) & 0xFF) as u8;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_lf_read_data(
    this: *const EfiSpiNorFlashProtocol,
    flash_address: u32,
    length_in_bytes: u32,
    buffer: *mut u8,
) -> EfiStatus {
    unsafe { mock_read_data(this, flash_address, length_in_bytes, buffer) }
}

unsafe extern "efiapi" fn mock_read_status(
    _this: *const EfiSpiNorFlashProtocol,
    length_in_bytes: u32,
    flash_status: *mut u8,
) -> EfiStatus {
    for i in 0..length_in_bytes {
        unsafe {
            *flash_status.add(i as usize) = 0x00;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_write_status(
    _this: *const EfiSpiNorFlashProtocol,
    _length_in_bytes: u32,
    _flash_status: *const u8,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_write_data(
    _this: *const EfiSpiNorFlashProtocol,
    _flash_address: u32,
    _length_in_bytes: u32,
    _buffer: *const u8,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_erase(
    _this: *const EfiSpiNorFlashProtocol,
    _flash_address: u32,
    _block_count: u32,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_spi_nor_flash() {
    let raw = EfiSpiNorFlashProtocol {
        spi_peripheral: core::ptr::null(),
        flash_size: 16 * 1024 * 1024,
        device_id: [0xEF, 0x40, 0x18],
        erase_block_bytes: 4096,
        get_flash_id: mock_get_flash_id,
        read_data: mock_read_data,
        lf_read_data: mock_lf_read_data,
        read_status: mock_read_status,
        write_status: mock_write_status,
        write_data: mock_write_data,
        erase: mock_erase,
    };

    let flash = SpiNorFlashProtocol::new(&raw);
    assert_eq!(flash.flash_size(), 16 * 1024 * 1024);
    assert_eq!(flash.erase_block_bytes(), 4096);
    assert_eq!(flash.device_id(), [0xEF, 0x40, 0x18]);

    let mut id_buf = [0u8; 3];
    assert!(flash.get_flash_id(&mut id_buf).is_ok());
    assert_eq!(id_buf, [0xEF, 0x40, 0x18]);

    let mut data_buf = [0u8; 4];
    assert!(flash.read_data(0x10, &mut data_buf).is_ok());
    assert_eq!(data_buf, [0x10, 0x11, 0x12, 0x13]);

    assert!(flash.erase(0, 1).is_ok());
}

unsafe extern "efiapi" fn mock_enter_4byte(_this: *const EfiSpiNorFlashProtocolEx) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_exit_4byte(_this: *const EfiSpiNorFlashProtocolEx) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_spi_nor_flash_ex() {
    let raw = EfiSpiNorFlashProtocolEx {
        version: 0x0100,
        enter_4byte: mock_enter_4byte,
        exit_4byte: mock_exit_4byte,
    };
    let flash_ex = SpiNorFlashProtocolEx::new(&raw);
    assert_eq!(flash_ex.version(), 0x0100);
    assert!(flash_ex.enter_4byte().is_ok());
    assert!(flash_ex.exit_4byte().is_ok());
}

unsafe extern "efiapi" fn mock_chip_select(
    _this: *const EfiSpiHcProtocol,
    _spi_peripheral: *const c_void,
    _pin_value: bool,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock(
    _this: *const EfiSpiHcProtocol,
    _spi_peripheral: *const c_void,
    clock_hz: *mut u32,
) -> EfiStatus {
    unsafe {
        *clock_hz = 50_000_000;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_transaction(
    _this: *const EfiSpiHcProtocol,
    _bus_transaction: *mut EfiSpiBusTransaction,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_spi_hc() {
    let raw = EfiSpiHcProtocol {
        attributes: hc_attributes::SUPPORTS_WRITE_THEN_READ_OPERATIONS
            | hc_attributes::SUPPORTS_4_BIT_DATA_BUS_WIDTH,
        frame_size_support_mask: 0x0000_0080,
        maximum_transfer_bytes: 65536,
        chip_select: mock_chip_select,
        clock: mock_clock,
        transaction: mock_transaction,
    };
    let hc = SpiHcProtocol::new(&raw);
    assert_eq!(hc.maximum_transfer_bytes(), 65536);
    assert!(unsafe { hc.chip_select(core::ptr::null(), true) }.is_ok());
    assert_eq!(
        unsafe { hc.set_clock(core::ptr::null(), 60_000_000) },
        Ok(50_000_000)
    );

    let mut trans = EfiSpiBusTransaction {
        spi_peripheral: core::ptr::null(),
        transaction_type: SpiTransactionType::WriteThenRead,
        debug_transaction: false,
        bus_width: 1,
        frame_size: 8,
        write_bytes: 0,
        write_buffer: core::ptr::null(),
        read_bytes: 0,
        read_buffer: core::ptr::null_mut(),
    };
    assert!(unsafe { hc.execute_transaction(&mut trans) }.is_ok());
}
