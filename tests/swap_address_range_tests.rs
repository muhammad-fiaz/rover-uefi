//! Integration tests for Swap Address Range protocol.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation
)]

use rover_uefi::abi::types::Bool;
use rover_uefi::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};
use rover_uefi::protocols::firmware::swap_address_range::{
    EfiSwapAddressRangeProtocol, SMM_SWAP_ADDRESS_RANGE_PROTOCOL_GUID,
    SWAP_ADDRESS_RANGE_PROTOCOL_GUID, SwapAddressRange, swap_lock,
};

#[test]
fn test_swap_address_range_guids_and_constants() {
    assert_eq!(
        SWAP_ADDRESS_RANGE_PROTOCOL_GUID,
        EfiGuid::new(
            0x1259_F60D,
            0xB754,
            0x468E,
            [0xA7, 0x89, 0x4D, 0xB8, 0x5D, 0x55, 0xE8, 0x7E]
        )
    );
    assert_eq!(
        SMM_SWAP_ADDRESS_RANGE_PROTOCOL_GUID,
        EfiGuid::new(
            0x67C4_F112,
            0x3385,
            0x4E55,
            [0x9C, 0x5B, 0xC0, 0x5B, 0x71, 0x7C, 0x42, 0x28]
        )
    );
    assert_eq!(swap_lock::UNSUPPORTED, 0);
    assert_eq!(swap_lock::SOFTWARE, 1);
    assert_eq!(swap_lock::HARDWARE, 2);
}

static mut MOCK_SWAP_STATE: Bool = 0;
static mut MOCK_LOCK_STATE: Bool = 0;

unsafe extern "efiapi" fn mock_get_range_location(
    _this: *mut EfiSwapAddressRangeProtocol,
    boot_block_base: *mut EfiPhysicalAddress,
    boot_block_size: *mut usize,
    backup_block_base: *mut EfiPhysicalAddress,
    backup_block_size: *mut usize,
) -> EfiStatus {
    unsafe {
        *boot_block_base = EfiPhysicalAddress::from_u64(0xFFE0_0000);
        *boot_block_size = 0x0010_0000;
        *backup_block_base = EfiPhysicalAddress::from_u64(0xFFF0_0000);
        *backup_block_size = 0x0010_0000;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_swap_state(
    _this: *mut EfiSwapAddressRangeProtocol,
    swap_state: *mut Bool,
) -> EfiStatus {
    unsafe {
        *swap_state = MOCK_SWAP_STATE;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_swap_state(
    _this: *mut EfiSwapAddressRangeProtocol,
    new_swap_state: Bool,
) -> EfiStatus {
    unsafe {
        MOCK_SWAP_STATE = new_swap_state;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_rtc_power_status(
    _this: *mut EfiSwapAddressRangeProtocol,
    rtc_power_failed: *mut Bool,
) -> EfiStatus {
    unsafe {
        *rtc_power_failed = 0;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_swap_lock_capability(
    _this: *mut EfiSwapAddressRangeProtocol,
    lock_capability: *mut u8,
) -> EfiStatus {
    unsafe {
        *lock_capability = swap_lock::HARDWARE | swap_lock::SOFTWARE;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_swap_lock(
    _this: *mut EfiSwapAddressRangeProtocol,
    _lock_capability: u8,
    new_lock_state: Bool,
) -> EfiStatus {
    unsafe {
        MOCK_LOCK_STATE = new_lock_state;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_swap_address_range_client() {
    let mut proto = EfiSwapAddressRangeProtocol {
        get_range_location: mock_get_range_location,
        get_swap_state: mock_get_swap_state,
        set_swap_state: mock_set_swap_state,
        get_rtc_power_status: mock_get_rtc_power_status,
        get_swap_lock_capability: mock_get_swap_lock_capability,
        set_swap_lock: mock_set_swap_lock,
    };

    let mut client = SwapAddressRange::new(&mut proto);
    let loc = client.get_range_location().unwrap();
    assert_eq!(loc.boot_block_base.to_u64(), 0xFFE0_0000);
    assert_eq!(loc.boot_block_size, 0x0010_0000);
    assert_eq!(loc.backup_block_base.to_u64(), 0xFFF0_0000);
    assert_eq!(loc.backup_block_size, 0x0010_0000);

    assert!(!client.is_swapped().unwrap());
    client.set_swapped(true).unwrap();
    assert!(client.is_swapped().unwrap());
    client.set_swapped(false).unwrap();
    assert!(!client.is_swapped().unwrap());

    assert!(!client.is_rtc_power_failed().unwrap());

    let caps = client.get_lock_capability().unwrap();
    assert_eq!(caps, swap_lock::HARDWARE | swap_lock::SOFTWARE);

    client.set_lock(swap_lock::HARDWARE, true).unwrap();
    assert_eq!(unsafe { MOCK_LOCK_STATE }, 1);
    client.set_lock(swap_lock::HARDWARE, false).unwrap();
    assert_eq!(unsafe { MOCK_LOCK_STATE }, 0);
}
