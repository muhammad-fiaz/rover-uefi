//! ARM SCMI protocols integration tests.

use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::driver::scmi::{
    ARM_SCMI_BASE_PROTOCOL_GUID, ARM_SCMI_CLOCK_PROTOCOL_GUID, ARM_SCMI_CLOCK2_PROTOCOL_GUID,
    ARM_SCMI_PERFORMANCE_PROTOCOL_GUID, EfiScmiBaseProtocol, EfiScmiClock2Protocol,
    EfiScmiClockProtocol, SCMI_BASE_PROTOCOL_VERSION_V1, SCMI_BASE_PROTOCOL_VERSION_V2,
    SCMI_CLOCK_PROTOCOL_VERSION, SCMI_CLOCK2_PROTOCOL_VERSION, SCMI_MAX_STR_LEN, ScmiBase,
    ScmiClock, ScmiClock2, ScmiClockRate, ScmiClockRateFormat, ScmiClockRateRange,
};

#[test]
fn test_scmi_guids_and_constants() {
    assert_eq!(
        ARM_SCMI_BASE_PROTOCOL_GUID,
        EfiGuid::new(
            0xD7E5_ABE9,
            0x33AB,
            0x418E,
            [0x9F, 0x91, 0x72, 0xDA, 0xE2, 0xBA, 0x8E, 0x2F]
        )
    );
    assert_eq!(
        ARM_SCMI_CLOCK_PROTOCOL_GUID,
        EfiGuid::new(
            0x91CE_67A8,
            0xE0AA,
            0x4012,
            [0xB9, 0x9F, 0xB6, 0xFC, 0xF3, 0x04, 0x8E, 0xAA]
        )
    );
    assert_eq!(
        ARM_SCMI_CLOCK2_PROTOCOL_GUID,
        EfiGuid::new(
            0xB8D8_CAF2,
            0x9E94,
            0x462C,
            [0xA8, 0x34, 0x6C, 0x99, 0xFC, 0x05, 0xEF, 0xCF]
        )
    );
    assert_eq!(
        ARM_SCMI_PERFORMANCE_PROTOCOL_GUID,
        EfiGuid::new(
            0x09B8_BA84,
            0x3DD3,
            0x49A6,
            [0xA0, 0x5A, 0x31, 0x34, 0xA5, 0xF0, 0x7B, 0xAD]
        )
    );

    assert_eq!(SCMI_BASE_PROTOCOL_VERSION_V1, 0x10000);
    assert_eq!(SCMI_BASE_PROTOCOL_VERSION_V2, 0x20000);
    assert_eq!(SCMI_CLOCK_PROTOCOL_VERSION, 0x10000);
    assert_eq!(SCMI_CLOCK2_PROTOCOL_VERSION, 1);
    assert_eq!(SCMI_MAX_STR_LEN, 16);
}

#[test]
fn test_scmi_clock_rate_structs() {
    assert_eq!(ScmiClockRateFormat::Discrete as u32, 0);
    assert_eq!(ScmiClockRateFormat::Linear as u32, 1);

    let range = ScmiClockRateRange {
        min: 1_000_000,
        max: 100_000_000,
        step: 1_000_000,
    };
    let clock_rate = ScmiClockRate { range };
    assert_eq!(unsafe { clock_rate.range.min }, 1_000_000);
    assert_eq!(unsafe { clock_rate.range.max }, 100_000_000);

    let discrete = ScmiClockRate { rate: 25_000_000 };
    assert_eq!(unsafe { discrete.rate }, 25_000_000);
}

unsafe extern "efiapi" fn mock_base_get_version(
    _this: *mut EfiScmiBaseProtocol,
    version: *mut u32,
) -> EfiStatus {
    if version.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *version = SCMI_BASE_PROTOCOL_VERSION_V2 };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_base_get_total_protocols(
    _this: *mut EfiScmiBaseProtocol,
    num: *mut u32,
) -> EfiStatus {
    if num.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *num = 5 };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_base_discover_vendor(
    _this: *mut EfiScmiBaseProtocol,
    vendor_identifier: *mut [u8; SCMI_MAX_STR_LEN],
) -> EfiStatus {
    if vendor_identifier.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let name = b"ARM-SCP\0\0\0\0\0\0\0\0\0";
    unsafe { *vendor_identifier = *name };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_base_discover_sub_vendor(
    _this: *mut EfiScmiBaseProtocol,
    vendor_identifier: *mut [u8; SCMI_MAX_STR_LEN],
) -> EfiStatus {
    if vendor_identifier.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let name = b"ROVER-FW\0\0\0\0\0\0\0\0";
    unsafe { *vendor_identifier = *name };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_base_discover_impl_version(
    _this: *mut EfiScmiBaseProtocol,
    impl_ver: *mut u32,
) -> EfiStatus {
    if impl_ver.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *impl_ver = 0x0102_0304 };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_base_discover_list_protocols(
    _this: *mut EfiScmiBaseProtocol,
    _size: *mut u32,
    _list: *mut u8,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

#[test]
fn test_scmi_base_wrapper() {
    let mut raw_proto = EfiScmiBaseProtocol {
        get_version: mock_base_get_version,
        get_total_protocols: mock_base_get_total_protocols,
        discover_vendor: mock_base_discover_vendor,
        discover_sub_vendor: mock_base_discover_sub_vendor,
        discover_implementation_version: mock_base_discover_impl_version,
        discover_list_protocols: mock_base_discover_list_protocols,
    };

    let mut client = ScmiBase::new(&mut raw_proto);
    assert_eq!(client.version(), Ok(SCMI_BASE_PROTOCOL_VERSION_V2));
    assert_eq!(client.total_protocols(), Ok(5));

    let vendor = client.discover_vendor().unwrap_or([0u8; SCMI_MAX_STR_LEN]);
    assert_eq!(&vendor[..7], b"ARM-SCP");

    let sub_vendor = client
        .discover_sub_vendor()
        .unwrap_or([0u8; SCMI_MAX_STR_LEN]);
    assert_eq!(&sub_vendor[..8], b"ROVER-FW");

    assert_eq!(client.discover_implementation_version(), Ok(0x0102_0304));

    assert!(unsafe { ScmiBase::from_raw(core::ptr::null_mut()) }.is_none());
}

static mut CURRENT_CLOCK_RATE: u64 = 48_000_000;

unsafe extern "efiapi" fn mock_clock_get_version(
    _this: *mut EfiScmiClockProtocol,
    version: *mut u32,
) -> EfiStatus {
    if version.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *version = SCMI_CLOCK_PROTOCOL_VERSION };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock_get_total_clocks(
    _this: *mut EfiScmiClockProtocol,
    total_clocks: *mut u32,
) -> EfiStatus {
    if total_clocks.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *total_clocks = 8 };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock_get_clock_attributes(
    _this: *mut EfiScmiClockProtocol,
    clock_id: u32,
    enabled: *mut bool,
    ascii_name: *mut [u8; SCMI_MAX_STR_LEN],
) -> EfiStatus {
    if enabled.is_null() || ascii_name.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if clock_id == 0 {
        unsafe {
            *enabled = true;
            *ascii_name = *b"UART0_CLK\0\0\0\0\0\0\0";
        }
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_clock_describe_rates(
    _this: *mut EfiScmiClockProtocol,
    _clock_id: u32,
    _format: *mut ScmiClockRateFormat,
    _total_rates: *mut u32,
    _rate_array_size: *mut u32,
    _rate_array: *mut ScmiClockRate,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock_rate_get(
    _this: *mut EfiScmiClockProtocol,
    clock_id: u32,
    rate: *mut u64,
) -> EfiStatus {
    if rate.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if clock_id == 0 {
        unsafe { *rate = CURRENT_CLOCK_RATE };
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_clock_rate_set(
    _this: *mut EfiScmiClockProtocol,
    clock_id: u32,
    rate: u64,
) -> EfiStatus {
    if clock_id == 0 {
        unsafe { CURRENT_CLOCK_RATE = rate };
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_scmi_clock_wrapper() {
    let mut raw_proto = EfiScmiClockProtocol {
        get_version: mock_clock_get_version,
        get_total_clocks: mock_clock_get_total_clocks,
        get_clock_attributes: mock_clock_get_clock_attributes,
        describe_rates: mock_clock_describe_rates,
        rate_get: mock_clock_rate_get,
        rate_set: mock_clock_rate_set,
    };

    let mut client = ScmiClock::new(&mut raw_proto);
    assert_eq!(client.version(), Ok(SCMI_CLOCK_PROTOCOL_VERSION));
    assert_eq!(client.total_clocks(), Ok(8));

    let (enabled, name) = client
        .get_clock_attributes(0)
        .unwrap_or((false, [0u8; SCMI_MAX_STR_LEN]));
    assert!(enabled);
    assert_eq!(&name[..9], b"UART0_CLK");

    assert_eq!(client.get_rate(0), Ok(48_000_000));
    assert_eq!(client.set_rate(0, 50_000_000), Ok(()));
    assert_eq!(client.get_rate(0), Ok(50_000_000));

    assert_eq!(client.get_rate(1), Err(EfiStatus::NOT_FOUND));

    assert!(unsafe { ScmiClock::from_raw(core::ptr::null_mut()) }.is_none());
}

static mut CLOCK2_ENABLED: bool = false;

unsafe extern "efiapi" fn mock_clock2_get_version(
    _this: *mut EfiScmiClock2Protocol,
    version: *mut u32,
) -> EfiStatus {
    if version.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *version = SCMI_CLOCK2_PROTOCOL_VERSION };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock2_get_total_clocks(
    _this: *mut EfiScmiClock2Protocol,
    total_clocks: *mut u32,
) -> EfiStatus {
    if total_clocks.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    unsafe { *total_clocks = 16 };
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock2_get_clock_attributes(
    _this: *mut EfiScmiClock2Protocol,
    _clock_id: u32,
    _enabled: *mut bool,
    _ascii_name: *mut [u8; SCMI_MAX_STR_LEN],
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock2_describe_rates(
    _this: *mut EfiScmiClock2Protocol,
    _clock_id: u32,
    _format: *mut ScmiClockRateFormat,
    _total_rates: *mut u32,
    _rate_array_size: *mut u32,
    _rate_array: *mut ScmiClockRate,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock2_rate_get(
    _this: *mut EfiScmiClock2Protocol,
    clock_id: u32,
    rate: *mut u64,
) -> EfiStatus {
    if rate.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    if clock_id == 2 {
        unsafe { *rate = 100_000_000 };
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_clock2_rate_set(
    _this: *mut EfiScmiClock2Protocol,
    _clock_id: u32,
    _rate: u64,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_clock2_enable(
    _this: *mut EfiScmiClock2Protocol,
    clock_id: u32,
    enable: bool,
) -> EfiStatus {
    if clock_id == 2 {
        unsafe { CLOCK2_ENABLED = enable };
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

#[test]
fn test_scmi_clock2_wrapper() {
    let mut raw_proto = EfiScmiClock2Protocol {
        get_version: mock_clock2_get_version,
        get_total_clocks: mock_clock2_get_total_clocks,
        get_clock_attributes: mock_clock2_get_clock_attributes,
        describe_rates: mock_clock2_describe_rates,
        rate_get: mock_clock2_rate_get,
        rate_set: mock_clock2_rate_set,
        version: 1 as UIntN,
        enable: mock_clock2_enable,
    };

    let mut client = ScmiClock2::new(&mut raw_proto);
    assert_eq!(client.version(), Ok(SCMI_CLOCK2_PROTOCOL_VERSION));
    assert_eq!(client.total_clocks(), Ok(16));
    assert_eq!(client.get_rate(2), Ok(100_000_000));
    assert_eq!(client.set_rate(2, 120_000_000), Ok(()));

    assert_eq!(client.enable(2, true), Ok(()));
    assert!(unsafe { CLOCK2_ENABLED });

    assert_eq!(client.enable(2, false), Ok(()));
    assert!(!unsafe { CLOCK2_ENABLED });

    assert_eq!(client.enable(5, true), Err(EfiStatus::NOT_FOUND));

    assert!(unsafe { ScmiClock2::from_raw(core::ptr::null_mut()) }.is_none());
}
