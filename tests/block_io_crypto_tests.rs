//! Integration tests for Block I/O Crypto Protocol.

use core::ffi::c_void;
use rover_uefi::base::{EfiEvent, EfiGuid, EfiStatus};
use rover_uefi::protocols::storage::{
    BlockIoCrypto, EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_CBC_BITLOCKER,
    EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_XTS, EFI_BLOCK_IO_CRYPTO_INDEX_ANY,
    EFI_BLOCK_IO_CRYPTO_PROTOCOL_GUID, EfiBlockIoCryptoCapabilities, EfiBlockIoCryptoCapability,
    EfiBlockIoCryptoConfigurationTableEntry, EfiBlockIoCryptoIvInput,
    EfiBlockIoCryptoIvInputAesCbcBitlocker, EfiBlockIoCryptoIvInputAesXts,
    EfiBlockIoCryptoProtocol, EfiBlockIoCryptoResponseConfigurationEntry, EfiBlockIoCryptoToken,
    EfiBlockIoMedia,
};

// --- Mock implementation ---

unsafe extern "efiapi" fn mock_crypto_reset(
    _this: *const EfiBlockIoCryptoProtocol,
    _extended_verification: bool,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_crypto_get_capabilities(
    _this: *const EfiBlockIoCryptoProtocol,
    capabilities: *mut EfiBlockIoCryptoCapabilities,
) -> EfiStatus {
    if capabilities.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        (*capabilities).supported = true;
        (*capabilities).key_count = 16;
        (*capabilities).capability_count = 2;
    }
    EfiStatus::SUCCESS
}

#[allow(clippy::cast_possible_truncation)]
unsafe extern "efiapi" fn mock_crypto_set_configuration(
    _this: *const EfiBlockIoCryptoProtocol,
    configuration_count: u64,
    configuration_table: *const EfiBlockIoCryptoConfigurationTableEntry,
    resulting_table: *mut EfiBlockIoCryptoResponseConfigurationEntry,
) -> EfiStatus {
    if configuration_count == 0 || configuration_table.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if !resulting_table.is_null() {
        unsafe {
            for i in 0..configuration_count as usize {
                let entry = &*configuration_table.add(i);
                *resulting_table.add(i) = EfiBlockIoCryptoResponseConfigurationEntry {
                    index: entry.index,
                    key_owner_guid: entry.key_owner_guid,
                    capability: entry.capability,
                };
            }
        }
    }
    EfiStatus::SUCCESS
}

#[allow(clippy::cast_possible_truncation)]
unsafe extern "efiapi" fn mock_crypto_get_configuration(
    _this: *const EfiBlockIoCryptoProtocol,
    start_index: u64,
    configuration_count: u64,
    _key_owner_guid: *const EfiGuid,
    configuration_table: *mut EfiBlockIoCryptoResponseConfigurationEntry,
) -> EfiStatus {
    if configuration_count == 0 || configuration_table.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        for i in 0..configuration_count as usize {
            *configuration_table.add(i) = EfiBlockIoCryptoResponseConfigurationEntry {
                index: start_index + i as u64,
                key_owner_guid: EFI_BLOCK_IO_CRYPTO_PROTOCOL_GUID,
                capability: EfiBlockIoCryptoCapability {
                    algorithm: EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_XTS,
                    key_size: 256,
                    crypto_block_size_bit_mask: 0x200,
                },
            };
        }
    }
    EfiStatus::SUCCESS
}

#[allow(clippy::cast_possible_truncation)]
unsafe extern "efiapi" fn mock_crypto_read_extended(
    _this: *const EfiBlockIoCryptoProtocol,
    _media_id: u32,
    _lba: u64,
    token: *mut EfiBlockIoCryptoToken,
    buffer_size: u64,
    buffer: *mut c_void,
    _index: *const u64,
    _crypto_iv_input: *const c_void,
) -> EfiStatus {
    if buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        let slice = core::slice::from_raw_parts_mut(buffer.cast::<u8>(), buffer_size as usize);
        slice.fill(0xAA);
        if !token.is_null() {
            (*token).transaction_status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_crypto_write_extended(
    _this: *const EfiBlockIoCryptoProtocol,
    _media_id: u32,
    _lba: u64,
    token: *mut EfiBlockIoCryptoToken,
    _buffer_size: u64,
    buffer: *const c_void,
    _index: *const u64,
    _crypto_iv_input: *const c_void,
) -> EfiStatus {
    if buffer.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if !token.is_null() {
            (*token).transaction_status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_crypto_flush(
    _this: *const EfiBlockIoCryptoProtocol,
    token: *mut EfiBlockIoCryptoToken,
) -> EfiStatus {
    unsafe {
        if !token.is_null() {
            (*token).transaction_status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_block_io_crypto_guids_and_constants() {
    assert_eq!(
        EFI_BLOCK_IO_CRYPTO_PROTOCOL_GUID,
        EfiGuid::new(
            0xA004_90BA,
            0x3F1A,
            0x4B4C,
            [0xAB, 0x90, 0x4F, 0xA9, 0x97, 0x26, 0xA1, 0xE8],
        )
    );
    assert_eq!(
        EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_XTS,
        EfiGuid::new(
            0x2F87_BA6A,
            0x5C04,
            0x4385,
            [0xA7, 0x80, 0xF3, 0xBF, 0x78, 0xA9, 0x7B, 0xEC],
        )
    );
    assert_eq!(
        EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_CBC_BITLOCKER,
        EfiGuid::new(
            0x689E_4C62,
            0x70BF,
            0x4CF3,
            [0x88, 0xBB, 0x33, 0xB3, 0x18, 0x26, 0x86, 0x70],
        )
    );
    assert_eq!(EFI_BLOCK_IO_CRYPTO_INDEX_ANY, 0xFFFF_FFFF_FFFF_FFFF);
}

#[test]
fn test_block_io_crypto_structures() {
    let iv_xts = EfiBlockIoCryptoIvInputAesXts {
        header: EfiBlockIoCryptoIvInput {
            input_size: core::mem::size_of::<EfiBlockIoCryptoIvInputAesXts>() as u64,
        },
        crypto_block_number: 100,
        crypto_block_byte_size: 512,
    };
    assert_eq!(iv_xts.crypto_block_number, 100);

    let iv_cbc = EfiBlockIoCryptoIvInputAesCbcBitlocker {
        header: EfiBlockIoCryptoIvInput {
            input_size: core::mem::size_of::<EfiBlockIoCryptoIvInputAesCbcBitlocker>() as u64,
        },
        crypto_block_byte_offset: 2048,
        crypto_block_byte_size: 512,
    };
    assert_eq!(iv_cbc.crypto_block_byte_offset, 2048);
}

#[test]
fn test_block_io_crypto_client_wrapper() {
    let mock_media = EfiBlockIoMedia {
        media_id: 1,
        removable_media: 0,
        media_present: 1,
        logical_partition: 0,
        read_only: 0,
        write_caching: 1,
        block_size: 512,
        io_align: 4,
        last_block: 100_000,
        lowest_aligned_lba: 0,
        logical_blocks_per_physical_block: 1,
        optimal_transfer_length_granularity: 1,
    };

    let mock_proto = EfiBlockIoCryptoProtocol {
        media: &raw const mock_media,
        reset: mock_crypto_reset,
        get_capabilities: mock_crypto_get_capabilities,
        set_configuration: mock_crypto_set_configuration,
        get_configuration: mock_crypto_get_configuration,
        read_extended: mock_crypto_read_extended,
        write_extended: mock_crypto_write_extended,
        flush_blocks: mock_crypto_flush,
    };

    let client = BlockIoCrypto::new(&raw const mock_proto);
    assert_eq!(client.raw(), &raw const mock_proto);

    assert!(client.media().is_some());
    if let Some(media) = client.media() {
        assert_eq!(media.block_size, 512);
    }

    assert!(client.reset(false).is_ok());

    let mut caps = EfiBlockIoCryptoCapabilities {
        supported: false,
        key_count: 0,
        capability_count: 0,
    };
    assert!(client.get_capabilities(&mut caps).is_ok());
    assert!(caps.supported);
    assert_eq!(caps.key_count, 16);

    let dummy_key = [0x42u8; 32];
    let config_entry = EfiBlockIoCryptoConfigurationTableEntry {
        index: 0,
        key_owner_guid: EFI_BLOCK_IO_CRYPTO_PROTOCOL_GUID,
        capability: EfiBlockIoCryptoCapability {
            algorithm: EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_XTS,
            key_size: 256,
            crypto_block_size_bit_mask: 0x200,
        },
        crypto_key: dummy_key.as_ptr().cast::<c_void>(),
    };
    let mut result_entries = [EfiBlockIoCryptoResponseConfigurationEntry {
        index: 99,
        key_owner_guid: EfiGuid::from_bytes([0u8; 16]),
        capability: EfiBlockIoCryptoCapability::default(),
    }];

    assert!(
        client
            .set_configuration(&[config_entry], Some(&mut result_entries))
            .is_ok()
    );
    assert_eq!(result_entries[0].index, 0);

    let mut get_entries = [EfiBlockIoCryptoResponseConfigurationEntry {
        index: 99,
        key_owner_guid: EfiGuid::from_bytes([0u8; 16]),
        capability: EfiBlockIoCryptoCapability::default(),
    }];
    assert!(client.get_configuration(0, &mut get_entries, None).is_ok());
    assert_eq!(get_entries[0].index, 0);

    let mut token = EfiBlockIoCryptoToken {
        event: EfiEvent::from_ptr(core::ptr::null_mut()),
        transaction_status: EfiStatus::NOT_READY,
    };

    let mut read_buf = [0u8; 512];
    assert!(
        client
            .read_extended(1, 0, &mut token, &mut read_buf, Some(0), None)
            .is_ok()
    );
    assert_eq!(read_buf[0], 0xAA);
    assert_eq!(token.transaction_status, EfiStatus::SUCCESS);

    let write_buf = [0xBBu8; 512];
    assert!(
        client
            .write_extended(1, 0, &mut token, &write_buf, Some(0), None)
            .is_ok()
    );
    assert_eq!(token.transaction_status, EfiStatus::SUCCESS);

    assert!(client.flush_blocks(&mut token).is_ok());
}
