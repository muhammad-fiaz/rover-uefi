//! Comprehensive integration tests for extended hardware, driver, and platform protocols:
//! Android Boot Image, Fastboot, Boot Control, AVB, OS Configuration,
//! Memory Attribute, Memory Accept, Device Tree Fixup,
//! SD/MMC Override, UFS, USB Ethernet, Media Sanitize, Tape I/O, NVDIMM, Xen, and ISA.

use core::mem::size_of;
use rover_uefi::base::{EfiGuid, EfiPhysicalAddress, EfiStatus};
use rover_uefi::protocols::android::*;
use rover_uefi::protocols::driver::isa::*;
use rover_uefi::protocols::driver::ovmf::OVMF_SEV_MEMORY_ACCEPTANCE_PROTOCOL_GUID;
use rover_uefi::protocols::driver::ovmf::*;
use rover_uefi::protocols::driver::xen::*;
use rover_uefi::protocols::firmware::dt_fixup::*;
use rover_uefi::protocols::firmware::memory_accept::*;
use rover_uefi::protocols::firmware::memory_attribute::*;
use rover_uefi::protocols::network::usb_ethernet::*;
use rover_uefi::protocols::storage::media_sanitize::*;
use rover_uefi::protocols::storage::nvdimm::*;
use rover_uefi::protocols::storage::sd_mmc_override::*;
use rover_uefi::protocols::storage::tape_io::*;
use rover_uefi::protocols::storage::ufs::*;

#[test]
fn test_android_boot_protocols() {
    assert_eq!(
        ANDROID_BOOTIMG_PROTOCOL_GUID,
        EfiGuid::new(
            0x9859_BB19,
            0x407C,
            0x4F8B,
            [0xBC, 0xE1, 0xF8, 0xDA, 0x65, 0x65, 0xF4, 0xA5]
        )
    );
    assert_eq!(
        ANDROID_FASTBOOT_PLATFORM_PROTOCOL_GUID,
        EfiGuid::new(
            0x5246_85A0,
            0x89A0,
            0x11E3,
            [0x9D, 0x4D, 0xBF, 0xA9, 0xF6, 0xA4, 0x03, 0x08]
        )
    );
    assert_eq!(
        ANDROID_FASTBOOT_TRANSPORT_PROTOCOL_GUID,
        EfiGuid::new(
            0x74BD_9FE0,
            0x8902,
            0x11E3,
            [0xB9, 0xD3, 0xF7, 0x22, 0x38, 0xFC, 0x9A, 0x31]
        )
    );
    assert_eq!(
        GBL_EFI_BOOT_CONTROL_PROTOCOL_GUID,
        EfiGuid::new(
            0xD382_DB1B,
            0x9AC2,
            0x11F0,
            [0x84, 0xC7, 0x04, 0x7B, 0xCB, 0xA9, 0x60, 0x19]
        )
    );
    assert_eq!(GBL_EFI_BOOT_CONTROL_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(
        GBL_EFI_AVB_PROTOCOL_GUID,
        EfiGuid::new(
            0x6BC6_6B9A,
            0xD5C9,
            0x4C02,
            [0x9D, 0xA9, 0x50, 0xAF, 0x19, 0x8D, 0x91, 0x2C]
        )
    );
    assert_eq!(GBL_EFI_AVB_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(
        GBL_EFI_OS_CONFIGURATION_PROTOCOL_GUID,
        EfiGuid::new(
            0xDDA0_D135,
            0xAA5B,
            0x42FF,
            [0x85, 0xAC, 0xE3, 0xAD, 0x6E, 0xFB, 0x46, 0x19]
        )
    );
    assert_eq!(GBL_EFI_OS_CONFIGURATION_PROTOCOL_REVISION, 0x0001_0000);

    let slot_a = GblEfiSlotInfo {
        suffix: 'a' as u32,
        unbootable_reason: GblEfiUnbootableReason::Unknown as u8,
        priority: 15,
        remaining_tries: 7,
        successful: 1,
    };
    assert_eq!(slot_a.suffix, 'a' as u32);
    assert_eq!(slot_a.priority, 15);

    let loaded_os = GblEfiLoadedOs::default();
    assert_eq!(loaded_os.kernel_size, 0);
}

#[test]
fn test_memory_attribute_and_accept_protocols() {
    assert_eq!(
        EFI_MEMORY_ATTRIBUTE_PROTOCOL_GUID,
        EfiGuid::new(
            0xF456_0CF6,
            0x40EC,
            0x4B4A,
            [0xA1, 0x92, 0xBF, 0x1D, 0x57, 0xD0, 0xB1, 0x89]
        )
    );
    assert_eq!(
        MEMORY_ACCEPT_PROTOCOL_GUID,
        EfiGuid::new(
            0x38C7_4800,
            0x5590,
            0x4DB4,
            [0xA0, 0xF3, 0x67, 0x5D, 0x9B, 0x8E, 0x80, 0x26]
        )
    );

    assert_eq!(memory_attribute::RO, 0x0002_0000);
    assert_eq!(memory_attribute::XP, 0x0000_4000);
    assert_eq!(memory_attribute::RP, 0x0000_2000);
    assert_eq!(memory_attribute::WB, 0x0000_0008);

    assert_eq!(
        DT_FIXUP_PROTOCOL_GUID,
        EfiGuid::new(
            0x60ED_6BA9,
            0xDFEF,
            0x4799,
            [0xAC, 0x7B, 0x75, 0xE0, 0xF8, 0x33, 0x45, 0x6C]
        )
    );
    assert_eq!(DT_FIXUP_PROTOCOL_REVISION, 0x0001_0000);
}

#[test]
fn test_virtual_machine_and_hypervisor_protocols() {
    assert_eq!(
        OVMF_LOADED_X86_LINUX_KERNEL_PROTOCOL_GUID,
        EfiGuid::new(
            0xA3ED_C05D,
            0xB618,
            0x4FF6,
            [0x95, 0x52, 0x76, 0xD7, 0x88, 0x63, 0x43, 0xC8]
        )
    );
    assert_eq!(
        OVMF_SEV_MEMORY_ACCEPTANCE_PROTOCOL_GUID,
        EfiGuid::new(
            0xC5A0_10FE,
            0x38A7,
            0x4531,
            [0x8A, 0x4A, 0x05, 0x00, 0xD2, 0xFD, 0x16, 0x49]
        )
    );
    assert_eq!(
        XENIO_PROTOCOL_GUID,
        EfiGuid::new(
            0x6EFA_C84F,
            0x0AB0,
            0x4747,
            [0x81, 0xBE, 0x85, 0x55, 0x62, 0x59, 0x04, 0x49]
        )
    );
    assert_eq!(
        XENBUS_PROTOCOL_GUID,
        EfiGuid::new(
            0x3D3C_A290,
            0xB9A5,
            0x11E3,
            [0xB7, 0x5D, 0xB8, 0xAC, 0x6F, 0x7D, 0x65, 0xE6]
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
        size_of::<OvmfLoadedX86LinuxKernel>(),
        size_of::<[usize; 8]>()
    );
    assert_eq!(XenBusState::Connected as u32, 4);
    assert_eq!(XenStoreStatus::Success as u32, 0);
}

#[test]
fn test_storage_and_network_protocols() {
    assert_eq!(
        SD_MMC_OVERRIDE_PROTOCOL_GUID,
        EfiGuid::new(
            0xEAF9_E3C1,
            0xC9CD,
            0x46DB,
            [0xA5, 0xE5, 0x5A, 0x12, 0x4C, 0x83, 0x23, 0x23]
        )
    );
    assert_eq!(SD_MMC_OVERRIDE_PROTOCOL_VERSION, 3);
    assert_eq!(SD_MMC_BUS_WIDTH_IGNORE, u8::MAX);
    assert_eq!(SD_MMC_CLOCK_FREQ_IGNORE, u32::MAX);

    assert_eq!(
        EFI_UFS_DEVICE_CONFIG_PROTOCOL_GUID,
        EfiGuid::new(
            0xB81B_FAB0,
            0x0EB3,
            0x4CF9,
            [0x84, 0x65, 0x7F, 0xA9, 0x86, 0x36, 0x16, 0x64]
        )
    );
    assert_eq!(
        UFS_HOST_CONTROLLER_PROTOCOL_GUID,
        EfiGuid::new(
            0xEBC0_1AF5,
            0x07A9,
            0x489E,
            [0xB7, 0xCE, 0xDC, 0x08, 0x9E, 0x45, 0x9B, 0x2F]
        )
    );
    assert_eq!(
        UFS_HC_PLATFORM_PROTOCOL_GUID,
        EfiGuid::new(
            0x3D18_BA13,
            0xD9B1,
            0x4DD4,
            [0xB9, 0x16, 0xD3, 0x07, 0x96, 0x53, 0x9E, 0xD8]
        )
    );

    assert_eq!(
        USB_ETHERNET_PROTOCOL_GUID,
        EfiGuid::new(
            0x8D89_69CC,
            0xFEB0,
            0x4303,
            [0xB2, 0x1A, 0x1F, 0x11, 0x6F, 0x38, 0x56, 0x43]
        )
    );
    assert_eq!(USB_CDC_CLASS, 0x02);
    assert_eq!(USB_CDC_ECM_SUBCLASS, 0x06);
    assert_eq!(USB_CDC_NCM_SUBCLASS, 0x0D);

    assert_eq!(
        MEDIA_SANITIZE_PROTOCOL_GUID,
        EfiGuid::new(
            0x0D79_9A99,
            0x25AF,
            0x429E,
            [0x92, 0x72, 0xD0, 0xB2, 0x7D, 0x6D, 0x5F, 0x14]
        )
    );
    assert_eq!(MEDIA_SANITIZE_PROTOCOL_REVISION, 0x0001_0000);
    assert_eq!(purge_action::BLOCK_ERASE, 0x0000_0002);
    assert_eq!(purge_action::CRYPTO_ERASE, 0x0000_0004);

    assert_eq!(
        EFI_TAPE_IO_PROTOCOL_GUID,
        EfiGuid::new(
            0x1E93_E633,
            0xD65A,
            0x459E,
            [0xAB, 0x84, 0x93, 0xD9, 0xEC, 0x26, 0x6D, 0x18]
        )
    );
    assert_eq!(size_of::<EfiTapeHeader>(), 1024);

    assert_eq!(
        EFI_NVDIMM_LABEL_PROTOCOL_GUID,
        EfiGuid::new(
            0xD40B_6B80,
            0x97D5,
            0x4282,
            [0xBB, 0x1D, 0x22, 0x3A, 0x16, 0x91, 0x80, 0x58]
        )
    );
    assert_eq!(EFI_NVDIMM_LABEL_INDEX_SIG_LEN, 16);
    assert_eq!(EFI_NVDIMM_LABEL_INDEX_ALIGN, 256);
}

#[test]
fn test_safe_client_wrappers_mock() {
    let mem_attr_raw = EfiMemoryAttributeProtocol {
        get_memory_attributes: Some(mock_get_mem_attrs),
        set_memory_attributes: Some(mock_set_mem_attrs),
        clear_memory_attributes: None,
    };
    let client = MemoryAttribute::new(&mem_attr_raw);
    let attrs = client.get_attributes(EfiPhysicalAddress::from_u64(0x1000), 0x1000);
    assert_eq!(attrs, Ok(memory_attribute::RO | memory_attribute::XP));
    assert_eq!(
        client.set_attributes(
            EfiPhysicalAddress::from_u64(0x1000),
            0x1000,
            memory_attribute::RO
        ),
        Ok(())
    );
    assert_eq!(
        client.clear_attributes(
            EfiPhysicalAddress::from_u64(0x1000),
            0x1000,
            memory_attribute::RO
        ),
        Err(EfiStatus::UNSUPPORTED)
    );

    let mem_accept_raw = MemoryAcceptProtocol {
        accept_memory: Some(mock_accept_memory),
    };
    let accept_client = MemoryAccept::new(&mem_accept_raw);
    assert_eq!(
        accept_client.accept(EfiPhysicalAddress::from_u64(0x2000), 4096),
        Ok(())
    );
}

unsafe extern "efiapi" fn mock_get_mem_attrs(
    _this: *const EfiMemoryAttributeProtocol,
    _base: EfiPhysicalAddress,
    _length: u64,
    attrs: *mut u64,
) -> EfiStatus {
    unsafe {
        *attrs = memory_attribute::RO | memory_attribute::XP;
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_set_mem_attrs(
    _this: *const EfiMemoryAttributeProtocol,
    _base: EfiPhysicalAddress,
    _length: u64,
    _attrs: u64,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_accept_memory(
    _this: *const MemoryAcceptProtocol,
    _start: EfiPhysicalAddress,
    _size: usize,
) -> EfiStatus {
    EfiStatus::SUCCESS
}
