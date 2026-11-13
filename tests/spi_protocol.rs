//! # Integration tests for SPI protocols
//!
//! Verifies SPI protocol structures, GUIDs, and mock operations.

#![no_std]

extern crate alloc;

use core::ffi::c_void;
use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::spi::{
    EFI_SPI_CONFIGURATION_GUID, EFI_SPI_HC_PROTOCOL_GUID, EFI_SPI_NOR_FLASH_PROTOCOL_GUID,
    EFI_SPI_SMM_CONFIGURATION_PROTOCOL_GUID, EFI_SPI_SMM_HC_PROTOCOL_GUID,
    EFI_SPI_SMM_NOR_FLASH_PROTOCOL_GUID, EfiSpiBus, EfiSpiBusTransaction,
    EfiSpiConfigurationProtocol, EfiSpiHcProtocol, EfiSpiIoProtocol, EfiSpiNorFlashProtocol,
    EfiSpiNorFlashProtocolEx, SPI_IO_SUPPORTS_4_BIT_DATA_BUS_WIDTH, SPI_NOR_FLASH_PROTOCOL_EX_GUID,
    SPI_SMM_NOR_FLASH_PROTOCOL_EX_GUID, SpiConfiguration, SpiHcProtocol, SpiIo,
    SpiNorFlashProtocol, SpiNorFlashProtocolEx, SpiTransactionType, hc_attributes,
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
    assert_eq!(
        EFI_SPI_SMM_HC_PROTOCOL_GUID.to_u128_le(),
        0xcb3e_e7ff_2c5c_548a_4470_2093_e9f0_2217
    );
    assert_eq!(
        EFI_SPI_SMM_CONFIGURATION_PROTOCOL_GUID.to_u128_le(),
        0x59ef_9d9c_4cfd_aaa3_45fd_171b_995c_6eca
    );
    assert_eq!(
        EFI_SPI_SMM_NOR_FLASH_PROTOCOL_GUID.to_u128_le(),
        0x9a2c_666d_ff87_0486_4666_fe14_aab1_8f19
    );
    assert_eq!(
        SPI_SMM_NOR_FLASH_PROTOCOL_EX_GUID.to_u128_le(),
        0xa294_d428_e49c_3eb9_4ba5_caa9_6743_5ded
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

unsafe extern "efiapi" fn mock_io_transaction(
    _this: *const EfiSpiIoProtocol,
    transaction_type: SpiTransactionType,
    _debug_trans: bool,
    _clock_hz: u32,
    _bus_width: u32,
    _frame_size: u32,
    write_bytes: u32,
    write_buffer: *const u8,
    read_bytes: u32,
    read_buffer: *mut u8,
) -> EfiStatus {
    match transaction_type {
        SpiTransactionType::FullDuplex => {
            if write_bytes != read_bytes || write_buffer.is_null() || read_buffer.is_null() {
                return EfiStatus::BAD_BUFFER_SIZE;
            }
            for i in 0..read_bytes {
                unsafe { *read_buffer.add(i as usize) = *write_buffer.add(i as usize) ^ 0xFF };
            }
            EfiStatus::SUCCESS
        }
        SpiTransactionType::WriteOnly => {
            if write_bytes == 0 || write_buffer.is_null() {
                return EfiStatus::BAD_BUFFER_SIZE;
            }
            EfiStatus::SUCCESS
        }
        SpiTransactionType::ReadOnly => {
            if read_bytes == 0 || read_buffer.is_null() {
                return EfiStatus::BAD_BUFFER_SIZE;
            }
            for i in 0..read_bytes {
                unsafe { *read_buffer.add(i as usize) = 0xAA };
            }
            EfiStatus::SUCCESS
        }
        SpiTransactionType::WriteThenRead => {
            if write_bytes == 0
                || write_buffer.is_null()
                || read_bytes == 0
                || read_buffer.is_null()
            {
                return EfiStatus::BAD_BUFFER_SIZE;
            }
            for i in 0..read_bytes {
                unsafe { *read_buffer.add(i as usize) = 0x55 };
            }
            EfiStatus::SUCCESS
        }
    }
}

unsafe extern "efiapi" fn mock_io_update_peripheral(
    _this: *const EfiSpiIoProtocol,
    _spi_peripheral: *const c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_spi_io() {
    let raw = EfiSpiIoProtocol {
        spi_peripheral: core::ptr::null(),
        original_spi_peripheral: core::ptr::null(),
        frame_size_support_mask: 0x0000_0080,
        maximum_transfer_bytes: 4096,
        attributes: SPI_IO_SUPPORTS_4_BIT_DATA_BUS_WIDTH,
        legacy_spi_protocol: core::ptr::null(),
        transaction: mock_io_transaction,
        update_spi_peripheral: mock_io_update_peripheral,
    };

    let io = SpiIo::new(&raw);
    assert_eq!(io.maximum_transfer_bytes(), 4096);
    assert_eq!(io.frame_size_support_mask(), 0x0000_0080);
    assert_eq!(io.attributes(), SPI_IO_SUPPORTS_4_BIT_DATA_BUS_WIDTH);

    let tx = [0x01, 0x02, 0x03];
    let mut rx = [0u8; 3];
    assert!(io.full_duplex(&tx, &mut rx, 1, 8, 10_000_000).is_ok());
    assert_eq!(rx, [0xFE, 0xFD, 0xFC]);

    let mut mismatched_rx = [0u8; 2];
    assert_eq!(
        io.full_duplex(&tx, &mut mismatched_rx, 1, 8, 10_000_000)
            .err(),
        Some(EfiStatus::BAD_BUFFER_SIZE)
    );

    assert!(io.write_only(&tx, 1, 8, 10_000_000).is_ok());

    let mut read_data = [0u8; 4];
    assert!(io.read_only(&mut read_data, 1, 8, 10_000_000).is_ok());
    assert_eq!(read_data, [0xAA, 0xAA, 0xAA, 0xAA]);

    assert!(
        io.write_then_read(&tx, &mut read_data, 1, 8, 10_000_000)
            .is_ok()
    );
    assert_eq!(read_data, [0x55, 0x55, 0x55, 0x55]);
}

#[test]
fn test_spi_configuration() {
    let bus0 = EfiSpiBus {
        friendly_name: core::ptr::null(),
        peripheral_list: core::ptr::null(),
        controller_path: core::ptr::null(),
        clock: None,
        clock_parameter: core::ptr::null_mut(),
    };
    let bus_list = [core::ptr::from_ref(&bus0)];

    let raw = EfiSpiConfigurationProtocol {
        bus_count: 1,
        bus_list: bus_list.as_ptr(),
    };

    let cfg = SpiConfiguration::new(&raw);
    assert_eq!(cfg.bus_count(), 1);
    assert!(cfg.bus(0).is_some());
    assert!(cfg.bus(1).is_none());
}
