//! # Intel Firmware Support Package (FSP) 2.x
//!
//! Provides headers, signatures, UPD structures, reset status codes, and parsing interfaces
//! described in the Intel Firmware Support Package External Architecture Specification v2.0 - v2.4.

use crate::base::{EfiPhysicalAddress, EfiStatus};
use core::ffi::c_void;

/// Fixed offset of the FSP Info Header in an FSP binary image.
pub const FSP_INFO_HEADER_OFFSET: usize = 0x94;

/// FSP Info Header signature: `'FSPH'` (`0x4850_5346`).
pub const FSP_INFO_HEADER_SIGNATURE: u32 = 0x4850_5346;

/// Image attribute: Graphics support.
pub const IMAGE_ATTRIBUTE_GRAPHICS_SUPPORT: u16 = 1 << 0;
/// Image attribute: Dispatch mode support.
pub const IMAGE_ATTRIBUTE_DISPATCH_MODE_SUPPORT: u16 = 1 << 1;
/// Image attribute: 64-bit mode support.
pub const IMAGE_ATTRIBUTE_64BIT_MODE_SUPPORT: u16 = 1 << 2;
/// Image attribute: Variable services support.
pub const IMAGE_ATTRIBUTE_VARIABLE_SERVICES_SUPPORT: u16 = 1 << 3;

/// FSP execution mode: IA32.
pub const FSP_IA32: u8 = 0;
/// FSP execution mode: X64.
pub const FSP_X64: u8 = 1;

/// FSP Component Type: FSP-T (Temporary RAM init).
pub const FSP_COMPONENT_T: u16 = 0x0001;
/// FSP Component Type: FSP-M (Memory init).
pub const FSP_COMPONENT_M: u16 = 0x0002;
/// FSP Component Type: FSP-S (Silicon init).
pub const FSP_COMPONENT_S: u16 = 0x0003;
/// FSP Component Type: FSP-I (SMM init).
pub const FSP_COMPONENT_I: u16 = 0x0004;
/// FSP Component Type: FSP-O (OEM post-memory init).
pub const FSP_COMPONENT_O: u16 = 0x0008;

/// FSP Reset Request Status Code: Cold Reset.
pub const FSP_STATUS_RESET_REQUIRED_COLD: usize = 1;
/// FSP Reset Request Status Code: Warm Reset.
pub const FSP_STATUS_RESET_REQUIRED_WARM: usize = 2;
/// FSP Reset Request Status Code: Type 3 Reset.
pub const FSP_STATUS_RESET_REQUIRED_3: usize = 3;
/// FSP Reset Request Status Code: Type 4 Reset.
pub const FSP_STATUS_RESET_REQUIRED_4: usize = 4;
/// FSP Reset Request Status Code: Type 5 Reset.
pub const FSP_STATUS_RESET_REQUIRED_5: usize = 5;
/// FSP Reset Request Status Code: Type 6 Reset.
pub const FSP_STATUS_RESET_REQUIRED_6: usize = 6;
/// FSP Reset Request Status Code: Type 7 Reset.
pub const FSP_STATUS_RESET_REQUIRED_7: usize = 7;
/// FSP Reset Request Status Code: Type 8 Reset.
pub const FSP_STATUS_RESET_REQUIRED_8: usize = 8;
/// FSP Variable Request Status Code.
pub const FSP_STATUS_VARIABLE_REQUEST: usize = 10;

/// Encodes an FSP reset request as an OEM `EfiStatus`.
#[inline]
#[must_use]
pub const fn encode_fsp_reset_request(reset_type: usize) -> EfiStatus {
    EfiStatus::from_raw((1 << (core::mem::size_of::<usize>() * 8 - 1)) | reset_type)
}

/// FSP Information Header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FspInfoHeader {
    /// Signature ('FSPH').
    pub signature: u32,
    /// Length of the FSP Information Header.
    pub header_length: u32,
    /// Reserved.
    pub reserved1: [u8; 2],
    /// Specification version in BCD format (e.g. 0x24 for v2.4).
    pub spec_version: u8,
    /// Revision of the FSP Information Header.
    pub header_revision: u8,
    /// Revision of the FSP binary.
    pub image_revision: u32,
    /// Image identifier string.
    pub image_id: [u8; 8],
    /// Size of the FSP binary image in bytes.
    pub image_size: u32,
    /// Base address of the FSP binary image.
    pub image_base: u32,
    /// Image attribute flags.
    pub image_attribute: u16,
    /// Component attribute flags.
    pub component_attribute: u16,
    /// Offset to configuration region (UPD).
    pub cfg_region_offset: u32,
    /// Size of configuration region.
    pub cfg_region_size: u32,
    /// Reserved.
    pub reserved2: [u8; 4],
    /// Entry offset for `TempRamInit` API.
    pub temp_ram_init_entry_offset: u32,
    /// Reserved.
    pub reserved3: [u8; 4],
    /// Entry offset for `FspNotifyPhase` API.
    pub notify_phase_entry_offset: u32,
    /// Entry offset for `FspMemoryInit` API.
    pub fsp_memory_init_entry_offset: u32,
    /// Entry offset for `TempRamExit` API.
    pub temp_ram_exit_entry_offset: u32,
    /// Entry offset for `FspSiliconInit` API.
    pub fsp_silicon_init_entry_offset: u32,
    /// Entry offset for `FspMultiPhaseSiInit` API.
    pub fsp_multi_phase_si_init_entry_offset: u32,
}

/// FSP notification phase.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FspPhase {
    /// After Temporary RAM init.
    AfterTempRamInit = 0x10,
    /// After Memory init.
    AfterMemoryInit = 0x20,
    /// After Silicon init.
    AfterSiliconInit = 0x30,
    /// Ready to boot.
    ReadyToBoot = 0x40,
    /// End of firmware execution.
    EndOfFirmware = 0xF0,
}

/// Notification parameter structure passed to `FspNotifyPhase`.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FspNotifyParams {
    /// Notification phase.
    pub phase: u32,
}

/// FSP UPD Header Configuration common to all UPD structures.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FspUpdHeader {
    /// UPD Region Signature (e.g. `XXXXXX_T`, `XXXXXX_M`, `XXXXXX_S`, `XXXXXX_I`).
    pub signature: u64,
    /// Revision of the data structure.
    pub revision: u8,
    /// Reserved.
    pub reserved: [u8; 23],
}

/// FSP-T Architecture UPD Configuration.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FsptArchUpd {
    /// Revision of the structure (1).
    pub revision: u8,
    /// Reserved.
    pub reserved: [u8; 3],
    /// Length of the structure in bytes (32).
    pub length: u32,
    /// Optional debug handler callback address.
    pub fsp_debug_handler: EfiPhysicalAddress,
    /// Reserved.
    pub reserved1: [u8; 16],
}

/// FSP-M Architecture UPD Configuration.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FspmArchUpd {
    /// Revision of the structure.
    pub revision: u8,
    /// Reserved.
    pub reserved: [u8; 3],
    /// Pointer to the non-volatile storage (NVS) data buffer.
    pub nvs_buffer_ptr: EfiPhysicalAddress,
    /// Temporary stack base address.
    pub stack_base: EfiPhysicalAddress,
    /// Temporary stack size.
    pub stack_size: u64,
    /// Size of memory below top of low usable memory for bootloader.
    pub boot_loader_tolum_size: u32,
    /// Current boot mode.
    pub boot_mode: u32,
    /// Optional event handler callback.
    pub fsp_event_handler: EfiPhysicalAddress,
    /// Reserved.
    pub reserved1: [u8; 16],
}

/// FSP-S Architecture UPD Configuration.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FspsArchUpd {
    /// Revision of the structure (1 or 2).
    pub revision: u8,
    /// Reserved.
    pub reserved: [u8; 3],
    /// Length of the structure in bytes (32).
    pub length: u32,
    /// Optional event handler callback.
    pub fsp_event_handler: EfiPhysicalAddress,
    /// Multi-phase silicon initialization enablement flag.
    pub enable_multi_phase_silicon_init: u8,
    /// Reserved.
    pub reserved1: [u8; 15],
}

/// FSP-I Architecture UPD Configuration.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct FspiArchUpd {
    /// Revision of the structure (1).
    pub revision: u8,
    /// Reserved.
    pub reserved: [u8; 3],
    /// Length of the structure in bytes (64).
    pub length: u32,
    /// Bootloader SMM Firmware Volume base address.
    pub bootloader_smm_fv_base_address: EfiPhysicalAddress,
    /// Bootloader SMM Firmware Volume length in bytes.
    pub bootloader_smm_fv_length: u64,
    /// Bootloader SMM FV context data base address.
    pub bootloader_smm_fv_context_data: EfiPhysicalAddress,
    /// Bootloader SMM FV context data length in bytes.
    pub bootloader_smm_fv_context_data_length: u16,
    /// Reserved.
    pub reserved1: [u8; 30],
}

/// `TempRamInit` entry point function prototype.
pub type FspTempRamInit = unsafe extern "efiapi" fn(temp_ram_init_params: *const ()) -> EfiStatus;

/// `FspMemoryInit` entry point function prototype.
pub type FspMemoryInit = unsafe extern "efiapi" fn(
    fsp_memory_init_params: *const (),
    hob_list_ptr: *mut *const c_void,
) -> EfiStatus;

/// `TempRamExit` entry point function prototype.
pub type FspTempRamExit = unsafe extern "efiapi" fn(temp_ram_exit_params: *const ()) -> EfiStatus;

/// `FspSiliconInit` entry point function prototype.
pub type FspSiliconInit =
    unsafe extern "efiapi" fn(fsp_silicon_init_params: *const ()) -> EfiStatus;

/// `FspNotifyPhase` entry point function prototype.
pub type FspNotifyPhase =
    unsafe extern "efiapi" fn(notify_params: *const FspNotifyParams) -> EfiStatus;

/// Error conditions encountered during FSP binary parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FspError {
    /// Buffer is smaller than minimum FSP info header offset and size.
    BufferTooSmall,
    /// Signature does not match `'FSPH'`.
    InvalidSignature,
    /// Header length is invalid.
    InvalidHeaderLength,
    /// Image size exceeds binary buffer length.
    ImageSizeOutOfBounds,
}

/// Parsed FSP Binary Image view.
#[derive(Clone, Copy, Debug)]
pub struct FspImage<'a> {
    data: &'a [u8],
    header: FspInfoHeader,
}

impl<'a> FspImage<'a> {
    /// Parses an FSP binary image from a byte slice.
    ///
    /// # Errors
    /// Returns [`FspError`] if buffer is too small or signature is invalid.
    pub fn parse(data: &'a [u8]) -> Result<Self, FspError> {
        let min_len = FSP_INFO_HEADER_OFFSET + core::mem::size_of::<FspInfoHeader>();
        if data.len() < min_len {
            return Err(FspError::BufferTooSmall);
        }

        let slice = &data[FSP_INFO_HEADER_OFFSET
            ..FSP_INFO_HEADER_OFFSET + core::mem::size_of::<FspInfoHeader>()];

        let mut sig_raw = [0_u8; 4];
        sig_raw.copy_from_slice(&slice[0..4]);
        let signature = u32::from_le_bytes(sig_raw);
        if signature != FSP_INFO_HEADER_SIGNATURE {
            return Err(FspError::InvalidSignature);
        }

        let mut hdr_len_raw = [0_u8; 4];
        hdr_len_raw.copy_from_slice(&slice[4..8]);
        let header_length = u32::from_le_bytes(hdr_len_raw);
        if (header_length as usize) < core::mem::size_of::<FspInfoHeader>() {
            return Err(FspError::InvalidHeaderLength);
        }

        let reserved1 = [slice[8], slice[9]];
        let spec_version = slice[10];
        let header_revision = slice[11];

        let mut img_rev_raw = [0_u8; 4];
        img_rev_raw.copy_from_slice(&slice[12..16]);
        let image_revision = u32::from_le_bytes(img_rev_raw);

        let mut image_id = [0_u8; 8];
        image_id.copy_from_slice(&slice[16..24]);

        let mut img_sz_raw = [0_u8; 4];
        img_sz_raw.copy_from_slice(&slice[24..28]);
        let image_size = u32::from_le_bytes(img_sz_raw);

        let mut img_base_raw = [0_u8; 4];
        img_base_raw.copy_from_slice(&slice[28..32]);
        let image_base = u32::from_le_bytes(img_base_raw);

        let mut attr_raw = [0_u8; 2];
        attr_raw.copy_from_slice(&slice[32..34]);
        let image_attribute = u16::from_le_bytes(attr_raw);

        let mut comp_raw = [0_u8; 2];
        comp_raw.copy_from_slice(&slice[34..36]);
        let component_attribute = u16::from_le_bytes(comp_raw);

        let mut cfg_off_raw = [0_u8; 4];
        cfg_off_raw.copy_from_slice(&slice[36..40]);
        let cfg_region_offset = u32::from_le_bytes(cfg_off_raw);

        let mut cfg_sz_raw = [0_u8; 4];
        cfg_sz_raw.copy_from_slice(&slice[40..44]);
        let cfg_region_size = u32::from_le_bytes(cfg_sz_raw);

        let mut reserved2 = [0_u8; 4];
        reserved2.copy_from_slice(&slice[44..48]);

        let mut temp_ram_init_raw = [0_u8; 4];
        temp_ram_init_raw.copy_from_slice(&slice[48..52]);
        let temp_ram_init_entry_offset = u32::from_le_bytes(temp_ram_init_raw);

        let mut reserved3 = [0_u8; 4];
        reserved3.copy_from_slice(&slice[52..56]);

        let mut notify_phase_raw = [0_u8; 4];
        notify_phase_raw.copy_from_slice(&slice[56..60]);
        let notify_phase_entry_offset = u32::from_le_bytes(notify_phase_raw);

        let mut fsp_mem_raw = [0_u8; 4];
        fsp_mem_raw.copy_from_slice(&slice[60..64]);
        let fsp_memory_init_entry_offset = u32::from_le_bytes(fsp_mem_raw);

        let mut temp_ram_exit_raw = [0_u8; 4];
        temp_ram_exit_raw.copy_from_slice(&slice[64..68]);
        let temp_ram_exit_entry_offset = u32::from_le_bytes(temp_ram_exit_raw);

        let mut fsp_si_raw = [0_u8; 4];
        fsp_si_raw.copy_from_slice(&slice[68..72]);
        let fsp_silicon_init_entry_offset = u32::from_le_bytes(fsp_si_raw);

        let mut fsp_multi_raw = [0_u8; 4];
        fsp_multi_raw.copy_from_slice(&slice[72..76]);
        let fsp_multi_phase_si_init_entry_offset = u32::from_le_bytes(fsp_multi_raw);

        let header = FspInfoHeader {
            signature,
            header_length,
            reserved1,
            spec_version,
            header_revision,
            image_revision,
            image_id,
            image_size,
            image_base,
            image_attribute,
            component_attribute,
            cfg_region_offset,
            cfg_region_size,
            reserved2,
            temp_ram_init_entry_offset,
            reserved3,
            notify_phase_entry_offset,
            fsp_memory_init_entry_offset,
            temp_ram_exit_entry_offset,
            fsp_silicon_init_entry_offset,
            fsp_multi_phase_si_init_entry_offset,
        };

        Ok(Self { data, header })
    }

    /// Returns the parsed FSP Information Header.
    #[must_use]
    pub const fn header(&self) -> &FspInfoHeader {
        &self.header
    }

    /// Returns the signature by value.
    #[must_use]
    pub fn signature(&self) -> u32 {
        self.header.signature
    }

    /// Returns the specification version in BCD format.
    #[must_use]
    pub fn spec_version(&self) -> u8 {
        self.header.spec_version
    }

    /// Returns the header revision.
    #[must_use]
    pub fn header_revision(&self) -> u8 {
        self.header.header_revision
    }

    /// Returns the image revision.
    #[must_use]
    pub fn image_revision(&self) -> u32 {
        self.header.image_revision
    }

    /// Returns the image ID.
    #[must_use]
    pub fn image_id(&self) -> [u8; 8] {
        self.header.image_id
    }

    /// Returns the total image size.
    #[must_use]
    pub fn image_size(&self) -> u32 {
        self.header.image_size
    }

    /// Returns the base address.
    #[must_use]
    pub fn image_base(&self) -> u32 {
        self.header.image_base
    }

    /// Returns the component type mask.
    #[must_use]
    pub fn component_type(&self) -> u16 {
        (self.header.component_attribute >> 12) & 0x0F
    }

    /// Returns whether the FSP image supports graphics display output.
    #[must_use]
    pub fn has_graphics_support(&self) -> bool {
        (self.header.image_attribute & IMAGE_ATTRIBUTE_GRAPHICS_SUPPORT) != 0
    }

    /// Returns whether the FSP image supports Dispatch Mode execution.
    #[must_use]
    pub fn has_dispatch_mode(&self) -> bool {
        (self.header.image_attribute & IMAGE_ATTRIBUTE_DISPATCH_MODE_SUPPORT) != 0
    }

    /// Returns whether the FSP image is 64-bit native mode.
    #[must_use]
    pub fn is_64bit_mode(&self) -> bool {
        (self.header.image_attribute & IMAGE_ATTRIBUTE_64BIT_MODE_SUPPORT) != 0
    }

    /// Returns the underlying raw slice.
    #[must_use]
    pub const fn data(&self) -> &'a [u8] {
        self.data
    }
}
