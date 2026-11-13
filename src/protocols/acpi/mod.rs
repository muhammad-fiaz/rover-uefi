//! # ACPI
//!
//! ACPI table parsing from byte slices.
//!
//! Physical addresses are never dereferenced
//! by these parsers. Callers supply mapped
//! memory as slices.

pub mod bert;
pub mod bgrt;
pub mod cedt;
pub mod checksum;
pub mod dbg2;
pub mod discovery;
pub mod dmar;
pub mod dsdt;
pub mod facs;
pub mod fadt;
pub mod fpdt;
pub mod gtdt;
pub mod guid;
pub mod header;
pub mod hest;
pub mod hpet;
pub mod iort;
pub mod madt;
pub mod mcfg;
pub mod nfit;
pub mod pptt;
pub mod prmt;
pub mod protocol;
pub mod root;
pub mod rsdp;
pub mod slit;
pub mod spcr;
pub mod srat;
pub mod ssdt;
pub mod table;
pub mod tpm2;
pub mod types;
pub mod validation;

pub use bert::{BertInfo, GenericErrorStatusBlock, parse_bert, parse_generic_error_status};
pub use bgrt::{BgrtInfo, parse_bgrt};
pub use cedt::{
    CedtEntry, CedtHeader, CxlFixedMemoryWindowStructure, CxlHostBridgeStructure, cedt_iter,
    parse_cedt_header, parse_cfmws, parse_chbs,
};
pub use checksum::acpi_checksum_ok;
pub use dbg2::{Dbg2Device, Dbg2Header, Dbg2Iter, dbg2_iter, parse_dbg2_header};
pub use discovery::find_config_entry;
pub use dmar::{DmarEntry, DmarHeader, DmarIter, dmar_iter, parse_dmar_header};
pub use dsdt::{AmlView, parse_dsdt};
pub use facs::{FacsInfo, parse_facs};
pub use fadt::{FadtInfo, parse_fadt};
pub use fpdt::{
    FPDT_RECORD_TYPE_DUAL_GUID_STRING_EVENT, FPDT_RECORD_TYPE_DYNAMIC_STRING_EVENT,
    FPDT_RECORD_TYPE_FIRMWARE_BASIC_BOOT_POINTER, FPDT_RECORD_TYPE_GUID_EVENT,
    FPDT_RECORD_TYPE_GUID_QWORD_EVENT, FPDT_RECORD_TYPE_GUID_QWORD_STRING_EVENT,
    FPDT_RECORD_TYPE_S3_PERFORMANCE_TABLE_POINTER, FPDT_RUNTIME_RECORD_TYPE_FIRMWARE_BASIC_BOOT,
    FPDT_RUNTIME_RECORD_TYPE_S3_RESUME, FPDT_RUNTIME_RECORD_TYPE_S3_SUSPEND, FpdtBasicBootRecord,
    FpdtGuidEventRecord, FpdtParsedRecord, FpdtRecordHeader, FpdtRecordIter, FpdtS3ResumeRecord,
    FpdtS3SuspendRecord, FpdtStringEventRecord, FpdtTableHeader, FpdtTableInfo, SIG_FBPT, SIG_S3PT,
    parse_fpdt_root,
};
pub use gtdt::{GtdtInfo, parse_gtdt};
pub use guid::{ACPI_10_GUID, ACPI_20_GUID, EFI_ACPI_TABLE_PROTOCOL_GUID};
pub use header::{AcpiHeader, parse_header, signature_matches};
pub use hest::{HestEntry, HestHeader, HestIter, hest_iter, parse_hest_header};
pub use hpet::{HpetInfo, parse_hpet};
pub use iort::{IortHeader, IortIter, IortNode, iort_iter, parse_iort_header};
pub use madt::{MadtHeader, madt_iter, parse_madt_header};
pub use mcfg::{McfgEntry, mcfg_iter};
pub use nfit::{
    NfitEntry, NfitHeader, NfitIter, SpaRangeStructure, nfit_iter, parse_nfit_header,
    parse_spa_range,
};
pub use pptt::{
    CacheTypeStructure, PpttEntry, PpttHeader, PpttIter, ProcessorHierarchyNode, parse_cache_node,
    parse_pptt_header, parse_processor_node, pptt_iter,
};
pub use prmt::{
    PrmHandler, PrmModule, PrmModuleIter, PrmtHeader, parse_prmt_header, prm_module_iter,
};
pub use protocol::{
    AcpiTableProtocol, EfiAcpiTableInstall, EfiAcpiTableProtocol, EfiAcpiTableUninstall,
};
pub use root::{rsdt_iter, validate_root_table, xsdt_iter};
pub use rsdp::{RSDP_SIGNATURE, RsdpInfo, parse_rsdp};
pub use slit::{SlitInfo, SlitIter, parse_slit, slit_iter};
pub use spcr::{SpcrInfo, parse_spcr};
pub use srat::{SratEntry, SratHeader, SratIter, parse_srat_header, srat_iter};
pub use ssdt::parse_ssdt;
pub use tpm2::{Tpm2Info, parse_tpm2};
pub use types::{
    SIG_BERT, SIG_BGRT, SIG_CEDT, SIG_DBG2, SIG_DMAR, SIG_DSDT, SIG_FACS, SIG_FADT, SIG_FPDT,
    SIG_GTDT, SIG_HEST, SIG_HPET, SIG_IORT, SIG_MADT, SIG_MCFG, SIG_NFIT, SIG_PPTT, SIG_PRMT,
    SIG_RSDT, SIG_SLIT, SIG_SPCR, SIG_SRAT, SIG_SSDT, SIG_TPM2, SIG_XSDT,
};
