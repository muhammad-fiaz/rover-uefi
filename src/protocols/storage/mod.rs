//! # Storage
//!
//! Block, disk, and file system protocols.
//!
//! Discovery uses `locate_protocol` for each
//! `GUID`. Physical devices, logical
//! partitions, volumes, and file handles are
//! distinct firmware objects.

pub mod ata_passthru;
pub mod block_io;
pub mod disk_io;
pub mod filesystem;
pub mod guid;
pub mod load_file;
pub mod nvme_passthru;
pub mod partition;
pub mod scsi_passthru;

pub use ata_passthru::{
    ATA_PASS_THRU_PROTOCOL_GUID as ATA_PASS_THRU_GUID, EfiAtaCommandBlock,
    EfiAtaPassThruCommandPacket, EfiAtaPassThruMode, EfiAtaPassThruProtocol, EfiAtaStatusBlock,
};
pub use block_io::{BlockIo, BlockIo2, EfiBlockIo2Protocol, EfiBlockIoMedia, EfiBlockIoProtocol};
pub use disk_io::{DiskIo, DiskIo2, EfiDiskIo2Protocol, EfiDiskIoProtocol};
pub use filesystem::{
    EfiFileProtocol, EfiSimpleFileSystemProtocol, FileProtocol, OwnedFile, SimpleFileSystem,
};
pub use guid::{
    ATA_PASS_THRU_PROTOCOL_GUID, BLOCK_IO_GUID, BLOCK_IO2_GUID, DISK_IO_GUID, DISK_IO2_GUID,
    EXT_SCSI_PASS_THRU_PROTOCOL_GUID, FILE_INFO_ID, FILE_SYSTEM_INFO_ID,
    FILE_SYSTEM_VOLUME_LABEL_ID, NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID, SIMPLE_FILE_SYSTEM_GUID,
};
pub use load_file::{
    EfiLoadFile, EfiLoadFile2, EfiLoadFile2Protocol, EfiLoadFileProtocol, LOAD_FILE_PROTOCOL_GUID,
    LOAD_FILE2_PROTOCOL_GUID, LoadFile, LoadFile2,
};
pub use nvme_passthru::{
    EfiNvmExpressCommand, EfiNvmExpressCompletion, EfiNvmExpressPassThruCommandPacket,
    EfiNvmExpressPassThruMode, EfiNvmExpressPassThruProtocol,
    NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID as NVME_PASS_THRU_GUID, NvmeCdw0,
};
pub use scsi_passthru::{
    EXT_SCSI_PASS_THRU_PROTOCOL_GUID as SCSI_PASS_THRU_GUID, EfiExtScsiPassThruMode,
    EfiExtScsiPassThruProtocol, EfiExtScsiPassThruScsiRequestPacket, TARGET_MAX_BYTES,
};
