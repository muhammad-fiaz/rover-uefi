//! Image loader, emulator, and debugger configuration integration tests.

use rover_uefi::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};
use rover_uefi::protocols::device_path::EfiDevicePath;
use rover_uefi::protocols::driver::image_loader::{
    DEBUGGER_CONFIGURATION_PROTOCOL_GUID, DEBUGGER_CONFIGURATION_REVISION, DebuggerConfiguration,
    EfiDebuggerConfigurationProtocol, EfiPe32ImageProtocol, EfiPeCoffImageEmulatorProtocol,
    PE32_IMAGE_PROTOCOL_GUID, PECOFF_IMAGE_EMULATOR_PROTOCOL_GUID, PECOFF_IMAGE_EMULATOR_VERSION,
    Pe32Image, PeCoffImageEmulator, load_pe_image_attributes,
};
use rover_uefi::tables::system::EfiImageEntryPoint;

#[test]
fn test_image_loader_guids() {
    let expected_emu = EfiGuid::new(
        0x96F4_6153,
        0x97A7,
        0x4793,
        [0xAC, 0xC1, 0xFA, 0x19, 0xBF, 0x78, 0xEA, 0x97],
    );
    assert_eq!(PECOFF_IMAGE_EMULATOR_PROTOCOL_GUID, expected_emu);
    assert_eq!(PECOFF_IMAGE_EMULATOR_VERSION, 1);

    let expected_pe32 = EfiGuid::new(
        0x5CB5_C776,
        0x60D5,
        0x45EE,
        [0x88, 0x3C, 0x45, 0x27, 0x08, 0xCD, 0x74, 0x3F],
    );
    assert_eq!(PE32_IMAGE_PROTOCOL_GUID, expected_pe32);

    let expected_dbg = EfiGuid::new(
        0x577D_959C,
        0xE967,
        0x4546,
        [0x86, 0x20, 0xC7, 0x78, 0xFA, 0xE5, 0xDA, 0x05],
    );
    assert_eq!(DEBUGGER_CONFIGURATION_PROTOCOL_GUID, expected_dbg);
    assert_eq!(DEBUGGER_CONFIGURATION_REVISION, 1);
}

#[test]
fn test_load_pe_image_attribute_constants() {
    assert_eq!(load_pe_image_attributes::NONE, 0x00);
    assert_eq!(load_pe_image_attributes::RUNTIME_REGISTRATION, 0x01);
    assert_eq!(
        load_pe_image_attributes::DEBUG_IMAGE_INFO_TABLE_REGISTRATION,
        0x02
    );
}

unsafe extern "efiapi" fn mock_is_supported(
    _this: *mut EfiPeCoffImageEmulatorProtocol,
    image_type: u16,
    _device_path: *const EfiDevicePath,
) -> bool {
    image_type == 0xAA64 // AArch64
}

unsafe extern "efiapi" fn mock_register_image(
    _this: *mut EfiPeCoffImageEmulatorProtocol,
    _image_base: EfiPhysicalAddress,
    _image_size: u64,
    entry_point: *mut Option<EfiImageEntryPoint>,
) -> EfiStatus {
    if entry_point.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_unregister_image(
    _this: *mut EfiPeCoffImageEmulatorProtocol,
    _image_base: EfiPhysicalAddress,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_pecoff_image_emulator_wrapper() {
    let mut mock_proto = EfiPeCoffImageEmulatorProtocol {
        is_image_supported: Some(mock_is_supported),
        register_image: Some(mock_register_image),
        unregister_image: Some(mock_unregister_image),
        version: PECOFF_IMAGE_EMULATOR_VERSION,
        machine_type: 0xAA64,
    };

    let Some(mut emu) = (unsafe { PeCoffImageEmulator::from_raw(&raw mut mock_proto) }) else {
        panic!("failed to wrap emulator protocol");
    };

    assert!(emu.is_image_supported(0xAA64, None));
    assert!(!emu.is_image_supported(0x8664, None));
    assert_eq!(emu.version(), 1);
    assert_eq!(emu.machine_type(), 0xAA64);

    let mut ep: Option<EfiImageEntryPoint> = None;
    let res_reg = emu.register_image(EfiPhysicalAddress::from_u64(0x1000_0000), 0x20000, &mut ep);
    assert_eq!(res_reg, Ok(()));

    let res_unreg = emu.unregister_image(EfiPhysicalAddress::from_u64(0x1000_0000));
    assert_eq!(res_unreg, Ok(()));
}

unsafe extern "efiapi" fn mock_unload_pe_image(
    _this: *mut EfiPe32ImageProtocol,
    image_handle: EfiHandle,
) -> EfiStatus {
    if image_handle.is_null() {
        EfiStatus::INVALID_PARAMETER
    } else {
        EfiStatus::SUCCESS
    }
}

#[test]
fn test_pe32_image_wrapper() {
    let mut mock_proto = EfiPe32ImageProtocol {
        load_pe_image: None,
        unload_pe_image: Some(mock_unload_pe_image),
    };

    let Some(mut loader) = (unsafe { Pe32Image::from_raw(&raw mut mock_proto) }) else {
        panic!("failed to wrap pe32 loader");
    };

    let res_invalid = loader.unload_image(EfiHandle::NULL);
    assert_eq!(res_invalid, Err(EfiStatus::INVALID_PARAMETER));

    let dummy_handle = EfiHandle::from_ptr(0x1234 as *mut core::ffi::c_void);
    let res_valid = loader.unload_image(dummy_handle);
    assert_eq!(res_valid, Ok(()));
}

#[test]
fn test_debugger_configuration_wrapper() {
    let dummy_data = 0xBEEF as *mut core::ffi::c_void;
    let mock_proto = EfiDebuggerConfigurationProtocol {
        revision: DEBUGGER_CONFIGURATION_REVISION,
        debugger_private_data: dummy_data,
    };

    let Some(dbg) = (unsafe { DebuggerConfiguration::from_raw(&raw const mock_proto) }) else {
        panic!("failed to wrap debugger configuration");
    };

    assert_eq!(dbg.revision(), 1);
    assert_eq!(dbg.private_data(), dummy_data);
}
