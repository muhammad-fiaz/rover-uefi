// SPDX-License-Identifier: Apache-2.0
//! Unit and integration tests for S3 Save State Protocol and S3 Boot Script engine.

use core::ffi::c_void;
use rover_uefi::abi::types::UIntN;
use rover_uefi::base::{EfiGuid, EfiStatus};
use rover_uefi::protocols::firmware::{
    EFI_BOOT_SCRIPT_DISPATCH_2_OPCODE, EFI_BOOT_SCRIPT_DISPATCH_OPCODE,
    EFI_BOOT_SCRIPT_INFORMATION_OPCODE, EFI_BOOT_SCRIPT_IO_POLL_OPCODE,
    EFI_BOOT_SCRIPT_IO_READ_WRITE_OPCODE, EFI_BOOT_SCRIPT_IO_WRITE_OPCODE,
    EFI_BOOT_SCRIPT_MEM_POLL_OPCODE, EFI_BOOT_SCRIPT_MEM_READ_WRITE_OPCODE,
    EFI_BOOT_SCRIPT_MEM_WRITE_OPCODE, EFI_BOOT_SCRIPT_PCI_CONFIG_POLL_OPCODE,
    EFI_BOOT_SCRIPT_PCI_CONFIG_READ_WRITE_OPCODE, EFI_BOOT_SCRIPT_PCI_CONFIG_WRITE_OPCODE,
    EFI_BOOT_SCRIPT_PCI_CONFIG2_POLL_OPCODE, EFI_BOOT_SCRIPT_PCI_CONFIG2_READ_WRITE_OPCODE,
    EFI_BOOT_SCRIPT_PCI_CONFIG2_WRITE_OPCODE, EFI_BOOT_SCRIPT_SMBUS_EXECUTE_OPCODE,
    EFI_BOOT_SCRIPT_STALL_OPCODE, EFI_S3_SAVE_STATE_PROTOCOL_GUID,
    EFI_S3_SMM_SAVE_STATE_PROTOCOL_GUID, EfiBootScriptWidth, EfiS3BootScriptPosition,
    EfiS3SaveStateProtocol, S3BootScriptRecord, S3BootScriptTable, S3BootScriptTarget, S3SaveState,
    s3_boot_script_pci_address,
};

// --- Mock Target for Boot Script Execution ---
struct MockHardwareTarget {
    io_ports: [u64; 16],
    mem_locations: [u64; 16],
    pci_registers: [u64; 16],
    total_stalled_us: u64,
}

impl MockHardwareTarget {
    const fn new() -> Self {
        Self {
            io_ports: [0; 16],
            mem_locations: [0; 16],
            pci_registers: [0; 16],
            total_stalled_us: 0,
        }
    }
}

impl S3BootScriptTarget for MockHardwareTarget {
    #[allow(clippy::cast_possible_truncation)]
    fn io_write(
        &mut self,
        _width: EfiBootScriptWidth,
        address: u64,
        val: u64,
    ) -> Result<(), EfiStatus> {
        let idx = (address as usize) % self.io_ports.len();
        self.io_ports[idx] = val;
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn io_read(&mut self, _width: EfiBootScriptWidth, address: u64) -> Result<u64, EfiStatus> {
        let idx = (address as usize) % self.io_ports.len();
        Ok(self.io_ports[idx])
    }

    #[allow(clippy::cast_possible_truncation)]
    fn mem_write(
        &mut self,
        _width: EfiBootScriptWidth,
        address: u64,
        val: u64,
    ) -> Result<(), EfiStatus> {
        let idx = ((address >> 2) as usize) % self.mem_locations.len();
        self.mem_locations[idx] = val;
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn mem_read(&mut self, _width: EfiBootScriptWidth, address: u64) -> Result<u64, EfiStatus> {
        let idx = ((address >> 2) as usize) % self.mem_locations.len();
        Ok(self.mem_locations[idx])
    }

    #[allow(clippy::cast_possible_truncation)]
    fn pci_write(
        &mut self,
        _width: EfiBootScriptWidth,
        _segment: u32,
        address: u64,
        val: u64,
    ) -> Result<(), EfiStatus> {
        let idx = (address as usize) % self.pci_registers.len();
        self.pci_registers[idx] = val;
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn pci_read(
        &mut self,
        _width: EfiBootScriptWidth,
        _segment: u32,
        address: u64,
    ) -> Result<u64, EfiStatus> {
        let idx = (address as usize) % self.pci_registers.len();
        Ok(self.pci_registers[idx])
    }

    fn stall(&mut self, microseconds: u64) -> Result<(), EfiStatus> {
        self.total_stalled_us += microseconds;
        Ok(())
    }
}

// --- Mock Protocol Callbacks ---
unsafe extern "efiapi" fn mock_s3_write(
    _this: *const EfiS3SaveStateProtocol,
    _opcode: UIntN,
    _arg1: *const c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_s3_insert(
    _this: *const EfiS3SaveStateProtocol,
    _before_or_after: bool,
    _position: *mut EfiS3BootScriptPosition,
    _opcode: UIntN,
    _arg1: *const c_void,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

static mut DUMMY_POSITION: u64 = 0xABCD_EF01;

unsafe extern "efiapi" fn mock_s3_label(
    _this: *const EfiS3SaveStateProtocol,
    _before_or_after: bool,
    create_if_not_found: bool,
    position: *mut EfiS3BootScriptPosition,
    _label: *const u8,
) -> EfiStatus {
    if !position.is_null() {
        unsafe {
            *position = core::ptr::addr_of_mut!(DUMMY_POSITION).cast::<c_void>();
        }
        EfiStatus::SUCCESS
    } else if create_if_not_found {
        EfiStatus::SUCCESS
    } else {
        EfiStatus::NOT_FOUND
    }
}

unsafe extern "efiapi" fn mock_s3_compare(
    _this: *const EfiS3SaveStateProtocol,
    pos1: EfiS3BootScriptPosition,
    pos2: EfiS3BootScriptPosition,
    relative_position: *mut UIntN,
) -> EfiStatus {
    if pos1.is_null() || pos2.is_null() || relative_position.is_null() {
        return EfiStatus::INVALID_PARAMETER;
    }
    let p1 = pos1 as usize;
    let p2 = pos2 as usize;
    let res = match p1.cmp(&p2) {
        core::cmp::Ordering::Less => usize::MAX,
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
    };
    unsafe {
        *relative_position = res as UIntN;
    }
    EfiStatus::SUCCESS
}

#[test]
fn s3_save_state_guids_and_opcodes() {
    assert_eq!(
        EFI_S3_SAVE_STATE_PROTOCOL_GUID,
        EfiGuid::new(
            0xE857_CAF6,
            0xC046,
            0x45DC,
            [0xBE, 0x3F, 0xEE, 0x07, 0x65, 0xFB, 0xA8, 0x87]
        )
    );
    assert_eq!(
        EFI_S3_SMM_SAVE_STATE_PROTOCOL_GUID,
        EfiGuid::new(
            0x320A_FE62,
            0xE593,
            0x49CB,
            [0xA9, 0xF1, 0xD4, 0xC2, 0xF4, 0xAF, 0x01, 0x4C]
        )
    );

    assert_eq!(EFI_BOOT_SCRIPT_IO_WRITE_OPCODE, 0x00);
    assert_eq!(EFI_BOOT_SCRIPT_IO_READ_WRITE_OPCODE, 0x01);
    assert_eq!(EFI_BOOT_SCRIPT_MEM_WRITE_OPCODE, 0x02);
    assert_eq!(EFI_BOOT_SCRIPT_MEM_READ_WRITE_OPCODE, 0x03);
    assert_eq!(EFI_BOOT_SCRIPT_PCI_CONFIG_WRITE_OPCODE, 0x04);
    assert_eq!(EFI_BOOT_SCRIPT_PCI_CONFIG_READ_WRITE_OPCODE, 0x05);
    assert_eq!(EFI_BOOT_SCRIPT_SMBUS_EXECUTE_OPCODE, 0x06);
    assert_eq!(EFI_BOOT_SCRIPT_STALL_OPCODE, 0x07);
    assert_eq!(EFI_BOOT_SCRIPT_DISPATCH_OPCODE, 0x08);
    assert_eq!(EFI_BOOT_SCRIPT_DISPATCH_2_OPCODE, 0x09);
    assert_eq!(EFI_BOOT_SCRIPT_INFORMATION_OPCODE, 0x0A);
    assert_eq!(EFI_BOOT_SCRIPT_PCI_CONFIG2_WRITE_OPCODE, 0x0B);
    assert_eq!(EFI_BOOT_SCRIPT_PCI_CONFIG2_READ_WRITE_OPCODE, 0x0C);
    assert_eq!(EFI_BOOT_SCRIPT_IO_POLL_OPCODE, 0x0D);
    assert_eq!(EFI_BOOT_SCRIPT_MEM_POLL_OPCODE, 0x0E);
    assert_eq!(EFI_BOOT_SCRIPT_PCI_CONFIG_POLL_OPCODE, 0x0F);
    assert_eq!(EFI_BOOT_SCRIPT_PCI_CONFIG2_POLL_OPCODE, 0x10);

    assert_eq!(EfiBootScriptWidth::Uint8.byte_size(), 1);
    assert_eq!(EfiBootScriptWidth::Uint16.byte_size(), 2);
    assert_eq!(EfiBootScriptWidth::Uint32.byte_size(), 4);
    assert_eq!(EfiBootScriptWidth::Uint64.byte_size(), 8);
    assert_eq!(EfiBootScriptWidth::FifoUint8.byte_size(), 1);
    assert_eq!(EfiBootScriptWidth::FillUint32.byte_size(), 4);
}

#[test]
fn s3_pci_address_encoding() {
    // Bus 0, Dev 2, Func 0, Reg 0x10
    let addr = s3_boot_script_pci_address(0, 2, 0, 0x10);
    assert_eq!(addr, (2 << 16) | 0x10);

    // Extended PCIe register 0x100
    let pcie_addr = s3_boot_script_pci_address(1, 0, 0, 0x100);
    assert_eq!(pcie_addr, (1 << 24) | (0x100u64 << 32));
}

#[test]
fn s3_save_state_client_wrapper() {
    let mock_proto = EfiS3SaveStateProtocol {
        write: mock_s3_write,
        insert: mock_s3_insert,
        label: mock_s3_label,
        compare: mock_s3_compare,
    };

    let client = S3SaveState::new(&mock_proto);
    assert!(!client.raw().is_null());

    let label_pos = client
        .label(false, true, None, "InitChipset")
        .unwrap_or(None);
    assert!(label_pos.is_some());

    let p1 = 0x1000 as *mut c_void;
    let p2 = 0x2000 as *mut c_void;
    let cmp = unsafe { client.compare(p1, p2) }.unwrap_or(0);
    assert_eq!(cmp, -1);

    let cmp_eq = unsafe { client.compare(p1, p1) }.unwrap_or(1);
    assert_eq!(cmp_eq, 0);

    let cmp_gt = unsafe { client.compare(p2, p1) }.unwrap_or(0);
    assert_eq!(cmp_gt, 1);
}

#[test]
fn s3_boot_script_table_execution() {
    let mut table = S3BootScriptTable::<32>::new();
    assert!(table.is_empty());
    assert_eq!(table.len(), 0);

    // 1. Write I/O port 0x80 with POST code 0x5A
    assert!(
        table
            .add(S3BootScriptRecord::IoWrite {
                width: EfiBootScriptWidth::Uint8,
                address: 0x80,
                val: 0x5A,
            })
            .is_ok()
    );

    // 2. Read-modify-write I/O port 0x80 (AND with 0x0F, OR with 0x20) => (0x5A & 0x0F) | 0x20 = 0x2A
    assert!(
        table
            .add(S3BootScriptRecord::IoReadWrite {
                width: EfiBootScriptWidth::Uint8,
                address: 0x80,
                data: 0x20,
                mask: 0x0F,
            })
            .is_ok()
    );

    // 3. Write memory address 0x1000 with 0xDEAD_BEEF
    assert!(
        table
            .add(S3BootScriptRecord::MemWrite {
                width: EfiBootScriptWidth::Uint32,
                address: 0x1000,
                val: 0xDEAD_BEEF,
            })
            .is_ok()
    );

    // 4. Memory read-modify-write on 0x1000
    assert!(
        table
            .add(S3BootScriptRecord::MemReadWrite {
                width: EfiBootScriptWidth::Uint32,
                address: 0x1000,
                data: 0x0000_1234,
                mask: 0xFFFF_0000,
            })
            .is_ok()
    );

    // 5. Write PCI register
    assert!(
        table
            .add(S3BootScriptRecord::PciConfigWrite {
                width: EfiBootScriptWidth::Uint32,
                address: 0x04,
                val: 0x07, // Bus master, memory, I/O enable
            })
            .is_ok()
    );

    // 6. Stall 100 microseconds
    assert!(
        table
            .add(S3BootScriptRecord::Stall { duration_us: 100 })
            .is_ok()
    );

    // 7. Informational marker
    assert!(
        table
            .add(S3BootScriptRecord::Information {
                tag: 1,
                val: 0xCAFE
            })
            .is_ok()
    );

    // 8. Memory poll on 0x1000 for value 0xDEAD_1234
    assert!(
        table
            .add(S3BootScriptRecord::MemPoll {
                width: EfiBootScriptWidth::Uint32,
                address: 0x1000,
                data: 0xDEAD_1234,
                mask: 0xFFFF_FFFF,
                delay_us: 10,
                loop_times: 5,
            })
            .is_ok()
    );

    assert_eq!(table.len(), 8);

    // Execute against mock hardware target
    let mut hw = MockHardwareTarget::new();
    let ops_executed = table.execute(&mut hw).unwrap_or(0);
    assert_eq!(ops_executed, 8);

    // Verify side effects
    let port_idx = (0x80usize) % hw.io_ports.len();
    assert_eq!(hw.io_ports[port_idx], 0x2A);

    let mem_idx = ((0x1000u64 >> 2) as usize) % hw.mem_locations.len();
    assert_eq!(hw.mem_locations[mem_idx], 0xDEAD_1234);

    let pci_idx = (0x04usize) % hw.pci_registers.len();
    assert_eq!(hw.pci_registers[pci_idx], 0x07);

    assert_eq!(hw.total_stalled_us, 100);
}
