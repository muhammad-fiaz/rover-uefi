//! # Linux x86 Boot Protocol and bzImage Support
//!
//! Provides structures, constants, and validation routines for the Linux x86
//! 16-bit, 32-bit, and 64-bit boot protocols, including the real-mode kernel header
//! (`SetupHeader`), zero-page parameters (`BootParams`), EFI handover protocol,
//! and E820 memory mapping.

#![allow(clippy::cast_possible_truncation)]

use core::mem::size_of;

/// Standard boot signature at offset `0x1FE` in the boot sector (`0xAA55`).
pub const BOOT_SIGNATURE: u16 = 0xAA55;

/// Magic identifier for Linux kernel setup header (`"HdrS"` in little-endian).
pub const SETUP_HEADER_MAGIC: u32 = 0x5372_6448;

/// Minimum setup header size required for protocol version 2.00+.
pub const MIN_SETUP_HEADER_SIZE: usize = 0x50;

/// Offset of the `SetupHeader` within the boot sector / real-mode kernel.
pub const SETUP_HEADER_OFFSET: usize = 0x1F1;

// E820 memory entry types
/// Normal usable system RAM.
pub const E820_RAM: u32 = 1;
/// Reserved memory; must not be used by the OS.
pub const E820_RESERVED: u32 = 2;
/// ACPI Reclaimable memory.
pub const E820_ACPI: u32 = 3;
/// ACPI Non-Volatile Sleep (NVS) memory.
pub const E820_NVS: u32 = 4;
/// Defective / unusable memory.
pub const E820_UNUSABLE: u32 = 5;
/// Persistent memory (PMEM).
pub const E820_PMEM: u32 = 7;

// Extended Load Flags (`xloadflags`)
/// 64-bit kernel entry point supported.
pub const XLF_KERNEL_64: u16 = 1 << 0;
/// Kernel can be loaded above 4GB physical address.
pub const XLF_CAN_BE_LOADED_ABOVE_4G: u16 = 1 << 1;
/// 32-bit EFI handover protocol supported.
pub const XLF_EFI_HANDOVER_32: u16 = 1 << 2;
/// 64-bit EFI handover protocol supported.
pub const XLF_EFI_HANDOVER_64: u16 = 1 << 3;
/// Kernel supports EFI kexec boot.
pub const XLF_EFI_KEXEC: u16 = 1 << 4;
/// 5-level paging supported.
pub const XLF_5LEVEL: u16 = 1 << 5;
/// 5-level paging enabled.
pub const XLF_5LEVEL_ENABLED: u16 = 1 << 6;

// Load flags (`load_flags`)
/// Kernel is loaded high (bzImage).
pub const FLAG_LOADED_HIGH: u8 = 1 << 0;
/// Quiet boot mode.
pub const FLAG_QUIET: u8 = 1 << 5;
/// Kernel is relocatable.
pub const FLAG_KEEP_SEGMENTS: u8 = 1 << 6;
/// Kernel can use heap.
pub const FLAG_CAN_USE_HEAP: u8 = 1 << 7;

/// Describes a physical memory segment in the E820 memory map.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct E820Entry {
    /// Physical start address of the segment.
    pub addr: u64,
    /// Size of the memory segment in bytes.
    pub size: u64,
    /// Memory segment type (`E820_*`).
    pub entry_type: u32,
}

impl E820Entry {
    /// Create a new E820 memory map entry.
    #[must_use]
    pub const fn new(addr: u64, size: u64, entry_type: u32) -> Self {
        Self {
            addr,
            size,
            entry_type,
        }
    }

    /// Check if the entry represents usable RAM.
    #[must_use]
    pub const fn is_ram(&self) -> bool {
        self.entry_type == E820_RAM
    }
}

/// Linux kernel real-mode setup header (defined in `arch/x86/boot/header.S`).
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetupHeader {
    /// Number of setup sectors.
    pub setup_secs: u8,
    /// Root flags.
    pub root_flags: u16,
    /// System size in 16-byte paragraphs.
    pub sys_size: u32,
    /// RAM size (obsolete).
    pub ram_size: u16,
    /// Video mode control.
    pub video_mode: u16,
    /// Root device number (obsolete).
    pub root_dev: u16,
    /// Boot sector signature (`0xAA55`).
    pub signature: u16,
    /// Jump instruction to setup code.
    pub jump: u16,
    /// Magic header signature (`SETUP_HEADER_MAGIC`, `"HdrS"`).
    pub header: u32,
    /// Boot protocol version (e.g. `0x020E` for 2.14).
    pub version: u16,
    /// Real-mode switch hook.
    pub su_switch: u16,
    /// Real-mode setup segment.
    pub setup_seg: u16,
    /// Start system segment.
    pub start_sys: u16,
    /// Kernel version string offset.
    pub kernel_ver: u16,
    /// Type of boot loader.
    pub loader_id: u8,
    /// Boot load flags (`FLAG_*`).
    pub load_flags: u8,
    /// Move size.
    pub move_size: u16,
    /// 32-bit protected-mode kernel entry point.
    pub code32_start: u32,
    /// 32-bit physical address of initial ramdisk.
    pub ramdisk_start: u32,
    /// Length of initial ramdisk in bytes.
    pub ramdisk_len: u32,
    /// Boot sector kludge.
    pub bootsect_kludge: u32,
    /// End of heap offset.
    pub heap_end: u16,
    /// Extended bootloader version.
    pub ext_loader_ver: u8,
    /// Extended bootloader ID.
    pub ext_loader_type: u8,
    /// 32-bit physical address of kernel command line string.
    pub cmd_line_ptr: u32,
    /// Highest legal initial ramdisk address.
    pub ramdisk_max: u32,
    /// Required physical alignment for relocatable kernel.
    pub kernel_alignment: u32,
    /// Non-zero if kernel is relocatable.
    pub relocatable_kernel: u8,
    /// Minimum alignment (expressed as power of 2).
    pub min_alignment: u8,
    /// Extended load flags (`XLF_*`).
    pub xloadflags: u16,
    /// Maximum size of command line.
    pub cmdline_size: u32,
    /// Hardware subarchitecture ID.
    pub hardware_subarch: u32,
    /// Hardware subarchitecture data.
    pub hardware_subarch_data: u64,
    /// Offset to kernel payload.
    pub payload_offset: u32,
    /// Length of kernel payload.
    pub payload_length: u32,
    /// 64-bit physical address of setup data linked list.
    pub setup_data: u64,
    /// Preferred physical load address.
    pub pref_address: u64,
    /// Linear memory size needed for kernel decompression.
    pub init_size: u32,
    /// Handover function offset from kernel base.
    pub handover_offset: u32,
}

impl SetupHeader {
    /// Validate that this setup header contains the correct magic and boot signature.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let sig = self.signature;
        let hdr = self.header;
        sig == BOOT_SIGNATURE && hdr == SETUP_HEADER_MAGIC
    }

    /// Check if the kernel is loaded high (bzImage).
    #[must_use]
    pub fn is_loaded_high(&self) -> bool {
        (self.load_flags & FLAG_LOADED_HIGH) != 0
    }

    /// Check if the kernel is relocatable in physical memory.
    #[must_use]
    pub fn is_relocatable(&self) -> bool {
        self.relocatable_kernel != 0
    }

    /// Check if 64-bit kernel entry point is supported.
    #[must_use]
    pub fn supports_64bit_kernel(&self) -> bool {
        (self.xloadflags & XLF_KERNEL_64) != 0
    }

    /// Check if 64-bit EFI handover protocol is supported.
    #[must_use]
    pub fn supports_efi_handover_64(&self) -> bool {
        (self.xloadflags & XLF_EFI_HANDOVER_64) != 0
    }

    /// Check if 32-bit EFI handover protocol is supported.
    #[must_use]
    pub fn supports_efi_handover_32(&self) -> bool {
        (self.xloadflags & XLF_EFI_HANDOVER_32) != 0
    }

    /// Check if the kernel can be loaded above 4GB physical memory.
    #[must_use]
    pub fn can_load_above_4g(&self) -> bool {
        (self.xloadflags & XLF_CAN_BE_LOADED_ABOVE_4G) != 0
    }

    /// Get the 64-bit EFI handover entry offset if supported.
    #[must_use]
    pub fn efi_handover_offset_64(&self) -> Option<u32> {
        if self.supports_efi_handover_64() {
            Some(self.handover_offset + 0x200)
        } else {
            None
        }
    }
}

/// Screen information for linear frame buffer passing to Linux.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScreenInfo {
    /// Cursor column.
    pub orig_x: u8,
    /// Cursor row.
    pub orig_y: u8,
    /// Extended memory size in KB.
    pub ext_mem_k: u16,
    /// Video page number.
    pub orig_video_page: u16,
    /// Video mode.
    pub orig_video_mode: u8,
    /// Video columns.
    pub orig_video_cols: u8,
    /// Screen flags.
    pub flags: u8,
    /// Reserved byte.
    pub unused2: u8,
    /// EGA BX register.
    pub orig_video_ega_bx: u16,
    /// Reserved word.
    pub unused3: u16,
    /// Video lines.
    pub orig_video_lines: u8,
    /// Non-zero if VGA display.
    pub orig_video_is_vga: u8,
    /// Video font point size.
    pub orig_video_points: u16,
    /// Linear frame buffer width in pixels.
    pub lfb_width: u16,
    /// Linear frame buffer height in pixels.
    pub lfb_height: u16,
    /// Color depth in bits per pixel.
    pub lfb_depth: u16,
    /// Physical base address of linear frame buffer.
    pub lfb_base: u32,
    /// Size of linear frame buffer in bytes.
    pub lfb_size: u32,
    /// Command line magic and offset.
    pub cl_magic: u16,
    /// Command line offset.
    pub cl_offset: u16,
    /// Bytes per scanline.
    pub lfb_linelength: u16,
    /// Red mask size in bits.
    pub red_size: u8,
    /// Red field position in bit offset.
    pub red_pos: u8,
    /// Green mask size in bits.
    pub green_size: u8,
    /// Green field position in bit offset.
    pub green_pos: u8,
    /// Blue mask size in bits.
    pub blue_size: u8,
    /// Blue field position in bit offset.
    pub blue_pos: u8,
    /// Reserved mask size in bits.
    pub rsvd_size: u8,
    /// Reserved field position in bit offset.
    pub rsvd_pos: u8,
    /// VESA PM segment.
    pub vesapm_seg: u16,
    /// VESA PM offset.
    pub vesapm_off: u16,
    /// Video memory pages.
    pub pages: u16,
    /// VESA attributes.
    pub vesa_attributes: u16,
    /// VBE capabilities.
    pub capabilities: u32,
    /// Reserved padding.
    pub reserved: [u8; 6],
}

/// EFI boot parameter handover information for Linux kernel.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EfiInfo {
    /// Loader signature (e.g. `"EL64"` or `"EL32"`).
    pub efi_loader_signature: u32,
    /// Lower 32 bits of UEFI system table address.
    pub efi_systab: u32,
    /// Memory descriptor size in bytes.
    pub efi_memdesc_size: u32,
    /// Memory descriptor version.
    pub efi_memdesc_version: u32,
    /// Lower 32 bits of memory map physical address.
    pub efi_memmap: u32,
    /// Memory map size in bytes.
    pub efi_memmap_size: u32,
    /// Upper 32 bits of UEFI system table address.
    pub efi_systab_hi: u32,
    /// Upper 32 bits of memory map physical address.
    pub efi_memmap_hi: u32,
}

impl EfiInfo {
    /// Create EFI info with 64-bit addresses.
    #[must_use]
    pub const fn new_64(
        systab_addr: u64,
        memmap_addr: u64,
        memmap_size: u32,
        desc_size: u32,
        desc_ver: u32,
    ) -> Self {
        Self {
            efi_loader_signature: u32::from_le_bytes(*b"EL64"),
            efi_systab: systab_addr as u32,
            efi_memdesc_size: desc_size,
            efi_memdesc_version: desc_ver,
            efi_memmap: memmap_addr as u32,
            efi_memmap_size: memmap_size,
            efi_systab_hi: (systab_addr >> 32) as u32,
            efi_memmap_hi: (memmap_addr >> 32) as u32,
        }
    }
}

/// Maximum number of E820 memory map entries in standard boot params.
pub const E820_MAX_ENTRIES: usize = 128;

/// Zero-page boot parameters block passed to Linux x86 kernel entry.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct BootParams {
    /// Screen information.
    pub screen_info: ScreenInfo,
    /// APM BIOS information.
    pub apm_bios_info: [u8; 0x14],
    /// Padding.
    pub pad2: [u8; 4],
    /// TBOOT shared memory address.
    pub tboot_addr: u64,
    /// Intel `SpeedStep` IST information.
    pub ist_info: [u8; 0x10],
    /// Padding.
    pub pad3: [u8; 16],
    /// Hard disk 0 geometry info.
    pub hd0_info: [u8; 16],
    /// Hard disk 1 geometry info.
    pub hd1_info: [u8; 16],
    /// MCA system description table.
    pub sys_desc_table: [u8; 0x10],
    /// OLPC Open Firmware header.
    pub olpc_ofw_header: [u8; 0x10],
    /// Reserved space.
    pub pad4: [u8; 128],
    /// EDID monitor capability info.
    pub edid_info: [u8; 0x80],
    /// EFI system table and memory map handover info.
    pub efi_info: EfiInfo,
    /// Alternative memory size in KB.
    pub alt_mem_k: u32,
    /// Scratch space.
    pub scratch: u32,
    /// Number of valid E820 entries.
    pub e820_entries: u8,
    /// EDD buffer entries count.
    pub eddbuf_entries: u8,
    /// EDD MBR signature buffer entries count.
    pub edd_mbr_sig_buf_entries: u8,
    /// Reserved padding.
    pub pad6: [u8; 6],
    /// Kernel setup header.
    pub hdr: SetupHeader,
    /// Padding between setup header and EDD buffer.
    pub pad7: [u8; 0x290 - 0x1F1 - size_of::<SetupHeader>()],
    /// EDD MBR signature buffer.
    pub edd_mbr_sig_buffer: [u32; 16],
    /// E820 memory map array.
    pub e820_map: [E820Entry; E820_MAX_ENTRIES],
    /// Padding.
    pub pad8: [u8; 48],
    /// EDD buffer.
    pub eddbuf: [u8; 0x1EC],
    /// Trailing padding to fill the 4096-byte page.
    pub pad9: [u8; 276],
}

/// Parses and validates a Linux `SetupHeader` from a raw kernel image buffer.
///
/// Returns a reference to the header if valid, or `None` if the buffer is too small
/// or does not contain the Linux boot signatures.
#[must_use]
pub fn parse_setup_header(buffer: &[u8]) -> Option<&SetupHeader> {
    if buffer.len() < SETUP_HEADER_OFFSET + size_of::<SetupHeader>() {
        return None;
    }

    let ptr = unsafe { buffer.as_ptr().add(SETUP_HEADER_OFFSET) };
    #[allow(clippy::cast_ptr_alignment)]
    let header = unsafe { &*(ptr.cast::<SetupHeader>()) };

    if header.is_valid() {
        Some(header)
    } else {
        None
    }
}
