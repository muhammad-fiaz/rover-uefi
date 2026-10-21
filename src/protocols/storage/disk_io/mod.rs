//! # Disk I/O
//!
//! Byte-oriented disk access.

pub mod protocol;

pub use protocol::{
    DiskIo, DiskIo2, EfiDiskCancelEx, EfiDiskFlushEx, EfiDiskIo2Protocol, EfiDiskIo2Token,
    EfiDiskIoProtocol, EfiDiskRead, EfiDiskReadEx, EfiDiskWrite, EfiDiskWriteEx,
};
