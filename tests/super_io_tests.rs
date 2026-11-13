//! Super I/O, Super I/O Control, and ISA Host Controller protocol integration tests.

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::driver::isa::{
    EFI_ISA_HC_PROTOCOL_GUID, EFI_ISA_HC_SERVICE_BINDING_PROTOCOL_GUID, EfiIsaHcProtocol,
    IsaHostController,
};
use rover_uefi::protocols::driver::super_io::{
    AcpiResourceHeaderPtr, EFI_SIO_CONTROL_PROTOCOL_GUID, EFI_SIO_PROTOCOL_GUID,
    EfiSioControlProtocol, EfiSioProtocol, EfiSioRegisterModify, SuperIo, SuperIoControl,
};

#[test]
fn test_super_io_guids() {
    assert_eq!(
        EFI_SIO_PROTOCOL_GUID,
        EfiGuid::new(
            0x215F_DD18,
            0xBD50,
            0x4FEB,
            [0x89, 0x0B, 0x58, 0xCA, 0x0B, 0x47, 0x39, 0xE9]
        )
    );
    assert_eq!(
        EFI_SIO_CONTROL_PROTOCOL_GUID,
        EfiGuid::new(
            0xB919_78DF,
            0x9FC1,
            0x427D,
            [0xBB, 0x05, 0x4C, 0x82, 0x84, 0x55, 0xCA, 0x27]
        )
    );
    assert_eq!(
        EFI_ISA_HC_PROTOCOL_GUID,
        EfiGuid::new(
            0xBCDA_F080,
            0x1BDE,
            0x4E22,
            [0xAE, 0x6A, 0x43, 0x54, 0x1E, 0x12, 0x8E, 0xC4]
        )
    );
    assert_eq!(
        EFI_ISA_HC_SERVICE_BINDING_PROTOCOL_GUID,
        EfiGuid::new(
            0xFAD7_933A,
            0x6C21,
            0x4234,
            [0xA4, 0x34, 0x0A, 0x8A, 0x0D, 0x2B, 0x07, 0x81]
        )
    );
}

static mut SIO_REGISTERS: [u8; 256] = [0u8; 256];

unsafe extern "efiapi" fn mock_sio_register_access(
    _this: *const EfiSioProtocol,
    write: bool,
    _exit_cfg_mode: bool,
    register: u8,
    value: *mut u8,
) -> EfiStatus {
    if value.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if write {
        unsafe {
            SIO_REGISTERS[register as usize] = *value;
        }
    } else {
        unsafe {
            *value = SIO_REGISTERS[register as usize];
        }
    }
    EfiStatus::SUCCESS
}

static mut DUMMY_RESOURCE_DESC: u32 = 0x1234_5678;

unsafe extern "efiapi" fn mock_sio_get_resources(
    _this: *const EfiSioProtocol,
    resource_list: *mut AcpiResourceHeaderPtr,
) -> EfiStatus {
    if resource_list.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *resource_list = AcpiResourceHeaderPtr {
            small_header: core::ptr::addr_of_mut!(DUMMY_RESOURCE_DESC).cast(),
        };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_sio_set_resources(
    _this: *const EfiSioProtocol,
    _resource_list: AcpiResourceHeaderPtr,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_sio_possible_resources(
    _this: *const EfiSioProtocol,
    resource_collection: *mut AcpiResourceHeaderPtr,
) -> EfiStatus {
    if resource_collection.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe {
        *resource_collection = AcpiResourceHeaderPtr {
            small_header: core::ptr::addr_of_mut!(DUMMY_RESOURCE_DESC).cast(),
        };
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_sio_modify(
    _this: *const EfiSioProtocol,
    command: *const EfiSioRegisterModify,
    number_of_commands: UIntN,
) -> EfiStatus {
    if command.is_null() && number_of_commands > 0 {
        return EfiStatus::INVALID_PARAMETER;
    }
    let slice = unsafe { core::slice::from_raw_parts(command, number_of_commands) };
    for cmd in slice {
        let idx = cmd.register as usize;
        unsafe {
            SIO_REGISTERS[idx] = (SIO_REGISTERS[idx] & cmd.and_mask) | cmd.or_mask;
        }
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_super_io_wrapper() {
    let raw_proto = EfiSioProtocol {
        register_access: mock_sio_register_access,
        get_resources: mock_sio_get_resources,
        set_resources: mock_sio_set_resources,
        possible_resources: mock_sio_possible_resources,
        modify: mock_sio_modify,
    };

    let client = SuperIo::new(&raw_proto);

    // Write and read register
    assert_eq!(client.write_register(0x30, 0x01, false), Ok(()));
    assert_eq!(client.read_register(0x30, true), Ok(0x01));

    // Batch modify
    let commands = [
        EfiSioRegisterModify {
            register: 0x30,
            and_mask: 0xFE,
            or_mask: 0x02,
        },
        EfiSioRegisterModify {
            register: 0x60,
            and_mask: 0x00,
            or_mask: 0x03,
        },
    ];
    assert_eq!(client.modify(&commands), Ok(()));
    assert_eq!(client.read_register(0x30, false), Ok(0x02));
    assert_eq!(client.read_register(0x60, false), Ok(0x03));

    // Get and set resources
    let res = client.get_resources().unwrap_or(AcpiResourceHeaderPtr {
        small_header: core::ptr::null_mut(),
    });
    assert!(!unsafe { res.small_header }.is_null());
    assert_eq!(client.set_resources(res), Ok(()));

    let possible = client
        .possible_resources()
        .unwrap_or(AcpiResourceHeaderPtr {
            small_header: core::ptr::null_mut(),
        });
    assert!(!unsafe { possible.small_header }.is_null());

    assert!(unsafe { SuperIo::from_raw(core::ptr::null()) }.is_none());
}

static mut DEVICE_ENABLED: bool = false;

unsafe extern "efiapi" fn mock_sio_control_enable(
    _this: *const EfiSioControlProtocol,
) -> EfiStatus {
    unsafe { DEVICE_ENABLED = true };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_sio_control_disable(
    _this: *const EfiSioControlProtocol,
) -> EfiStatus {
    unsafe { DEVICE_ENABLED = false };
    EfiStatus::SUCCESS
}

#[test]
fn test_super_io_control_wrapper() {
    let raw_proto = EfiSioControlProtocol {
        version: 1,
        enable_device: mock_sio_control_enable,
        disable_device: mock_sio_control_disable,
    };

    let client = SuperIoControl::new(&raw_proto);
    assert_eq!(client.version(), 1);

    assert_eq!(client.enable(), Ok(()));
    assert!(unsafe { DEVICE_ENABLED });

    assert_eq!(client.disable(), Ok(()));
    assert!(!unsafe { DEVICE_ENABLED });

    assert!(unsafe { SuperIoControl::from_raw(core::ptr::null()) }.is_none());
}

unsafe extern "efiapi" fn mock_isa_hc_open_io(
    _this: *const EfiIsaHcProtocol,
    io_address: u16,
    io_length: u16,
    io_aperture_handle: *mut u64,
) -> EfiStatus {
    if io_aperture_handle.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if io_address == 0x3F8 && io_length == 8 {
        unsafe { *io_aperture_handle = 0x1000 };
        EfiStatus::SUCCESS
    } else {
        EfiStatus::OUT_OF_RESOURCES
    }
}

unsafe extern "efiapi" fn mock_isa_hc_close_io(
    _this: *const EfiIsaHcProtocol,
    io_aperture_handle: u64,
) -> EfiStatus {
    if io_aperture_handle == 0x1000 {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_isa_host_controller_wrapper() {
    let raw_proto = EfiIsaHcProtocol {
        version: 0x0001_0000,
        open_io_aperture: Some(mock_isa_hc_open_io),
        close_io_aperture: Some(mock_isa_hc_close_io),
    };

    let client = IsaHostController::new(&raw_proto);
    assert_eq!(client.version(), 0x0001_0000);

    let handle = client.open_io_aperture(0x3F8, 8).unwrap_or(0);
    assert_eq!(handle, 0x1000);

    assert_eq!(client.close_io_aperture(handle), Ok(()));
    assert_eq!(
        client.open_io_aperture(0x100, 16),
        Err(EfiStatus::OUT_OF_RESOURCES)
    );

    assert!(unsafe { IsaHostController::from_raw(core::ptr::null()) }.is_none());
}
