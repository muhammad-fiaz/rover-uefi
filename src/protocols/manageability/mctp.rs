//! # Management Component Transport Protocol (MCTP)
//!
//! Provides the MCTP protocol interface defined by DMTF DSP0236
//! and UEFI Manageability specifications.

use crate::base::{EfiGuid, EfiStatus};

/// The MCTP Protocol `GUID`.
pub const MCTP_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE934_65C1,
    0x9A31,
    0x4C96,
    [0x92, 0x56, 0x22, 0x0A, 0xE1, 0x80, 0xB4, 0x1B],
);

/// MCTP Message Types.
pub mod message_type {
    /// MCTP Control message.
    pub const CONTROL: u8 = 0x00;
    /// Platform Level Data Model (PLDM).
    pub const PLDM: u8 = 0x01;
    /// Network Controller Sideband Interface (NC-SI).
    pub const NCSI: u8 = 0x02;
    /// Ethernet over MCTP.
    pub const ETHERNET: u8 = 0x03;
    /// NVM Express Management Interface (NVMe-MI).
    pub const NVME_MI: u8 = 0x04;
    /// Security Protocol and Data Model (SPDM).
    pub const SPDM: u8 = 0x05;
    /// Secured MCTP.
    pub const SECURED_MCTP: u8 = 0x06;
    /// Vendor Defined Message (PCI).
    pub const VDM_PCI: u8 = 0x7E;
    /// Vendor Defined Message (IANA).
    pub const VDM_IANA: u8 = 0x7F;
}

pub use message_type as mctp_type;

/// Submits an MCTP message across the transport interface.
pub type EfiMctpSubmitCommand = unsafe extern "efiapi" fn(
    this: *mut EfiMctpProtocol,
    mctp_type: u8,
    source_eid: *const u8,
    dest_eid: *const u8,
    req_integrity_check: bool,
    req_data: *const u8,
    req_size: u32,
    req_timeout: u32,
    resp_data: *mut u8,
    resp_size: *mut u32,
    resp_timeout: u32,
    additional_error: *mut u32,
) -> EfiStatus;

/// The EFI MCTP Protocol.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiMctpProtocol {
    /// Submits an MCTP message and waits for response.
    pub submit_command: EfiMctpSubmitCommand,
}

/// Safe idiomatic client wrapper for [`EfiMctpProtocol`].
pub struct MctpProtocol<'a> {
    proto: &'a mut EfiMctpProtocol,
}

impl<'a> MctpProtocol<'a> {
    /// Wraps a mutable reference to an EFI MCTP protocol instance.
    pub fn new(proto: &'a mut EfiMctpProtocol) -> Self {
        Self { proto }
    }

    /// Submits an MCTP message across the transport interface and awaits a response.
    ///
    /// # Errors
    /// Returns an error status if transmission fails or times out.
    #[allow(clippy::too_many_arguments, clippy::cast_possible_truncation)]
    pub fn submit_message(
        &mut self,
        mctp_type: u8,
        source_eid: Option<u8>,
        dest_eid: Option<u8>,
        integrity_check: bool,
        request: &[u8],
        req_timeout_ms: u32,
        response: &mut [u8],
        resp_timeout_ms: u32,
    ) -> Result<(usize, u32), EfiStatus> {
        let src_val = source_eid.unwrap_or(0);
        let src_ptr = if source_eid.is_some() {
            core::ptr::addr_of!(src_val)
        } else {
            core::ptr::null()
        };

        let dst_val = dest_eid.unwrap_or(0);
        let dst_ptr = if dest_eid.is_some() {
            core::ptr::addr_of!(dst_val)
        } else {
            core::ptr::null()
        };

        let req_ptr = request.as_ptr();
        let req_size = request.len() as u32;
        let resp_ptr = response.as_mut_ptr();
        let mut resp_size = response.len() as u32;
        let mut add_err: u32 = 0;

        let status = unsafe {
            (self.proto.submit_command)(
                self.proto,
                mctp_type,
                src_ptr,
                dst_ptr,
                integrity_check,
                req_ptr,
                req_size,
                req_timeout_ms,
                resp_ptr,
                &raw mut resp_size,
                resp_timeout_ms,
                &raw mut add_err,
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok((resp_size as usize, add_err))
        } else {
            Err(status)
        }
    }
}
