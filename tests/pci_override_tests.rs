//! PCI Override and Incompatible Device Support protocol integration tests.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiHandle, EfiStatus};
use rover_uefi::protocols::pci::resource_allocation::{
    EFI_INCOMPATIBLE_PCI_DEVICE_SUPPORT_PROTOCOL_GUID, EFI_PCI_OVERRIDE_GUID,
    EfiIncompatiblePciDeviceSupportProtocol, EfiPciControllerResourceAllocationPhase,
    EfiPciExecutionPhase, EfiPciHostBridgeResourceAllocationPhase, EfiPciPlatformProtocol,
    IncompatiblePciDeviceSupport, PciOverride,
};

#[test]
fn test_pci_override_guids() {
    assert_eq!(
        EFI_PCI_OVERRIDE_GUID,
        EfiGuid::new(
            0xB5B3_5764,
            0x460C,
            0x4A06,
            [0x99, 0xFC, 0x77, 0xA1, 0x7C, 0x1B, 0x5C, 0xEB]
        )
    );
    assert_eq!(
        EFI_INCOMPATIBLE_PCI_DEVICE_SUPPORT_PROTOCOL_GUID,
        EfiGuid::new(
            0xEB23_F55A,
            0x7863,
            0x4AC2,
            [0x8D, 0x3D, 0x95, 0x65, 0x35, 0xDE, 0x03, 0x75]
        )
    );
}

unsafe extern "efiapi" fn mock_platform_notify(
    _this: *const EfiPciPlatformProtocol,
    _host_bridge: EfiHandle,
    _phase: EfiPciHostBridgeResourceAllocationPhase,
    _exec_phase: EfiPciExecutionPhase,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_platform_prep_controller(
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
    if pci_policy.is_null() {
        EfiStatus::INVALID_PARAMETER
    } else {
        unsafe { *pci_policy = 0x03 };
        EfiStatus::SUCCESS
    }
}

unsafe extern "efiapi" fn mock_get_pci_rom(
    _this: *const EfiPciPlatformProtocol,
    _pci_handle: EfiHandle,
    rom_image: *mut *mut c_void,
    rom_size: *mut UIntN,
) -> EfiStatus {
    if rom_image.is_null() || rom_size.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::NOT_FOUND
}

#[test]
fn test_pci_override_wrapper() {
    let raw_proto = EfiPciPlatformProtocol {
        platform_notify: mock_platform_notify,
        platform_prep_controller: mock_platform_prep_controller,
        get_platform_policy: mock_get_platform_policy,
        get_pci_rom: mock_get_pci_rom,
    };

    let pci_override = unsafe { PciOverride::from_raw(core::ptr::from_ref(&raw_proto)) };
    let platform = pci_override.as_platform();

    let dummy_handle = EfiHandle::from_ptr(0x1000 as *mut c_void);
    assert_eq!(
        platform.notify_phase(
            dummy_handle,
            EfiPciHostBridgeResourceAllocationPhase::BeginBusAllocation,
            EfiPciExecutionPhase::BeforePciHostBridge,
        ),
        Ok(())
    );

    assert_eq!(platform.get_platform_policy(), Ok(0x03));
    assert_eq!(
        platform.get_pci_rom(dummy_handle),
        Err(EfiStatus::NOT_FOUND)
    );
}

static mut DUMMY_DESCRIPTOR: u64 = 0xDEAD_BEEF;

unsafe extern "efiapi" fn mock_check_device(
    _this: *const EfiIncompatiblePciDeviceSupportProtocol,
    vendor_id: UIntN,
    device_id: UIntN,
    _revision_id: UIntN,
    _subsystem_vendor_id: UIntN,
    _subsystem_device_id: UIntN,
    configuration: *mut *mut c_void,
) -> EfiStatus {
    if configuration.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if vendor_id == 0x8086 && device_id == 0x1234 {
        unsafe {
            *configuration = core::ptr::addr_of_mut!(DUMMY_DESCRIPTOR).cast();
        }
    } else {
        unsafe {
            *configuration = core::ptr::null_mut();
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_incompatible_pci_device_support() {
    let raw_proto = EfiIncompatiblePciDeviceSupportProtocol {
        check_device: mock_check_device,
    };

    let client =
        unsafe { IncompatiblePciDeviceSupport::from_raw(core::ptr::from_ref(&raw_proto)) };

    // Device with special resource requirement
    let res = unsafe { client.check_device(0x8086, 0x1234, 0, 0, 0) };
    assert!(res.is_ok());
    let opt = res.unwrap_or_default();
    assert!(opt.is_some());
    let ptr = opt.unwrap_or(core::ptr::null_mut());
    assert_eq!(unsafe { *ptr.cast::<u64>() }, 0xDEAD_BEEF);

    // Normal device without incompatibility
    let normal_res = unsafe { client.check_device(0x10DE, 0x5678, 0, 0, 0) };
    assert_eq!(normal_res, Ok(None));
}
