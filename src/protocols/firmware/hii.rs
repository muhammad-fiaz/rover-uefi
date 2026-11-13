//! # Human Interface Infrastructure (HII)
//!
//! Provides the core UEFI Human Interface Infrastructure protocols, package parsing,
//! string management, configuration routing, font rendering, popup dialogs, and image
//! decoders as defined in UEFI Specification Chapters 32–34.

use crate::abi::types::{Bool, UIntN};
use crate::base::{EfiGuid, EfiHandle, EfiStatus};
use crate::protocols::graphics::pixel::EfiGraphicsOutputBltPixel;
use core::ffi::c_void;

// --- Handle Types ---

/// Strongly typed handle for an HII database package list entry (`EFI_HII_HANDLE`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EfiHiiHandle(pub *mut c_void);

impl EfiHiiHandle {
    /// Null / unassigned HII handle.
    pub const NULL: Self = Self(core::ptr::null_mut());

    /// Creates an HII handle from a raw pointer.
    #[must_use]
    pub const fn from_ptr(ptr: *mut c_void) -> Self {
        Self(ptr)
    }

    /// Returns the underlying raw pointer.
    #[must_use]
    pub const fn as_ptr(self) -> *mut c_void {
        self.0
    }

    /// Returns `true` if this is a null handle.
    #[must_use]
    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
}

/// An HII string identifier within a package list (`EFI_STRING_ID`).
pub type EfiStringId = u16;

/// Null string identifier reserved in UEFI HII.
pub const EFI_STRING_ID_NULL: EfiStringId = 0;

// --- Standard HII Protocol GUIDs ---

/// HII Database Protocol `GUID` (`EFI_HII_DATABASE_PROTOCOL_GUID`).
pub const EFI_HII_DATABASE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xEF9F_C172,
    0xA1B2,
    0x4693,
    [0xB3, 0x27, 0x6D, 0x32, 0xFC, 0x41, 0x60, 0x42],
);

/// HII Database Protocol `GUID` alias.
pub const HII_DATABASE_GUID: EfiGuid = EFI_HII_DATABASE_PROTOCOL_GUID;

/// HII forms package `GUID`.
pub const HII_FORMS_GUID: EfiGuid = EfiGuid::new(
    0x92D9_A4FD,
    0xB76D,
    0x4064,
    [0x8A, 0x56, 0x54, 0x6E, 0x94, 0x4E, 0xC3, 0x00],
);

/// HII string package `GUID`.
pub const HII_STRING_PACKAGE_GUID: EfiGuid = EfiGuid::new(
    0xD7C4_0F9E,
    0x8810,
    0x475F,
    [0x9C, 0x72, 0xCC, 0x55, 0x85, 0x94, 0x1D, 0x00],
);

/// HII configuration access protocol `GUID`.
pub const HII_CONFIG_ACCESS_GUID: EfiGuid = EfiGuid::new(
    0x4D5A_18D2,
    0xD2C9,
    0x4833,
    [0xB6, 0xEF, 0x47, 0x46, 0xBA, 0x0E, 0x0A, 0x00],
);

/// HII font alter protocol `GUID`.
pub const HII_FONT_ALTER_GUID: EfiGuid = EfiGuid::new(
    0x6194_CBE5,
    0x5A3B,
    0x418E,
    [0x9D, 0x23, 0xC2, 0x5A, 0x0D, 0x21, 0xFC, 0x00],
);

/// HII String Protocol `GUID` (`EFI_HII_STRING_PROTOCOL_GUID`).
pub const EFI_HII_STRING_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0FD9_6974,
    0x23AA,
    0x4CDC,
    [0xB9, 0xCB, 0x98, 0xD1, 0x77, 0x50, 0x32, 0x2A],
);

/// HII Configuration Routing Protocol `GUID` (`EFI_HII_CONFIG_ROUTING_PROTOCOL_GUID`).
pub const EFI_HII_CONFIG_ROUTING_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x587E_72D7,
    0xCC50,
    0x4F79,
    [0x82, 0x09, 0xCA, 0x29, 0x1F, 0xC1, 0xA1, 0x0F],
);

/// HII Font Protocol `GUID` (`EFI_HII_FONT_PROTOCOL_GUID`).
pub const EFI_HII_FONT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE9CA_4775,
    0x8657,
    0x47FC,
    [0x97, 0xE7, 0x7E, 0xD6, 0x5A, 0x08, 0x43, 0x24],
);

/// HII Image Protocol `GUID` (`EFI_HII_IMAGE_PROTOCOL_GUID`).
pub const EFI_HII_IMAGE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x31A6_406A,
    0x6B56,
    0x4E25,
    [0x89, 0x21, 0x99, 0x2D, 0xC3, 0xFC, 0xFE, 0x16],
);

/// HII Popup Protocol `GUID` (`EFI_HII_POPUP_PROTOCOL_GUID`).
pub const EFI_HII_POPUP_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x4311_EDC0,
    0x6054,
    0x46D4,
    [0x9E, 0x40, 0x89, 0x3E, 0xA9, 0x52, 0xFC, 0xCC],
);

/// HII Image Decoder Protocol `GUID` (`EFI_HII_IMAGE_DECODER_PROTOCOL_GUID`).
pub const EFI_HII_IMAGE_DECODER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9E66_F251,
    0x727C,
    0x418C,
    [0xBF, 0xD6, 0xC2, 0xB4, 0x25, 0x28, 0x18, 0xEA],
);

/// JPEG Image Decoder Name `GUID`.
pub const EFI_HII_IMAGE_DECODER_NAME_JPEG_GUID: EfiGuid = EfiGuid::new(
    0xEFEF_D093,
    0x0D9B,
    0x46EB,
    [0xA8, 0x56, 0x48, 0x35, 0x07, 0x00, 0xC9, 0x08],
);

/// PNG Image Decoder Name `GUID`.
pub const EFI_HII_IMAGE_DECODER_NAME_PNG_GUID: EfiGuid = EfiGuid::new(
    0xAF06_0190,
    0x5E3A,
    0x4025,
    [0xAF, 0xBD, 0xE1, 0xF9, 0x05, 0xBF, 0xAA, 0x4C],
);

/// Event signaled when the default keyboard layout changes.
pub const EFI_HII_SET_KEYBOARD_LAYOUT_EVENT_GUID: EfiGuid = EfiGuid::new(
    0x1498_2A35,
    0x5A29,
    0x498D,
    [0xA2, 0x44, 0x63, 0x98, 0x8C, 0x74, 0x1E, 0x93],
);

// --- Package Header & Types ---

/// Package types in an HII package list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HiiPackageType {
    /// All package types (used for querying / notifications).
    All = 0x00,
    /// GUID-qualified package.
    Guid = 0x01,
    /// Forms package (IFR opcodes).
    Forms = 0x02,
    /// String package (multilingual text).
    Strings = 0x03,
    /// Fonts package.
    Fonts = 0x04,
    /// Images package.
    Images = 0x05,
    /// Simple fonts package.
    SimpleFonts = 0x06,
    /// Device path package.
    DevicePath = 0x07,
    /// Keyboard layout package.
    KeyboardLayout = 0x08,
    /// Animations package.
    Animations = 0x09,
    /// End of package list marker.
    End = 0xDF,
}

/// Package list header preceding a series of HII packages (`EFI_HII_PACKAGE_LIST_HEADER`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiHiiPackageListHeader {
    /// GUID identifying this package list.
    pub package_list_guid: EfiGuid,
    /// Total length in bytes of the package list including this header.
    pub package_length: u32,
}

/// Standard HII package header (`EFI_HII_PACKAGE_HEADER`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HiiPackageHeader {
    /// The full package length in bytes.
    pub package_length: u32,
    /// The package type.
    pub package_type: u8,
}

/// Parses an HII package header.
///
/// Bounded parser returning `None` if `input` is shorter than 4 bytes or declared
/// package length exceeds buffer length.
#[must_use]
pub fn parse_package_header(input: &[u8]) -> Option<HiiPackageHeader> {
    if input.len() < 4 {
        return None;
    }
    let b0 = input[0];
    let b1 = input[1];
    let b2 = input[2];
    let b3 = input[3];

    let package_length = u32::from(b0) | (u32::from(b1) << 8) | (u32::from(b2) << 16);
    let package_type = b3;

    if (package_length as usize) > input.len() || package_length < 4 {
        return None;
    }

    Some(HiiPackageHeader {
        package_length,
        package_type,
    })
}

/// Returns whether a package type is a standard defined UEFI package type.
#[must_use]
pub fn is_known_package_type(package_type: u8) -> bool {
    matches!(package_type, 0x00..=0x09 | 0xDF)
}

// --- 1. HII Database Protocol ---

/// Database notification change types (`EFI_HII_DATABASE_NOTIFY_TYPE`).
pub mod notify_type {
    /// Notification on new package list registration.
    pub const NEW_PACK: usize = 0x0000_0001;
    /// Notification on package list removal.
    pub const REMOVE_PACK: usize = 0x0000_0002;
    /// Notification on package list export.
    pub const EXPORT_PACK: usize = 0x0000_0004;
    /// Notification on package addition to existing list.
    pub const ADD_PACK: usize = 0x0000_0008;
}

/// Callback notification function pointer for HII database change events.
pub type EfiHiiDatabaseNotify = unsafe extern "efiapi" fn(
    package_type: u8,
    package_guid: *const EfiGuid,
    package: *const c_void,
    handle: EfiHiiHandle,
    notify_type: UIntN,
) -> EfiStatus;

/// Key descriptor entry within a physical keyboard layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiKeyDescriptor {
    /// EFI scan code or key identifier.
    pub key: u16,
    /// Unicode character produced when unshifted.
    pub unicode_char: u16,
    /// Unicode character produced when shifted.
    pub shifted_unicode_char: u16,
    /// Unicode character produced with `AltGr`.
    pub alt_gr_unicode_char: u16,
    /// Unicode character produced with Shift + `AltGr`.
    pub shifted_alt_gr_unicode_char: u16,
    /// Modifier state flags.
    pub modifier: u16,
}

/// Physical keyboard layout definition (`EFI_HII_KEYBOARD_LAYOUT`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiHiiKeyboardLayout {
    /// Total layout byte length.
    pub layout_length: u16,
    /// Unique identifier for this keyboard layout.
    pub guid: EfiGuid,
    /// Byte offset to the description string.
    pub layout_descriptor_string_offset: u32,
    /// Number of key descriptors following the header.
    pub descriptor_count: u8,
}

/// Raw interface structure for the UEFI HII Database Protocol (`EFI_HII_DATABASE_PROTOCOL`).
#[derive(Debug)]
#[repr(C)]
pub struct EfiHiiDatabaseProtocol {
    /// Adds a package list to the database and returns a handle.
    pub new_package_list: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        package_list: *const EfiHiiPackageListHeader,
        driver_handle: EfiHandle,
        handle: *mut EfiHiiHandle,
    ) -> EfiStatus,
    /// Removes a package list from the database.
    pub remove_package_list: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        handle: EfiHiiHandle,
    ) -> EfiStatus,
    /// Updates an existing package list.
    pub update_package_list: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        handle: EfiHiiHandle,
        package_list: *const EfiHiiPackageListHeader,
    ) -> EfiStatus,
    /// Lists handles of package lists matching type and GUID criteria.
    pub list_package_lists: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        package_type: u8,
        package_guid: *const EfiGuid,
        handle_buffer_length: *mut UIntN,
        handle_buffer: *mut EfiHiiHandle,
    ) -> EfiStatus,
    /// Exports package lists into a buffer.
    pub export_package_lists: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        handle: EfiHiiHandle,
        buffer_size: *mut UIntN,
        buffer: *mut EfiHiiPackageListHeader,
    ) -> EfiStatus,
    /// Registers a package notification callback.
    pub register_package_notify: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        package_type: u8,
        package_guid: *const EfiGuid,
        package_notify_fn: EfiHiiDatabaseNotify,
        notify_type: UIntN,
        notify_handle: *mut EfiHandle,
    ) -> EfiStatus,
    /// Unregisters a package notification callback.
    pub unregister_package_notify: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        notify_handle: EfiHandle,
    ) -> EfiStatus,
    /// Retrieves GUIDs of available keyboard layouts.
    pub find_keyboard_layouts: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        key_guid_buffer_length: *mut u16,
        key_guid_buffer: *mut EfiGuid,
    ) -> EfiStatus,
    /// Retrieves a specific keyboard layout.
    pub get_keyboard_layout: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        key_guid: *const EfiGuid,
        keyboard_layout_length: *mut u16,
        keyboard_layout: *mut EfiHiiKeyboardLayout,
    ) -> EfiStatus,
    /// Sets the current system keyboard layout.
    pub set_keyboard_layout: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        key_guid: *const EfiGuid,
    ) -> EfiStatus,
    /// Returns the EFI driver handle associated with a package list.
    pub get_package_list_handle: unsafe extern "efiapi" fn(
        this: *const EfiHiiDatabaseProtocol,
        package_list_handle: EfiHiiHandle,
        driver_handle: *mut EfiHandle,
    ) -> EfiStatus,
}

/// Safe idiomatic client wrapper for the UEFI HII Database Protocol.
#[derive(Debug)]
pub struct HiiDatabase<'a> {
    protocol: &'a EfiHiiDatabaseProtocol,
}

impl<'a> HiiDatabase<'a> {
    /// HII Database Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_HII_DATABASE_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiHiiDatabaseProtocol) -> Self {
        Self { protocol }
    }

    /// Adds a package list to the HII database.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if memory allocation fails.
    /// Returns `EfiStatus::INVALID_PARAMETER` if package list is invalid.
    pub fn new_package_list(
        &self,
        package_list: &EfiHiiPackageListHeader,
        driver_handle: EfiHandle,
    ) -> Result<EfiHiiHandle, EfiStatus> {
        let mut handle = EfiHiiHandle::NULL;
        let status = unsafe {
            (self.protocol.new_package_list)(
                self.protocol,
                package_list,
                driver_handle,
                &raw mut handle,
            )
        };
        if status.is_success() {
            Ok(handle)
        } else {
            Err(status)
        }
    }

    /// Removes a package list from the database.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if `handle` is not in the database.
    pub fn remove_package_list(&self, handle: EfiHiiHandle) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.remove_package_list)(self.protocol, handle) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the driver handle associated with an HII package list handle.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if `handle` is not valid.
    pub fn get_driver_handle(&self, handle: EfiHiiHandle) -> Result<EfiHandle, EfiStatus> {
        let mut driver_handle = EfiHandle::NULL;
        let status = unsafe {
            (self.protocol.get_package_list_handle)(self.protocol, handle, &raw mut driver_handle)
        };
        if status.is_success() {
            Ok(driver_handle)
        } else {
            Err(status)
        }
    }
}

// --- 2. HII String Protocol ---

/// Raw interface structure for the UEFI HII String Protocol (`EFI_HII_STRING_PROTOCOL`).
#[derive(Debug)]
#[repr(C)]
pub struct EfiHiiStringProtocol {
    /// Adds a new string to a package list.
    pub new_string: unsafe extern "efiapi" fn(
        this: *const EfiHiiStringProtocol,
        package_list: EfiHiiHandle,
        string_id: *mut EfiStringId,
        language: *const u8,
        language_name: *const u16,
        string: *const u16,
        string_font_info: *const c_void,
    ) -> EfiStatus,
    /// Retrieves a string by ID in a specific language.
    pub get_string: unsafe extern "efiapi" fn(
        this: *const EfiHiiStringProtocol,
        language: *const u8,
        package_list: EfiHiiHandle,
        string_id: EfiStringId,
        string: *mut u16,
        string_size: *mut UIntN,
        string_font_info: *mut *mut c_void,
    ) -> EfiStatus,
    /// Updates an existing string in a package list.
    pub set_string: unsafe extern "efiapi" fn(
        this: *const EfiHiiStringProtocol,
        package_list: EfiHiiHandle,
        string_id: EfiStringId,
        language: *const u8,
        string: *const u16,
        string_font_info: *const c_void,
    ) -> EfiStatus,
    /// Returns the null-separated list of supported languages.
    pub get_languages: unsafe extern "efiapi" fn(
        this: *const EfiHiiStringProtocol,
        package_list: EfiHiiHandle,
        languages: *mut u8,
        languages_size: *mut UIntN,
    ) -> EfiStatus,
    /// Returns secondary languages associated with a primary language.
    pub get_secondary_languages: unsafe extern "efiapi" fn(
        this: *const EfiHiiStringProtocol,
        package_list: EfiHiiHandle,
        primary_language: *const u8,
        secondary_languages: *mut u8,
        secondary_languages_size: *mut UIntN,
    ) -> EfiStatus,
}

/// Safe idiomatic client wrapper for the UEFI HII String Protocol.
#[derive(Debug)]
pub struct HiiString<'a> {
    protocol: &'a EfiHiiStringProtocol,
}

impl<'a> HiiString<'a> {
    /// HII String Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_HII_STRING_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiHiiStringProtocol) -> Self {
        Self { protocol }
    }

    /// Adds a string to an HII package list.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if package list is missing.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` on allocation failure.
    pub fn new_string(
        &self,
        package_list: EfiHiiHandle,
        language: &[u8],
        string: &[u16],
    ) -> Result<EfiStringId, EfiStatus> {
        let mut string_id: EfiStringId = EFI_STRING_ID_NULL;
        let status = unsafe {
            (self.protocol.new_string)(
                self.protocol,
                package_list,
                &raw mut string_id,
                language.as_ptr(),
                core::ptr::null(),
                string.as_ptr(),
                core::ptr::null(),
            )
        };
        if status.is_success() {
            Ok(string_id)
        } else {
            Err(status)
        }
    }

    /// Retrieves a string by ID into a caller-supplied buffer.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if `buffer` is too small.
    /// Returns `EfiStatus::NOT_FOUND` if string ID does not exist.
    pub fn get_string(
        &self,
        package_list: EfiHiiHandle,
        string_id: EfiStringId,
        language: &[u8],
        buffer: &mut [u16],
    ) -> Result<usize, EfiStatus> {
        let mut size = core::mem::size_of_val(buffer);
        let status = unsafe {
            (self.protocol.get_string)(
                self.protocol,
                language.as_ptr(),
                package_list,
                string_id,
                buffer.as_mut_ptr(),
                &raw mut size,
                core::ptr::null_mut(),
            )
        };
        if status.is_success() {
            Ok(size / core::mem::size_of::<u16>())
        } else {
            Err(status)
        }
    }
}

// --- 3. HII Configuration Routing Protocol ---

/// Raw interface structure for the UEFI HII Configuration Routing Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiHiiConfigRoutingProtocol {
    /// Extracts configuration from drivers corresponding to a request string.
    pub extract_config: unsafe extern "efiapi" fn(
        this: *const EfiHiiConfigRoutingProtocol,
        request: *const u16,
        progress: *mut *const u16,
        results: *mut *mut u16,
    ) -> EfiStatus,
    /// Exports configuration for all registered drivers.
    pub export_config: unsafe extern "efiapi" fn(
        this: *const EfiHiiConfigRoutingProtocol,
        results: *mut *mut u16,
    ) -> EfiStatus,
    /// Routes configuration changes to drivers.
    pub route_config: unsafe extern "efiapi" fn(
        this: *const EfiHiiConfigRoutingProtocol,
        configuration: *const u16,
        progress: *mut *const u16,
    ) -> EfiStatus,
    /// Helper to convert binary block to configuration string.
    pub block_to_config: unsafe extern "efiapi" fn(
        this: *const EfiHiiConfigRoutingProtocol,
        config_request: *const u16,
        block: *const u8,
        block_size: UIntN,
        config: *mut *mut u16,
        progress: *mut *const u16,
    ) -> EfiStatus,
    /// Helper to convert configuration response string to binary block.
    pub config_to_block: unsafe extern "efiapi" fn(
        this: *const EfiHiiConfigRoutingProtocol,
        config_resp: *const u16,
        block: *mut u8,
        block_size: *mut UIntN,
        progress: *mut *const u16,
    ) -> EfiStatus,
    /// Extracts alternative configuration from larger string.
    pub get_alt_config: unsafe extern "efiapi" fn(
        this: *const EfiHiiConfigRoutingProtocol,
        config_resp: *const u16,
        guid: *const EfiGuid,
        name: *const u16,
        device_path: *const c_void,
        alt_cfg_id: *const u16,
        alt_cfg_resp: *mut *mut u16,
    ) -> EfiStatus,
}

/// Safe idiomatic client wrapper for the UEFI HII Configuration Routing Protocol.
#[derive(Debug)]
pub struct HiiConfigRouting<'a> {
    protocol: &'a EfiHiiConfigRoutingProtocol,
}

impl<'a> HiiConfigRouting<'a> {
    /// HII Configuration Routing Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_HII_CONFIG_ROUTING_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiHiiConfigRoutingProtocol) -> Self {
        Self { protocol }
    }

    /// Routes configuration response string to appropriate drivers.
    ///
    /// # Safety
    ///
    /// `config` must point to a valid null-terminated UCS-2/UTF-16 configuration string.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if string syntax is invalid.
    /// Returns `EfiStatus::NOT_FOUND` if target driver is missing.
    pub unsafe fn route_config(&self, config: *const u16) -> Result<(), EfiStatus> {
        let mut progress = core::ptr::null();
        let status =
            unsafe { (self.protocol.route_config)(self.protocol, config, &raw mut progress) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// --- 4. HII Font Protocol & Text Rendering ---

/// HII string rendering output flags.
pub mod hii_out_flags {
    /// Clip text at bounding box boundaries.
    pub const CLIP: u32 = 0x0000_0001;
    /// Wrap text at bounding box boundaries.
    pub const WRAP: u32 = 0x0000_0002;
    /// Clip only whole lines vertically.
    pub const CLIP_CLEAN_Y: u32 = 0x0000_0004;
    /// Clip only whole glyphs horizontally.
    pub const CLIP_CLEAN_X: u32 = 0x0000_0008;
    /// Transparent text background.
    pub const TRANSPARENT: u32 = 0x0000_0010;
    /// Ignore rendering if glyph is missing.
    pub const IGNORE_IF_NO_GLYPH: u32 = 0x0000_0020;
    /// Direct rendering to screen.
    pub const DIRECT_TO_SCREEN: u32 = 0x0000_0080;
}

/// Row information describing text line placement in rendered bitmap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct HiiRowInfo {
    /// Starting character index for this line.
    pub start_index: UIntN,
    /// Ending character index for this line.
    pub end_index: UIntN,
    /// Height of the line in pixels.
    pub line_height: UIntN,
    /// Width of text on the line in pixels.
    pub line_width: UIntN,
    /// Font baseline offset in pixels from bottom of row.
    pub baseline_offset: UIntN,
}

/// Font description info structure (`EFI_FONT_INFO`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct FontInfo {
    /// Font style flags (bold, italic).
    pub font_style: u32,
    /// Character cell height in pixels.
    pub font_size: u16,
    /// First character of font name (null-terminated).
    pub font_name: [u16; 1],
}

/// Font display color and mask configuration (`EFI_FONT_DISPLAY_INFO`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct FontDisplayInfo {
    /// Foreground pixel color.
    pub foreground_color: EfiGraphicsOutputBltPixel,
    /// Background pixel color.
    pub background_color: EfiGraphicsOutputBltPixel,
    /// Mask flags determining fallback behavior.
    pub font_info_mask: u32,
    /// Basic font parameters.
    pub font_info: FontInfo,
}

/// Output surface descriptor for HII rendering (`EFI_IMAGE_OUTPUT`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiImageOutput {
    /// Surface width in pixels.
    pub width: u16,
    /// Surface height in pixels.
    pub height: u16,
    /// Destination pointer (`EfiGraphicsOutputBltPixel` bitmap or screen protocol).
    pub image_surface: *mut c_void,
}

/// Raw interface structure for the UEFI HII Font Protocol (`EFI_HII_FONT_PROTOCOL`).
#[derive(Debug)]
#[repr(C)]
pub struct EfiHiiFontProtocol {
    /// Renders a string to a bitmap or screen using specified font and colors.
    pub string_to_image: unsafe extern "efiapi" fn(
        this: *const EfiHiiFontProtocol,
        flags: u32,
        string: *const u16,
        string_info: *const FontDisplayInfo,
        blt: *mut *mut EfiImageOutput,
        blt_x: UIntN,
        blt_y: UIntN,
        row_info_array: *mut *mut HiiRowInfo,
        row_info_array_size: *mut UIntN,
        column_info_array: *mut UIntN,
    ) -> EfiStatus,
    /// Renders a string looked up from a package list by ID.
    pub string_id_to_image: unsafe extern "efiapi" fn(
        this: *const EfiHiiFontProtocol,
        flags: u32,
        package_list: EfiHiiHandle,
        string_id: EfiStringId,
        language: *const u8,
        string_info: *const FontDisplayInfo,
        blt: *mut *mut EfiImageOutput,
        blt_x: UIntN,
        blt_y: UIntN,
        row_info_array: *mut *mut HiiRowInfo,
        row_info_array_size: *mut UIntN,
        column_info_array: *mut UIntN,
    ) -> EfiStatus,
    /// Converts a character glyph to bitmap.
    pub get_glyph: unsafe extern "efiapi" fn(
        this: *const EfiHiiFontProtocol,
        char_code: u16,
        string_info: *const FontDisplayInfo,
        blt: *mut *mut EfiImageOutput,
        baseline: *mut UIntN,
    ) -> EfiStatus,
    /// Iterates through matching available fonts.
    pub get_font_info: unsafe extern "efiapi" fn(
        this: *const EfiHiiFontProtocol,
        font_handle: *mut *mut c_void,
        string_info_in: *const FontDisplayInfo,
        string_info_out: *mut *mut FontDisplayInfo,
        string: *const u16,
    ) -> EfiStatus,
}

/// Safe idiomatic client wrapper for the UEFI HII Font Protocol.
#[derive(Debug)]
pub struct HiiFont<'a> {
    protocol: &'a EfiHiiFontProtocol,
}

impl<'a> HiiFont<'a> {
    /// HII Font Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_HII_FONT_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiHiiFontProtocol) -> Self {
        Self { protocol }
    }

    /// Renders a Unicode character glyph into an image output surface.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if surface allocation fails.
    /// Returns `EfiStatus::DEVICE_ERROR` on rendering failure.
    pub fn get_glyph(
        &self,
        char_code: u16,
        font_info: Option<&FontDisplayInfo>,
    ) -> Result<*mut EfiImageOutput, EfiStatus> {
        let mut blt = core::ptr::null_mut();
        let font_ptr = match font_info {
            Some(f) => core::ptr::from_ref::<FontDisplayInfo>(f),
            None => core::ptr::null(),
        };
        let status = unsafe {
            (self.protocol.get_glyph)(
                self.protocol,
                char_code,
                font_ptr,
                &raw mut blt,
                core::ptr::null_mut(),
            )
        };
        if status.is_success() {
            Ok(blt)
        } else {
            Err(status)
        }
    }
}

// --- 5. HII Popup Dialog Protocol ---

/// Visual presentation style of a popup box.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum HiiPopupStyle {
    /// Informational message popup.
    Info = 0,
    /// Warning message popup.
    Warning = 1,
    /// Error message popup.
    Error = 2,
}

/// Button configuration displayed in a popup dialog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum HiiPopupType {
    /// Single "OK" button.
    Ok = 0,
    /// "OK" and "Cancel" buttons.
    OkCancel = 1,
    /// "Yes" and "No" buttons.
    YesNo = 2,
    /// "Yes", "No", and "Cancel" buttons.
    YesNoCancel = 3,
}

/// User button selection returned from a popup dialog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum HiiPopupSelection {
    /// User selected "OK".
    Ok = 0,
    /// User selected "Cancel".
    Cancel = 1,
    /// User selected "Yes".
    Yes = 2,
    /// User selected "No".
    No = 3,
}

/// Raw interface structure for the UEFI HII Popup Protocol (`EFI_HII_POPUP_PROTOCOL`).
#[derive(Debug)]
#[repr(C)]
pub struct EfiHiiPopupProtocol {
    /// Protocol revision (must be 1).
    pub revision: u64,
    /// Displays a modal popup window and returns user selection.
    pub create_popup: unsafe extern "efiapi" fn(
        this: *const EfiHiiPopupProtocol,
        popup_style: HiiPopupStyle,
        popup_type: HiiPopupType,
        hii_handle: EfiHiiHandle,
        message: EfiStringId,
        user_selection: *mut HiiPopupSelection,
    ) -> EfiStatus,
}

/// Safe idiomatic client wrapper for the UEFI HII Popup Protocol.
#[derive(Debug)]
pub struct HiiPopup<'a> {
    protocol: &'a EfiHiiPopupProtocol,
}

impl<'a> HiiPopup<'a> {
    /// HII Popup Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_HII_POPUP_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiHiiPopupProtocol) -> Self {
        Self { protocol }
    }

    /// Displays a popup dialog and waits for user interaction.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::INVALID_PARAMETER` if string ID or handle is invalid.
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if dialog memory cannot be allocated.
    pub fn create_popup(
        &self,
        style: HiiPopupStyle,
        popup_type: HiiPopupType,
        handle: EfiHiiHandle,
        message_id: EfiStringId,
    ) -> Result<HiiPopupSelection, EfiStatus> {
        let mut selection = HiiPopupSelection::Ok;
        let status = unsafe {
            (self.protocol.create_popup)(
                self.protocol,
                style,
                popup_type,
                handle,
                message_id,
                &raw mut selection,
            )
        };
        if status.is_success() {
            Ok(selection)
        } else {
            Err(status)
        }
    }
}

// --- 6. HII Image Decoder Protocol ---

/// Pixel color format decoded by an image decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum HiiImageDecoderColorType {
    /// RGB color format.
    Rgb = 0x00,
    /// RGBA color format with alpha channel.
    Rgba = 0x01,
    /// CMYK color format.
    Cmyk = 0x02,
    /// Unknown / proprietary color format.
    Unknown = 0xFF,
}

/// Image information header returned by decoder (`EFI_HII_IMAGE_DECODER_IMAGE_INFO_HEADER`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct HiiImageDecoderHeader {
    /// GUID identifier of decoder format (e.g. JPEG or PNG).
    pub decoder_name: EfiGuid,
    /// Total size of image information structure in bytes.
    pub image_info_size: u16,
    /// Decoded image width in pixels.
    pub image_width: u16,
    /// Decoded image height in pixels.
    pub image_height: u16,
    /// Color model used in image.
    pub color_type: HiiImageDecoderColorType,
    /// Color depth in bits per pixel.
    pub color_depth_in_bits: u8,
}

/// Raw interface structure for the UEFI HII Image Decoder Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiHiiImageDecoderProtocol {
    /// Retrieves supported image decoder GUIDs.
    pub get_image_decoder_name: unsafe extern "efiapi" fn(
        this: *const EfiHiiImageDecoderProtocol,
        decoder_name: *mut *mut EfiGuid,
        number_of_decoder_names: *mut u16,
    ) -> EfiStatus,
    /// Parses image raw bytes and returns dimensions and metadata.
    pub get_image_info: unsafe extern "efiapi" fn(
        this: *const EfiHiiImageDecoderProtocol,
        image: *const c_void,
        size_of_image: UIntN,
        image_info: *mut *mut HiiImageDecoderHeader,
    ) -> EfiStatus,
    /// Decodes image raw bytes into an image output buffer.
    pub decode_image: unsafe extern "efiapi" fn(
        this: *const EfiHiiImageDecoderProtocol,
        image: *const c_void,
        image_raw_data_size: UIntN,
        bitmap: *mut *mut EfiImageOutput,
        transparent: Bool,
    ) -> EfiStatus,
}

/// Safe idiomatic client wrapper for the UEFI HII Image Decoder Protocol.
#[derive(Debug)]
pub struct HiiImageDecoder<'a> {
    protocol: &'a EfiHiiImageDecoderProtocol,
}

impl<'a> HiiImageDecoder<'a> {
    /// HII Image Decoder Protocol `GUID`.
    pub const GUID: EfiGuid = EFI_HII_IMAGE_DECODER_PROTOCOL_GUID;

    /// Creates a new safe wrapper from a reference to the raw protocol.
    #[must_use]
    pub const fn new(protocol: &'a EfiHiiImageDecoderProtocol) -> Self {
        Self { protocol }
    }

    /// Queries image dimensions and color model from raw compressed bytes.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if format is not supported by decoder.
    /// Returns `EfiStatus::BAD_BUFFER_SIZE` if memory allocation fails.
    pub fn get_image_info(&self, image: &[u8]) -> Result<HiiImageDecoderHeader, EfiStatus> {
        let mut info_ptr = core::ptr::null_mut();
        let status = unsafe {
            (self.protocol.get_image_info)(
                self.protocol,
                image.as_ptr().cast::<c_void>(),
                image.len(),
                &raw mut info_ptr,
            )
        };
        if status.is_success() && !info_ptr.is_null() {
            let header = unsafe { *info_ptr };
            Ok(header)
        } else {
            Err(status)
        }
    }
}
