//! # ACPI
//!
//! ACPI table parsing from byte slices.
//!
//! Physical addresses are never dereferenced
//! by these parsers. Callers supply mapped
//! memory as slices.

pub mod bgrt;
pub mod checksum;
pub mod discovery;
pub mod dmar;
pub mod dsdt;
pub mod facs;
pub mod fadt;
pub mod gtdt;
pub mod guid;
pub mod header;
pub mod hpet;
pub mod madt;
pub mod mcfg;
pub mod protocol;
pub mod root;
pub mod rsdp;
pub mod slit;
pub mod spcr;
pub mod srat;
pub mod ssdt;
pub mod table;
pub mod types;
pub mod validation;

pub use bgrt::{BgrtInfo, parse_bgrt};
pub use checksum::acpi_checksum_ok;
pub use discovery::find_config_entry;
pub use dmar::{DmarEntry, DmarHeader, DmarIter, dmar_iter, parse_dmar_header};
pub use dsdt::{AmlView, parse_dsdt};
pub use facs::{FacsInfo, parse_facs};
pub use fadt::{FadtInfo, parse_fadt};
pub use gtdt::{GtdtInfo, parse_gtdt};
pub use guid::{ACPI_10_GUID, ACPI_20_GUID, EFI_ACPI_TABLE_PROTOCOL_GUID};
pub use header::{AcpiHeader, parse_header, signature_matches};
pub use hpet::{HpetInfo, parse_hpet};
pub use madt::{MadtHeader, madt_iter, parse_madt_header};
pub use mcfg::{McfgEntry, mcfg_iter};
pub use protocol::{
    AcpiTableProtocol, EfiAcpiTableInstall, EfiAcpiTableProtocol, EfiAcpiTableUninstall,
};
pub use root::{rsdt_iter, validate_root_table, xsdt_iter};
pub use rsdp::{RSDP_SIGNATURE, RsdpInfo, parse_rsdp};
pub use slit::{SlitInfo, SlitIter, parse_slit, slit_iter};
pub use spcr::{SpcrInfo, parse_spcr};
pub use srat::{SratEntry, SratHeader, SratIter, parse_srat_header, srat_iter};
pub use ssdt::parse_ssdt;
pub use types::{
    SIG_BGRT, SIG_DMAR, SIG_DSDT, SIG_FACS, SIG_FADT, SIG_GTDT, SIG_HPET, SIG_MADT, SIG_MCFG,
    SIG_RSDT, SIG_SLIT, SIG_SPCR, SIG_SRAT, SIG_SSDT, SIG_XSDT,
};
