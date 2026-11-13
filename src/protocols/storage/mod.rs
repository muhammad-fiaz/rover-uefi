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
pub mod disk_info;
pub mod disk_io;
pub mod erase_block;
pub mod file_explorer;
pub mod filesystem;
pub mod guid;
pub mod load_file;
pub mod media_sanitize;
pub mod mmc_host;
pub mod nvdimm;
pub mod nvme_passthru;
pub mod partition;
pub mod ram_disk;
pub mod scsi_passthru;
pub mod sd_mmc;
pub mod sd_mmc_override;
pub mod storage_security;
pub mod tape_io;
pub mod ufs;

pub use ata_passthru::{
    ATA_ATAPI_POLICY_PROTOCOL_GUID, ATA_ATAPI_POLICY_VERSION,
    ATA_PASS_THRU_PROTOCOL_GUID as ATA_PASS_THRU_GUID, AtaAtapiPolicy, EfiAtaAtapiPolicyProtocol,
    EfiAtaCommandBlock, EfiAtaPassThruCommandPacket, EfiAtaPassThruMode, EfiAtaPassThruProtocol,
    EfiAtaStatusBlock, devsleep as ata_devsleep, puis as ata_puis,
};
pub use block_io::{BlockIo, BlockIo2, EfiBlockIo2Protocol, EfiBlockIoMedia, EfiBlockIoProtocol};
pub use disk_info::{
    EFI_DISK_INFO_AHCI_INTERFACE_GUID, EFI_DISK_INFO_IDE_INTERFACE_GUID,
    EFI_DISK_INFO_NVME_INTERFACE_GUID, EFI_DISK_INFO_PROTOCOL_GUID,
    EFI_DISK_INFO_SCSI_INTERFACE_GUID, EFI_DISK_INFO_SD_MMC_INTERFACE_GUID,
    EFI_DISK_INFO_UFS_INTERFACE_GUID, EFI_DISK_INFO_USB_INTERFACE_GUID, EfiDiskInfoIdentify,
    EfiDiskInfoInquiry, EfiDiskInfoProtocol, EfiDiskInfoSenseData, EfiDiskInfoWhichIde,
};
pub use disk_io::{DiskIo, DiskIo2, EfiDiskIo2Protocol, EfiDiskIoProtocol};
pub use erase_block::{
    EFI_ERASE_BLOCK_PROTOCOL_GUID, EfiEraseBlockProtocol, EfiEraseBlockToken, EfiEraseBlocks,
};
pub use file_explorer::{
    EfiFileExplorerChooseFile, EfiFileExplorerChooseHandler, EfiFileExplorerProtocol,
    FILE_EXPLORER_PROTOCOL_GUID, FileExplorer,
};
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
pub use media_sanitize::{
    BlockMediaClear, BlockMediaFormat, BlockMediaPurge, MEDIA_SANITIZE_PROTOCOL_GUID,
    MEDIA_SANITIZE_PROTOCOL_REVISION, MediaSanitize, MediaSanitizeProtocol,
    format_action as media_format_action, purge_action as media_purge_action,
};
pub use mmc_host::{
    EMBEDDED_MMC_HOST_PROTOCOL_GUID, EMMC_TIMING_BACKWARD, EMMC_TIMING_HS26, EMMC_TIMING_HS52,
    EMMC_TIMING_HS52_DDR_1V2, EMMC_TIMING_HS52_DDR_1V8, EMMC_TIMING_HS200_SDR_1V2,
    EMMC_TIMING_HS200_SDR_1V8, EMMC_TIMING_HS400_DDR_1V2, EMMC_TIMING_HS400_DDR_1V8, MMC_ACMD41,
    MMC_ACMD51, MMC_CMD_LONG_RESPONSE, MMC_CMD_NO_CRC_RESPONSE, MMC_CMD_WAIT_RESPONSE, MMC_CMD0,
    MMC_CMD1, MMC_CMD2, MMC_CMD3, MMC_CMD5, MMC_CMD6, MMC_CMD7, MMC_CMD8, MMC_CMD9, MMC_CMD11,
    MMC_CMD12, MMC_CMD13, MMC_CMD16, MMC_CMD17, MMC_CMD18, MMC_CMD20, MMC_CMD23, MMC_CMD24,
    MMC_CMD25, MMC_CMD55, MMC_HOST_PROTOCOL_REVISION, MMC_RESPONSE_TYPE_R1, MMC_RESPONSE_TYPE_R1B,
    MMC_RESPONSE_TYPE_R2, MMC_RESPONSE_TYPE_R3, MMC_RESPONSE_TYPE_R6, MMC_RESPONSE_TYPE_R7,
    MmcBuildDevicePath, MmcHost, MmcHostProtocol, MmcIsCardPresent, MmcIsMultiBlock, MmcIsReadOnly,
    MmcNotifyState, MmcReadBlockData, MmcReceiveResponse, MmcSendCommand, MmcSetIos, MmcState,
    MmcWriteBlockData, mmc_cmd,
};
pub use nvdimm::{
    EFI_NVDIMM_LABEL_INDEX_ALIGN, EFI_NVDIMM_LABEL_INDEX_SIG_LEN, EFI_NVDIMM_LABEL_PROTOCOL_GUID,
    EfiNvdimmLabelIndexBlock, EfiNvdimmLabelProtocol, EfiNvdimmLabelStorageInformation,
    EfiNvdimmLabelStorageRead, EfiNvdimmLabelStorageWrite, NvdimmLabel,
};
pub use nvme_passthru::{
    EfiNvmExpressCommand, EfiNvmExpressCompletion, EfiNvmExpressPassThruCommandPacket,
    EfiNvmExpressPassThruMode, EfiNvmExpressPassThruProtocol,
    NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID as NVME_PASS_THRU_GUID, NvmeCdw0,
};
pub use ram_disk::{
    EfiRamDiskProtocol, EfiRamDiskRegister, EfiRamDiskUnregister, PERSISTENT_VIRTUAL_CD_GUID,
    PERSISTENT_VIRTUAL_DISK_GUID, RAM_DISK_PROTOCOL_GUID, RamDiskProtocol, VIRTUAL_CD_GUID,
    VIRTUAL_DISK_GUID,
};
pub use scsi_passthru::{
    EXT_SCSI_PASS_THRU_PROTOCOL_GUID as SCSI_PASS_THRU_GUID, EfiExtScsiPassThruMode,
    EfiExtScsiPassThruProtocol, EfiExtScsiPassThruScsiRequestPacket, TARGET_MAX_BYTES,
};
pub use sd_mmc::{
    EFI_SD_MMC_PASS_THRU_PROTOCOL_GUID, EfiSdMmcCommandBlock, EfiSdMmcPassThruCommandPacket,
    EfiSdMmcPassThruProtocol, EfiSdMmcStatusBlock, SdMmcCommandType, SdMmcPassThruProtocol,
    SdMmcResponseType,
};
pub use sd_mmc_override::{
    EmmcDriverStrengthType, SD_MMC_BUS_WIDTH_IGNORE, SD_MMC_CLOCK_FREQ_IGNORE,
    SD_MMC_DRIVER_STRENGTH_IGNORE, SD_MMC_OVERRIDE_PROTOCOL_GUID, SD_MMC_OVERRIDE_PROTOCOL_VERSION,
    SdDriverStrengthType, SdMmcBusMode, SdMmcCapability, SdMmcDriverStrength, SdMmcNotifyPhase,
    SdMmcOperatingParameters, SdMmcOverride, SdMmcOverrideProtocol, SdMmcPhaseType,
};
pub use storage_security::{
    EFI_STORAGE_SECURITY_COMMAND_PROTOCOL_GUID, EfiStorageSecurityCommandProtocol,
    EfiStorageSecurityReceiveData, EfiStorageSecuritySendData, StorageSecurityCommand,
};
pub use tape_io::{
    EFI_TAPE_IO_PROTOCOL_GUID, EfiTapeHeader, EfiTapeIoProtocol, EfiTapeRead, EfiTapeReset,
    EfiTapeRewind, EfiTapeSpace, EfiTapeWrite, EfiTapeWriteFm, TapeIo,
};
pub use ufs::{
    EFI_UFS_DEVICE_CONFIG_PROTOCOL_GUID, EfiUfsDeviceConfigProtocol, EfiUfsDeviceConfigRwAttribute,
    EfiUfsDeviceConfigRwDescriptor, EfiUfsDeviceConfigRwFlag, UFS_HC_PLATFORM_PROTOCOL_GUID,
    UFS_HOST_CONTROLLER_PROTOCOL_GUID, UfsCardRefClkFreq, UfsDeviceConfig, UfsHcDriverInterface,
    UfsHcInfo, UfsHcPlatform, UfsHcPlatformCallbackPhase, UfsHcPlatformProtocol, UicCommand,
};
