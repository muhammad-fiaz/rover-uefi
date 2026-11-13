//! Integration tests for QEMU virtual hardware, `VirtIO` GPU, `BootLogo2`,
//! ARM Multi-Core PPI, and Non-Discoverable Devices.

use core::mem::size_of;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::driver::non_discoverable::*;
use rover_uefi::protocols::driver::qemu_fw_cfg::*;
use rover_uefi::protocols::driver::virtio::*;
use rover_uefi::protocols::firmware::ppi::arm_mp_core::*;
use rover_uefi::protocols::graphics::boot_logo::*;
use rover_uefi::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;

#[test]
fn test_qemu_fw_cfg_and_ramfb_layouts() {
    assert_eq!(FW_CFG_IO_SELECTOR, 0x510);
    assert_eq!(FW_CFG_IO_DATA, 0x511);
    assert_eq!(FW_CFG_IO_DMA_ADDRESS, 0x514);
    assert_eq!(FW_CFG_F_DMA, 1 << 1);
    assert_eq!(RAMFB_FOURCC_XRGB8888, 0x3432_5258);
    assert_eq!(QEMU_FW_CFG_FNAME_SIZE, 56);

    assert_eq!(
        QEMU_RAMFB_GUID,
        EfiGuid::new(
            0x5574_23A1,
            0x63AB,
            0x406C,
            [0xBE, 0x7E, 0x91, 0xCD, 0xBC, 0x08, 0xC4, 0x57]
        )
    );

    assert_eq!(size_of::<FwCfgDmaAccess>(), 16);
    assert_eq!(size_of::<FwCfgFile>(), 64);
    assert_eq!(size_of::<QemuRamfbConfig>(), 28);

    let dma = FwCfgDmaAccess {
        control: (dma_ctl::SELECT | dma_ctl::READ).to_be(),
        length: 64u32.to_be(),
        address: 0x1000_0000u64.to_be(),
    };
    let dma_control = dma.control;
    assert_eq!(dma_control, (0x0008 | 0x0002u32).to_be());
}

#[test]
fn test_virtio_gpu_control_structures() {
    assert_eq!(size_of::<VirtioGpuControlHeader>(), 24);
    assert_eq!(size_of::<VirtioGpuRectangle>(), 16);
    assert_eq!(size_of::<VirtioGpuResourceCreate2d>(), 40);
    assert_eq!(size_of::<VirtioGpuResourceUnref>(), 32);
    assert_eq!(size_of::<VirtioGpuMemEntry>(), 16);
    assert_eq!(size_of::<VirtioGpuResourceAttachBacking>(), 48);
    assert_eq!(size_of::<VirtioGpuResourceDetachBacking>(), 32);
    assert_eq!(size_of::<VirtioGpuSetScanout>(), 48);
    assert_eq!(size_of::<VirtioGpuTransferToHost2d>(), 56);
    assert_eq!(size_of::<VirtioGpuResourceFlush>(), 48);
    assert_eq!(
        size_of::<VirtioGpuRespDisplayInfo>(),
        size_of::<VirtioGpuControlHeader>() + VIRTIO_GPU_MAX_SCANOUTS * size_of::<VirtioGpuPmode>()
    );

    let rect = VirtioGpuRectangle {
        x: 0,
        y: 0,
        width: 1920,
        height: 1080,
    };
    let create_cmd = VirtioGpuResourceCreate2d {
        header: VirtioGpuControlHeader {
            ctrl_type: VirtioGpuCtrlType::CmdResourceCreate2d as u32,
            flags: 0,
            fence_id: 0,
            ctx_id: 0,
            padding: 0,
        },
        resource_id: 1,
        format: VirtioGpuFormat::B8G8R8X8Unorm as u32,
        width: rect.width,
        height: rect.height,
    };
    let ctrl_type = create_cmd.header.ctrl_type;
    let resource_id = create_cmd.resource_id;
    assert_eq!(ctrl_type, VirtioGpuCtrlType::CmdResourceCreate2d as u32);
    assert_eq!(resource_id, 1);
}

#[test]
fn test_non_discoverable_device_guids() {
    assert_eq!(
        NON_DISCOVERABLE_DEVICE_PROTOCOL_GUID,
        EfiGuid::new(
            0x0D51_905B,
            0xB77E,
            0x452A,
            [0xA2, 0xC0, 0xEC, 0xA0, 0xCC, 0x8D, 0x51, 0x4A]
        )
    );
    assert_eq!(
        NON_DISCOVERABLE_AHCI_DEVICE_GUID,
        EfiGuid::new(
            0xC7D3_5798,
            0xE4D2,
            0x4A93,
            [0xB1, 0x45, 0x54, 0x88, 0x9F, 0x02, 0x58, 0x4B]
        )
    );
    assert_eq!(
        NON_DISCOVERABLE_NVME_DEVICE_GUID,
        EfiGuid::new(
            0xC5F2_5542,
            0x2A79,
            0x4A26,
            [0x81, 0xBB, 0x4E, 0xA6, 0x32, 0x33, 0xB3, 0x09]
        )
    );
    assert_eq!(
        NON_DISCOVERABLE_XHCI_DEVICE_GUID,
        EfiGuid::new(
            0xB1BE_0BC5,
            0x6C28,
            0x442D,
            [0xAA, 0x37, 0x15, 0x1B, 0x42, 0x57, 0xBD, 0x78]
        )
    );
}

#[test]
fn test_boot_logo2_protocol() {
    unsafe extern "efiapi" fn mock_set_boot_logo2(
        _this: *mut BootLogo2Protocol,
        _blt_buffer: *const EfiGraphicsOutputBltPixel,
        destination_x: usize,
        destination_y: usize,
        width: usize,
        height: usize,
    ) -> EfiStatus {
        if width == 100 && height == 100 && destination_x == 10 && destination_y == 10 {
            EfiStatus::SUCCESS
        } else {
            EfiStatus::INVALID_PARAMETER
        }
    }

    let protocol = BootLogo2Protocol {
        set_boot_logo: Some(mock_set_boot_logo2),
        get_boot_logo: None,
    };

    assert_eq!(
        BOOT_LOGO2_PROTOCOL_GUID,
        EfiGuid::new(
            0x4B5D_C1DF,
            0x1EAA,
            0x48B2,
            [0xA7, 0xE9, 0xEA, 0xC4, 0x89, 0xA0, 0x0B, 0x5C]
        )
    );

    let mut mock_prot = protocol;
    let status = match mock_prot.set_boot_logo {
        Some(set_fn) => unsafe {
            (set_fn)(&raw mut mock_prot, core::ptr::null(), 10, 10, 100, 100)
        },
        None => EfiStatus::UNSUPPORTED,
    };
    assert_eq!(status, EfiStatus::SUCCESS);
}

#[test]
fn test_arm_mp_core_info_ppi() {
    assert_eq!(
        ARM_MP_CORE_INFO_PPI_GUID,
        EfiGuid::new(
            0x6847_CC74,
            0xE9EC,
            0x4F8F,
            [0xA2, 0x9D, 0xAB, 0x44, 0xE7, 0x54, 0xA8, 0xFC]
        )
    );
    assert_eq!(
        ARM_MP_CORE_INFO_GUID,
        EfiGuid::new(
            0xA4EE_0728,
            0xE5D7,
            0x4AC5,
            [0xB2, 0x1E, 0x65, 0x8E, 0xD8, 0x57, 0xE8, 0x34]
        )
    );
    assert_eq!(MAX_CPUS_PER_MPCORE_SYSTEM, 4);

    let core0 = ArmCoreInfo {
        mpidr: 0x8000_0000,
        mailbox_set_address: 0x4000_0000,
        mailbox_get_address: 0x4000_0008,
        mailbox_clear_address: 0x4000_0010,
        mailbox_clear_value: 0,
    };
    assert_eq!(core0.mpidr, 0x8000_0000);
}
