//! # Graphics
//!
//! Graphics output and EDID protocols.
//!
//! The graphics wrapper borrows the firmware
//! protocol instance. Mode information is
//! copied before the firmware buffer is freed.
//! Framebuffer metadata describes physical
//! memory that callers must map through the
//! operating system.

pub mod blt;
pub mod framebuffer;
pub mod guid;
pub mod mode;
pub mod pixel;
pub mod protocol;
pub mod validation;

pub use blt::{BltOperation, BltRectangle, BltRequest};
pub use framebuffer::FramebufferInfo;
pub use guid::{EDID_ACTIVE_GUID, EDID_DISCOVERED_GUID, GRAPHICS_OUTPUT_GUID};
pub use mode::{EfiGraphicsOutputModeInformation, EfiGraphicsOutputProtocolMode};
pub use pixel::{EfiGraphicsOutputBltPixel, EfiPixelBitmask, PixelFormat};
pub use protocol::{
    Edid, EfiEdidActiveProtocol, EfiEdidDiscoveredProtocol, EfiGraphicsOutputProtocol,
    GraphicsOutput,
};
