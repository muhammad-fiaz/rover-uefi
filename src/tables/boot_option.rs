//! # UEFI Boot Options and Load Options
//!
//! Provides data structures, attributes, and zero-copy parsers for UEFI load options
//! (`Boot####`, `Driver####`, `SysPrep####`, `PlatformRecovery####`), key options
//! (`Key####`), boot order variables (`BootOrder`, `DriverOrder`, `SysPrepOrder`),
//! and standard firmware boot paths.

use crate::base::EfiStatus;
use crate::protocols::console::EfiInputKey;

/// Standard UEFI variable name for the ordered boot option list: `"BootOrder"`.
pub const EFI_BOOT_ORDER_VARIABLE_NAME: &str = "BootOrder";

/// Standard UEFI variable name for the one-time next boot option: `"BootNext"`.
pub const EFI_BOOT_NEXT_VARIABLE_NAME: &str = "BootNext";

/// Standard UEFI variable name for the currently booted option: `"BootCurrent"`.
pub const EFI_BOOT_CURRENT_VARIABLE_NAME: &str = "BootCurrent";

/// Standard UEFI variable name for boot option capabilities: `"BootOptionSupport"`.
pub const EFI_BOOT_OPTION_SUPPORT_VARIABLE_NAME: &str = "BootOptionSupport";

/// Standard UEFI variable name for ordered driver options: `"DriverOrder"`.
pub const EFI_DRIVER_ORDER_VARIABLE_NAME: &str = "DriverOrder";

/// Standard UEFI variable name for ordered system prep options: `"SysPrepOrder"`.
pub const EFI_SYS_PREP_ORDER_VARIABLE_NAME: &str = "SysPrepOrder";

/// Standard UEFI variable name for ordered platform recovery options: `"PlatformRecoveryOrder"`.
pub const EFI_PLATFORM_RECOVERY_ORDER_VARIABLE_NAME: &str = "PlatformRecoveryOrder";

/// Standard UEFI variable name for boot manager timeout in seconds: `"Timeout"`.
pub const EFI_TIMEOUT_VARIABLE_NAME: &str = "Timeout";

/// Standard UEFI variable name for default console input device path: `"ConIn"`.
pub const EFI_CON_IN_VARIABLE_NAME: &str = "ConIn";

/// Standard UEFI variable name for default console output device path: `"ConOut"`.
pub const EFI_CON_OUT_VARIABLE_NAME: &str = "ConOut";

/// Standard UEFI variable name for default error output device path: `"ErrOut"`.
pub const EFI_ERR_OUT_VARIABLE_NAME: &str = "ErrOut";

/// Standard UEFI variable name for candidate console input devices: `"ConInDev"`.
pub const EFI_CON_IN_DEV_VARIABLE_NAME: &str = "ConInDev";

/// Standard UEFI variable name for candidate console output devices: `"ConOutDev"`.
pub const EFI_CON_OUT_DEV_VARIABLE_NAME: &str = "ConOutDev";

/// Standard UEFI variable name for candidate console error devices: `"ErrOutDev"`.
pub const EFI_ERR_OUT_DEV_VARIABLE_NAME: &str = "ErrOutDev";

/// Deprecated standard variable name for supported ISO 639-2 language codes: `"LangCodes"`.
pub const EFI_LANG_CODES_VARIABLE_NAME: &str = "LangCodes";

/// Deprecated standard variable name for current ISO 639-2 language code: `"Lang"`.
pub const EFI_LANG_VARIABLE_NAME: &str = "Lang";

/// Standard UEFI variable name for supported RFC 4646 language codes: `"PlatformLangCodes"`.
pub const EFI_PLATFORM_LANG_CODES_VARIABLE_NAME: &str = "PlatformLangCodes";

/// Standard UEFI variable name for default RFC 4646 language code: `"PlatformLang"`.
pub const EFI_PLATFORM_LANG_VARIABLE_NAME: &str = "PlatformLang";

/// Removable media boot file path for `x86_64` systems.
pub const EFI_REMOVABLE_MEDIA_FILE_NAME_X64: &str = "\\EFI\\BOOT\\BOOTX64.EFI";

/// Removable media boot file path for `IA32` (i686) systems.
pub const EFI_REMOVABLE_MEDIA_FILE_NAME_IA32: &str = "\\EFI\\BOOT\\BOOTIA32.EFI";

/// Removable media boot file path for `AArch64` systems.
pub const EFI_REMOVABLE_MEDIA_FILE_NAME_AARCH64: &str = "\\EFI\\BOOT\\BOOTAA64.EFI";

/// Removable media boot file path for `RISC-V` 64-bit systems.
pub const EFI_REMOVABLE_MEDIA_FILE_NAME_RISCV64: &str = "\\EFI\\BOOT\\BOOTRISCV64.EFI";

/// Removable media boot file path for `LoongArch` 64-bit systems.
pub const EFI_REMOVABLE_MEDIA_FILE_NAME_LOONGARCH64: &str = "\\EFI\\BOOT\\BOOTLOONGARCH64.EFI";

/// Capsule payload delivery directory on an EFI System Partition.
pub const EFI_CAPSULE_FILE_DIRECTORY: &str = "\\EFI\\UpdateCapsule\\";

/// Maximum valid option number for load option variables (`0x10000`).
pub const LOAD_OPTION_NUMBER_MAX: u32 = 0x0001_0000;

/// Unassigned sentinel option number for dynamic option assignment (`0x10000`).
pub const LOAD_OPTION_NUMBER_UNASSIGNED: u32 = 0x0001_0000;

/// Boot Option Support: hot key boot options are supported.
pub const EFI_BOOT_OPTION_SUPPORT_KEY: u32 = 0x0000_0001;

/// Boot Option Support: interactive application boot options are supported.
pub const EFI_BOOT_OPTION_SUPPORT_APP: u32 = 0x0000_0002;

/// Boot Option Support: system preparation boot options are supported.
pub const EFI_BOOT_OPTION_SUPPORT_SYSPREP: u32 = 0x0000_0010;

/// Boot Option Support: maximum number of simultaneous keys supported.
pub const EFI_BOOT_OPTION_SUPPORT_COUNT: u32 = 0x0000_0300;

/// Load option is active and will be considered during boot dispatch.
pub const LOAD_OPTION_ACTIVE: u32 = 0x0000_0001;

/// Driver load option requires reconnecting all controllers after dispatch.
pub const LOAD_OPTION_FORCE_RECONNECT: u32 = 0x0000_0002;

/// Option is hidden from interactive firmware selection menus.
pub const LOAD_OPTION_HIDDEN: u32 = 0x0000_0008;

/// Mask isolating category bits in load option attributes.
pub const LOAD_OPTION_CATEGORY_MASK: u32 = 0x0000_1F00;

/// Standard operating system boot option category.
pub const LOAD_OPTION_CATEGORY_BOOT: u32 = 0x0000_0000;

/// Interactive firmware application / shell category.
pub const LOAD_OPTION_CATEGORY_APP: u32 = 0x0000_0100;

/// Hot key modifier flag: Shift key must be pressed.
pub const BOOT_MANAGER_SHIFT_PRESSED: u32 = 0x0000_0001;

/// Hot key modifier flag: Control key must be pressed.
pub const BOOT_MANAGER_CONTROL_PRESSED: u32 = 0x0000_0002;

/// Hot key modifier flag: Alt key must be pressed.
pub const BOOT_MANAGER_ALT_PRESSED: u32 = 0x0000_0004;

/// Hot key modifier flag: Logo (Windows/Command) key must be pressed.
pub const BOOT_MANAGER_LOGO_PRESSED: u32 = 0x0000_0008;

/// Hot key modifier flag: Menu key must be pressed.
pub const BOOT_MANAGER_MENU_KEY_PRESSED: u32 = 0x0000_0010;

/// Hot key modifier flag: `SysReq` key must be pressed.
pub const BOOT_MANAGER_SYS_REQ_PRESSED: u32 = 0x0000_0020;

/// Classification of boot manager load options.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LoadOptionType {
    /// UEFI driver load option (`Driver####`).
    Driver = 0,
    /// System preparation load option (`SysPrep####`).
    SysPrep = 1,
    /// Standard boot option (`Boot####`).
    Boot = 2,
    /// Platform recovery load option (`PlatformRecovery####`).
    PlatformRecovery = 3,
}

impl LoadOptionType {
    /// Returns the prefix string for this load option type (e.g. `"Boot"`).
    #[must_use]
    pub const fn prefix(self) -> &'static str {
        match self {
            Self::Driver => "Driver",
            Self::SysPrep => "SysPrep",
            Self::Boot => "Boot",
            Self::PlatformRecovery => "PlatformRecovery",
        }
    }

    /// Returns the corresponding order variable name, if any.
    #[must_use]
    pub const fn order_variable_name(self) -> Option<&'static str> {
        match self {
            Self::Driver => Some(EFI_DRIVER_ORDER_VARIABLE_NAME),
            Self::SysPrep => Some(EFI_SYS_PREP_ORDER_VARIABLE_NAME),
            Self::Boot => Some(EFI_BOOT_ORDER_VARIABLE_NAME),
            Self::PlatformRecovery => None,
        }
    }
}

/// Standard console types managed by the Boot Manager.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ConsoleType {
    /// Standard console input.
    ConIn = 0,
    /// Standard console output.
    ConOut = 1,
    /// Standard console error output.
    ErrOut = 2,
    /// Candidate console input devices.
    ConInDev = 3,
    /// Candidate console output devices.
    ConOutDev = 4,
    /// Candidate console error devices.
    ErrOutDev = 5,
}

impl ConsoleType {
    /// Returns the UEFI variable name for this console type.
    #[must_use]
    pub const fn variable_name(self) -> &'static str {
        match self {
            Self::ConIn => EFI_CON_IN_VARIABLE_NAME,
            Self::ConOut => EFI_CON_OUT_VARIABLE_NAME,
            Self::ErrOut => EFI_ERR_OUT_VARIABLE_NAME,
            Self::ConInDev => EFI_CON_IN_DEV_VARIABLE_NAME,
            Self::ConOutDev => EFI_CON_OUT_DEV_VARIABLE_NAME,
            Self::ErrOutDev => EFI_ERR_OUT_DEV_VARIABLE_NAME,
        }
    }
}

/// Fixed-length header of a serialized `EFI_LOAD_OPTION`.
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct EfiLoadOptionHeader {
    /// Option attributes bitmask (`LOAD_OPTION_ACTIVE`, `LOAD_OPTION_HIDDEN`, etc.).
    pub attributes: u32,
    /// Length in bytes of the following device path list.
    pub file_path_list_length: u16,
}

/// Errors encountered when parsing or serializing UEFI load options and order variables.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadOptionError {
    /// Buffer is too small to contain the required structure.
    BufferTooSmall,
    /// Null terminator missing in UTF-16 description string.
    UnterminatedDescription,
    /// Invalid UTF-16 character or alignment.
    InvalidDescription,
    /// Device path length exceeds available buffer data or is zero.
    InvalidDevicePathLength,
    /// Destination buffer is insufficient to store serialized bytes.
    DestinationBufferTooSmall,
    /// Option index is already present in the order list.
    AlreadyExists,
    /// Option index was not found in the order list.
    NotFound,
    /// Option index exceeds valid 16-bit range (`0xFFFF`).
    OptionNumberOutOfRange,
    /// No free option numbers are available in range `[0x0000, 0xFFFF]`.
    NoFreeOptionNumber,
}

impl From<LoadOptionError> for EfiStatus {
    fn from(err: LoadOptionError) -> Self {
        match err {
            LoadOptionError::BufferTooSmall
            | LoadOptionError::DestinationBufferTooSmall
            | LoadOptionError::InvalidDevicePathLength => Self::BUFFER_TOO_SMALL,
            LoadOptionError::UnterminatedDescription
            | LoadOptionError::InvalidDescription
            | LoadOptionError::OptionNumberOutOfRange => Self::INVALID_PARAMETER,
            LoadOptionError::AlreadyExists => Self::ALREADY_STARTED,
            LoadOptionError::NotFound => Self::NOT_FOUND,
            LoadOptionError::NoFreeOptionNumber => Self::OUT_OF_RESOURCES,
        }
    }
}

/// Validates whether a raw byte buffer represents a valid serialized load option.
#[must_use]
pub fn validate_load_option(bytes: &[u8]) -> bool {
    let header_size = core::mem::size_of::<EfiLoadOptionHeader>();
    if bytes.len() <= header_size {
        return false;
    }

    let len_bytes: [u8; 2] = match bytes[4..6].try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };
    let file_path_list_length = u16::from_le_bytes(len_bytes) as usize;
    if file_path_list_length == 0 {
        return false;
    }

    let desc_bytes = &bytes[6..];
    let max_pairs = desc_bytes.len() / 2;
    let mut found_null = false;
    let mut desc_u16_len = 0;

    for i in 0..max_pairs {
        let u16_val = u16::from_le_bytes([desc_bytes[2 * i], desc_bytes[2 * i + 1]]);
        if u16_val == 0 {
            desc_u16_len = i;
            found_null = true;
            break;
        }
    }

    if !found_null || desc_u16_len == 0 {
        return false;
    }

    let desc_total_bytes = (desc_u16_len + 1) * 2;
    let total_required = 6 + desc_total_bytes + file_path_list_length;
    bytes.len() >= total_required
}

/// Zero-copy parsed view over a serialized UEFI load option variable (`Boot####`, etc.).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadOptionView<'a> {
    /// Load option attributes.
    pub attributes: u32,
    /// Null-terminated UTF-16 description string without trailing null.
    pub description: &'a [u16],
    /// Raw packed device path array.
    pub file_path_list: &'a [u8],
    /// Optional binary data passed to the loaded image.
    pub optional_data: &'a [u8],
}

impl<'a> LoadOptionView<'a> {
    /// Parses a serialized UEFI load option from raw byte slice.
    ///
    /// # Errors
    ///
    /// Returns [`LoadOptionError`] if the buffer is truncated or malformed.
    #[allow(clippy::cast_ptr_alignment)]
    pub fn parse(bytes: &'a [u8]) -> Result<Self, LoadOptionError> {
        let header_size = core::mem::size_of::<EfiLoadOptionHeader>();
        if bytes.len() < header_size {
            return Err(LoadOptionError::BufferTooSmall);
        }

        let attr_bytes: [u8; 4] = bytes[0..4]
            .try_into()
            .map_err(|_| LoadOptionError::BufferTooSmall)?;
        let len_bytes: [u8; 2] = bytes[4..6]
            .try_into()
            .map_err(|_| LoadOptionError::BufferTooSmall)?;

        let attributes = u32::from_le_bytes(attr_bytes);
        let file_path_list_length = u16::from_le_bytes(len_bytes) as usize;

        // Description starts at offset 6 as UTF-16 until null u16 (0x0000).
        let desc_bytes = &bytes[6..];
        let mut desc_u16_len = 0;
        let mut found_null = false;

        let max_pairs = desc_bytes.len() / 2;
        for i in 0..max_pairs {
            let u16_val = u16::from_le_bytes([desc_bytes[2 * i], desc_bytes[2 * i + 1]]);
            if u16_val == 0 {
                desc_u16_len = i;
                found_null = true;
                break;
            }
        }

        if !found_null {
            return Err(LoadOptionError::UnterminatedDescription);
        }

        let desc_byte_len = desc_u16_len * 2;
        let desc_slice_bytes = &desc_bytes[..desc_byte_len];
        let desc_ptr = desc_slice_bytes.as_ptr();
        if desc_ptr as usize % core::mem::align_of::<u16>() != 0 {
            return Err(LoadOptionError::InvalidDescription);
        }

        let description =
            unsafe { core::slice::from_raw_parts(desc_ptr.cast::<u16>(), desc_u16_len) };

        // After null terminator (2 bytes)
        let dp_offset = 6 + desc_byte_len + 2;
        if bytes.len() < dp_offset + file_path_list_length {
            return Err(LoadOptionError::InvalidDevicePathLength);
        }

        let file_path_list = &bytes[dp_offset..dp_offset + file_path_list_length];
        let optional_data = &bytes[dp_offset + file_path_list_length..];

        Ok(Self {
            attributes,
            description,
            file_path_list,
            optional_data,
        })
    }

    /// Returns `true` if the option is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        (self.attributes & LOAD_OPTION_ACTIVE) != 0
    }

    /// Returns `true` if the option requires reconnecting controllers after dispatch.
    #[must_use]
    pub const fn is_force_reconnect(&self) -> bool {
        (self.attributes & LOAD_OPTION_FORCE_RECONNECT) != 0
    }

    /// Returns `true` if the option is marked hidden.
    #[must_use]
    pub const fn is_hidden(&self) -> bool {
        (self.attributes & LOAD_OPTION_HIDDEN) != 0
    }

    /// Returns `true` if the option is an interactive application rather than OS boot target.
    #[must_use]
    pub const fn is_app(&self) -> bool {
        (self.attributes & LOAD_OPTION_CATEGORY_MASK) == LOAD_OPTION_CATEGORY_APP
    }

    /// Calculates total serialized byte size of this load option.
    #[must_use]
    pub fn serialized_size(&self) -> usize {
        core::mem::size_of::<EfiLoadOptionHeader>()
            + (self.description.len() + 1) * 2
            + self.file_path_list.len()
            + self.optional_data.len()
    }

    /// Serializes this load option into the provided destination buffer.
    ///
    /// # Errors
    ///
    /// Returns [`LoadOptionError::DestinationBufferTooSmall`] if `buffer` is too small.
    pub fn serialize(&self, buffer: &mut [u8]) -> Result<usize, LoadOptionError> {
        let needed = self.serialized_size();
        if buffer.len() < needed {
            return Err(LoadOptionError::DestinationBufferTooSmall);
        }

        let attr_bytes = self.attributes.to_le_bytes();
        let len_bytes = u16::try_from(self.file_path_list.len())
            .map_err(|_| LoadOptionError::DestinationBufferTooSmall)?
            .to_le_bytes();

        buffer[0..4].copy_from_slice(&attr_bytes);
        buffer[4..6].copy_from_slice(&len_bytes);

        let mut offset = 6;
        for &ch in self.description {
            buffer[offset..offset + 2].copy_from_slice(&ch.to_le_bytes());
            offset += 2;
        }
        buffer[offset..offset + 2].copy_from_slice(&0u16.to_le_bytes());
        offset += 2;

        buffer[offset..offset + self.file_path_list.len()].copy_from_slice(self.file_path_list);
        offset += self.file_path_list.len();

        buffer[offset..offset + self.optional_data.len()].copy_from_slice(self.optional_data);
        offset += self.optional_data.len();

        Ok(offset)
    }
}

/// Bitfield options for UEFI hot key boot descriptors.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EfiBootKeyData(pub u32);

impl EfiBootKeyData {
    /// Creates a new key data descriptor with specified modifiers.
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)]
    pub const fn new(
        shift: bool,
        control: bool,
        alt: bool,
        logo: bool,
        menu: bool,
        sysreq: bool,
        input_key_count: u8,
    ) -> Self {
        let mut val = 0u32;
        if shift {
            val |= 1 << 8;
        }
        if control {
            val |= 1 << 9;
        }
        if alt {
            val |= 1 << 10;
        }
        if logo {
            val |= 1 << 11;
        }
        if menu {
            val |= 1 << 12;
        }
        if sysreq {
            val |= 1 << 13;
        }
        val |= ((input_key_count & 0x03) as u32) << 30;
        Self(val)
    }

    /// Creates key data from modifier bitmask (`BOOT_MANAGER_SHIFT_PRESSED`, etc.).
    #[must_use]
    pub const fn from_modifiers(modifiers: u32, input_key_count: u8) -> Self {
        let mut val = 0u32;
        if (modifiers & BOOT_MANAGER_SHIFT_PRESSED) != 0 {
            val |= 1 << 8;
        }
        if (modifiers & BOOT_MANAGER_CONTROL_PRESSED) != 0 {
            val |= 1 << 9;
        }
        if (modifiers & BOOT_MANAGER_ALT_PRESSED) != 0 {
            val |= 1 << 10;
        }
        if (modifiers & BOOT_MANAGER_LOGO_PRESSED) != 0 {
            val |= 1 << 11;
        }
        if (modifiers & BOOT_MANAGER_MENU_KEY_PRESSED) != 0 {
            val |= 1 << 12;
        }
        if (modifiers & BOOT_MANAGER_SYS_REQ_PRESSED) != 0 {
            val |= 1 << 13;
        }
        val |= ((input_key_count & 0x03) as u32) << 30;
        Self(val)
    }

    /// Revision of the key option descriptor (typically 0).
    #[must_use]
    pub const fn revision(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// `true` if Shift modifier key must be pressed.
    #[must_use]
    pub const fn shift_pressed(self) -> bool {
        (self.0 & (1 << 8)) != 0
    }

    /// `true` if Control modifier key must be pressed.
    #[must_use]
    pub const fn control_pressed(self) -> bool {
        (self.0 & (1 << 9)) != 0
    }

    /// `true` if Alt modifier key must be pressed.
    #[must_use]
    pub const fn alt_pressed(self) -> bool {
        (self.0 & (1 << 10)) != 0
    }

    /// `true` if Logo (Windows/Command) modifier key must be pressed.
    #[must_use]
    pub const fn logo_pressed(self) -> bool {
        (self.0 & (1 << 11)) != 0
    }

    /// `true` if Menu modifier key must be pressed.
    #[must_use]
    pub const fn menu_pressed(self) -> bool {
        (self.0 & (1 << 12)) != 0
    }

    /// `true` if `SysReq` modifier key must be pressed.
    #[must_use]
    pub const fn sysreq_pressed(self) -> bool {
        (self.0 & (1 << 13)) != 0
    }

    /// Number of distinct key codes required (0 to 3).
    #[must_use]
    pub const fn input_key_count(self) -> u8 {
        ((self.0 >> 30) & 0x03) as u8
    }
}

/// Fixed header of a UEFI key option variable (`Key####`).
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct EfiKeyOptionHeader {
    /// Hot key options and modifier mask.
    pub key_data: u32,
    /// Expected CRC-32 of the corresponding `Boot####` variable.
    pub boot_option_crc: u32,
    /// Target boot option index (e.g. 0x0001 for `Boot0001`).
    pub boot_option: u16,
}

/// Parsed view of a UEFI hot key option variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyOptionView<'a> {
    /// Hot key options and modifier mask.
    pub key_data: EfiBootKeyData,
    /// CRC-32 checksum to validate against the target `Boot####` option.
    pub boot_option_crc: u32,
    /// Target boot option index.
    pub boot_option: u16,
    /// Array of input keys (0 to 3 elements).
    pub keys: &'a [EfiInputKey],
}

impl<'a> KeyOptionView<'a> {
    /// Parses a serialized key option from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns [`LoadOptionError::BufferTooSmall`] if buffer is smaller than header or key list.
    #[allow(clippy::cast_ptr_alignment)]
    pub fn parse(bytes: &'a [u8]) -> Result<Self, LoadOptionError> {
        let header_size = core::mem::size_of::<EfiKeyOptionHeader>();
        if bytes.len() < header_size {
            return Err(LoadOptionError::BufferTooSmall);
        }

        let kd = u32::from_le_bytes(
            bytes[0..4]
                .try_into()
                .map_err(|_| LoadOptionError::BufferTooSmall)?,
        );
        let crc = u32::from_le_bytes(
            bytes[4..8]
                .try_into()
                .map_err(|_| LoadOptionError::BufferTooSmall)?,
        );
        let opt = u16::from_le_bytes(
            bytes[8..10]
                .try_into()
                .map_err(|_| LoadOptionError::BufferTooSmall)?,
        );

        let key_data = EfiBootKeyData(kd);
        let count = key_data.input_key_count() as usize;
        let key_elem_size = core::mem::size_of::<EfiInputKey>();
        let needed_keys_len = count * key_elem_size;

        if bytes.len() < header_size + needed_keys_len {
            return Err(LoadOptionError::BufferTooSmall);
        }

        let keys_bytes = &bytes[header_size..header_size + needed_keys_len];
        let keys_ptr = keys_bytes.as_ptr();
        if keys_ptr as usize % core::mem::align_of::<EfiInputKey>() != 0 {
            return Err(LoadOptionError::BufferTooSmall);
        }

        let keys = unsafe { core::slice::from_raw_parts(keys_ptr.cast::<EfiInputKey>(), count) };

        Ok(Self {
            key_data,
            boot_option_crc: crc,
            boot_option: opt,
            keys,
        })
    }

    /// Returns the serialized byte size of this key option.
    #[must_use]
    pub fn serialized_size(&self) -> usize {
        core::mem::size_of::<EfiKeyOptionHeader>() + core::mem::size_of_val(self.keys)
    }

    /// Serializes this key option into the provided destination buffer.
    ///
    /// # Errors
    ///
    /// Returns [`LoadOptionError::DestinationBufferTooSmall`] if `buffer` is too small.
    pub fn serialize(&self, buffer: &mut [u8]) -> Result<usize, LoadOptionError> {
        let needed = self.serialized_size();
        if buffer.len() < needed {
            return Err(LoadOptionError::DestinationBufferTooSmall);
        }

        buffer[0..4].copy_from_slice(&self.key_data.0.to_le_bytes());
        buffer[4..8].copy_from_slice(&self.boot_option_crc.to_le_bytes());
        buffer[8..10].copy_from_slice(&self.boot_option.to_le_bytes());

        let mut offset = 10;
        let key_size = core::mem::size_of::<EfiInputKey>();
        for key in self.keys {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    core::ptr::from_ref::<EfiInputKey>(key).cast::<u8>(),
                    buffer.as_mut_ptr().add(offset),
                    key_size,
                );
            }
            offset += key_size;
        }

        Ok(offset)
    }

    /// Validates whether this key option matches the target boot option payload CRC-32.
    #[must_use]
    pub fn validate_crc(&self, boot_option_bytes: &[u8], crc32_fn: impl Fn(&[u8]) -> u32) -> bool {
        self.boot_option_crc == crc32_fn(boot_option_bytes)
    }
}

/// Zero-copy iterator and view over a `BootOrder`, `DriverOrder`, or `SysPrepOrder` variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BootOrderView<'a> {
    raw: &'a [u8],
}

impl<'a> BootOrderView<'a> {
    /// Creates a view over an order variable buffer.
    #[must_use]
    pub const fn new(raw: &'a [u8]) -> Self {
        Self { raw }
    }

    /// Number of option indices in this order list.
    #[must_use]
    pub const fn count(&self) -> usize {
        self.raw.len() / 2
    }

    /// Returns option index at position `index`.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<u16> {
        let offset = index * 2;
        if offset + 2 <= self.raw.len() {
            Some(u16::from_le_bytes([self.raw[offset], self.raw[offset + 1]]))
        } else {
            None
        }
    }

    /// Returns the position of `option_number` in the order list if found.
    #[must_use]
    pub fn find(&self, option_number: u16) -> Option<usize> {
        for (i, opt) in self.iter().enumerate() {
            if opt == option_number {
                return Some(i);
            }
        }
        None
    }

    /// Returns an iterator yielding each boot option index.
    #[must_use]
    pub const fn iter(&self) -> BootOrderIter<'a> {
        BootOrderIter {
            view: *self,
            pos: 0,
        }
    }
}

impl<'a> IntoIterator for BootOrderView<'a> {
    type Item = u16;
    type IntoIter = BootOrderIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &BootOrderView<'a> {
    type Item = u16;
    type IntoIter = BootOrderIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over option indices in an order list.
#[derive(Clone, Debug)]
pub struct BootOrderIter<'a> {
    view: BootOrderView<'a>,
    pos: usize,
}

impl Iterator for BootOrderIter<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.view.get(self.pos)?;
        self.pos += 1;
        Some(item)
    }
}

/// Inserts a new option number into an existing order slice at `position`.
///
/// # Errors
///
/// Returns [`LoadOptionError::AlreadyExists`] if `option_number` is already present,
/// or [`LoadOptionError::DestinationBufferTooSmall`] if `out` is smaller than `order.len() + 1`.
pub fn add_option_number_to_order(
    order: &[u16],
    option_number: u16,
    position: usize,
    out: &mut [u16],
) -> Result<usize, LoadOptionError> {
    if order.contains(&option_number) {
        return Err(LoadOptionError::AlreadyExists);
    }
    if out.len() < order.len() + 1 {
        return Err(LoadOptionError::DestinationBufferTooSmall);
    }

    let insert_pos = core::cmp::min(position, order.len());
    out[..insert_pos].copy_from_slice(&order[..insert_pos]);
    out[insert_pos] = option_number;
    out[(insert_pos + 1)..=order.len()].copy_from_slice(&order[insert_pos..]);

    Ok(order.len() + 1)
}

/// Removes an option number from an existing order slice.
///
/// # Errors
///
/// Returns [`LoadOptionError::NotFound`] if `option_number` is not present,
/// or [`LoadOptionError::DestinationBufferTooSmall`] if `out` is smaller than `order.len() - 1`.
pub fn remove_option_number_from_order(
    order: &[u16],
    option_number: u16,
    out: &mut [u16],
) -> Result<usize, LoadOptionError> {
    let pos = order
        .iter()
        .position(|&x| x == option_number)
        .ok_or(LoadOptionError::NotFound)?;

    if out.len() < order.len() - 1 {
        return Err(LoadOptionError::DestinationBufferTooSmall);
    }

    out[..pos].copy_from_slice(&order[..pos]);
    out[pos..order.len() - 1].copy_from_slice(&order[pos + 1..]);

    Ok(order.len() - 1)
}

/// Finds the lowest unused option number in range `[0x0000, 0xFFFF]`.
///
/// Ensures the allocated number is neither present in the `order` array nor equals `boot_next`.
///
/// # Errors
///
/// Returns [`LoadOptionError::NoFreeOptionNumber`] if all 65,536 option numbers are occupied.
pub fn find_free_option_number(
    order: &[u16],
    boot_next: Option<u16>,
) -> Result<u16, LoadOptionError> {
    let limit = order.len() + usize::from(boot_next.is_some()) + 1;
    let max_check = core::cmp::min(limit, LOAD_OPTION_NUMBER_MAX as usize);

    for candidate in 0..max_check {
        let opt = u16::try_from(candidate).map_err(|_| LoadOptionError::NoFreeOptionNumber)?;
        if !order.contains(&opt) && boot_next != Some(opt) {
            return Ok(opt);
        }
    }

    Err(LoadOptionError::NoFreeOptionNumber)
}

/// Formats a 16-bit option number into standard 4-digit uppercase hexadecimal ASCII.
const fn hex_digit(nibble: u8) -> u8 {
    if nibble < 10 {
        b'0' + nibble
    } else {
        b'A' + (nibble - 10)
    }
}

/// Formats a `Boot####` variable name into the provided 8-byte buffer and returns string slice.
pub fn format_boot_option_name(index: u16, out: &mut [u8; 8]) -> &str {
    out[0] = b'B';
    out[1] = b'o';
    out[2] = b'o';
    out[3] = b't';
    out[4] = hex_digit(((index >> 12) & 0x0F) as u8);
    out[5] = hex_digit(((index >> 8) & 0x0F) as u8);
    out[6] = hex_digit(((index >> 4) & 0x0F) as u8);
    out[7] = hex_digit((index & 0x0F) as u8);
    core::str::from_utf8(out).unwrap_or("Boot0000")
}

/// Formats a `Driver####` variable name into the provided 10-byte buffer and returns string slice.
pub fn format_driver_option_name(index: u16, out: &mut [u8; 10]) -> &str {
    out[0] = b'D';
    out[1] = b'r';
    out[2] = b'i';
    out[3] = b'v';
    out[4] = b'e';
    out[5] = b'r';
    out[6] = hex_digit(((index >> 12) & 0x0F) as u8);
    out[7] = hex_digit(((index >> 8) & 0x0F) as u8);
    out[8] = hex_digit(((index >> 4) & 0x0F) as u8);
    out[9] = hex_digit((index & 0x0F) as u8);
    core::str::from_utf8(out).unwrap_or("Driver0000")
}

/// Formats a `SysPrep####` variable name into the provided 11-byte buffer and returns string slice.
pub fn format_sysprep_option_name(index: u16, out: &mut [u8; 11]) -> &str {
    out[0] = b'S';
    out[1] = b'y';
    out[2] = b's';
    out[3] = b'P';
    out[4] = b'r';
    out[5] = b'e';
    out[6] = b'p';
    out[7] = hex_digit(((index >> 12) & 0x0F) as u8);
    out[8] = hex_digit(((index >> 8) & 0x0F) as u8);
    out[9] = hex_digit(((index >> 4) & 0x0F) as u8);
    out[10] = hex_digit((index & 0x0F) as u8);
    core::str::from_utf8(out).unwrap_or("SysPrep0000")
}

/// Formats a `PlatformRecovery####` variable name into the provided 20-byte buffer.
pub fn format_platform_recovery_option_name(index: u16, out: &mut [u8; 20]) -> &str {
    let prefix = b"PlatformRecovery";
    out[..16].copy_from_slice(prefix);
    out[16] = hex_digit(((index >> 12) & 0x0F) as u8);
    out[17] = hex_digit(((index >> 8) & 0x0F) as u8);
    out[18] = hex_digit(((index >> 4) & 0x0F) as u8);
    out[19] = hex_digit((index & 0x0F) as u8);
    core::str::from_utf8(out).unwrap_or("PlatformRecovery0000")
}

/// Formats any load option variable name given its type and option index.
pub fn format_load_option_name(
    option_type: LoadOptionType,
    index: u16,
    out: &mut [u8; 20],
) -> &str {
    match option_type {
        LoadOptionType::Driver => {
            let mut buf = [0u8; 10];
            let s = format_driver_option_name(index, &mut buf);
            out[..10].copy_from_slice(s.as_bytes());
            core::str::from_utf8(&out[..10]).unwrap_or("Driver0000")
        }
        LoadOptionType::SysPrep => {
            let mut buf = [0u8; 11];
            let s = format_sysprep_option_name(index, &mut buf);
            out[..11].copy_from_slice(s.as_bytes());
            core::str::from_utf8(&out[..11]).unwrap_or("SysPrep0000")
        }
        LoadOptionType::Boot => {
            let mut buf = [0u8; 8];
            let s = format_boot_option_name(index, &mut buf);
            out[..8].copy_from_slice(s.as_bytes());
            core::str::from_utf8(&out[..8]).unwrap_or("Boot0000")
        }
        LoadOptionType::PlatformRecovery => format_platform_recovery_option_name(index, out),
    }
}

/// Formats a `Key####` variable name into the provided 7-byte buffer and returns string slice.
pub fn format_key_option_name(index: u16, out: &mut [u8; 7]) -> &str {
    out[0] = b'K';
    out[1] = b'e';
    out[2] = b'y';
    out[3] = hex_digit(((index >> 12) & 0x0F) as u8);
    out[4] = hex_digit(((index >> 8) & 0x0F) as u8);
    out[5] = hex_digit(((index >> 4) & 0x0F) as u8);
    out[6] = hex_digit((index & 0x0F) as u8);
    core::str::from_utf8(out).unwrap_or("Key0000")
}

/// Parses a 4-digit uppercase or lowercase hex string into a `u16` index.
fn parse_hex_4(s: &str) -> Option<u16> {
    if s.len() != 4 {
        return None;
    }
    let mut val = 0u16;
    for b in s.bytes() {
        val <<= 4;
        match b {
            b'0'..=b'9' => val |= u16::from(b - b'0'),
            b'a'..=b'f' => val |= u16::from(b - b'a' + 10),
            b'A'..=b'F' => val |= u16::from(b - b'A' + 10),
            _ => return None,
        }
    }
    Some(val)
}

/// Checks whether `name` is a valid load option variable name and returns `(LoadOptionType, u16)`.
#[must_use]
pub fn is_valid_load_option_variable_name(name: &str) -> Option<(LoadOptionType, u16)> {
    if name.len() < 5 {
        return None;
    }

    let prefix_len = name.len() - 4;
    let prefix = &name[..prefix_len];
    let hex_part = &name[prefix_len..];

    let opt_type = if prefix.eq_ignore_ascii_case("Driver") {
        LoadOptionType::Driver
    } else if prefix.eq_ignore_ascii_case("SysPrep") {
        LoadOptionType::SysPrep
    } else if prefix.eq_ignore_ascii_case("Boot") {
        LoadOptionType::Boot
    } else if prefix.eq_ignore_ascii_case("PlatformRecovery") {
        LoadOptionType::PlatformRecovery
    } else {
        return None;
    };

    let index = parse_hex_4(hex_part)?;
    Some((opt_type, index))
}

/// Checks whether `name` is a valid `Key####` variable name and returns the option number.
#[must_use]
pub fn is_valid_key_option_variable_name(name: &str) -> Option<u16> {
    if name.len() == 7 && (name.starts_with("Key") || name.starts_with("KEY")) {
        parse_hex_4(&name[3..])
    } else {
        None
    }
}

/// Parses a `Boot####` variable name into its 16-bit option index.
#[must_use]
pub fn parse_boot_option_index(name: &str) -> Option<u16> {
    if name.len() == 8 && (name.starts_with("Boot") || name.starts_with("BOOT")) {
        parse_hex_4(&name[4..])
    } else {
        None
    }
}
