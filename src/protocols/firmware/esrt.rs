//! # ESRT Management Protocol
//!
//! Provides services to register, update, query, and lock firmware resource entries
//! in the EFI System Resource Table (ESRT) cache repository, as well as synchronizing
//! entries with installed Firmware Management Protocol (FMP) instances.

use crate::base::{EfiGuid, EfiStatus};
pub use crate::tables::esrt::{
    EFI_SYSTEM_RESOURCE_TABLE_FIRMWARE_RESOURCE_VERSION, EFI_SYSTEM_RESOURCE_TABLE_GUID,
    ESRT_FW_TYPE_DEVICEFIRMWARE, ESRT_FW_TYPE_SYSTEMFIRMWARE, ESRT_FW_TYPE_UEFIDRIVER,
    ESRT_FW_TYPE_UNKNOWN, ESRT_MANAGEMENT_PROTOCOL_GUID, EfiSystemResourceEntry,
    EfiSystemResourceTable, EsrtGetEntry, EsrtLockRepository, EsrtManagementProtocol,
    EsrtRegisterEntry, EsrtSyncFmp, EsrtUnregisterEntry, EsrtUpdateEntry,
    LAST_ATTEMPT_STATUS_ERROR_AUTH_ERROR, LAST_ATTEMPT_STATUS_ERROR_INCORRECT_VERSION,
    LAST_ATTEMPT_STATUS_ERROR_INSUFFICIENT_RESOURCES, LAST_ATTEMPT_STATUS_ERROR_INVALID_FORMAT,
    LAST_ATTEMPT_STATUS_ERROR_PWR_EVT_AC, LAST_ATTEMPT_STATUS_ERROR_PWR_EVT_BATT,
    LAST_ATTEMPT_STATUS_ERROR_UNSATISFIED_DEPENDENCIES, LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL,
    LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL_VENDOR_RANGE_MAX,
    LAST_ATTEMPT_STATUS_ERROR_UNSUCCESSFUL_VENDOR_RANGE_MIN, LAST_ATTEMPT_STATUS_SUCCESS,
};

/// High-level client wrapper providing ergonomic and safe access to the
/// ESRT Management Protocol.
#[derive(Debug)]
pub struct EsrtManagement<'a> {
    protocol: &'a mut EsrtManagementProtocol,
}

impl<'a> EsrtManagement<'a> {
    /// Creates a new ESRT management client wrapping the given protocol instance.
    pub fn new(protocol: &'a mut EsrtManagementProtocol) -> Self {
        Self { protocol }
    }

    /// Retrieves an ESRT entry from the cache by its firmware class GUID.
    ///
    /// # Errors
    ///
    /// Returns an error status if no entry matching `fw_class` exists in the ESRT cache.
    pub fn get_entry(&self, fw_class: &EfiGuid) -> Result<EfiSystemResourceEntry, EfiStatus> {
        let mut entry = EfiSystemResourceEntry {
            fw_class: EfiGuid::ZERO,
            fw_type: 0,
            fw_version: 0,
            lowest_supported_fw_version: 0,
            capsule_flags: 0,
            last_attempt_version: 0,
            last_attempt_status: 0,
        };

        let status = unsafe { (self.protocol.get_esrt_entry)(fw_class, &raw mut entry) };
        if status.is_success() {
            Ok(entry)
        } else {
            Err(status)
        }
    }

    /// Updates an existing ESRT entry in the cache repository.
    ///
    /// # Errors
    ///
    /// Returns an error status if the repository is locked or the entry is not found.
    pub fn update_entry(&mut self, entry: &EfiSystemResourceEntry) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.update_esrt_entry)(entry) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Registers a new ESRT entry into the cache repository.
    ///
    /// # Errors
    ///
    /// Returns an error status if the entry already exists or capacity is exhausted.
    pub fn register_entry(&mut self, entry: &EfiSystemResourceEntry) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.register_esrt_entry)(entry) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Unregisters an ESRT entry from the cache repository by firmware class GUID.
    ///
    /// # Errors
    ///
    /// Returns an error status if `fw_class` is not present in the repository.
    pub fn unregister_entry(&mut self, fw_class: &EfiGuid) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.unregister_esrt_entry)(fw_class) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Synchronizes the cached ESRT repository with all installed FMP instances.
    ///
    /// # Errors
    ///
    /// Returns an error status if discovery or synchronization with FMP instances fails.
    pub fn sync_fmp(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.sync_esrt_fmp)() };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Locks the ESRT repository, making it read-only for the remainder of boot.
    ///
    /// # Errors
    ///
    /// Returns an error status if locking the repository fails.
    pub fn lock_repository(&mut self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.protocol.lock_esrt_repository)() };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}
