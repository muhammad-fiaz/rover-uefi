//! # File system
//!
//! Simple file system and file protocols.

pub mod directory;
pub mod fat;
pub mod file;
pub mod info;
pub mod metadata;
pub mod protocol;
pub mod validation;

pub use file::{
    EfiFileClose, EfiFileDelete, EfiFileFlush, EfiFileFlushEx, EfiFileGetInfo, EfiFileGetPosition,
    EfiFileOpen, EfiFileOpenEx, EfiFileProtocol, EfiFileRead, EfiFileReadEx, EfiFileSetInfo,
    EfiFileSetPosition, EfiFileWrite, EfiFileWriteEx, FILE_ARCHIVE, FILE_DIRECTORY, FILE_HIDDEN,
    FILE_MODE_CREATE, FILE_MODE_READ, FILE_MODE_WRITE, FILE_READ_ONLY, FILE_SYSTEM, FileProtocol,
    OwnedFile,
};
pub use info::EfiFileIoToken;
pub use metadata::{
    EfiFileInfo, EfiFileSystemInfo, EfiFileSystemVolumeLabel, SIZE_OF_EFI_FILE_INFO,
    SIZE_OF_EFI_FILE_SYSTEM_INFO, SIZE_OF_EFI_FILE_SYSTEM_VOLUME_LABEL, validate_file_info,
    validate_file_system_info,
};
pub use protocol::{EfiSimpleFileSystemOpenVolume, EfiSimpleFileSystemProtocol, SimpleFileSystem};
