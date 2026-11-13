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
pub mod boot_logo;
pub mod edid;
pub mod framebuffer;
pub mod guid;
pub mod mode;
pub mod pixel;
pub mod platform_logo;
pub mod protocol;
pub mod validation;

pub use blt::{BltOperation, BltRectangle, BltRequest};
pub use boot_logo::{
    BOOT_LOGO2_PROTOCOL_GUID, BootLogo2, BootLogo2Protocol, GetBootLogo2, SetBootLogo2,
};
pub use edid::{
    EFI_EDID_ACTIVE_PROTOCOL_GUID, EFI_EDID_DISCOVERED_PROTOCOL_GUID,
    EFI_EDID_OVERRIDE_DONT_OVERRIDE, EFI_EDID_OVERRIDE_ENABLE_HOT_PLUG,
    EFI_EDID_OVERRIDE_PROTOCOL_GUID, EdidInfo, EfiEdidOverrideGetEdid, EfiEdidOverrideProtocol,
    parse_edid_block,
};
pub use framebuffer::FramebufferInfo;
pub use guid::{EDID_ACTIVE_GUID, EDID_DISCOVERED_GUID, EDID_OVERRIDE_GUID, GRAPHICS_OUTPUT_GUID};
pub use mode::{EfiGraphicsOutputModeInformation, EfiGraphicsOutputProtocolMode};
pub use pixel::{EfiGraphicsOutputBltPixel, EfiPixelBitmask, PixelFormat};
pub use platform_logo::{
    EfiPlatformLogoGetImage, EfiPlatformLogoProtocol, PLATFORM_LOGO_PROTOCOL_GUID, PlatformLogo,
    PlatformLogoImage, PlatformLogoImageType,
};
pub use protocol::{
    Edid, EfiEdidActiveProtocol, EfiEdidDiscoveredProtocol, EfiGraphicsOutputProtocol,
    GraphicsOutput,
};
