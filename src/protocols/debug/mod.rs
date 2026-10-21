//! # Debug protocols
//!
//! Debug support and debug port access.

pub mod guid;
pub mod protocol;
pub mod types;

pub use guid::{DEBUG_PORT_GUID, DEBUG_SUPPORT_GUID};
pub use protocol::{DebugPort, DebugSupport, EfiDebugPortProtocol, EfiDebugSupportProtocol};
pub use types::InstructionSetArchitecture;
