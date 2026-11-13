//! # Capabilities
//!
//! Bounded capability list traversal.
//!
//! Traversal stops on cycles, invalid
//! pointers, or the fixed limit.

//! # Capabilities
//!
//! Conventional PCI and PCI Express Extended Capability list traversal.
//!
//! Traversal stops on cycles, invalid pointers, or the fixed limit.

/// The maximum conventional capabilities per device.
pub const MAX_CAPABILITIES: usize = 32;

/// The maximum PCI Express extended capabilities per device.
pub const MAX_EXT_CAPABILITIES: usize = 64;

/// Base offset of PCI Express Extended Configuration Space.
pub const PCI_EXP_EXT_CONFIG_SPACE_BASE: u16 = 0x100;

/// Limit of PCI Express Extended Configuration Space (4096 bytes).
pub const PCI_EXP_EXT_CONFIG_SPACE_LIMIT: u16 = 0x1000;

// Conventional PCI Capability IDs (8-bit)
/// PCI Power Management Interface.
pub const PCI_CAP_PM: u8 = 0x01;
/// Accelerated Graphics Port.
pub const PCI_CAP_AGP: u8 = 0x02;
/// Vital Product Data.
pub const PCI_CAP_VPD: u8 = 0x03;
/// Slot Identification.
pub const PCI_CAP_SLOT_ID: u8 = 0x04;
/// Message Signalled Interrupts.
pub const PCI_CAP_MSI: u8 = 0x05;
/// `CompactPCI` Hot Swap.
pub const PCI_CAP_HOTSWAP: u8 = 0x06;
/// PCI-X.
pub const PCI_CAP_PCIX: u8 = 0x07;
/// `HyperTransport`.
pub const PCI_CAP_HT: u8 = 0x08;
/// Vendor Specific.
pub const PCI_CAP_VENDOR: u8 = 0x09;
/// Debug Port.
pub const PCI_CAP_DEBUG: u8 = 0x0A;
/// `CompactPCI` Central Resource Control.
pub const PCI_CAP_CPCI_CRC: u8 = 0x0B;
/// PCI Hot-Plug.
pub const PCI_CAP_HOTPLUG: u8 = 0x0C;
/// PCI Bridge Subsystem Vendor ID.
pub const PCI_CAP_BRIDGE_SUBSYS: u8 = 0x0D;
/// AGP 8x.
pub const PCI_CAP_AGP8X: u8 = 0x0E;
/// Secure Device.
pub const PCI_CAP_SECURE_DEV: u8 = 0x0F;
/// PCI Express.
pub const PCI_CAP_EXP: u8 = 0x10;
/// MSI-X.
pub const PCI_CAP_MSIX: u8 = 0x11;
/// SATA Direct Configuration.
pub const PCI_CAP_SATA: u8 = 0x12;
/// Advanced Features.
pub const PCI_CAP_AF: u8 = 0x13;
/// Enhanced Allocation.
pub const PCI_CAP_EA: u8 = 0x14;

// PCI Express Extended Capability IDs (16-bit)
/// Advanced Error Reporting.
pub const PCI_EXP_EXT_CAP_AER: u16 = 0x0001;
/// Virtual Channel.
pub const PCI_EXP_EXT_CAP_VC: u16 = 0x0002;
/// Device Serial Number.
pub const PCI_EXP_EXT_CAP_DSN: u16 = 0x0003;
/// Power Budgeting.
pub const PCI_EXP_EXT_CAP_PWRB: u16 = 0x0004;
/// Root Complex Link Declaration.
pub const PCI_EXP_EXT_CAP_RCLD: u16 = 0x0005;
/// Root Complex Internal Link Control.
pub const PCI_EXP_EXT_CAP_RCILC: u16 = 0x0006;
/// Root Complex Event Collector Endpoint Association.
pub const PCI_EXP_EXT_CAP_RCEC: u16 = 0x0007;
/// Multi-Function Virtual Channel.
pub const PCI_EXP_EXT_CAP_MFVC: u16 = 0x0008;
/// Virtual Channel 9.
pub const PCI_EXP_EXT_CAP_VC9: u16 = 0x0009;
/// Root Complex Register Block Header.
pub const PCI_EXP_EXT_CAP_RCRB: u16 = 0x000A;
/// Vendor-Specific Extended Capability.
pub const PCI_EXP_EXT_CAP_VNDR: u16 = 0x000B;
/// Configuration Access Correlation.
pub const PCI_EXP_EXT_CAP_CAC: u16 = 0x000C;
/// Access Control Services.
pub const PCI_EXP_EXT_CAP_ACS: u16 = 0x000D;
/// Alternative Routing-ID Interpretation.
pub const PCI_EXP_EXT_CAP_ARI: u16 = 0x000E;
/// Address Translation Services.
pub const PCI_EXP_EXT_CAP_ATS: u16 = 0x000F;
/// Single Root I/O Virtualization.
pub const PCI_EXP_EXT_CAP_SRIOV: u16 = 0x0010;
/// Multi-Root I/O Virtualization.
pub const PCI_EXP_EXT_CAP_MRIOV: u16 = 0x0011;
/// Multicast.
pub const PCI_EXP_EXT_CAP_MCAST: u16 = 0x0012;
/// Page Request Interface.
pub const PCI_EXP_EXT_CAP_PRI: u16 = 0x0013;
/// Resizable BAR.
pub const PCI_EXP_EXT_CAP_REBAR: u16 = 0x0015;
/// Dynamic Power Allocation.
pub const PCI_EXP_EXT_CAP_DPA: u16 = 0x0016;
/// TLP Processing Hints (TPH) Requester.
pub const PCI_EXP_EXT_CAP_TPH: u16 = 0x0017;
/// Latency Tolerance Reporting.
pub const PCI_EXP_EXT_CAP_LTR: u16 = 0x0018;
/// Secondary PCI Express.
pub const PCI_EXP_EXT_CAP_SECPCI: u16 = 0x0019;
/// Protocol Multiplexing (PMUX).
pub const PCI_EXP_EXT_CAP_PMUX: u16 = 0x001A;
/// Process Address Space ID (PASID).
pub const PCI_EXP_EXT_CAP_PASID: u16 = 0x001B;
/// LN Requester (LNR).
pub const PCI_EXP_EXT_CAP_LNR: u16 = 0x001C;
/// Downstream Port Containment (DPC).
pub const PCI_EXP_EXT_CAP_DPC: u16 = 0x001D;
/// L1 PM Substates.
pub const PCI_EXP_EXT_CAP_L1SS: u16 = 0x001E;
/// Precision Time Measurement (PTM).
pub const PCI_EXP_EXT_CAP_PTM: u16 = 0x001F;
/// Designated Vendor-Specific Extended Capability (DVSEC).
pub const PCI_EXP_EXT_CAP_DVSEC: u16 = 0x0023;
/// Data Link Feature.
pub const PCI_EXP_EXT_CAP_DLF: u16 = 0x0025;
/// Physical Layer 16.0 GT/s.
pub const PCI_EXP_EXT_CAP_PL_16GT: u16 = 0x0026;
/// Data Object Exchange (DOE).
pub const PCI_EXP_EXT_CAP_DOE: u16 = 0x002E;
/// Integrity and Data Encryption (IDE).
pub const PCI_EXP_EXT_CAP_IDE: u16 = 0x0030;

/// A conventional PCI capability entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Capability {
    /// The capability ID.
    pub id: u8,
    /// The offset in configuration space.
    pub offset: u8,
}

/// Returns the capability ID at an offset in conventional configuration space.
#[must_use]
pub fn capability_at(buffer: &[u8], offset: u8) -> Option<Capability> {
    let index = usize::from(offset);
    let id = *buffer.get(index)?;
    let _next = *buffer.get(index.checked_add(1)?)?;
    Some(Capability { id, offset })
}

/// Iterates conventional capabilities from the start offset.
pub struct CapabilityIter<'a> {
    bytes: &'a [u8],
    offset: u8,
    remaining: usize,
}

impl Iterator for CapabilityIter<'_> {
    type Item = Capability;

    /// Returns the next capability.
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.offset == 0 {
            return None;
        }
        let current = capability_at(self.bytes, self.offset)?;
        self.remaining -= 1;
        let next_offset = *self.bytes.get(usize::from(self.offset).checked_add(1)?)?;
        self.offset = next_offset;
        Some(current)
    }
}

/// Creates a conventional capability iterator.
///
/// Returns `None` when the start offset is zero.
#[must_use]
pub fn capability_iter(bytes: &[u8], start: u8) -> Option<CapabilityIter<'_>> {
    if start == 0 {
        None
    } else {
        Some(CapabilityIter {
            bytes,
            offset: start,
            remaining: MAX_CAPABILITIES,
        })
    }
}

/// Finds a conventional capability by its ID.
#[must_use]
pub fn find_capability(bytes: &[u8], start: u8, cap_id: u8) -> Option<Capability> {
    capability_iter(bytes, start)?.find(|c| c.id == cap_id)
}

/// A PCI Express Extended Capability entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExtendedCapability {
    /// 16-bit extended capability identifier.
    pub id: u16,
    /// 4-bit capability version.
    pub version: u8,
    /// Offset in `PCIe` extended configuration space (`>= 0x100`).
    pub offset: u16,
}

/// Returns the extended capability at an offset in `PCIe` extended configuration space.
#[must_use]
pub fn extended_capability_at(buffer: &[u8], offset: u16) -> Option<ExtendedCapability> {
    if offset < PCI_EXP_EXT_CONFIG_SPACE_BASE || offset % 4 != 0 {
        return None;
    }
    let idx = usize::from(offset);
    if idx.checked_add(4)? > buffer.len() {
        return None;
    }

    let id = u16::from_le_bytes([buffer[idx], buffer[idx + 1]]);
    let version = buffer[idx + 2] & 0x0F;

    // A capability ID of 0 or 0xFFFF represents empty or unmapped ECAM space.
    if id == 0 || id == 0xFFFF {
        return None;
    }

    Some(ExtendedCapability {
        id,
        version,
        offset,
    })
}

/// Iterates PCI Express Extended Capabilities starting at offset `0x100` or custom start.
pub struct ExtendedCapabilityIter<'a> {
    bytes: &'a [u8],
    offset: u16,
    remaining: usize,
}

impl Iterator for ExtendedCapabilityIter<'_> {
    type Item = ExtendedCapability;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0
            || self.offset < PCI_EXP_EXT_CONFIG_SPACE_BASE
            || self.offset >= PCI_EXP_EXT_CONFIG_SPACE_LIMIT
        {
            return None;
        }

        let current = extended_capability_at(self.bytes, self.offset)?;
        self.remaining -= 1;

        let idx = usize::from(self.offset);
        let next_offset = u16::from_le_bytes([self.bytes[idx + 2], self.bytes[idx + 3]]) >> 4;

        self.offset = next_offset;
        Some(current)
    }
}

/// Creates a PCI Express Extended Capability iterator.
///
/// Starts traversing at `start` (defaults to `0x100` if given `0x100`).
#[must_use]
pub fn extended_capability_iter(bytes: &[u8], start: u16) -> Option<ExtendedCapabilityIter<'_>> {
    if !(PCI_EXP_EXT_CONFIG_SPACE_BASE..PCI_EXP_EXT_CONFIG_SPACE_LIMIT).contains(&start)
        || start % 4 != 0
    {
        None
    } else {
        Some(ExtendedCapabilityIter {
            bytes,
            offset: start,
            remaining: MAX_EXT_CAPABILITIES,
        })
    }
}

/// Finds a PCI Express Extended Capability by its 16-bit extended capability ID.
#[must_use]
pub fn find_extended_capability(
    bytes: &[u8],
    start: u16,
    ext_cap_id: u16,
) -> Option<ExtendedCapability> {
    extended_capability_iter(bytes, start)?.find(|c| c.id == ext_cap_id)
}
