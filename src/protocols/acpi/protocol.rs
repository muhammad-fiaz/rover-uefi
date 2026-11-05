//! # ACPI Table protocol
//!
//! Defined in UEFI Specification 2.3+ (Section 21.1 / `MdePkg/Include/Protocol/AcpiTable.h`).
//!
//! Provides the ability for firmware components, drivers, and loaders to
//! install and uninstall ACPI tables dynamically in the platform RSDT/XSDT.

use core::ffi::c_void;

use crate::abi::types::UIntN;
use crate::base::EfiStatus;

/// Installs an ACPI table into the platform ACPI table tree (RSDT/XSDT).
pub type EfiAcpiTableInstall = unsafe extern "efiapi" fn(
    *mut EfiAcpiTableProtocol,
    *const c_void,
    UIntN,
    *mut UIntN,
) -> EfiStatus;

/// Uninstalls an ACPI table from the platform ACPI table tree using its unique key.
pub type EfiAcpiTableUninstall =
    unsafe extern "efiapi" fn(*mut EfiAcpiTableProtocol, UIntN) -> EfiStatus;

/// The raw ACPI Table protocol interface (`EFI_ACPI_TABLE_PROTOCOL`).
#[derive(Debug)]
#[repr(C)]
pub struct EfiAcpiTableProtocol {
    /// Inserts an ACPI table into the platform table list.
    pub install_acpi_table: EfiAcpiTableInstall,
    /// Removes an ACPI table from the platform table list by key.
    pub uninstall_acpi_table: EfiAcpiTableUninstall,
}

/// A safe wrapper for [`EfiAcpiTableProtocol`].
pub struct AcpiTableProtocol<'a> {
    protocol: &'a EfiAcpiTableProtocol,
}

impl<'a> AcpiTableProtocol<'a> {
    /// Creates a borrowed wrapper around the protocol interface.
    #[must_use]
    pub const fn new(protocol: &'a EfiAcpiTableProtocol) -> Self {
        Self { protocol }
    }

    /// Returns a reference to the raw protocol structure.
    #[must_use]
    pub const fn protocol(&self) -> &'a EfiAcpiTableProtocol {
        self.protocol
    }

    /// Installs an ACPI table buffer into the system RSDT/XSDT.
    ///
    /// Returns the unique `TableKey` referencing the installed table.
    ///
    /// # Errors
    ///
    /// Returns [`EfiStatus::INVALID_PARAMETER`] if the buffer is empty or size does not match,
    /// [`EfiStatus::OUT_OF_RESOURCES`] if allocation fails, or
    /// [`EfiStatus::ACCESS_DENIED`] if duplicate table signatures are disallowed.
    pub fn install_acpi_table(&self, buffer: &[u8]) -> Result<UIntN, EfiStatus> {
        if buffer.is_empty() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let mut key: UIntN = 0;
        let status = unsafe {
            (self.protocol.install_acpi_table)(
                ptr,
                buffer.as_ptr().cast(),
                buffer.len(),
                &raw mut key,
            )
        };
        if status.is_error() {
            Err(status)
        } else {
            Ok(key)
        }
    }

    /// Removes an ACPI table from the system RSDT/XSDT using the key returned by [`Self::install_acpi_table`].
    ///
    /// # Errors
    ///
    /// Returns [`EfiStatus::NOT_FOUND`] if the key does not refer to a registered table.
    pub fn uninstall_acpi_table(&self, table_key: UIntN) -> Result<(), EfiStatus> {
        let ptr = core::ptr::from_ref(self.protocol).cast_mut();
        let status = unsafe { (self.protocol.uninstall_acpi_table)(ptr, table_key) };
        if status.is_error() {
            Err(status)
        } else {
            Ok(())
        }
    }
}

impl core::fmt::Debug for AcpiTableProtocol<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AcpiTableProtocol")
            .field("protocol", &self.protocol)
            .finish()
    }
}
