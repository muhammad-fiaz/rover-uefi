//! # QEMU Firmware Configuration (`fw_cfg`) and `ramfb` Interface
//!
//! Provides constants, register definitions, DMA structures, and `ramfb` display
//! configurations used by QEMU virtual platforms across x86, ARM64, and `RISC-V`.
//!
//! Reference: QEMU `docs/specs/fw_cfg.rst`.

use crate::base::EfiGuid;

/// QEMU `ramfb` Device Path GUID.
pub const QEMU_RAMFB_GUID: EfiGuid = EfiGuid::new(
    0x5574_23A1,
    0x63AB,
    0x406C,
    [0xBE, 0x7E, 0x91, 0xCD, 0xBC, 0x08, 0xC4, 0x57],
);

/// Size in bytes of firmware configuration file names in `fw_cfg`.
pub const QEMU_FW_CFG_FNAME_SIZE: usize = 56;

/// I/O port address for QEMU `fw_cfg` item selector.
pub const FW_CFG_IO_SELECTOR: u16 = 0x510;
/// I/O port address for QEMU `fw_cfg` data register.
pub const FW_CFG_IO_DATA: u16 = 0x511;
/// I/O port address for QEMU `fw_cfg` 64-bit DMA address register.
pub const FW_CFG_IO_DMA_ADDRESS: u16 = 0x514;

/// `fw_cfg` DMA feature flag in the revision bitmap.
pub const FW_CFG_F_DMA: u32 = 1 << 1;

/// DMA control bits (big-endian on wire).
pub mod dma_ctl {
    /// Error flag set by host if DMA transfer fails.
    pub const ERROR: u32 = 1 << 0;
    /// Read operation from host to guest RAM.
    pub const READ: u32 = 1 << 1;
    /// Skip bytes without copying.
    pub const SKIP: u32 = 1 << 2;
    /// Select specified item key before transfer.
    pub const SELECT: u32 = 1 << 3;
    /// Write operation from guest RAM to host.
    pub const WRITE: u32 = 1 << 4;
}

/// Standard QEMU `fw_cfg` item keys.
pub mod item_key {
    /// 4-byte signature ("QEMU").
    pub const SIGNATURE: u16 = 0x0000;
    /// Interface revision and feature bits.
    pub const ID: u16 = 0x0001;
    /// System UUID.
    pub const UUID: u16 = 0x0002;
    /// RAM size in bytes.
    pub const RAM_SIZE: u16 = 0x0003;
    /// Graphics enabled flag.
    pub const NOGRAPHIC: u16 = 0x0004;
    /// Number of SMP CPUs.
    pub const NB_CPUS: u16 = 0x0005;
    /// Machine architecture ID.
    pub const MACHINE_ID: u16 = 0x0006;
    /// Linux kernel load address.
    pub const KERNEL_ADDR: u16 = 0x0007;
    /// Linux kernel size.
    pub const KERNEL_SIZE: u16 = 0x0008;
    /// Kernel command line string.
    pub const KERNEL_CMDLINE: u16 = 0x0009;
    /// Initrd load address.
    pub const INITRD_ADDR: u16 = 0x000A;
    /// Initrd size in bytes.
    pub const INITRD_SIZE: u16 = 0x000B;
    /// Boot device index.
    pub const BOOT_DEVICE: u16 = 0x000C;
    /// NUMA node topology data.
    pub const NUMA: u16 = 0x000D;
    /// Maximum supported CPU count.
    pub const MAX_CPUS: u16 = 0x000F;
    /// Directory listing of named `fw_cfg` files.
    pub const FILE_DIR: u16 = 0x0019;

    // x86-specific item keys
    /// ACPI tables provided by QEMU.
    pub const X86_ACPI_TABLES: u16 = 0x8000;
    /// SMBIOS tables provided by QEMU.
    pub const X86_SMBIOS_TABLES: u16 = 0x8001;
    /// E820 memory map table.
    pub const X86_E820_TABLE: u16 = 0x8003;
    /// HPET timer configuration data.
    pub const X86_HPET_DATA: u16 = 0x8004;
}

/// Communication structure for QEMU `fw_cfg` DMA method.
///
/// NOTE: All fields are encoded in **big-endian** per QEMU specification.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FwCfgDmaAccess {
    /// Control flags bitmask (`dma_ctl`).
    pub control: u32,
    /// Length of transfer in bytes.
    pub length: u32,
    /// Physical address in guest RAM.
    pub address: u64,
}

/// Named file entry in `fw_cfg` file directory (`item_key::FILE_DIR`).
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FwCfgFile {
    /// Size of the file in bytes (big-endian).
    pub size: u32,
    /// Selector key assigned to this file (big-endian).
    pub select: u16,
    /// Reserved.
    pub reserved: u16,
    /// NUL-terminated file path/name (up to 56 characters).
    pub name: [u8; QEMU_FW_CFG_FNAME_SIZE],
}

/// `FourCC` identifier for `DRM_FORMAT_XRGB8888` (0x34325258) used by QEMU `ramfb`.
pub const RAMFB_FOURCC_XRGB8888: u32 = 0x3432_5258;

/// Configuration block written to `etc/ramfb` `fw_cfg` item to configure display.
///
/// NOTE: All fields are encoded in **big-endian** on wire.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QemuRamfbConfig {
    /// Physical address of guest linear framebuffer (big-endian).
    pub address: u64,
    /// DRM `FourCC` pixel format (e.g. `RAMFB_FOURCC_XRGB8888`, big-endian).
    pub fourcc: u32,
    /// Display flags (big-endian).
    pub flags: u32,
    /// Display width in pixels (big-endian).
    pub width: u32,
    /// Display height in pixels (big-endian).
    pub height: u32,
    /// Display stride (bytes per line, big-endian).
    pub stride: u32,
}
