//! # Manageability Protocols
//!
//! Protocols for system manageability and Baseboard Management Controller (BMC) interfaces.

pub mod ipmi;
pub mod ipmi_blob;
pub mod ipmi_fru;
pub mod ipmi_sdr;
pub mod ipmi_sel;
pub mod mctp;
pub mod pldm;
pub mod pldm_smbios;

pub use ipmi::{
    EfiIpmiProtocol, EfiIpmiSubmitCommand, IPMI_PROTOCOL_GUID, SMM_IPMI_PROTOCOL_GUID, app_cmd,
    completion_code, kcs, net_fn,
};
pub use ipmi_blob::{
    BLOB_MAX_DATA_PER_PACKET, BlobStatResult, IPMI_BLOB_TRANSFER_PROTOCOL_GUID,
    IPMI_OEM_BLOB_TRANSFER_CMD, IpmiBlobClose, IpmiBlobCommit, IpmiBlobDelete, IpmiBlobEnumerate,
    IpmiBlobGetCount, IpmiBlobOpen, IpmiBlobRead, IpmiBlobSessionStat, IpmiBlobStat,
    IpmiBlobTransfer, IpmiBlobTransferProtocol, IpmiBlobWrite, IpmiBlobWriteMeta, blob_flags,
};
pub use ipmi_fru::{
    FRU_AREA_OFFSET_MULTIPLIER, FruBoardInfo, FruCommonHeader, FruProductInfo, fru_checksum_ok,
    parse_fru_board_info, parse_fru_common_header, parse_fru_product_info,
};
pub use ipmi_sdr::{
    SdrFullSensor, SdrHeader, convert_sensor_reading, parse_full_sensor, parse_sdr_header,
    sdr_record_type,
};
pub use ipmi_sel::{
    SEL_RECORD_LENGTH, SelIter, SelSystemEventRecord, parse_sel_record, sel_iter, sel_record_type,
    sensor_type,
};
pub use mctp::{
    EfiMctpProtocol, EfiMctpSubmitCommand, MCTP_PROTOCOL_GUID, message_type as mctp_type,
};
pub use pldm::{
    EfiPldmProtocol, EfiPldmSubmitCommand, PLDM_PROTOCOL_GUID, base_cmd as pldm_base_cmd, pldm_type,
};
pub use pldm_smbios::{
    PLDM_SMBIOS_TRANSFER_PROTOCOL_GUID, PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION,
    PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION_MAJOR, PLDM_SMBIOS_TRANSFER_PROTOCOL_VERSION_MINOR,
    PLDM_TERMINUS_ID_UNASSIGNED, PldmGetSmbiosStructureByHandle, PldmGetSmbiosStructureByType,
    PldmGetSmbiosStructureTable, PldmGetSmbiosStructureTableMetadata, PldmSetSmbiosStructureTable,
    PldmSetSmbiosStructureTableMetadata, PldmSetSmbiosTransferTerminusId,
    PldmSmbiosStructureTableMetadata, PldmSmbiosTransfer, PldmSmbiosTransferProtocol,
    PldmSmbiosTransferProtocolFunctions, PldmSmbiosTransferProtocolV1_0,
    command_code as pldm_smbios_cmd, completion_code as pldm_smbios_cc,
    transfer_flag as pldm_smbios_transfer_flag, transfer_op as pldm_smbios_transfer_op,
};
