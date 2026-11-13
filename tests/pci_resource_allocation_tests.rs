//! Integration tests for PCI Host Bridge Resource Allocation, Hot Plug, and Platform protocols.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::device_path::types::EfiDevicePath;
use rover_uefi::protocols::pci::*;

static mut LAST_PHASE: Option<EfiPciHostBridgeResourceAllocationPhase> = None;
static mut LAST_BUS_CONFIG: *const c_void = core::ptr::null();

unsafe extern "efiapi" fn mock_notify_phase(
    _this: *const EfiPciHostBridgeResourceAllocationProtocol,
    phase: EfiPciHostBridgeResourceAllocationPhase,
) -> EfiStatus {
    unsafe {
        LAST_PHASE = Some(phase);
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_next_root_bridge(
    _this: *const EfiPciHostBridgeResourceAllocationProtocol,
    root_bridge_handle: *mut EfiHandle,
) -> EfiStatus {
    unsafe {
        if (*root_bridge_handle).is_null() {
            *root_bridge_handle = EfiHandle::from_ptr(0x1000 as *mut c_void);
            EfiStatus::SUCCESS
        } else if *root_bridge_handle == EfiHandle::from_ptr(0x1000 as *mut c_void) {
            *root_bridge_handle = EfiHandle::from_ptr(0x2000 as *mut c_void);
            EfiStatus::SUCCESS
        } else {
            EfiStatus::NOT_FOUND
        }
    }
}

unsafe extern "efiapi" fn mock_get_alloc_attributes(
    _this: *const EfiPciHostBridgeResourceAllocationProtocol,
    root_bridge_handle: EfiHandle,
    attributes: *mut u64,
) -> EfiStatus {
    if root_bridge_handle == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        unsafe {
            *attributes = EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM | EFI_PCI_HOST_BRIDGE_MEM64_DECODE;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

unsafe extern "efiapi" fn mock_start_bus_enumeration(
    _this: *const EfiPciHostBridgeResourceAllocationProtocol,
    _root_bridge_handle: EfiHandle,
    configuration: *mut *mut c_void,
) -> EfiStatus {
    unsafe {
        *configuration = 0xABC0 as *mut c_void;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_bus_numbers(
    _this: *const EfiPciHostBridgeResourceAllocationProtocol,
    _root_bridge_handle: EfiHandle,
    configuration: *const c_void,
) -> EfiStatus {
    unsafe {
        LAST_BUS_CONFIG = configuration;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_submit_resources(
    _this: *const EfiPciHostBridgeResourceAllocationProtocol,
    _root_bridge_handle: EfiHandle,
    _configuration: *const c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_proposed_resources(
    _this: *const EfiPciHostBridgeResourceAllocationProtocol,
    _root_bridge_handle: EfiHandle,
    configuration: *mut *mut c_void,
) -> EfiStatus {
    unsafe {
        *configuration = 0xDEF0 as *mut c_void;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_preprocess_controller(
    _this: *const EfiPciHostBridgeResourceAllocationProtocol,
    _root_bridge_handle: EfiHandle,
    pci_address: u64,
    _phase: EfiPciControllerResourceAllocationPhase,
) -> EfiStatus {
    if pci_address == 0x0102_0300 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::DEVICE_ERROR
    }
}

unsafe extern "efiapi" fn mock_submit_fixed(
    _this: *const EfiPciHostBridgeFixedResourceProtocol,
    root_bridge_handle: EfiHandle,
    _config: *const c_void,
) -> EfiStatus {
    if root_bridge_handle == EfiHandle::from_ptr(0x1000 as *mut c_void) {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

unsafe extern "efiapi" fn mock_get_root_hpc_list(
    _this: *const EfiPciHotPlugInitProtocol,
    hpc_count: *mut UIntN,
    hpc_list: *mut *mut EfiHpcLocation,
) -> EfiStatus {
    unsafe {
        *hpc_count = 2;
        *hpc_list = 0x3000 as *mut EfiHpcLocation;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_initialize_root_hpc(
    _this: *const EfiPciHotPlugInitProtocol,
    _hpc_device_path: *const EfiDevicePath,
    _hpc_pci_address: u64,
    _event: *const c_void,
    hpc_state: *mut u16,
) -> EfiStatus {
    unsafe {
        *hpc_state = hpc_state::ENABLED;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_resource_padding(
    _this: *const EfiPciHotPlugInitProtocol,
    _hpc_device_path: *const EfiDevicePath,
    _hpc_pci_address: u64,
    hpc_state: *mut u16,
    padding: *mut *mut c_void,
    attributes: *mut EfiHpcPaddingAttributes,
) -> EfiStatus {
    unsafe {
        *hpc_state = hpc_state::ENABLED;
        *padding = 0x4000 as *mut c_void;
        *attributes = EfiHpcPaddingAttributes::PaddingPciBus;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_hotplug_notify(
    _this: *const EfiPciHotplugRequestProtocol,
    operation: EfiPciHotplugOperation,
    _controller: EfiHandle,
    _remaining_device_path: *const EfiDevicePath,
    number_of_children: *mut u8,
    _child_handle_buffer: *mut EfiHandle,
) -> EfiStatus {
    if operation == EfiPciHotplugOperation::Add {
        unsafe {
            *number_of_children = 1;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::INVALID_PARAMETER
    }
}

unsafe extern "efiapi" fn mock_platform_notify(
    _this: *const EfiPciPlatformProtocol,
    _host_bridge: EfiHandle,
    _phase: EfiPciHostBridgeResourceAllocationPhase,
    _exec_phase: EfiPciExecutionPhase,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_prep_controller(
    _this: *const EfiPciPlatformProtocol,
    _host_bridge: EfiHandle,
    _root_bridge: EfiHandle,
    _pci_address: u64,
    _phase: EfiPciControllerResourceAllocationPhase,
    _exec_phase: EfiPciExecutionPhase,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_platform_policy(
    _this: *const EfiPciPlatformProtocol,
    pci_policy: *mut u32,
) -> EfiStatus {
    unsafe {
        *pci_policy =
            pci_platform_policy::RESERVE_ISA_IO_ALIAS | pci_platform_policy::RESERVE_VGA_IO_ALIAS;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_get_pci_rom(
    _this: *const EfiPciPlatformProtocol,
    pci_handle: EfiHandle,
    rom_image: *mut *mut c_void,
    rom_size: *mut UIntN,
) -> EfiStatus {
    if pci_handle == EfiHandle::from_ptr(0x7000 as *mut c_void) {
        unsafe {
            *rom_image = 0x8000 as *mut c_void;
            *rom_size = 0x1000;
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_pci_resource_allocation_guids_and_constants() {
    assert_eq!(
        EFI_PCI_HOST_BRIDGE_RESOURCE_ALLOCATION_PROTOCOL_GUID,
        EfiGuid::new(
            0xCF80_34BE,
            0x6768,
            0x4D8B,
            [0xB7, 0x39, 0x7C, 0xCE, 0x68, 0x3A, 0x9F, 0xBE]
        )
    );

    assert_eq!(
        EFI_PCI_HOT_PLUG_INIT_PROTOCOL_GUID,
        EfiGuid::new(
            0xAA0E_8BC1,
            0xDABC,
            0x46B0,
            [0xA8, 0x44, 0x37, 0xB8, 0x16, 0x9B, 0x2B, 0xEA]
        )
    );

    assert_eq!(
        EFI_PCI_HOTPLUG_REQUEST_PROTOCOL_GUID,
        EfiGuid::new(
            0x19CB_87AB,
            0x2CB9,
            0x4665,
            [0x83, 0x60, 0xDD, 0xCF, 0x60, 0x54, 0xF7, 0x9D]
        )
    );

    assert_eq!(
        EFI_PCI_PLATFORM_PROTOCOL_GUID,
        EfiGuid::new(
            0x07D7_5280,
            0x27D4,
            0x4D69,
            [0x90, 0xD0, 0x56, 0x43, 0xE2, 0x38, 0xB3, 0x41]
        )
    );

    assert_eq!(
        PCI_HOST_BRIDGE_FIXED_RESOURCE_PROTOCOL_GUID,
        EfiGuid::new(
            0xBB46_5AEA,
            0x5165,
            0x4FDF,
            [0xB4, 0xB0, 0xFD, 0x3A, 0x1D, 0x4C, 0xC6, 0xD8]
        )
    );

    assert_eq!(EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM, 1);
    assert_eq!(EFI_PCI_HOST_BRIDGE_MEM64_DECODE, 2);
    assert_eq!(EFI_RESOURCE_SATISFIED, 0);
    assert_eq!(EFI_RESOURCE_NOT_SATISFIED, u64::MAX);

    assert_eq!(pci_platform_policy::RESERVE_NONE_IO_ALIAS, 0);
    assert_eq!(pci_platform_policy::RESERVE_ISA_IO_ALIAS, 1);
    assert_eq!(pci_platform_policy::RESERVE_ISA_IO_NO_ALIAS, 2);
    assert_eq!(pci_platform_policy::RESERVE_VGA_IO_ALIAS, 4);
    assert_eq!(pci_platform_policy::RESERVE_VGA_IO_NO_ALIAS, 8);

    assert_eq!(hpc_state::INITIALIZED, 1);
    assert_eq!(hpc_state::ENABLED, 2);
}

#[test]
fn test_pci_host_bridge_resource_allocation_wrapper() {
    let raw_proto = EfiPciHostBridgeResourceAllocationProtocol {
        notify_phase: mock_notify_phase,
        get_next_root_bridge: mock_get_next_root_bridge,
        get_alloc_attributes: mock_get_alloc_attributes,
        start_bus_enumeration: mock_start_bus_enumeration,
        set_bus_numbers: mock_set_bus_numbers,
        submit_resources: mock_submit_resources,
        get_proposed_resources: mock_get_proposed_resources,
        preprocess_controller: mock_preprocess_controller,
    };

    let client = unsafe { PciHostBridgeResourceAllocation::from_raw(&raw const raw_proto) };

    assert!(
        client
            .notify_phase(EfiPciHostBridgeResourceAllocationPhase::BeginEnumeration)
            .is_ok()
    );
    assert_eq!(
        unsafe { LAST_PHASE },
        Some(EfiPciHostBridgeResourceAllocationPhase::BeginEnumeration)
    );

    let first_rb = match client.get_next_root_bridge(None) {
        Ok(h) => h,
        Err(e) => panic!("expected root bridge: {e:?}"),
    };
    assert_eq!(first_rb, EfiHandle::from_ptr(0x1000 as *mut c_void));
    let second_rb = match client.get_next_root_bridge(Some(first_rb)) {
        Ok(h) => h,
        Err(e) => panic!("expected second root bridge: {e:?}"),
    };
    assert_eq!(second_rb, EfiHandle::from_ptr(0x2000 as *mut c_void));
    assert_eq!(
        client.get_next_root_bridge(Some(second_rb)),
        Err(EfiStatus::NOT_FOUND)
    );

    let attrs = match client.get_alloc_attributes(first_rb) {
        Ok(a) => a,
        Err(e) => panic!("expected attributes: {e:?}"),
    };
    assert_eq!(
        attrs,
        EFI_PCI_HOST_BRIDGE_COMBINE_MEM_PMEM | EFI_PCI_HOST_BRIDGE_MEM64_DECODE
    );

    let config = match client.start_bus_enumeration(first_rb) {
        Ok(c) => c,
        Err(e) => panic!("expected config: {e:?}"),
    };
    assert_eq!(config, 0xABC0 as *mut c_void);

    assert!(unsafe { client.set_bus_numbers(first_rb, 0x1234 as *const c_void) }.is_ok());
    assert_eq!(unsafe { LAST_BUS_CONFIG }, 0x1234 as *const c_void);

    assert!(unsafe { client.submit_resources(first_rb, 0x5678 as *const c_void) }.is_ok());

    let proposed = match client.get_proposed_resources(first_rb) {
        Ok(p) => p,
        Err(e) => panic!("expected proposed resources: {e:?}"),
    };
    assert_eq!(proposed, 0xDEF0 as *mut c_void);

    assert!(
        client
            .preprocess_controller(
                first_rb,
                0x0102_0300,
                EfiPciControllerResourceAllocationPhase::BeforeResourceCollection
            )
            .is_ok()
    );
    assert_eq!(
        client.preprocess_controller(
            first_rb,
            0x9999_9999,
            EfiPciControllerResourceAllocationPhase::BeforeResourceCollection
        ),
        Err(EfiStatus::DEVICE_ERROR)
    );
}

#[test]
fn test_pci_fixed_resource_and_hotplug_wrappers() {
    let fixed_proto = EfiPciHostBridgeFixedResourceProtocol {
        submit_fixed_resources: mock_submit_fixed,
    };
    let fixed_client = unsafe { PciHostBridgeFixedResource::from_raw(&raw const fixed_proto) };

    assert!(
        unsafe {
            fixed_client.submit_fixed_resources(
                EfiHandle::from_ptr(0x1000 as *mut c_void),
                0x2000 as *const c_void,
            )
        }
        .is_ok()
    );
    assert_eq!(
        unsafe {
            fixed_client.submit_fixed_resources(
                EfiHandle::from_ptr(0x9999 as *mut c_void),
                0x2000 as *const c_void,
            )
        },
        Err(EfiStatus::INVALID_PARAMETER)
    );

    let hp_init_proto = EfiPciHotPlugInitProtocol {
        get_root_hpc_list: mock_get_root_hpc_list,
        initialize_root_hpc: mock_initialize_root_hpc,
        get_resource_padding: mock_get_resource_padding,
    };
    let hp_init_client = unsafe { PciHotPlugInit::from_raw(&raw const hp_init_proto) };

    let (list, count) = match hp_init_client.get_root_hpc_list() {
        Ok(res) => res,
        Err(e) => panic!("expected hpc list: {e:?}"),
    };
    assert_eq!(count, 2);
    assert_eq!(list, 0x3000 as *mut EfiHpcLocation);

    let dummy_dp = EfiDevicePath::new(0x7F, 0xFF, 4);
    let state = match hp_init_client.initialize_root_hpc(&dummy_dp, 0x1234, None) {
        Ok(s) => s,
        Err(e) => panic!("expected init root hpc state: {e:?}"),
    };
    assert_eq!(state, hpc_state::ENABLED);

    let (pad_state, padding, attrs) = match hp_init_client.get_resource_padding(&dummy_dp, 0x1234) {
        Ok(p) => p,
        Err(e) => panic!("expected padding: {e:?}"),
    };
    assert_eq!(pad_state, hpc_state::ENABLED);
    assert_eq!(padding, 0x4000 as *mut c_void);
    assert_eq!(attrs, EfiHpcPaddingAttributes::PaddingPciBus);

    let hp_req_proto = EfiPciHotplugRequestProtocol {
        notify: mock_hotplug_notify,
    };
    let hp_req_client = unsafe { PciHotplugRequest::from_raw(&raw const hp_req_proto) };

    let mut children = [EfiHandle::NULL];
    let res = hp_req_client.notify(
        EfiPciHotplugOperation::Add,
        EfiHandle::from_ptr(0x5000 as *mut c_void),
        None,
        &mut children,
    );
    assert_eq!(res, Ok(1));
}

#[test]
fn test_pci_platform_wrapper() {
    let proto = EfiPciPlatformProtocol {
        platform_notify: mock_platform_notify,
        platform_prep_controller: mock_prep_controller,
        get_platform_policy: mock_get_platform_policy,
        get_pci_rom: mock_get_pci_rom,
    };
    let client = unsafe { PciPlatform::from_raw(&raw const proto) };

    assert!(
        client
            .notify_phase(
                EfiHandle::from_ptr(0x1000 as *mut c_void),
                EfiPciHostBridgeResourceAllocationPhase::BeginEnumeration,
                EfiPciExecutionPhase::BeforePciHostBridge
            )
            .is_ok()
    );

    assert!(
        client
            .prep_controller(
                EfiHandle::from_ptr(0x1000 as *mut c_void),
                EfiHandle::from_ptr(0x2000 as *mut c_void),
                0x1234,
                EfiPciControllerResourceAllocationPhase::BeforeChildBusEnumeration,
                EfiPciExecutionPhase::BeforePciHostBridge
            )
            .is_ok()
    );

    let policy = match client.get_platform_policy() {
        Ok(p) => p,
        Err(e) => panic!("expected platform policy: {e:?}"),
    };
    assert_eq!(
        policy,
        pci_platform_policy::RESERVE_ISA_IO_ALIAS | pci_platform_policy::RESERVE_VGA_IO_ALIAS
    );

    let (rom_ptr, rom_size) = match client.get_pci_rom(EfiHandle::from_ptr(0x7000 as *mut c_void)) {
        Ok(r) => r,
        Err(e) => panic!("expected rom: {e:?}"),
    };
    assert_eq!(rom_ptr, 0x8000 as *mut c_void);
    assert_eq!(rom_size, 0x1000);

    assert_eq!(
        client.get_pci_rom(EfiHandle::from_ptr(0x9999 as *mut c_void)),
        Err(EfiStatus::NOT_FOUND)
    );
}
