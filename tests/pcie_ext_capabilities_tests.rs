//! Unit tests for PCI conventional capabilities and PCI Express extended capabilities traversal.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use rover_uefi::protocols::pci::capabilities::{
    Capability, ExtendedCapability, PCI_CAP_EXP, PCI_CAP_MSI, PCI_CAP_MSIX, PCI_CAP_PM,
    PCI_EXP_EXT_CAP_AER, PCI_EXP_EXT_CAP_DOE, PCI_EXP_EXT_CAP_DSN, PCI_EXP_EXT_CAP_LTR,
    PCI_EXP_EXT_CAP_SRIOV, capability_at, capability_iter, extended_capability_at,
    extended_capability_iter, find_capability, find_extended_capability,
};

#[test]
fn test_conventional_pci_capabilities() {
    let mut config_space = [0u8; 256];

    // Cap at 0x40: ID = PM (0x01), Next = 0x50
    config_space[0x40] = PCI_CAP_PM;
    config_space[0x41] = 0x50;

    // Cap at 0x50: ID = MSI (0x05), Next = 0x70
    config_space[0x50] = PCI_CAP_MSI;
    config_space[0x51] = 0x70;

    // Cap at 0x70: ID = PCIe (0x10), Next = 0x00 (end of list)
    config_space[0x70] = PCI_CAP_EXP;
    config_space[0x71] = 0x00;

    let cap40 = capability_at(&config_space, 0x40).unwrap();
    assert_eq!(cap40.id, PCI_CAP_PM);
    assert_eq!(cap40.offset, 0x40);

    let caps: Vec<Capability> = capability_iter(&config_space, 0x40).unwrap().collect();
    assert_eq!(caps.len(), 3);
    assert_eq!(caps[0].id, PCI_CAP_PM);
    assert_eq!(caps[0].offset, 0x40);
    assert_eq!(caps[1].id, PCI_CAP_MSI);
    assert_eq!(caps[1].offset, 0x50);
    assert_eq!(caps[2].id, PCI_CAP_EXP);
    assert_eq!(caps[2].offset, 0x70);

    assert_eq!(
        find_capability(&config_space, 0x40, PCI_CAP_MSI),
        Some(Capability {
            id: PCI_CAP_MSI,
            offset: 0x50
        })
    );
    assert_eq!(find_capability(&config_space, 0x40, PCI_CAP_MSIX), None);
}

#[test]
fn test_pcie_extended_capabilities() {
    let mut ecam = [0u8; 4096];

    // Helper to write a 32-bit PCIe extended capability header
    let write_ext_cap = |buf: &mut [u8], offset: usize, id: u16, version: u8, next: u16| {
        let raw =
            u32::from(id) | ((u32::from(version) & 0x0F) << 16) | ((u32::from(next) & 0xFFF) << 20);
        buf[offset..offset + 4].copy_from_slice(&raw.to_le_bytes());
    };

    // First ext cap at 0x100: AER (0x0001), version 2, next = 0x150
    write_ext_cap(&mut ecam, 0x100, PCI_EXP_EXT_CAP_AER, 2, 0x150);

    // Second ext cap at 0x150: DSN (0x0003), version 1, next = 0x200
    write_ext_cap(&mut ecam, 0x150, PCI_EXP_EXT_CAP_DSN, 1, 0x200);

    // Third ext cap at 0x200: SRIOV (0x0010), version 1, next = 0x280
    write_ext_cap(&mut ecam, 0x200, PCI_EXP_EXT_CAP_SRIOV, 1, 0x280);

    // Fourth ext cap at 0x280: DOE (0x002E), version 1, next = 0 (end)
    write_ext_cap(&mut ecam, 0x280, PCI_EXP_EXT_CAP_DOE, 1, 0);

    let ext0 = extended_capability_at(&ecam, 0x100).unwrap();
    assert_eq!(ext0.id, PCI_EXP_EXT_CAP_AER);
    assert_eq!(ext0.version, 2);
    assert_eq!(ext0.offset, 0x100);

    let list: Vec<ExtendedCapability> = extended_capability_iter(&ecam, 0x100).unwrap().collect();
    assert_eq!(list.len(), 4);
    assert_eq!(list[0].id, PCI_EXP_EXT_CAP_AER);
    assert_eq!(list[0].offset, 0x100);
    assert_eq!(list[1].id, PCI_EXP_EXT_CAP_DSN);
    assert_eq!(list[1].offset, 0x150);
    assert_eq!(list[2].id, PCI_EXP_EXT_CAP_SRIOV);
    assert_eq!(list[2].offset, 0x200);
    assert_eq!(list[3].id, PCI_EXP_EXT_CAP_DOE);
    assert_eq!(list[3].offset, 0x280);

    assert_eq!(
        find_extended_capability(&ecam, 0x100, PCI_EXP_EXT_CAP_SRIOV),
        Some(ExtendedCapability {
            id: PCI_EXP_EXT_CAP_SRIOV,
            version: 1,
            offset: 0x200
        })
    );

    assert_eq!(
        find_extended_capability(&ecam, 0x100, PCI_EXP_EXT_CAP_LTR),
        None
    );

    // Unaligned offset must fail
    assert_eq!(extended_capability_at(&ecam, 0x101), None);
    // Offset below 0x100 must fail
    assert_eq!(extended_capability_at(&ecam, 0x80), None);
}
