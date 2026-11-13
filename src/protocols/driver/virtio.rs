//! # `VirtIO` Device Protocol and QEMU Virtual Hardware
//!
//! Provides the `VirtIO` device protocol abstraction and standard subsystem IDs
//! defined in the OASIS `VirtIO` Specification and EDK II `OvmfPkg`, plus `VirtIO` GPU
//! 2D and 3D acceleration command structures.

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// `VirtIO` Device Protocol GUID.
pub const VIRTIO_DEVICE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xFA92_0010,
    0x6785,
    0x4941,
    [0xB6, 0xEC, 0x49, 0x8C, 0x57, 0x9F, 0x16, 0x0A],
);

/// `VirtIO` Specification Revision Macro: `Major[31:24].Minor[23:16].Revision[15:0]`.
#[inline]
#[must_use]
pub const fn virtio_spec_revision(major: u8, minor: u8, revision: u16) -> u32 {
    ((major as u32) << 24) | ((minor as u32) << 16) | (revision as u32)
}

/// `VirtIO` Subsystem Device IDs.
pub mod device_id {
    /// Network device.
    pub const NETWORK: u16 = 1;
    /// Block device.
    pub const BLOCK: u16 = 2;
    /// Console device.
    pub const CONSOLE: u16 = 3;
    /// Entropy (RNG) device.
    pub const ENTROPY: u16 = 4;
    /// Memory balloon device.
    pub const BALLOON: u16 = 5;
    /// SCSI host bus adapter.
    pub const SCSI: u16 = 8;
    /// 9P transport.
    pub const NINE_P: u16 = 9;
    /// GPU device.
    pub const GPU: u16 = 16;
    /// Input device.
    pub const INPUT: u16 = 18;
    /// Socket device (vsock).
    pub const SOCKET: u16 = 19;
    /// Crypto device.
    pub const CRYPTO: u16 = 20;
    /// `VirtIO` file system (virtio-fs).
    pub const FS: u16 = 26;
}

pub use device_id as virtio_device_id;
pub use status as virtio_status;

/// `VirtIO` Device Status Bits.
pub mod status {
    /// Guest OS has noticed the device.
    pub const ACKNOWLEDGE: u8 = 1;
    /// Guest OS knows how to drive the device.
    pub const DRIVER: u8 = 2;
    /// Driver is set up and ready to drive the device.
    pub const DRIVER_OK: u8 = 4;
    /// Feature negotiation complete.
    pub const FEATURES_OK: u8 = 8;
    /// Device has experienced an error from which it cannot recover.
    pub const DEVICE_NEEDS_RESET: u8 = 64;
    /// Driver gave up on device.
    pub const FAILED: u8 = 128;
}

/// `VirtIO` Map Operation for Shared Memory Access.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtioMapOperation {
    /// Read operation from system memory by bus master.
    BusMasterRead = 0,
    /// Write operation to system memory by bus master.
    BusMasterWrite = 1,
    /// Common buffer accessible by both host and bus master.
    BusMasterCommonBuffer = 2,
}

// Function pointer types for the full EfiVirtioDeviceProtocol:

/// Gets device features.
pub type VirtioGetDeviceFeatures =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, features: *mut u64) -> EfiStatus;

/// Sets guest features.
pub type VirtioSetGuestFeatures =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, features: u64) -> EfiStatus;

/// Sets queue address.
pub type VirtioSetQueueAddress =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, ring: *const c_void) -> EfiStatus;

/// Sets queue selection.
pub type VirtioSetQueueSel =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, sel: u16) -> EfiStatus;

/// Sets queue notification.
pub type VirtioSetQueueNotify =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, sel: u16) -> EfiStatus;

/// Sets queue alignment.
pub type VirtioSetQueueAlign =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, align: u32) -> EfiStatus;

/// Sets page size.
pub type VirtioSetPageSize =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, page_size: u32) -> EfiStatus;

/// Gets maximum queue size.
pub type VirtioGetQueueNumMax = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    queue_num_max: *mut u16,
) -> EfiStatus;

/// Sets queue size.
pub type VirtioSetQueueNum =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, queue_num: u16) -> EfiStatus;

/// Gets device status.
pub type VirtioGetDeviceStatus = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    device_status: *mut u8,
) -> EfiStatus;

/// Sets device status.
pub type VirtioSetDeviceStatus =
    unsafe extern "efiapi" fn(this: *mut EfiVirtioDeviceProtocol, device_status: u8) -> EfiStatus;

/// Writes device-specific header register.
pub type VirtioDeviceWrite = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    field_offset: usize,
    field_size: usize,
    buffer_size: usize,
    buffer: *const c_void,
) -> EfiStatus;

/// Reads device-specific header register.
pub type VirtioDeviceRead = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    field_offset: usize,
    field_size: usize,
    buffer_size: usize,
    buffer: *mut c_void,
) -> EfiStatus;

/// Allocates shared pages.
pub type VirtioAllocateSharedPages = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    num_pages: usize,
    host_address: *mut *mut c_void,
) -> EfiStatus;

/// Frees shared pages.
pub type VirtioFreeSharedPages = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    num_pages: usize,
    host_address: *mut c_void,
) -> EfiStatus;

/// Maps shared buffer for device DMA.
pub type VirtioMapShared = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    operation: VirtioMapOperation,
    host_address: *mut c_void,
    number_of_bytes: *mut usize,
    device_address: *mut u64,
    mapping: *mut *mut c_void,
) -> EfiStatus;

/// Unmaps shared buffer.
pub type VirtioUnmapShared = unsafe extern "efiapi" fn(
    this: *mut EfiVirtioDeviceProtocol,
    mapping: *mut c_void,
) -> EfiStatus;

/// The complete EDK II / `OvmfPkg` `VirtIO` Device Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiVirtioDeviceProtocol {
    /// Specification revision encoded with `virtio_spec_revision`.
    pub revision: u32,
    /// Subsystem device ID.
    pub subsystem_device_id: i32,
    /// Gets 64-bit device features.
    pub get_device_features: VirtioGetDeviceFeatures,
    /// Sets 64-bit guest features.
    pub set_guest_features: VirtioSetGuestFeatures,
    /// Sets ring queue address.
    pub set_queue_address: VirtioSetQueueAddress,
    /// Sets queue selector.
    pub set_queue_sel: VirtioSetQueueSel,
    /// Notifies device of queue updates.
    pub set_queue_notify: VirtioSetQueueNotify,
    /// Sets queue alignment.
    pub set_queue_align: VirtioSetQueueAlign,
    /// Sets page size.
    pub set_page_size: VirtioSetPageSize,
    /// Gets maximum queue size.
    pub get_queue_num_max: VirtioGetQueueNumMax,
    /// Sets queue size.
    pub set_queue_num: VirtioSetQueueNum,
    /// Gets device status byte.
    pub get_device_status: VirtioGetDeviceStatus,
    /// Sets device status byte.
    pub set_device_status: VirtioSetDeviceStatus,
    /// Writes device-specific registers.
    pub write_device: VirtioDeviceWrite,
    /// Reads device-specific registers.
    pub read_device: VirtioDeviceRead,
    /// Allocates shared pages.
    pub allocate_shared_pages: VirtioAllocateSharedPages,
    /// Frees shared pages.
    pub free_shared_pages: VirtioFreeSharedPages,
    /// Maps buffer for device DMA.
    pub map_shared_buffer: VirtioMapShared,
    /// Unmaps device DMA buffer.
    pub unmap_shared_buffer: VirtioUnmapShared,
}

// -----------------------------------------------------------------------------
// VirtIO GPU Subsystem Definitions (OvmfPkg / VirtioGpu.h)
// -----------------------------------------------------------------------------

/// `VirtIO` GPU Control Queue Index.
pub const VIRTIO_GPU_CONTROL_QUEUE: u16 = 0;

/// `VirtIO` GPU Flag Fence.
pub const VIRTIO_GPU_FLAG_FENCE: u32 = 1;

/// `VirtIO` GPU Control Types (Commands & Responses).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtioGpuCtrlType {
    /// Get display scanout info command.
    CmdGetDisplayInfo = 0x0100,
    /// Create 2D resource command.
    CmdResourceCreate2d = 0x0101,
    /// Unreference 2D resource command.
    CmdResourceUnref = 0x0102,
    /// Set scanout resource command.
    CmdSetScanout = 0x0103,
    /// Flush resource to display command.
    CmdResourceFlush = 0x0104,
    /// Transfer guest RAM buffer to host 2D resource command.
    CmdTransferToHost2d = 0x0105,
    /// Attach backing guest memory to host resource.
    CmdResourceAttachBacking = 0x0106,
    /// Detach backing guest memory from host resource.
    CmdResourceDetachBacking = 0x0107,

    /// Response OK without extra payload.
    RespOkNodata = 0x1100,
    /// Response OK with display info payload.
    RespOkDisplayInfo = 0x1101,
}

/// `VirtIO` GPU Control Header.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuControlHeader {
    /// Command type (`VirtioGpuCtrlType` or response).
    pub ctrl_type: u32,
    /// Flags (e.g. `VIRTIO_GPU_FLAG_FENCE`).
    pub flags: u32,
    /// Fence ID.
    pub fence_id: u64,
    /// Context ID (unused in 2D).
    pub ctx_id: u32,
    /// Padding.
    pub padding: u32,
}

/// `VirtIO` GPU Rectangle.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuRectangle {
    /// X coordinate.
    pub x: u32,
    /// Y coordinate.
    pub y: u32,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

/// `VirtIO` GPU Pixel Formats.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtioGpuFormat {
    /// 32-bit depth, BGRX component order (`DRM_FORMAT_XRGB8888`).
    B8G8R8X8Unorm = 2,
}

/// `VirtIO` GPU Resource Create 2D Command.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuResourceCreate2d {
    /// Control header.
    pub header: VirtioGpuControlHeader,
    /// Non-zero resource ID.
    pub resource_id: u32,
    /// Pixel format (from `VirtioGpuFormat`).
    pub format: u32,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

/// `VirtIO` GPU Resource Unref Command.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuResourceUnref {
    /// Control header.
    pub header: VirtioGpuControlHeader,
    /// Resource ID.
    pub resource_id: u32,
    /// Reserved padding.
    pub padding: u32,
}

/// `VirtIO` GPU Memory Entry for Scatter-Gather lists.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuMemEntry {
    /// Guest physical address.
    pub addr: u64,
    /// Length of contiguous buffer in bytes.
    pub length: u32,
    /// Reserved padding.
    pub padding: u32,
}

/// `VirtIO` GPU Resource Attach Backing Command.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuResourceAttachBacking {
    /// Control header.
    pub header: VirtioGpuControlHeader,
    /// Resource ID.
    pub resource_id: u32,
    /// Number of memory entries (typically 1 for contiguous EFI framebuffer).
    pub nr_entries: u32,
    /// First memory entry.
    pub entry: VirtioGpuMemEntry,
}

/// `VirtIO` GPU Resource Detach Backing Command.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuResourceDetachBacking {
    /// Control header.
    pub header: VirtioGpuControlHeader,
    /// Resource ID.
    pub resource_id: u32,
    /// Reserved padding.
    pub padding: u32,
}

/// `VirtIO` GPU Set Scanout Command.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuSetScanout {
    /// Control header.
    pub header: VirtioGpuControlHeader,
    /// Rectangle viewport.
    pub rectangle: VirtioGpuRectangle,
    /// Scanout display head ID.
    pub scanout_id: u32,
    /// Resource ID bound to scanout.
    pub resource_id: u32,
}

/// `VirtIO` GPU Transfer to Host 2D Command.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuTransferToHost2d {
    /// Control header.
    pub header: VirtioGpuControlHeader,
    /// Destination rectangle.
    pub rectangle: VirtioGpuRectangle,
    /// Byte offset in host resource backing.
    pub offset: u64,
    /// Resource ID.
    pub resource_id: u32,
    /// Reserved padding.
    pub padding: u32,
}

/// `VirtIO` GPU Resource Flush Command.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuResourceFlush {
    /// Control header.
    pub header: VirtioGpuControlHeader,
    /// Rectangle region to refresh.
    pub rectangle: VirtioGpuRectangle,
    /// Resource ID.
    pub resource_id: u32,
    /// Reserved padding.
    pub padding: u32,
}

/// Maximum number of scanouts supported by `VirtIO` GPU display response.
pub const VIRTIO_GPU_MAX_SCANOUTS: usize = 16;

/// Display Mode scanout entry.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtioGpuPmode {
    /// Viewport bounds.
    pub rectangle: VirtioGpuRectangle,
    /// Non-zero if enabled.
    pub enabled: u32,
    /// Mode flags.
    pub flags: u32,
}

/// `VirtIO` GPU Display Information Response.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct VirtioGpuRespDisplayInfo {
    /// Response header (type `RespOkDisplayInfo`).
    pub header: VirtioGpuControlHeader,
    /// Per-scanout display configurations.
    pub pmodes: [VirtioGpuPmode; VIRTIO_GPU_MAX_SCANOUTS],
}
