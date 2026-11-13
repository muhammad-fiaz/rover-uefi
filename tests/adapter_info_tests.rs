//! Integration tests for Adapter Information Protocol (AIP).

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::network::{
    AdapterInformation, EFI_ADAPTER_INFO_CDAT_TYPE_GUID, EFI_ADAPTER_INFO_MEDIA_STATE_GUID,
    EFI_ADAPTER_INFO_MEDIA_TYPE_ETHERNET, EFI_ADAPTER_INFO_MEDIA_TYPE_GUID,
    EFI_ADAPTER_INFO_MEDIA_TYPE_WIRELESS, EFI_ADAPTER_INFO_NETWORK_BOOT_GUID,
    EFI_ADAPTER_INFO_SAN_MAC_ADDRESS_GUID, EFI_ADAPTER_INFO_UNDI_IPV6_SUPPORT_GUID,
    EFI_ADAPTER_INFORMATION_PROTOCOL_GUID, EfiAdapterInfoMediaState, EfiAdapterInfoMediaType,
    EfiAdapterInfoNetworkBoot, EfiAdapterInfoSanMacAddress, EfiAdapterInfoUndiIpv6Support,
    EfiAdapterInformationProtocol, EfiMacAddress,
};

static mut MOCK_MEDIA_STATE: EfiAdapterInfoMediaState = EfiAdapterInfoMediaState {
    media_state: EfiStatus::SUCCESS,
};

static mut SUPPORTED_TYPES: [EfiGuid; 3] = [
    EFI_ADAPTER_INFO_MEDIA_STATE_GUID,
    EFI_ADAPTER_INFO_NETWORK_BOOT_GUID,
    EFI_ADAPTER_INFO_SAN_MAC_ADDRESS_GUID,
];

unsafe extern "efiapi" fn mock_get_information(
    _this: *const EfiAdapterInformationProtocol,
    information_type: *const EfiGuid,
    information_block: *mut *mut c_void,
    information_block_size: *mut UIntN,
) -> EfiStatus {
    if information_type.is_null() || information_block.is_null() || information_block_size.is_null()
    {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if *information_type == EFI_ADAPTER_INFO_MEDIA_STATE_GUID {
            *information_block = core::ptr::addr_of_mut!(MOCK_MEDIA_STATE).cast::<c_void>();
            *information_block_size = core::mem::size_of::<EfiAdapterInfoMediaState>() as UIntN;
            EfiStatus::SUCCESS
        } else {
            *information_block_size = 0;
            EfiStatus::NOT_FOUND
        }
    }
}

unsafe extern "efiapi" fn mock_set_information(
    _this: *const EfiAdapterInformationProtocol,
    information_type: *const EfiGuid,
    information_block: *const c_void,
    information_block_size: UIntN,
) -> EfiStatus {
    if information_type.is_null() || information_block.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        if *information_type == EFI_ADAPTER_INFO_MEDIA_STATE_GUID {
            if information_block_size == core::mem::size_of::<EfiAdapterInfoMediaState>() as UIntN {
                let state =
                    core::ptr::read_unaligned(information_block.cast::<EfiAdapterInfoMediaState>());
                MOCK_MEDIA_STATE = state;
                EfiStatus::SUCCESS
            } else {
                EfiStatus::BAD_BUFFER_SIZE
            }
        } else {
            EfiStatus::UNSUPPORTED
        }
    }
}

unsafe extern "efiapi" fn mock_get_supported_types(
    _this: *const EfiAdapterInformationProtocol,
    info_types_buffer: *mut *mut EfiGuid,
    info_types_buffer_count: *mut UIntN,
) -> EfiStatus {
    if info_types_buffer.is_null() || info_types_buffer_count.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *info_types_buffer = core::ptr::addr_of_mut!(SUPPORTED_TYPES[0]);
        *info_types_buffer_count = 3;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_adapter_info_guids_and_constants() {
    assert_eq!(
        EFI_ADAPTER_INFORMATION_PROTOCOL_GUID,
        EfiGuid::new(
            0xE5DD_1403,
            0xD622,
            0xC24E,
            [0x84, 0x88, 0xC7, 0x1B, 0x17, 0xF5, 0xE8, 0x02],
        )
    );
    assert_eq!(
        EFI_ADAPTER_INFO_MEDIA_STATE_GUID,
        EfiGuid::new(
            0xD7C7_4207,
            0xA831,
            0x4A26,
            [0xB1, 0xF5, 0xD1, 0x93, 0x06, 0x5C, 0xE8, 0xB6],
        )
    );
    assert_eq!(
        EFI_ADAPTER_INFO_NETWORK_BOOT_GUID,
        EfiGuid::new(
            0x1FBD_2960,
            0x4130,
            0x41E5,
            [0x94, 0xAC, 0xD2, 0xCF, 0x03, 0x7F, 0xB3, 0x7C],
        )
    );
    assert_eq!(
        EFI_ADAPTER_INFO_SAN_MAC_ADDRESS_GUID,
        EfiGuid::new(
            0x114D_A5EF,
            0x2CF1,
            0x4E12,
            [0x9B, 0xBB, 0xC4, 0x70, 0xB5, 0x52, 0x05, 0xD9],
        )
    );
    assert_eq!(
        EFI_ADAPTER_INFO_UNDI_IPV6_SUPPORT_GUID,
        EfiGuid::new(
            0x4BD5_6BE3,
            0x4975,
            0x4D8A,
            [0xA0, 0xAD, 0xC4, 0x91, 0x20, 0x4B, 0x5D, 0x4D],
        )
    );
    assert_eq!(
        EFI_ADAPTER_INFO_MEDIA_TYPE_GUID,
        EfiGuid::new(
            0x8484_472F,
            0x71EC,
            0x411A,
            [0xB3, 0x9C, 0x62, 0xCD, 0x94, 0xD9, 0x91, 0x6E],
        )
    );
    assert_eq!(
        EFI_ADAPTER_INFO_CDAT_TYPE_GUID,
        EfiGuid::new(
            0x77AF_24D1,
            0xB6F0,
            0x42B9,
            [0x83, 0xF5, 0x8F, 0xE6, 0xE8, 0x3E, 0xB6, 0xF0],
        )
    );

    assert_eq!(EFI_ADAPTER_INFO_MEDIA_TYPE_ETHERNET, 1);
    assert_eq!(EFI_ADAPTER_INFO_MEDIA_TYPE_WIRELESS, 2);
}

#[test]
fn test_adapter_info_structures() {
    let media_type = EfiAdapterInfoMediaType {
        media_type: EFI_ADAPTER_INFO_MEDIA_TYPE_ETHERNET,
    };
    assert_eq!(media_type.media_type, 1);

    let boot = EfiAdapterInfoNetworkBoot {
        iscsi_ipv4_boot_capability: true,
        iscsi_ipv6_boot_capability: false,
        fcoe_boot_capability: true,
        offload_capability: true,
        iscsi_mpio_capability: false,
        iscsi_ipv4_boot: true,
        iscsi_ipv6_boot: false,
        fcoe_boot: false,
    };
    assert!(boot.iscsi_ipv4_boot_capability);

    let san = EfiAdapterInfoSanMacAddress {
        san_mac_address: EfiMacAddress::from_mac6([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]),
    };
    assert_eq!(
        san.san_mac_address.to_mac6(),
        [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]
    );

    let undi = EfiAdapterInfoUndiIpv6Support { ipv6_support: true };
    assert!(undi.ipv6_support);
}

#[test]
fn test_adapter_information_client_wrapper() {
    let mock_proto = EfiAdapterInformationProtocol {
        get_information: mock_get_information,
        set_information: mock_set_information,
        get_supported_types: mock_get_supported_types,
    };

    let aip = AdapterInformation::new(&raw const mock_proto);

    // Get information
    let (block_ptr, block_size) = aip
        .get_information_raw(&EFI_ADAPTER_INFO_MEDIA_STATE_GUID)
        .unwrap_or((core::ptr::null_mut(), 0));
    assert!(!block_ptr.is_null());
    assert_eq!(block_size, core::mem::size_of::<EfiAdapterInfoMediaState>());

    let state = unsafe { core::ptr::read_unaligned(block_ptr.cast::<EfiAdapterInfoMediaState>()) };
    assert_eq!(state.media_state, EfiStatus::SUCCESS);

    // Unknown type returns error
    assert!(
        aip.get_information_raw(&EFI_ADAPTER_INFO_CDAT_TYPE_GUID)
            .is_err()
    );

    // Set information
    let new_state = EfiAdapterInfoMediaState {
        media_state: EfiStatus::NO_MEDIA,
    };
    let new_state_bytes = unsafe {
        core::slice::from_raw_parts(
            core::ptr::addr_of!(new_state).cast::<u8>(),
            core::mem::size_of::<EfiAdapterInfoMediaState>(),
        )
    };
    assert!(
        aip.set_information(&EFI_ADAPTER_INFO_MEDIA_STATE_GUID, new_state_bytes)
            .is_ok()
    );
    let current_media = unsafe { core::ptr::read(core::ptr::addr_of!(MOCK_MEDIA_STATE)) };
    assert_eq!(current_media.media_state, EfiStatus::NO_MEDIA);

    // Set unknown type
    assert!(
        aip.set_information(&EFI_ADAPTER_INFO_CDAT_TYPE_GUID, &[0u8; 4])
            .is_err()
    );

    // Get supported types
    let (types_ptr, count) = aip
        .get_supported_types_raw()
        .unwrap_or((core::ptr::null_mut(), 0));
    assert!(!types_ptr.is_null());
    assert_eq!(count, 3);
    let types_slice = unsafe { core::slice::from_raw_parts(types_ptr, count) };
    assert_eq!(types_slice[0], EFI_ADAPTER_INFO_MEDIA_STATE_GUID);
    assert_eq!(types_slice[1], EFI_ADAPTER_INFO_NETWORK_BOOT_GUID);
    assert_eq!(types_slice[2], EFI_ADAPTER_INFO_SAN_MAC_ADDRESS_GUID);
}
