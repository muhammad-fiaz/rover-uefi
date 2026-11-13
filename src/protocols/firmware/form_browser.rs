//! # Form Browser 2 and Form Display Engine Protocols
//!
//! Defined by the UEFI Specification (Section 34.2) and Platform Form Display Architecture:
//! - `EFI_FORM_BROWSER2_PROTOCOL`: Primary forms-based user interface browser allowing drivers
//!   to leverage interactive configuration and routing.
//! - Form Display Engine Protocol: Abstracts rendering, question layout, confirmation dialogues,
//!   and user input gathering for HII forms.

use crate::abi::types::{Bool, EFI_FALSE, EFI_TRUE, UIntN};
use crate::base::{EfiGuid, EfiStatus};
use crate::protocols::firmware::hii::EfiHiiHandle;
use core::ffi::c_void;

/// Form Browser 2 Protocol GUID: `{b9d4c360-bcfb-4f9b-9298-53c136982258}`.
pub const EFI_FORM_BROWSER2_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB9D4_C360,
    0xBCFB,
    0x4F9B,
    [0x92, 0x98, 0x53, 0xC1, 0x36, 0x98, 0x22, 0x58],
);

/// Form Display Engine Protocol GUID: `{9bbe29e9-fda1-41ec-ad52-452213742d2e}`.
pub const FORM_DISPLAY_ENGINE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9BBE_29E9,
    0xFDA1,
    0x41EC,
    [0xAD, 0x52, 0x45, 0x22, 0x13, 0x74, 0x2D, 0x2E],
);

/// HII Platform Setup Formset GUID: `{9303b241-0cf2-4931-8e47-ad7e308f7eac}`.
pub const EFI_HII_PLATFORM_SETUP_FORMSET_GUID: EfiGuid = EfiGuid::new(
    0x9303_B241,
    0x0CF2,
    0x4931,
    [0x8E, 0x47, 0xAD, 0x7E, 0x30, 0x8F, 0x7E, 0xAC],
);

/// HII User Credential Formset GUID: `{337f48d3-56f5-4399-cd83-247e812e8078}`.
pub const EFI_HII_USER_CREDENTIAL_FORMSET_GUID: EfiGuid = EfiGuid::new(
    0x337F_48D3,
    0x56F5,
    0x4399,
    [0xCD, 0x83, 0x24, 0x7E, 0x81, 0x2E, 0x80, 0x78],
);

/// Form identifier within an HII form set.
pub type EfiFormId = u16;

/// Screen descriptor designating text coordinate boundaries for browser windows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct EfiScreenDescriptor {
    /// Leftmost column from the left edge of the screen.
    pub left_column: UIntN,
    /// Rightmost column on the screen.
    pub right_column: UIntN,
    /// Top row from the top of the screen.
    pub top_row: UIntN,
    /// Bottom row on the screen.
    pub bottom_row: UIntN,
}

/// Browser action requests recommended by form operations.
pub mod browser_action_request {
    use crate::abi::types::UIntN;

    /// No action required.
    pub const NONE: UIntN = 0;
    /// System reset requested.
    pub const RESET: UIntN = 1;
    /// Submit uncommitted settings.
    pub const SUBMIT: UIntN = 2;
    /// Exit the browser.
    pub const EXIT: UIntN = 3;
    /// Submit form settings and exit.
    pub const FORM_SUBMIT_EXIT: UIntN = 4;
    /// Discard form settings and exit.
    pub const FORM_DISCARD_EXIT: UIntN = 5;
    /// Apply form settings immediately.
    pub const FORM_APPLY: UIntN = 6;
    /// Discard form settings.
    pub const FORM_DISCARD: UIntN = 7;
    /// Reconnect driver controllers.
    pub const RECONNECT: UIntN = 8;
    /// Apply question value change.
    pub const QUESTION_APPLY: UIntN = 9;
}

/// Form display engine status and error codes.
pub mod display_status {
    /// Operation succeeded.
    pub const SUCCESS: u32 = 0x0000_0000;
    /// Generic browser display error flag.
    pub const ERROR_FLAG: u32 = 0x8000_0000;
    /// Form submission failed.
    pub const SUBMIT_FAIL: u32 = ERROR_FLAG | 0x01;
    /// Suppressed by `no_submit_if` statement.
    pub const NO_SUBMIT_IF: u32 = ERROR_FLAG | 0x02;
    /// Target form was not found.
    pub const FORM_NOT_FOUND: u32 = ERROR_FLAG | 0x03;
    /// Form is suppressed.
    pub const FORM_SUPPRESS: u32 = ERROR_FLAG | 0x04;
    /// Required protocol was not found.
    pub const PROTOCOL_NOT_FOUND: u32 = ERROR_FLAG | 0x05;
    /// Inconsistent configuration statement.
    pub const INCONSISTENT_IF: u32 = ERROR_FLAG | 0x06;
    /// Warning statement condition met.
    pub const WARNING_IF: u32 = ERROR_FLAG | 0x07;
    /// Form submission failed due to `no_submit_if`.
    pub const SUBMIT_FAIL_NO_SUBMIT_IF: u32 = ERROR_FLAG | 0x08;
    /// Driver reconnection required.
    pub const RECONNECT_REQUIRED: u32 = ERROR_FLAG | 0x09;
    /// Driver reconnection failed.
    pub const RECONNECT_FAIL: u32 = ERROR_FLAG | 0x0A;
    /// Reconnect required and pending changes saved.
    pub const RECONNECT_SAVE_CHANGES: u32 = ERROR_FLAG | 0x0B;
}

/// Value container for HII statement and question values.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiHiiValue {
    /// HII opcode data type (e.g., boolean, integer, string, buffer).
    pub val_type: u8,
    /// Reserved alignment padding.
    pub reserved: [u8; 7],
    /// Pointer to buffer data if string or buffer.
    pub buffer: *mut u8,
    /// Length of buffer data in bytes.
    pub buffer_len: u16,
    /// Low 64-bit value scalar.
    pub value: u64,
}

impl Default for EfiHiiValue {
    fn default() -> Self {
        Self {
            val_type: 0,
            reserved: [0; 7],
            buffer: core::ptr::null_mut(),
            buffer_len: 0,
            value: 0,
        }
    }
}

/// User input event structure returned by the Form Display Engine.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct UserInput {
    /// Pointer to selected statement opcode.
    pub selected_statement: *mut c_void,
    /// Input value associated with the statement.
    pub input_value: EfiHiiValue,
    /// Triggered action (discard, default, submit, reset, exit).
    pub action: u32,
    /// Default identifier if action is default.
    pub default_id: u16,
}

// ---------------------------------------------------------------------------
// Form Browser 2 Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer to initialize and display HII configuration forms.
pub type EfiSendForm2 = unsafe extern "efiapi" fn(
    this: *const EfiFormBrowser2Protocol,
    handles: *const EfiHiiHandle,
    handle_count: UIntN,
    form_set_guid: *const EfiGuid,
    form_id: EfiFormId,
    screen_dimensions: *const EfiScreenDescriptor,
    action_request: *mut UIntN,
) -> EfiStatus;

/// Function pointer to retrieve or set uncommitted state data.
pub type EfiBrowserCallback2 = unsafe extern "efiapi" fn(
    this: *const EfiFormBrowser2Protocol,
    results_data_size: *mut UIntN,
    results_data: *mut u16,
    retrieve_data: Bool,
    variable_guid: *const EfiGuid,
    variable_name: *const u16,
) -> EfiStatus;

/// Low-level Form Browser 2 Protocol table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiFormBrowser2Protocol {
    /// Displays configuration forms for the specified handles.
    pub send_form: EfiSendForm2,
    /// Retrieves or sets uncommitted browser state data.
    pub browser_callback: EfiBrowserCallback2,
}

// ---------------------------------------------------------------------------
// Form Display Engine Protocol ABI
// ---------------------------------------------------------------------------

/// Function pointer to display one form and return user input.
pub type FormDisplayFn = unsafe extern "efiapi" fn(
    form_data: *const c_void,
    user_input_data: *mut UserInput,
) -> EfiStatus;

/// Function pointer to exit display and restore screen state.
pub type ExitDisplayFn = unsafe extern "efiapi" fn();

/// Function pointer to confirm how to handle modified form data.
pub type ConfirmDataChangeFn = unsafe extern "efiapi" fn() -> UIntN;

/// Low-level Form Display Engine Protocol table.
#[repr(C)]
#[derive(Debug)]
pub struct EfiFormDisplayEngineProtocol {
    /// Renders form data and awaits user input.
    pub form_display: FormDisplayFn,
    /// Cleans up display resources and clears screen.
    pub exit_display: ExitDisplayFn,
    /// Prompts user to confirm saving or discarding changed data.
    pub confirm_data_change: ConfirmDataChangeFn,
}

// ---------------------------------------------------------------------------
// Safe Client Wrappers
// ---------------------------------------------------------------------------

/// Safe client wrapper around `EfiFormBrowser2Protocol`.
#[derive(Debug, Clone, Copy)]
pub struct FormBrowser2 {
    proto: *const EfiFormBrowser2Protocol,
}

impl FormBrowser2 {
    /// Wraps a raw pointer to `EfiFormBrowser2Protocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiFormBrowser2Protocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiFormBrowser2Protocol) -> Self {
        Self { proto }
    }

    /// Initializes and displays configuration forms associated with the given HII handles.
    ///
    /// If `form_set_guid` is `None`, `EFI_HII_PLATFORM_SETUP_FORMSET_GUID` is selected.
    /// If `form_id` is 0, the first enabled form is rendered.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if the form set or form is not found.
    /// Returns `EfiStatus::INVALID_PARAMETER` if handles slice is empty.
    pub fn send_form(
        &self,
        handles: &[EfiHiiHandle],
        form_set_guid: Option<&EfiGuid>,
        form_id: EfiFormId,
        screen_dimensions: Option<&EfiScreenDescriptor>,
    ) -> Result<UIntN, EfiStatus> {
        if handles.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }

        let guid_ptr = form_set_guid.map_or(core::ptr::null(), core::ptr::from_ref);
        let screen_ptr = screen_dimensions.map_or(core::ptr::null(), core::ptr::from_ref);
        let mut action: UIntN = browser_action_request::NONE;

        let status = unsafe {
            ((*self.proto).send_form)(
                self.proto,
                handles.as_ptr(),
                handles.len(),
                guid_ptr,
                form_id,
                screen_ptr,
                core::ptr::addr_of_mut!(action),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(action)
        } else {
            Err(status)
        }
    }

    /// Retrieves uncommitted form state data from the browser.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::BUFFER_TOO_SMALL` if `results_buffer` is insufficient.
    /// Returns `EfiStatus::UNSUPPORTED` if uncommitted state is unavailable.
    pub fn retrieve_uncommitted_data(
        &self,
        variable_guid: Option<&EfiGuid>,
        variable_name: Option<&[u16]>,
        results_buffer: &mut [u16],
    ) -> Result<usize, EfiStatus> {
        let guid_ptr = variable_guid.map_or(core::ptr::null(), core::ptr::from_ref);
        let name_ptr = variable_name.map_or(core::ptr::null(), <[u16]>::as_ptr);
        let mut size_in_bytes: UIntN = core::mem::size_of_val(results_buffer);

        let status = unsafe {
            ((*self.proto).browser_callback)(
                self.proto,
                core::ptr::addr_of_mut!(size_in_bytes),
                results_buffer.as_mut_ptr(),
                EFI_TRUE,
                guid_ptr,
                name_ptr,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(size_in_bytes / core::mem::size_of::<u16>())
        } else {
            Err(status)
        }
    }

    /// Sets uncommitted form state data into the browser.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if uncommitted state modification is not supported.
    pub fn set_uncommitted_data(
        &self,
        variable_guid: Option<&EfiGuid>,
        variable_name: Option<&[u16]>,
        results_data: &mut [u16],
    ) -> Result<(), EfiStatus> {
        let guid_ptr = variable_guid.map_or(core::ptr::null(), core::ptr::from_ref);
        let name_ptr = variable_name.map_or(core::ptr::null(), <[u16]>::as_ptr);
        let mut size_in_bytes: UIntN = core::mem::size_of_val(results_data);

        let status = unsafe {
            ((*self.proto).browser_callback)(
                self.proto,
                core::ptr::addr_of_mut!(size_in_bytes),
                results_data.as_mut_ptr(),
                EFI_FALSE,
                guid_ptr,
                name_ptr,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Safe client wrapper around `EfiFormDisplayEngineProtocol`.
#[derive(Debug, Clone, Copy)]
pub struct FormDisplayEngine {
    proto: *const EfiFormDisplayEngineProtocol,
}

impl FormDisplayEngine {
    /// Wraps a raw pointer to `EfiFormDisplayEngineProtocol`.
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized, valid `EfiFormDisplayEngineProtocol`.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiFormDisplayEngineProtocol) -> Self {
        Self { proto }
    }

    /// Displays one form and returns the collected user input event.
    ///
    /// # Safety
    ///
    /// `form_data` must point to a valid form display engine form descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error status if form rendering fails.
    pub unsafe fn display_form(&self, form_data: *const c_void) -> Result<UserInput, EfiStatus> {
        let mut user_input = UserInput::default();
        let status =
            unsafe { ((*self.proto).form_display)(form_data, core::ptr::addr_of_mut!(user_input)) };
        if status == EfiStatus::SUCCESS {
            Ok(user_input)
        } else {
            Err(status)
        }
    }

    /// Exits display mode and clears the screen to its original state.
    pub fn exit_display(&self) {
        unsafe {
            ((*self.proto).exit_display)();
        }
    }

    /// Prompts the user to confirm whether pending modified form data should be saved or discarded.
    ///
    /// Returns the selected action (e.g. submit, discard, none).
    #[must_use]
    pub fn confirm_data_change(&self) -> UIntN {
        unsafe { ((*self.proto).confirm_data_change)() }
    }
}
