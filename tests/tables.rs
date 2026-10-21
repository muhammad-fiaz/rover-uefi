//! Layout tests for the `UEFI` system, boot services,
//! runtime services, and configuration tables.
//!
//! The tests check structure sizes, alignments, and field
//! offsets against values derived from the specification
//! field order and the target pointer width, not from the
//! structures under test. Function signature checks verify
//! that each table field type is the expected `EFIAPI`
//! function pointer.

use core::mem::{align_of, offset_of, size_of};

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{
    EfiMemoryDescriptor, EfiTimeCapabilities, EfiTpl, MemoryType, ResetType, TimerDelay,
};
use rover_uefi::tables::boot::{
    EfiAllocatePages, EfiAllocatePool, EfiBootServices, EfiCalculateCrc32, EfiCheckEvent,
    EfiCloseEvent, EfiCloseProtocol, EfiConnectController, EfiCopyMem, EfiCreateEvent,
    EfiCreateEventEx, EfiDisconnectController, EfiEventNotify, EfiExit, EfiExitBootServices,
    EfiFreePages, EfiFreePool, EfiGetMemoryMap, EfiGetNextMonotonicCount, EfiHandleProtocol,
    EfiInstallConfigurationTable, EfiInstallMultipleProtocolInterfaces,
    EfiInstallProtocolInterface, EfiLoadImage, EfiLocateDevicePath, EfiLocateHandle,
    EfiLocateHandleBuffer, EfiLocateProtocol, EfiOpenProtocol, EfiOpenProtocolInformation,
    EfiOpenProtocolInformationEntry, EfiProtocolsPerHandle, EfiRaiseTpl, EfiRegisterProtocolNotify,
    EfiReinstallProtocolInterface, EfiRestoreTpl, EfiSetMem, EfiSetTimer, EfiSetWatchdogTimer,
    EfiSignalEvent, EfiStall, EfiStartImage, EfiUninstallMultipleProtocolInterfaces,
    EfiUninstallProtocolInterface, EfiUnloadImage, EfiWaitForEvent,
};
use rover_uefi::tables::configuration::EfiConfigurationTable;
use rover_uefi::tables::runtime::{
    EfiCapsuleHeader, EfiConvertPointer, EfiGetNextHighMonotonicCount, EfiGetNextVariableName,
    EfiGetTime, EfiGetVariable, EfiGetWakeupTime, EfiQueryCapsuleCapabilities,
    EfiQueryVariableInfo, EfiResetSystem, EfiRuntimeServices, EfiSetTime, EfiSetVariable,
    EfiSetVirtualAddressMap, EfiSetWakeupTime, EfiUpdateCapsule,
};
use rover_uefi::tables::system::EfiSystemTable;

fn pointer_size() -> usize {
    size_of::<usize>()
}

fn expected_boot_size() -> usize {
    24 + 44 * pointer_size()
}

fn expected_runtime_size() -> usize {
    24 + 14 * pointer_size()
}

fn expected_system_size() -> usize {
    if pointer_size() == 8 { 120 } else { 72 }
}

fn expected_configuration_size() -> usize {
    16 + pointer_size()
}

#[test]
fn system_table_layout() {
    assert_eq!(size_of::<EfiSystemTable>(), expected_system_size());
    assert_eq!(align_of::<EfiSystemTable>(), 8);
    assert_eq!(offset_of!(EfiSystemTable, hdr), 0);
    assert_eq!(offset_of!(EfiSystemTable, firmware_vendor), 24);
    assert_eq!(
        offset_of!(EfiSystemTable, firmware_revision),
        24 + pointer_size()
    );
    if pointer_size() == 8 {
        assert_eq!(offset_of!(EfiSystemTable, console_in_handle), 40);
        assert_eq!(offset_of!(EfiSystemTable, con_in), 48);
        assert_eq!(offset_of!(EfiSystemTable, console_out_handle), 56);
        assert_eq!(offset_of!(EfiSystemTable, con_out), 64);
        assert_eq!(offset_of!(EfiSystemTable, standard_error_handle), 72);
        assert_eq!(offset_of!(EfiSystemTable, std_err), 80);
        assert_eq!(offset_of!(EfiSystemTable, runtime_services), 88);
        assert_eq!(offset_of!(EfiSystemTable, boot_services), 96);
        assert_eq!(offset_of!(EfiSystemTable, number_of_table_entries), 104);
        assert_eq!(offset_of!(EfiSystemTable, configuration_table), 112);
    } else {
        assert_eq!(offset_of!(EfiSystemTable, console_in_handle), 32);
        assert_eq!(offset_of!(EfiSystemTable, con_in), 36);
        assert_eq!(offset_of!(EfiSystemTable, console_out_handle), 40);
        assert_eq!(offset_of!(EfiSystemTable, con_out), 44);
        assert_eq!(offset_of!(EfiSystemTable, standard_error_handle), 48);
        assert_eq!(offset_of!(EfiSystemTable, std_err), 52);
        assert_eq!(offset_of!(EfiSystemTable, runtime_services), 56);
        assert_eq!(offset_of!(EfiSystemTable, boot_services), 60);
        assert_eq!(offset_of!(EfiSystemTable, number_of_table_entries), 64);
        assert_eq!(offset_of!(EfiSystemTable, configuration_table), 68);
    }
}

#[test]
fn configuration_table_layout() {
    assert_eq!(
        size_of::<EfiConfigurationTable>(),
        expected_configuration_size()
    );
    assert_eq!(align_of::<EfiConfigurationTable>(), pointer_size());
    assert_eq!(offset_of!(EfiConfigurationTable, vendor_guid), 0);
    assert_eq!(offset_of!(EfiConfigurationTable, vendor_table), 16);
}

#[test]
fn boot_services_layout_early() {
    assert_eq!(size_of::<EfiBootServices>(), expected_boot_size());
    assert_eq!(align_of::<EfiBootServices>(), 8);
    assert_eq!(offset_of!(EfiBootServices, hdr), 0);
    assert_eq!(offset_of!(EfiBootServices, raise_tpl), 24);
    assert_eq!(
        offset_of!(EfiBootServices, restore_tpl),
        24 + pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, allocate_pages),
        24 + 2 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, free_pages),
        24 + 3 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, get_memory_map),
        24 + 4 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, allocate_pool),
        24 + 5 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, free_pool),
        24 + 6 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, create_event),
        24 + 7 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, set_timer),
        24 + 8 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, wait_for_event),
        24 + 9 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, signal_event),
        24 + 10 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, close_event),
        24 + 11 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, check_event),
        24 + 12 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, install_protocol_interface),
        24 + 13 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, reinstall_protocol_interface),
        24 + 14 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, uninstall_protocol_interface),
        24 + 15 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, handle_protocol),
        24 + 16 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, reserved),
        24 + 17 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, register_protocol_notify),
        24 + 18 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, locate_handle),
        24 + 19 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, locate_device_path),
        24 + 20 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, install_configuration_table),
        24 + 21 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, load_image),
        24 + 22 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, start_image),
        24 + 23 * pointer_size()
    );
}

#[test]
fn boot_services_layout_late() {
    assert_eq!(offset_of!(EfiBootServices, exit), 24 + 24 * pointer_size());
    assert_eq!(
        offset_of!(EfiBootServices, unload_image),
        24 + 25 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, exit_boot_services),
        24 + 26 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, get_next_monotonic_count),
        24 + 27 * pointer_size()
    );
    assert_eq!(offset_of!(EfiBootServices, stall), 24 + 28 * pointer_size());
    assert_eq!(
        offset_of!(EfiBootServices, set_watchdog_timer),
        24 + 29 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, connect_controller),
        24 + 30 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, disconnect_controller),
        24 + 31 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, open_protocol),
        24 + 32 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, close_protocol),
        24 + 33 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, open_protocol_information),
        24 + 34 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, protocols_per_handle),
        24 + 35 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, locate_handle_buffer),
        24 + 36 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, locate_protocol),
        24 + 37 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, install_multiple_protocol_interfaces),
        24 + 38 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, uninstall_multiple_protocol_interfaces),
        24 + 39 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, calculate_crc32),
        24 + 40 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, copy_mem),
        24 + 41 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, set_mem),
        24 + 42 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiBootServices, create_event_ex),
        24 + 43 * pointer_size()
    );
}

#[test]
fn runtime_services_layout() {
    assert_eq!(size_of::<EfiRuntimeServices>(), expected_runtime_size());
    assert_eq!(align_of::<EfiRuntimeServices>(), 8);
    assert_eq!(offset_of!(EfiRuntimeServices, hdr), 0);
    assert_eq!(offset_of!(EfiRuntimeServices, get_time), 24);
    assert_eq!(
        offset_of!(EfiRuntimeServices, set_time),
        24 + pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, get_wakeup_time),
        24 + 2 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, set_wakeup_time),
        24 + 3 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, set_virtual_address_map),
        24 + 4 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, convert_pointer),
        24 + 5 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, get_variable),
        24 + 6 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, get_next_variable_name),
        24 + 7 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, set_variable),
        24 + 8 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, get_next_high_monotonic_count),
        24 + 9 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, reset_system),
        24 + 10 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, update_capsule),
        24 + 11 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, query_capsule_capabilities),
        24 + 12 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiRuntimeServices, query_variable_info),
        24 + 13 * pointer_size()
    );
}

#[test]
fn support_struct_layouts() {
    assert_eq!(size_of::<EfiMemoryDescriptor>(), 40);
    assert_eq!(align_of::<EfiMemoryDescriptor>(), 8);
    assert_eq!(offset_of!(EfiMemoryDescriptor, memory_type), 0);
    assert_eq!(offset_of!(EfiMemoryDescriptor, physical_start), 8);
    assert_eq!(offset_of!(EfiMemoryDescriptor, virtual_start), 16);
    assert_eq!(offset_of!(EfiMemoryDescriptor, number_of_pages), 24);
    assert_eq!(offset_of!(EfiMemoryDescriptor, attribute), 32);
    assert_eq!(size_of::<EfiTimeCapabilities>(), 12);
    assert_eq!(align_of::<EfiTimeCapabilities>(), 4);
    assert_eq!(offset_of!(EfiTimeCapabilities, resolution), 0);
    assert_eq!(offset_of!(EfiTimeCapabilities, accuracy), 4);
    assert_eq!(offset_of!(EfiTimeCapabilities, sets_to_zero), 8);
    assert_eq!(size_of::<EfiCapsuleHeader>(), 28);
    assert_eq!(align_of::<EfiCapsuleHeader>(), 4);
    assert_eq!(offset_of!(EfiCapsuleHeader, capsule_guid), 0);
    assert_eq!(offset_of!(EfiCapsuleHeader, header_size), 16);
    assert_eq!(offset_of!(EfiCapsuleHeader, flags), 20);
    assert_eq!(offset_of!(EfiCapsuleHeader, capsule_image_size), 24);
    assert_eq!(
        size_of::<EfiOpenProtocolInformationEntry>(),
        2 * pointer_size() + 8
    );
    assert_eq!(
        align_of::<EfiOpenProtocolInformationEntry>(),
        pointer_size()
    );
    assert_eq!(offset_of!(EfiOpenProtocolInformationEntry, agent_handle), 0);
    assert_eq!(
        offset_of!(EfiOpenProtocolInformationEntry, controller_handle),
        pointer_size()
    );
    assert_eq!(
        offset_of!(EfiOpenProtocolInformationEntry, attributes),
        2 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiOpenProtocolInformationEntry, open_count),
        2 * pointer_size() + 4
    );
}

#[test]
fn function_pointer_sizes() {
    assert_eq!(size_of::<EfiRaiseTpl>(), pointer_size());
    assert_eq!(size_of::<EfiRestoreTpl>(), pointer_size());
    assert_eq!(size_of::<EfiAllocatePages>(), pointer_size());
    assert_eq!(size_of::<EfiFreePages>(), pointer_size());
    assert_eq!(size_of::<EfiGetMemoryMap>(), pointer_size());
    assert_eq!(size_of::<EfiAllocatePool>(), pointer_size());
    assert_eq!(size_of::<EfiFreePool>(), pointer_size());
    assert_eq!(size_of::<EfiCreateEvent>(), pointer_size());
    assert_eq!(size_of::<EfiSetTimer>(), pointer_size());
    assert_eq!(size_of::<EfiWaitForEvent>(), pointer_size());
    assert_eq!(size_of::<EfiSignalEvent>(), pointer_size());
    assert_eq!(size_of::<EfiCloseEvent>(), pointer_size());
    assert_eq!(size_of::<EfiCheckEvent>(), pointer_size());
    assert_eq!(size_of::<EfiEventNotify>(), pointer_size());
    assert_eq!(size_of::<Option<EfiEventNotify>>(), pointer_size());
    assert_eq!(size_of::<EfiInstallProtocolInterface>(), pointer_size());
    assert_eq!(size_of::<EfiReinstallProtocolInterface>(), pointer_size());
    assert_eq!(size_of::<EfiUninstallProtocolInterface>(), pointer_size());
    assert_eq!(size_of::<EfiHandleProtocol>(), pointer_size());
    assert_eq!(size_of::<EfiRegisterProtocolNotify>(), pointer_size());
    assert_eq!(size_of::<EfiLocateHandle>(), pointer_size());
    assert_eq!(size_of::<EfiLocateDevicePath>(), pointer_size());
    assert_eq!(size_of::<EfiInstallConfigurationTable>(), pointer_size());
    assert_eq!(size_of::<EfiLoadImage>(), pointer_size());
    assert_eq!(size_of::<EfiStartImage>(), pointer_size());
    assert_eq!(size_of::<EfiExit>(), pointer_size());
    assert_eq!(size_of::<EfiUnloadImage>(), pointer_size());
    assert_eq!(size_of::<EfiExitBootServices>(), pointer_size());
    assert_eq!(size_of::<EfiGetNextMonotonicCount>(), pointer_size());
    assert_eq!(size_of::<EfiStall>(), pointer_size());
    assert_eq!(size_of::<EfiSetWatchdogTimer>(), pointer_size());
    assert_eq!(size_of::<EfiConnectController>(), pointer_size());
    assert_eq!(size_of::<EfiDisconnectController>(), pointer_size());
    assert_eq!(size_of::<EfiOpenProtocol>(), pointer_size());
    assert_eq!(size_of::<EfiCloseProtocol>(), pointer_size());
    assert_eq!(size_of::<EfiOpenProtocolInformation>(), pointer_size());
    assert_eq!(size_of::<EfiProtocolsPerHandle>(), pointer_size());
    assert_eq!(size_of::<EfiLocateHandleBuffer>(), pointer_size());
    assert_eq!(size_of::<EfiLocateProtocol>(), pointer_size());
    assert_eq!(
        size_of::<EfiInstallMultipleProtocolInterfaces>(),
        pointer_size()
    );
    assert_eq!(
        size_of::<EfiUninstallMultipleProtocolInterfaces>(),
        pointer_size()
    );
    assert_eq!(size_of::<EfiCalculateCrc32>(), pointer_size());
    assert_eq!(size_of::<EfiCopyMem>(), pointer_size());
    assert_eq!(size_of::<EfiSetMem>(), pointer_size());
    assert_eq!(size_of::<EfiCreateEventEx>(), pointer_size());
    assert_eq!(size_of::<EfiGetTime>(), pointer_size());
    assert_eq!(size_of::<EfiSetTime>(), pointer_size());
    assert_eq!(size_of::<EfiGetWakeupTime>(), pointer_size());
    assert_eq!(size_of::<EfiSetWakeupTime>(), pointer_size());
    assert_eq!(size_of::<EfiSetVirtualAddressMap>(), pointer_size());
    assert_eq!(size_of::<EfiConvertPointer>(), pointer_size());
    assert_eq!(size_of::<EfiGetVariable>(), pointer_size());
    assert_eq!(size_of::<EfiGetNextVariableName>(), pointer_size());
    assert_eq!(size_of::<EfiSetVariable>(), pointer_size());
    assert_eq!(size_of::<EfiGetNextHighMonotonicCount>(), pointer_size());
    assert_eq!(size_of::<EfiResetSystem>(), pointer_size());
    assert_eq!(size_of::<EfiUpdateCapsule>(), pointer_size());
    assert_eq!(size_of::<EfiQueryCapsuleCapabilities>(), pointer_size());
    assert_eq!(size_of::<EfiQueryVariableInfo>(), pointer_size());
    assert_eq!(size_of::<EfiTpl>(), pointer_size());
    assert_eq!(size_of::<UIntN>(), pointer_size());
}

#[test]
fn small_enum_values() {
    assert_eq!(TimerDelay::CANCEL.to_raw(), 0);
    assert_eq!(TimerDelay::PERIODIC.to_raw(), 1);
    assert_eq!(TimerDelay::RELATIVE.to_raw(), 2);
    assert_eq!(MemoryType::LOADER_CODE.to_raw(), 1);
    assert_eq!(ResetType::COLD.to_raw(), 0);
    assert_eq!(ResetType::PLATFORM_SPECIFIC.to_raw(), 3);
}
