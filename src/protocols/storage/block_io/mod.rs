//! # Block I/O
//!
//! Block device access.

pub mod media;
pub mod protocol;
pub mod validation;

pub use media::EfiBlockIoMedia;
pub use protocol::{
    BlockIo, BlockIo2, EfiBlockFlush, EfiBlockFlushEx, EfiBlockIo2Protocol, EfiBlockIo2Token,
    EfiBlockIoProtocol, EfiBlockRead, EfiBlockReadEx, EfiBlockReset, EfiBlockResetEx,
    EfiBlockWrite, EfiBlockWriteEx,
};
