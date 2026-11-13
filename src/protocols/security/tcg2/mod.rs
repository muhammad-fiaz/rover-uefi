//! # TCG2
//!
//! Platform measurement operations.

pub mod capability;
pub mod command;
pub mod event;
pub mod event_log;
pub mod pcr;
pub mod protocol;

pub use command::{
    TPM_CC_GET_RANDOM, TPM_CC_HASH_SEQUENCE_START, TPM_CC_PCR_EXTEND, TPM_CC_PCR_READ,
    TPM_CC_SELF_TEST, TPM_CC_SEQUENCE_COMPLETE, TPM_CC_SEQUENCE_UPDATE, TPM_CC_SHUTDOWN,
    TPM_CC_STARTUP, TPM_RC_BAD_TAG, TPM_RC_FAILURE, TPM_RC_INITIALIZE, TPM_RC_LOCALITY,
    TPM_RC_SUCCESS, TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS, TPM_SU_CLEAR, TPM_SU_STATE,
    TpmCommandError, TpmCommandHeader, TpmResponseHeader, build_get_random_command,
    build_self_test_command, build_startup_command, parse_response_header,
};
pub use protocol::{EfiTcg2Protocol, Tcg2Protocol};
