//! Integration tests for Platform Configuration Database (PCD) subsystem.

use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::{
    EFI_GET_PCD_INFO_PROTOCOL_GUID, EFI_PCD_DEFAULT_SKU_ID, EFI_PCD_INVALID_TOKEN_NUMBER,
    EFI_PCD_PROTOCOL_GUID, EFI_PEI_GET_PCD_INFO_PPI_GUID, EFI_PEI_PCD_PPI_GUID, EfiPcdType,
    PcdDatabase, PcdEntry, PcdValue,
};

const TEST_TOKEN_SPACE: EfiGuid = EfiGuid::new(
    0x1122_3344,
    0x5566,
    0x7788,
    [0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00],
);

#[test]
fn test_pcd_guids_and_constants() {
    assert_ne!(EFI_PCD_PROTOCOL_GUID, EFI_GET_PCD_INFO_PROTOCOL_GUID);
    assert_ne!(EFI_PEI_PCD_PPI_GUID, EFI_PEI_GET_PCD_INFO_PPI_GUID);
    assert_eq!(EFI_PCD_INVALID_TOKEN_NUMBER, 0);
    assert_eq!(EFI_PCD_DEFAULT_SKU_ID, 0);
}

#[test]
fn test_pcd_value_types_and_sizes() {
    let u8_val = PcdValue::U8(42);
    assert_eq!(u8_val.pcd_type(), EfiPcdType::Type8);
    assert_eq!(u8_val.size(), 1);

    let u16_val = PcdValue::U16(1024);
    assert_eq!(u16_val.pcd_type(), EfiPcdType::Type16);
    assert_eq!(u16_val.size(), 2);

    let u32_val = PcdValue::U32(0xdead_beef);
    assert_eq!(u32_val.pcd_type(), EfiPcdType::Type32);
    assert_eq!(u32_val.size(), 4);

    let u64_val = PcdValue::U64(0x0123_4567_89ab_cdef);
    assert_eq!(u64_val.pcd_type(), EfiPcdType::Type64);
    assert_eq!(u64_val.size(), 8);

    let bool_val = PcdValue::Bool(true);
    assert_eq!(bool_val.pcd_type(), EfiPcdType::TypeBool);
    assert_eq!(bool_val.size(), 1);

    let bytes_val = PcdValue::Bytes(b"RoverFirmware");
    assert_eq!(bytes_val.pcd_type(), EfiPcdType::TypePtr);
    assert_eq!(bytes_val.size(), 13);
}

#[test]
fn test_pcd_database_registration_and_lookup() {
    let mut db = PcdDatabase::<16>::new();
    assert_eq!(db.count(), 0);
    assert_eq!(db.sku(), 0);

    // Register valid entries
    let entry_u8 = PcdEntry {
        token_space: TEST_TOKEN_SPACE,
        token_number: 1,
        name: "PcdDebugPrintErrorLevel",
        sku_id: 0,
        value: PcdValue::U8(0x80),
    };
    assert!(db.register(entry_u8).is_ok());

    let entry_u32 = PcdEntry {
        token_space: TEST_TOKEN_SPACE,
        token_number: 2,
        name: "PcdSerialBaudRate",
        sku_id: 0,
        value: PcdValue::U32(115_200),
    };
    assert!(db.register(entry_u32).is_ok());

    let entry_bool = PcdEntry {
        token_space: TEST_TOKEN_SPACE,
        token_number: 3,
        name: "PcdSecureBootEnable",
        sku_id: 0,
        value: PcdValue::Bool(true),
    };
    assert!(db.register(entry_bool).is_ok());

    let entry_str = PcdEntry {
        token_space: TEST_TOKEN_SPACE,
        token_number: 4,
        name: "PcdPlatformVendor",
        sku_id: 0,
        value: PcdValue::Bytes(b"RoverOS"),
    };
    assert!(db.register(entry_str).is_ok());
    assert_eq!(db.count(), 4);

    // Read back values
    if let Ok(val) = db.get_u8(&TEST_TOKEN_SPACE, 1) {
        assert_eq!(val, 0x80);
    } else {
        panic!("failed to retrieve u8 pcd");
    }

    if let Ok(val) = db.get_u32(&TEST_TOKEN_SPACE, 2) {
        assert_eq!(val, 115_200);
    } else {
        panic!("failed to retrieve u32 pcd");
    }

    if let Ok(val) = db.get_bool(&TEST_TOKEN_SPACE, 3) {
        assert!(val);
    } else {
        panic!("failed to retrieve bool pcd");
    }

    if let Ok(val) = db.get_bytes(&TEST_TOKEN_SPACE, 4) {
        assert_eq!(val, b"RoverOS");
    } else {
        panic!("failed to retrieve bytes pcd");
    }

    // Type mismatch errors
    assert!(matches!(
        db.get_u32(&TEST_TOKEN_SPACE, 1),
        Err(EfiStatus::INVALID_PARAMETER)
    ));
    assert!(matches!(
        db.get_bool(&TEST_TOKEN_SPACE, 2),
        Err(EfiStatus::INVALID_PARAMETER)
    ));

    // Missing token
    assert!(matches!(
        db.get_u8(&TEST_TOKEN_SPACE, 99),
        Err(EfiStatus::NOT_FOUND)
    ));

    // Update existing token
    assert!(db.set_u8(&TEST_TOKEN_SPACE, 1, 0x8f).is_ok());
    if let Ok(val) = db.get_u8(&TEST_TOKEN_SPACE, 1) {
        assert_eq!(val, 0x8f);
    }

    // Lookup by name
    if let Some((guid, tok)) = db.find_token_by_name("PcdSerialBaudRate") {
        assert_eq!(guid, TEST_TOKEN_SPACE);
        assert_eq!(tok, 2);
    } else {
        panic!("token lookup by name failed");
    }
}

#[test]
fn test_pcd_database_sku_overrides_and_invalid_tokens() {
    let mut db = PcdDatabase::<8>::new();

    // Reject token 0
    let invalid_entry = PcdEntry {
        token_space: TEST_TOKEN_SPACE,
        token_number: 0,
        name: "PcdInvalid",
        sku_id: 0,
        value: PcdValue::U8(1),
    };
    assert!(matches!(
        db.register(invalid_entry),
        Err(EfiStatus::INVALID_PARAMETER)
    ));

    // Default SKU entry (sku_id = 0)
    let def_entry = PcdEntry {
        token_space: TEST_TOKEN_SPACE,
        token_number: 10,
        name: "PcdMemoryChannelCount",
        sku_id: 0,
        value: PcdValue::U8(2),
    };
    assert!(db.register(def_entry).is_ok());

    // SKU-specific override (sku_id = 1)
    let sku1_entry = PcdEntry {
        token_space: TEST_TOKEN_SPACE,
        token_number: 10,
        name: "PcdMemoryChannelCount",
        sku_id: 1,
        value: PcdValue::U8(4),
    };
    assert!(db.register(sku1_entry).is_ok());

    // With active SKU 0, we get the default
    db.set_sku(0);
    if let Ok(val) = db.get_u8(&TEST_TOKEN_SPACE, 10) {
        assert_eq!(val, 2);
    }

    // Switch to active SKU 1, we get 4
    db.set_sku(1);
    if let Ok(val) = db.get_u8(&TEST_TOKEN_SPACE, 10) {
        assert_eq!(val, 4);
    }

    // Switch to active SKU 2 (not defined), should fallback to default SKU 0 (val = 2)
    db.set_sku(2);
    if let Ok(val) = db.get_u8(&TEST_TOKEN_SPACE, 10) {
        assert_eq!(val, 2);
    }
}
