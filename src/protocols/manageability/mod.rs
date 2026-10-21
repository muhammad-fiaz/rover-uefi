//! # Manageability Protocols
//!
//! Protocols for system manageability and Baseboard Management Controller (BMC) interfaces.

pub mod ipmi;
pub mod mctp;
pub mod pldm;

pub use ipmi::{
    EfiIpmiProtocol, EfiIpmiSubmitCommand, IPMI_PROTOCOL_GUID, SMM_IPMI_PROTOCOL_GUID, app_cmd,
    completion_code, kcs, net_fn,
};
pub use mctp::{
    EfiMctpProtocol, EfiMctpSubmitCommand, MCTP_PROTOCOL_GUID, message_type as mctp_type,
};
pub use pldm::{
    EfiPldmProtocol, EfiPldmSubmitCommand, PLDM_PROTOCOL_GUID, base_cmd as pldm_base_cmd, pldm_type,
};
