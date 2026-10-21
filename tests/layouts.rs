//! Layout and value tests for the foundational `UEFI` types.
//!
//! The tests verify the sizes, alignments and field offsets
//! fixed by the specification, and check the status, identifier
//! and address conversions that depend on those layouts.

use core::mem::{align_of, offset_of, size_of};
use rover_uefi::abi::types::{EFI_PAGE_MASK, EFI_PAGE_SHIFT, EFI_PAGE_SIZE, size_to_pages};
use rover_uefi::base::event::EfiEvent;
use rover_uefi::base::guid::EfiGuid;
use rover_uefi::base::handle::EfiHandle;
use rover_uefi::base::physical_address::EfiPhysicalAddress;
use rover_uefi::base::status::{ERROR_BIT, EfiStatus, MAX_WARNING};
use rover_uefi::base::time::EfiTime;
use rover_uefi::base::types::{AllocationType, MemoryType};
use rover_uefi::base::virtual_address::EfiVirtualAddress;
use rover_uefi::tables::header::EfiTableHeader;

#[test]
fn guid_layout() {
    assert_eq!(size_of::<EfiGuid>(), 16);
    assert_eq!(align_of::<EfiGuid>(), 4);
    let guid = EfiGuid::new(
        0x1234_5678,
        0x9ABC,
        0xDEF0,
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
    );
    assert_eq!(offset_of!(EfiGuid, data1), 0);
    assert_eq!(offset_of!(EfiGuid, data2), 4);
    assert_eq!(offset_of!(EfiGuid, data3), 6);
    assert_eq!(offset_of!(EfiGuid, data4), 8);
    let bytes = guid.to_bytes();
    assert_eq!(bytes[0..4], [0x78, 0x56, 0x34, 0x12]);
    assert_eq!(bytes[4..6], [0xBC, 0x9A]);
    assert_eq!(bytes[6..8], [0xF0, 0xDE]);
    assert_eq!(EfiGuid::from_bytes(bytes), guid);
    assert!(EfiGuid::ZERO.is_zero());
    assert!(!guid.is_zero());
}

#[test]
fn status_encoding() {
    assert!(EfiStatus::SUCCESS.is_success());
    assert!(!EfiStatus::SUCCESS.is_error());
    assert!(!EfiStatus::SUCCESS.is_warning());
    assert!(EfiStatus::LOAD_ERROR.is_error());
    assert!(EfiStatus::INVALID_PARAMETER.is_error());
    assert!(EfiStatus::HTTP_ERROR.is_error());
    assert!(EfiStatus::WARN_RESET_REQUIRED.is_warning());
    assert!(!EfiStatus::WARN_RESET_REQUIRED.is_error());
    assert_eq!(EfiStatus::LOAD_ERROR.code(), 1);
    assert_eq!(EfiStatus::INVALID_PARAMETER.code(), 2);
    assert_eq!(EfiStatus::HTTP_ERROR.code(), 35);
    assert_eq!(
        EfiStatus::from_raw(ERROR_BIT),
        EfiStatus::from_raw(ERROR_BIT)
    );
    assert!(EfiStatus::from_raw(MAX_WARNING).is_warning());
}

#[test]
fn time_layout() {
    assert_eq!(size_of::<EfiTime>(), 16);
    assert_eq!(align_of::<EfiTime>(), 4);
    let time = EfiTime {
        year: 2026,
        month: 11,
        day: 7,
        hour: 4,
        minute: 30,
        second: 0,
        pad1: 0,
        nanosecond: 0,
        timezone: 0,
        daylight: 0,
        pad2: 0,
    };
    assert_eq!(offset_of!(EfiTime, year), 0);
    assert_eq!(offset_of!(EfiTime, month), 2);
    assert_eq!(offset_of!(EfiTime, day), 3);
    assert_eq!(offset_of!(EfiTime, hour), 4);
    assert_eq!(offset_of!(EfiTime, minute), 5);
    assert_eq!(offset_of!(EfiTime, second), 6);
    assert_eq!(offset_of!(EfiTime, pad1), 7);
    assert_eq!(offset_of!(EfiTime, nanosecond), 8);
    assert_eq!(offset_of!(EfiTime, timezone), 12);
    assert_eq!(offset_of!(EfiTime, daylight), 14);
    assert_eq!(offset_of!(EfiTime, pad2), 15);
    assert_eq!(time.year, 2026);
    assert_eq!(time.month, 11);
    assert_eq!(time.pad1, 0);
    assert_eq!(time.pad2, 0);
}

#[test]
fn table_header_layout() {
    assert_eq!(size_of::<EfiTableHeader>(), 24);
    assert_eq!(align_of::<EfiTableHeader>(), 8);
    assert_eq!(offset_of!(EfiTableHeader, signature), 0);
    assert_eq!(offset_of!(EfiTableHeader, revision), 8);
    assert_eq!(offset_of!(EfiTableHeader, header_size), 12);
    assert_eq!(offset_of!(EfiTableHeader, crc32), 16);
    assert_eq!(offset_of!(EfiTableHeader, reserved), 20);
}

#[test]
fn handle_event_addresses() {
    assert_eq!(size_of::<EfiHandle>(), size_of::<usize>());
    assert_eq!(size_of::<EfiEvent>(), size_of::<usize>());
    assert_eq!(size_of::<EfiPhysicalAddress>(), 8);
    assert_eq!(size_of::<EfiVirtualAddress>(), 8);
    assert!(EfiHandle::NULL.is_null());
    assert!(EfiEvent::NULL.is_null());
    assert!(EfiPhysicalAddress::NULL.is_null());
    assert!(EfiVirtualAddress::NULL.is_null());
    assert_eq!(EfiPhysicalAddress::from_u64(0x1234).to_u64(), 0x1234);
    assert_eq!(EfiVirtualAddress::from_u64(0x5678).to_u64(), 0x5678);
}

#[test]
fn page_conversions() {
    assert_eq!(EFI_PAGE_SIZE, 4096);
    assert_eq!(EFI_PAGE_SHIFT, 12);
    assert_eq!(EFI_PAGE_MASK, 0xFFF);
    assert_eq!(size_to_pages(0), 0);
    assert_eq!(size_to_pages(1), 1);
    assert_eq!(size_to_pages(4096), 1);
    assert_eq!(size_to_pages(4097), 2);
}

#[test]
fn memory_types() {
    assert_eq!(MemoryType::LOADER_CODE.to_raw(), 1);
    assert_eq!(MemoryType::PERSISTENT_MEMORY.to_raw(), 13);
    assert_eq!(MemoryType::UNACCEPTED_MEMORY.to_raw(), 14);
    assert_eq!(MemoryType::from_raw_checked(0), Some(MemoryType::RESERVED));
    assert_eq!(MemoryType::from_raw_checked(15), None);
    assert_eq!(MemoryType::from_raw_checked(0x8000_0000), None);
    assert_eq!(AllocationType::ANY_PAGES.to_raw(), 0);
    assert_eq!(AllocationType::ADDRESS.to_raw(), 2);
    assert_eq!(AllocationType::from_raw_checked(3), None);
}
