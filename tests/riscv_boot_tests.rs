//! Tests for RISC-V Boot Protocol.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use rover_uefi::abi::types::UIntN;
use rover_uefi::arch::riscv::boot_protocol::{
    RISCV_BOOT_PROTOCOL_GUID, RISCV_BOOT_PROTOCOL_REVISION, RiscvBoot, RiscvBootProtocol,
};
use rover_uefi::base::EfiStatus;

unsafe extern "efiapi" fn mock_get_boot_hart_id(
    _this: *mut RiscvBootProtocol,
    boot_hart_id: *mut UIntN,
) -> EfiStatus {
    unsafe {
        *boot_hart_id = 4;
    }
    EfiStatus::SUCCESS
}

#[test]
fn test_riscv_boot_protocol() {
    assert_eq!(
        RISCV_BOOT_PROTOCOL_GUID.to_string(),
        "ccd15fec-6f73-4eec-8395-3e69e4b940bf"
    );
    assert_eq!(RISCV_BOOT_PROTOCOL_REVISION, 0x0001_0000);

    let mut raw = RiscvBootProtocol {
        revision: RISCV_BOOT_PROTOCOL_REVISION,
        get_boot_hart_id: Some(mock_get_boot_hart_id),
    };

    let mut client = RiscvBoot::new(&mut raw);
    assert_eq!(client.revision(), 0x0001_0000);
    let hart_id = client.boot_hart_id().expect("should return hart id");
    assert_eq!(hart_id, 4);
}
