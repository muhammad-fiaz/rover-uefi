//! # Secure Boot
//!
//! Variables, signatures and authenticated-variable helpers.
//.
//! Helpers are read-oriented. This module never modifies Secure
//! Boot variables or keys and never disables Secure Boot.
//! Setting variables requires a caller-managed authorization
//! contract outside this crate's safe surface.

pub mod authentication;
pub mod signature;
pub mod validation;
pub mod variables;

pub use signature::{
    EfiSignatureList, SignatureListInfo, signature_data_iter, signature_list_iter,
};
