//! Tests for `SMBus`, Bluetooth, and `WiFi` 2 hardware protocols and ABI specifications.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::bluetooth::{
    BluetoothAddress, EFI_BLUETOOTH_CONFIG_PROTOCOL_GUID, EFI_BLUETOOTH_HC_PROTOCOL_GUID,
    EFI_BLUETOOTH_IO_PROTOCOL_GUID, EfiBluetoothHcProtocol, EfiBluetoothIoProtocol,
};
use rover_uefi::protocols::network::wifi2::{
    EFI_MAX_SSID_LEN, EFI_WIFI2_PROTOCOL_GUID, EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID,
    Efi80211BssType, Efi80211ConnectNetworkResultCode, Efi80211Ssid,
    EfiWirelessMacConnectionIiProtocol,
};
use rover_uefi::protocols::smbus::{
    EFI_SMBUS_HC_PROTOCOL_GUID, EfiSmbusDeviceAddress, EfiSmbusDeviceMap, EfiSmbusHcProtocol,
    EfiSmbusOperation, EfiSmbusUdid,
};

#[test]
fn test_smbus_guid_and_abi() {
    assert_eq!(
        EFI_SMBUS_HC_PROTOCOL_GUID,
        EfiGuid::new(
            0xE49D_33ED,
            0x513D,
            0x4634,
            [0xB6, 0x98, 0x6F, 0x55, 0xAA, 0x75, 0x1C, 0x1B]
        )
    );

    let dev_addr = EfiSmbusDeviceAddress::new(0x50); // Typical DIMM SPD EEPROM address
    assert_eq!(dev_addr.address(), 0x50);

    let op = EfiSmbusOperation::ReadByte;
    assert_eq!(op as u32, 4);

    let map = EfiSmbusDeviceMap {
        smbus_device_address: dev_addr,
        smbus_device_udid: EfiSmbusUdid {
            vendor_specific_id: 0x1234_5678,
            subsystem_device_id: 0x0001,
            subsystem_vendor_id: 0x8086,
            interface: 0,
            device_id: 0x0042,
            vendor_id: 0x8086,
            vendor_revision: 1,
            device_capabilities: 0,
        },
    };
    assert_eq!(map.smbus_device_address.address(), 0x50);
    assert_eq!(map.smbus_device_udid.vendor_id, 0x8086);

    // EfiSmbusHcProtocol has 4 function pointer fields
    assert_eq!(
        core::mem::size_of::<EfiSmbusHcProtocol>(),
        4 * core::mem::size_of::<usize>()
    );
}

#[test]
fn test_bluetooth_guid_and_abi() {
    assert_eq!(
        EFI_BLUETOOTH_HC_PROTOCOL_GUID,
        EfiGuid::new(
            0xB393_0571,
            0xBEBA,
            0x4FC5,
            [0x92, 0x03, 0x94, 0x27, 0x24, 0x2E, 0x6A, 0x43]
        )
    );
    assert_eq!(
        EFI_BLUETOOTH_IO_PROTOCOL_GUID,
        EfiGuid::new(
            0x4673_13DE,
            0x4E30,
            0x43F1,
            [0x94, 0x3E, 0x32, 0x3F, 0x89, 0x84, 0x5D, 0xb5]
        )
    );
    assert_eq!(
        EFI_BLUETOOTH_CONFIG_PROTOCOL_GUID,
        EfiGuid::new(
            0x6296_0CF3,
            0x40FF,
            0x4263,
            [0xA7, 0x7C, 0xDF, 0xDE, 0xBD, 0x19, 0x1B, 0x4B]
        )
    );

    let mac = BluetoothAddress::new([0x00, 0x1A, 0x7D, 0xDA, 0x71, 0x13]);
    assert_eq!(mac.address[0], 0x00);
    assert_eq!(mac.address[5], 0x13);

    // EfiBluetoothHcProtocol has 9 function pointer fields
    assert_eq!(
        core::mem::size_of::<EfiBluetoothHcProtocol>(),
        9 * core::mem::size_of::<usize>()
    );

    // EfiBluetoothIoProtocol has 5 function pointer fields
    assert_eq!(
        core::mem::size_of::<EfiBluetoothIoProtocol>(),
        5 * core::mem::size_of::<usize>()
    );
}

#[test]
fn test_wifi2_guid_and_abi() {
    assert_eq!(
        EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID,
        EfiGuid::new(
            0x1B0F_B9BF,
            0x699D,
            0x4FDD,
            [0xA7, 0xC3, 0x25, 0x46, 0x68, 0x1B, 0xF6, 0x3B]
        )
    );
    assert_eq!(
        EFI_WIFI2_PROTOCOL_GUID,
        EFI_WIRELESS_MAC_CONNECTION_II_PROTOCOL_GUID
    );

    assert_eq!(EFI_MAX_SSID_LEN, 32);

    let ssid = Efi80211Ssid {
        ssid_len: 8,
        ssid: [
            b'R', b'o', b'v', b'e', b'r', b'N', b'e', b't', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    };
    assert_eq!(ssid.ssid_len, 8);
    assert_eq!(&ssid.ssid[0..8], b"RoverNet");

    let bss = Efi80211BssType::Infrastructure;
    assert_eq!(bss as u32, 0);

    let res = Efi80211ConnectNetworkResultCode::Success;
    assert_eq!(res as u32, 0);

    // EfiWirelessMacConnectionIiProtocol has 3 function pointers
    assert_eq!(
        core::mem::size_of::<EfiWirelessMacConnectionIiProtocol>(),
        3 * core::mem::size_of::<usize>()
    );
}
