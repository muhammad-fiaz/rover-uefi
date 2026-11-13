//! # Memory Profile Protocol
//!
//! Provides services to track, record, and query memory allocations, pool usage,
//! image footprints, and leak analysis across UEFI DXE and Management Mode (MM/SMM).

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus, MemoryType};
use crate::protocols::device_path::types::EfiDevicePath;
use core::ffi::c_void;

/// Memory Profile Protocol GUID: `{821c9a09-541a-40f6-9f43-0ad193a12cfe}`.
pub const MEMORY_PROFILE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x821C_9A09,
    0x541A,
    0x40F6,
    [0x9F, 0x43, 0x0A, 0xD1, 0x93, 0xA1, 0x2C, 0xFE],
);

/// SMM Memory Profile Protocol GUID: `{e22bbcca-516a-46a8-80e2-6745e83693bd}`.
pub const SMM_MEMORY_PROFILE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE22B_BCCA,
    0x516A,
    0x46A8,
    [0x80, 0xE2, 0x67, 0x45, 0xE8, 0x36, 0x93, 0xBD],
);

/// Context header signature ('MPCT').
pub const MEMORY_PROFILE_CONTEXT_SIGNATURE: u32 = 0x5443_504D;
/// Context structure revision.
pub const MEMORY_PROFILE_CONTEXT_REVISION: u16 = 0x0002;

/// Driver info signature ('MPDI').
pub const MEMORY_PROFILE_DRIVER_INFO_SIGNATURE: u32 = 0x4944_504D;
/// Driver info structure revision.
pub const MEMORY_PROFILE_DRIVER_INFO_REVISION: u16 = 0x0003;

/// Basic memory profile action types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MemoryProfileAction {
    /// Allocate memory pages.
    AllocatePages = 1,
    /// Free memory pages.
    FreePages = 2,
    /// Allocate memory pool.
    AllocatePool = 3,
    /// Free memory pool.
    FreePool = 4,
}

/// Common header for memory profile data structures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct MemoryProfileCommonHeader {
    /// 4-byte ASCII signature.
    pub signature: u32,
    /// Structure length in bytes.
    pub length: u16,
    /// Structure revision.
    pub revision: u16,
}

/// Summary context header for memory profile snapshots.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryProfileContext {
    /// Common header.
    pub header: MemoryProfileCommonHeader,
    /// Current total allocated memory in bytes.
    pub current_total_usage: u64,
    /// Peak total memory usage in bytes.
    pub peak_total_usage: u64,
    /// Total image size of all tracked firmware drivers.
    pub total_image_size: u64,
    /// Number of tracked driver images.
    pub image_count: u32,
    /// Allocation sequence counter.
    pub sequence_count: u32,
}

/// Function pointer for `EfiMemoryProfileGetData`.
pub type EfiMemoryProfileGetData = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryProfileProtocol,
    profile_size: *mut u64,
    profile_buffer: *mut c_void,
) -> EfiStatus;

/// Function pointer for `EfiMemoryProfileRegisterImage`.
pub type EfiMemoryProfileRegisterImage = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryProfileProtocol,
    file_path: *const EfiDevicePath,
    image_base: u64,
    image_size: u64,
    file_type: u8,
) -> EfiStatus;

/// Function pointer for `EfiMemoryProfileUnregisterImage`.
pub type EfiMemoryProfileUnregisterImage = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryProfileProtocol,
    file_path: *const EfiDevicePath,
    image_base: u64,
    image_size: u64,
) -> EfiStatus;

/// Function pointer for `EfiMemoryProfileGetRecordingState`.
pub type EfiMemoryProfileGetRecordingState = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryProfileProtocol,
    recording_state: *mut bool,
) -> EfiStatus;

/// Function pointer for `EfiMemoryProfileSetRecordingState`.
pub type EfiMemoryProfileSetRecordingState = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryProfileProtocol,
    recording_state: bool,
) -> EfiStatus;

/// Function pointer for `EfiMemoryProfileRecord`.
pub type EfiMemoryProfileRecord = unsafe extern "efiapi" fn(
    this: *mut EfiMemoryProfileProtocol,
    caller_address: u64,
    action: MemoryProfileAction,
    memory_type: MemoryType,
    buffer: *mut c_void,
    size: UIntN,
    action_string: *const u8,
) -> EfiStatus;

/// Raw ABI structure for the Memory Profile Protocol.
#[derive(Debug)]
#[repr(C)]
pub struct EfiMemoryProfileProtocol {
    /// Retrieves recorded memory profile data.
    pub get_data: EfiMemoryProfileGetData,
    /// Registers a driver image for memory tracking.
    pub register_image: EfiMemoryProfileRegisterImage,
    /// Unregisters a driver image from memory tracking.
    pub unregister_image: EfiMemoryProfileUnregisterImage,
    /// Gets current recording state (enabled/disabled).
    pub get_recording_state: EfiMemoryProfileGetRecordingState,
    /// Sets recording state (enabled/disabled).
    pub set_recording_state: EfiMemoryProfileSetRecordingState,
    /// Records a memory allocation or deallocation event.
    pub record: EfiMemoryProfileRecord,
}

/// SMM alias for Memory Profile Protocol.
pub type EfiSmmMemoryProfileProtocol = EfiMemoryProfileProtocol;

/// Safe client wrapper around `EfiMemoryProfileProtocol`.
pub struct MemoryProfile<'a> {
    proto: &'a mut EfiMemoryProfileProtocol,
}

impl<'a> MemoryProfile<'a> {
    /// Creates a new `MemoryProfile` wrapper.
    #[must_use]
    pub fn new(proto: &'a mut EfiMemoryProfileProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiMemoryProfileProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiMemoryProfileProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *mut EfiMemoryProfileProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *mut EfiMemoryProfileProtocol {
        core::ptr::from_ref(self.proto).cast_mut()
    }

    /// Retrieves memory profile data into `buffer`.
    ///
    /// Returns the number of bytes written to `buffer`.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL` if `buffer` is too small, or an error status.
    pub fn get_data(&mut self, buffer: &mut [u8]) -> Result<usize, EfiStatus> {
        let mut size = buffer.len() as u64;
        let status = unsafe {
            (self.proto.get_data)(
                self.proto,
                core::ptr::from_mut(&mut size),
                buffer.as_mut_ptr().cast::<c_void>(),
            )
        };
        if status == EfiStatus::SUCCESS {
            usize::try_from(size).map_err(|_| EfiStatus::BAD_BUFFER_SIZE)
        } else {
            Err(status)
        }
    }

    /// Returns whether memory profile recording is currently active.
    ///
    /// # Errors
    ///
    /// Returns an error status if query fails.
    pub fn is_recording_enabled(&mut self) -> Result<bool, EfiStatus> {
        let mut state = false;
        let status = unsafe {
            (self.proto.get_recording_state)(self.proto, core::ptr::from_mut(&mut state))
        };
        if status == EfiStatus::SUCCESS {
            Ok(state)
        } else {
            Err(status)
        }
    }

    /// Enables or disables memory profile recording.
    ///
    /// # Errors
    ///
    /// Returns an error status if setting state fails.
    pub fn set_recording_enabled(&mut self, enabled: bool) -> Result<(), EfiStatus> {
        let status = unsafe { (self.proto.set_recording_state)(self.proto, enabled) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers a driver image for memory profiling.
    ///
    /// # Errors
    ///
    /// Returns an error status if registration fails.
    pub fn register_image(
        &mut self,
        file_path: Option<&EfiDevicePath>,
        image_base: u64,
        image_size: u64,
        file_type: u8,
    ) -> Result<(), EfiStatus> {
        let path_ptr = match file_path {
            Some(p) => core::ptr::from_ref(p),
            None => core::ptr::null(),
        };
        let status = unsafe {
            (self.proto.register_image)(self.proto, path_ptr, image_base, image_size, file_type)
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unregisters a driver image from memory profiling.
    ///
    /// # Errors
    ///
    /// Returns an error status if unregistration fails.
    pub fn unregister_image(
        &mut self,
        file_path: Option<&EfiDevicePath>,
        image_base: u64,
        image_size: u64,
    ) -> Result<(), EfiStatus> {
        let path_ptr = match file_path {
            Some(p) => core::ptr::from_ref(p),
            None => core::ptr::null(),
        };
        let status =
            unsafe { (self.proto.unregister_image)(self.proto, path_ptr, image_base, image_size) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Records a memory allocation or deallocation event.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `buffer` is valid or null according to the memory action
    /// being recorded.
    ///
    /// # Errors
    ///
    /// Returns an error status if recording fails or out of resources.
    pub unsafe fn record(
        &mut self,
        caller_address: u64,
        action: MemoryProfileAction,
        memory_type: MemoryType,
        buffer: *mut c_void,
        size: usize,
        action_string: Option<&str>,
    ) -> Result<(), EfiStatus> {
        let mut str_buf = [0u8; 64];
        let str_ptr = match action_string {
            Some(s) => {
                let bytes = s.as_bytes();
                if bytes.len() >= str_buf.len() {
                    return Err(EfiStatus::INVALID_PARAMETER);
                }
                str_buf[..bytes.len()].copy_from_slice(bytes);
                str_buf.as_ptr()
            }
            None => core::ptr::null(),
        };

        let status = unsafe {
            (self.proto.record)(
                self.proto,
                caller_address,
                action,
                memory_type,
                buffer,
                size as UIntN,
                str_ptr,
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
