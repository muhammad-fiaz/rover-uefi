//! Unit tests for boot services helpers.
//!
//! The tests cover pure validation and parsing
//! helpers that do not require firmware. Firmware
//! calls themselves are verified only structurally
//! until firmware execution is available.

use rover_uefi::abi::types::{EFI_PAGE_MASK, EFI_PAGE_SIZE, pages_to_size, size_to_pages};
use rover_uefi::base::{EfiStatus, MemoryType, ResetType, TimerDelay};
use rover_uefi::services::boot::events::{
    EVT_NOTIFY_SIGNAL, EVT_NOTIFY_WAIT, EVT_RUNTIME, EVT_TIMER,
};
use rover_uefi::services::boot::memory::{memory_map_descriptors, validate_memory_map_buffer};
use rover_uefi::tables::boot::{OPEN_PROTOCOL_BY_DRIVER, OPEN_PROTOCOL_EXCLUSIVE};

#[test]
fn page_helpers() {
    assert_eq!(EFI_PAGE_SIZE, 4096);
    assert_eq!(EFI_PAGE_MASK, 0xFFF);
    assert_eq!(size_to_pages(0), 0);
    assert_eq!(size_to_pages(4096), 1);
    assert_eq!(size_to_pages(4097), 2);
    assert_eq!(pages_to_size(1), Some(4096));
    assert_eq!(pages_to_size(0), Some(0));
}

#[test]
fn memory_map_validation() {
    assert!(validate_memory_map_buffer(80, 40));
    assert!(validate_memory_map_buffer(0, 40));
    assert!(!validate_memory_map_buffer(81, 40));
    assert!(!validate_memory_map_buffer(80, 0));
}

#[test]
fn memory_map_iteration() {
    let mut buffer = [0_u8; 80];
    buffer[0..4].copy_from_slice(&7_u32.to_le_bytes());
    buffer[8..16].copy_from_slice(&0x1000_u64.to_le_bytes());
    buffer[24..32].copy_from_slice(&2_u64.to_le_bytes());
    buffer[32..40].copy_from_slice(&0x8000_0000_0000_0000_u64.to_le_bytes());
    buffer[40..44].copy_from_slice(&8_u32.to_le_bytes());
    buffer[48..56].copy_from_slice(&0x2000_u64.to_le_bytes());
    buffer[64..72].copy_from_slice(&1_u64.to_le_bytes());
    let Some(mut iter) = memory_map_descriptors(&buffer, 40) else {
        panic!("expected an iterator");
    };
    let Some(first) = iter.next() else {
        panic!("expected a first descriptor");
    };
    assert_eq!(first.memory_type(), MemoryType::UNUSABLE_MEMORY);
    assert_eq!(first.physical_start().to_u64(), 0x1000);
    assert_eq!(first.number_of_pages(), 2);
    assert_eq!(first.attribute(), 0x8000_0000_0000_0000);
    let Some(second) = iter.next() else {
        panic!("expected a second descriptor");
    };
    assert_eq!(second.memory_type(), MemoryType::ACPI_RECLAIM_MEMORY);
    assert_eq!(second.physical_start().to_u64(), 0x2000);
    assert!(iter.next().is_none());
    assert!(memory_map_descriptors(&buffer, 0).is_none());
}

#[test]
fn event_and_protocol_constants() {
    assert_eq!(EVT_TIMER, 0x8000_0000);
    assert_eq!(EVT_RUNTIME, 0x4000_0000);
    assert_eq!(EVT_NOTIFY_WAIT, 0x0000_0100);
    assert_eq!(EVT_NOTIFY_SIGNAL, 0x0000_0200);
    assert_eq!(OPEN_PROTOCOL_BY_DRIVER, 0x10);
    assert_eq!(OPEN_PROTOCOL_EXCLUSIVE, 0x20);
}

#[test]
fn small_enum_checks() {
    assert_eq!(TimerDelay::CANCEL.to_raw(), 0);
    assert_eq!(TimerDelay::from_raw_checked(3), None);
    assert_eq!(ResetType::COLD.to_raw(), 0);
    assert_eq!(EfiStatus::SUCCESS.code(), 0);
}

#[test]
fn status_isolation() {
    assert!(EfiStatus::INVALID_PARAMETER.is_error());
    assert!(EfiStatus::WARN_BUFFER_TOO_SMALL.is_warning());
    assert!(EfiStatus::SUCCESS.is_success());
}
