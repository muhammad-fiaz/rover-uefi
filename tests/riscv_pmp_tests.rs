//! Unit tests for RISC-V Physical Memory Protection (PMP) CSRs and NAPOT range encoding/decoding.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use rover_uefi::arch::riscv::csr::{
    CSR_MCOUNTEREN, CSR_MEDELEG, CSR_MIDELEG, CSR_PMPADDR0, CSR_PMPADDR15, CSR_PMPCFG0,
    CSR_PMPCFG3, PmpAddressMode, PmpConfig, decode_pmp_napot, encode_pmp_napot,
};

#[test]
fn test_riscv_pmp_csr_constants() {
    assert_eq!(CSR_MEDELEG, 0x302);
    assert_eq!(CSR_MIDELEG, 0x303);
    assert_eq!(CSR_MCOUNTEREN, 0x306);
    assert_eq!(CSR_PMPCFG0, 0x3A0);
    assert_eq!(CSR_PMPCFG3, 0x3A3);
    assert_eq!(CSR_PMPADDR0, 0x3B0);
    assert_eq!(CSR_PMPADDR15, 0x3BF);
}

#[test]
fn test_riscv_pmp_config_builder_and_decoder() {
    // R + W, Mode NAPOT, Locked
    let cfg = PmpConfig::new(true, true, false, PmpAddressMode::Napot, true);
    assert!(cfg.is_readable());
    assert!(cfg.is_writable());
    assert!(!cfg.is_executable());
    assert!(cfg.is_locked());
    assert_eq!(cfg.address_mode(), PmpAddressMode::Napot);

    let raw = cfg.to_raw();
    assert_eq!(raw, PmpConfig::R | PmpConfig::W | (3 << 3) | PmpConfig::L);

    // Read only, TOR, not locked
    let cfg2 = PmpConfig::new(true, false, false, PmpAddressMode::Tor, false);
    assert!(cfg2.is_readable());
    assert!(!cfg2.is_writable());
    assert!(!cfg2.is_executable());
    assert!(!cfg2.is_locked());
    assert_eq!(cfg2.address_mode(), PmpAddressMode::Tor);
}

#[test]
fn test_riscv_pmp_napot_encoding_and_decoding() {
    // 64 KiB region at 0x8000_0000
    let base: u64 = 0x8000_0000;
    let size: u64 = 64 * 1024; // 65536 bytes = 2^16

    let encoded = encode_pmp_napot(base, size).expect("should encode");
    let (decoded_base, decoded_size) = decode_pmp_napot(encoded);

    assert_eq!(decoded_base, base);
    assert_eq!(decoded_size, size);

    // 8-byte minimum NAPOT region at 0x1000
    let base_min: u64 = 0x1000;
    let size_min: u64 = 8;
    let encoded_min = encode_pmp_napot(base_min, size_min).expect("should encode 8 bytes");
    let (d_base, d_size) = decode_pmp_napot(encoded_min);
    assert_eq!(d_base, base_min);
    assert_eq!(d_size, size_min);

    // Non-power-of-two size must fail
    assert!(encode_pmp_napot(0x1000, 1000).is_none());
    // Size < 8 must fail
    assert!(encode_pmp_napot(0x1000, 4).is_none());
    // Unaligned base must fail
    assert!(encode_pmp_napot(0x1004, 64).is_none());
}
