//! # Security architecture
//!
//! Authentication callbacks.

pub mod protocol;
pub mod security2;

pub use protocol::{EfiSecurityArchProtocol, EfiSecurityFileAuthenticationState, SecurityArch};
pub use security2::{EfiSecurity2ArchProtocol, EfiSecurity2FileAuthentication, Security2Arch};
