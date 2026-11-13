//! # S3 Save State Protocol and S3 Boot Script Engine
//!
//! Defined by the UEFI Platform Initialization (PI) Specification Volume 5: Standards.
//!
//! Provides services to record and replay low-level I/O, memory, and PCI configuration
//! operations required during system resume from ACPI S3 sleep state.

use crate::abi::types::UIntN;
use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// PI S3 Save State Protocol GUID: `{e857caf6-c046-45dc-be3f-ee0765fba887}`.
pub const EFI_S3_SAVE_STATE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE857_CAF6,
    0xC046,
    0x45DC,
    [0xBE, 0x3F, 0xEE, 0x07, 0x65, 0xFB, 0xA8, 0x87],
);

/// PI S3 SMM Save State Protocol GUID: `{320afe62-e593-49cb-a9f1-d4c2f4af014c}`.
pub const EFI_S3_SMM_SAVE_STATE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x320A_FE62,
    0xE593,
    0x49CB,
    [0xA9, 0xF1, 0xD4, 0xC2, 0xF4, 0xAF, 0x01, 0x4C],
);

// --- Standard Boot Script Opcode Definitions ---
/// Boot script I/O write opcode.
pub const EFI_BOOT_SCRIPT_IO_WRITE_OPCODE: usize = 0x00;
/// Boot script I/O read-modify-write opcode.
pub const EFI_BOOT_SCRIPT_IO_READ_WRITE_OPCODE: usize = 0x01;
/// Boot script memory write opcode.
pub const EFI_BOOT_SCRIPT_MEM_WRITE_OPCODE: usize = 0x02;
/// Boot script memory read-modify-write opcode.
pub const EFI_BOOT_SCRIPT_MEM_READ_WRITE_OPCODE: usize = 0x03;
/// Boot script PCI configuration write opcode.
pub const EFI_BOOT_SCRIPT_PCI_CONFIG_WRITE_OPCODE: usize = 0x04;
/// Boot script PCI configuration read-modify-write opcode.
pub const EFI_BOOT_SCRIPT_PCI_CONFIG_READ_WRITE_OPCODE: usize = 0x05;
/// Boot script `SMBus` execute opcode.
pub const EFI_BOOT_SCRIPT_SMBUS_EXECUTE_OPCODE: usize = 0x06;
/// Boot script stall/delay opcode.
pub const EFI_BOOT_SCRIPT_STALL_OPCODE: usize = 0x07;
/// Boot script dispatch opcode.
pub const EFI_BOOT_SCRIPT_DISPATCH_OPCODE: usize = 0x08;
/// Boot script dispatch with context opcode.
pub const EFI_BOOT_SCRIPT_DISPATCH_2_OPCODE: usize = 0x09;
/// Boot script informational record opcode.
pub const EFI_BOOT_SCRIPT_INFORMATION_OPCODE: usize = 0x0A;
/// Boot script PCI configuration 2 (with segment) write opcode.
pub const EFI_BOOT_SCRIPT_PCI_CONFIG2_WRITE_OPCODE: usize = 0x0B;
/// Boot script PCI configuration 2 (with segment) read-modify-write opcode.
pub const EFI_BOOT_SCRIPT_PCI_CONFIG2_READ_WRITE_OPCODE: usize = 0x0C;
/// Boot script I/O poll opcode.
pub const EFI_BOOT_SCRIPT_IO_POLL_OPCODE: usize = 0x0D;
/// Boot script memory poll opcode.
pub const EFI_BOOT_SCRIPT_MEM_POLL_OPCODE: usize = 0x0E;
/// Boot script PCI configuration poll opcode.
pub const EFI_BOOT_SCRIPT_PCI_CONFIG_POLL_OPCODE: usize = 0x0F;
/// Boot script PCI configuration 2 poll opcode.
pub const EFI_BOOT_SCRIPT_PCI_CONFIG2_POLL_OPCODE: usize = 0x10;

/// Data width for boot script operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiBootScriptWidth {
    /// 8-bit unsigned integer width.
    Uint8 = 0,
    /// 16-bit unsigned integer width.
    Uint16 = 1,
    /// 32-bit unsigned integer width.
    Uint32 = 2,
    /// 64-bit unsigned integer width.
    Uint64 = 3,
    /// 8-bit FIFO transfer width.
    FifoUint8 = 4,
    /// 16-bit FIFO transfer width.
    FifoUint16 = 5,
    /// 32-bit FIFO transfer width.
    FifoUint32 = 6,
    /// 64-bit FIFO transfer width.
    FifoUint64 = 7,
    /// 8-bit fill operation width.
    FillUint8 = 8,
    /// 16-bit fill operation width.
    FillUint16 = 9,
    /// 32-bit fill operation width.
    FillUint32 = 10,
    /// 64-bit fill operation width.
    FillUint64 = 11,
    /// Maximum width enumeration delimiter.
    Maximum = 12,
}

impl EfiBootScriptWidth {
    /// Returns the byte size of this width.
    #[must_use]
    pub const fn byte_size(self) -> usize {
        match self {
            Self::Uint8 | Self::FifoUint8 | Self::FillUint8 => 1,
            Self::Uint16 | Self::FifoUint16 | Self::FillUint16 => 2,
            Self::Uint32 | Self::FifoUint32 | Self::FillUint32 => 4,
            Self::Uint64 | Self::FifoUint64 | Self::FillUint64 => 8,
            Self::Maximum => 0,
        }
    }
}

/// Helper function to encode a PCI configuration address for S3 boot scripts.
#[inline]
#[must_use]
pub const fn pci_address(bus: u32, dev: u32, func: u32, reg: u32) -> u64 {
    let base = ((bus as u64) << 24) | ((dev as u64) << 16) | ((func as u64) << 8);
    if reg < 256 {
        base | (reg as u64)
    } else {
        base | ((reg as u64) << 32)
    }
}

/// Opaque position pointer within a boot script table.
pub type EfiS3BootScriptPosition = *mut c_void;

/// Raw ABI write function pointer for `EFI_S3_SAVE_STATE_PROTOCOL`.
pub type EfiS3SaveStateWrite = unsafe extern "efiapi" fn(
    this: *const EfiS3SaveStateProtocol,
    opcode: UIntN,
    arg1: *const c_void,
) -> EfiStatus;

/// Raw ABI insert function pointer for `EFI_S3_SAVE_STATE_PROTOCOL`.
pub type EfiS3SaveStateInsert = unsafe extern "efiapi" fn(
    this: *const EfiS3SaveStateProtocol,
    before_or_after: bool,
    position: *mut EfiS3BootScriptPosition,
    opcode: UIntN,
    arg1: *const c_void,
) -> EfiStatus;

/// Raw ABI label function pointer for `EFI_S3_SAVE_STATE_PROTOCOL`.
pub type EfiS3SaveStateLabel = unsafe extern "efiapi" fn(
    this: *const EfiS3SaveStateProtocol,
    before_or_after: bool,
    create_if_not_found: bool,
    position: *mut EfiS3BootScriptPosition,
    label: *const u8,
) -> EfiStatus;

/// Raw ABI compare function pointer for `EFI_S3_SAVE_STATE_PROTOCOL`.
pub type EfiS3SaveStateCompare = unsafe extern "efiapi" fn(
    this: *const EfiS3SaveStateProtocol,
    position1: EfiS3BootScriptPosition,
    position2: EfiS3BootScriptPosition,
    relative_position: *mut UIntN,
) -> EfiStatus;

/// The raw ABI structure for `EFI_S3_SAVE_STATE_PROTOCOL`.
#[derive(Debug)]
#[repr(C)]
pub struct EfiS3SaveStateProtocol {
    /// Stores an operation code in the S3 boot script.
    pub write: EfiS3SaveStateWrite,
    /// Inserts an operation code at a specific position.
    pub insert: EfiS3SaveStateInsert,
    /// Finds or creates a label in the boot script.
    pub label: EfiS3SaveStateLabel,
    /// Compares two positions in the boot script table.
    pub compare: EfiS3SaveStateCompare,
}

/// SMM alias for S3 Save State Protocol.
pub type EfiS3SmmSaveStateProtocol = EfiS3SaveStateProtocol;

/// Safe client wrapper around `EFI_S3_SAVE_STATE_PROTOCOL`.
pub struct S3SaveState<'a> {
    proto: &'a EfiS3SaveStateProtocol,
}

impl<'a> S3SaveState<'a> {
    /// Creates a new `S3SaveState` client wrapper.
    #[must_use]
    pub const fn new(proto: &'a EfiS3SaveStateProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiS3SaveStateProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiS3SaveStateProtocol`.
    #[must_use]
    pub unsafe fn from_raw(proto: *const EfiS3SaveStateProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &*proto },
            })
        }
    }

    /// Returns a raw pointer to the underlying protocol.
    #[must_use]
    pub fn raw(&self) -> *const EfiS3SaveStateProtocol {
        core::ptr::from_ref(self.proto)
    }

    /// Finds or creates a label in the boot script.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if label doesn't exist and `create_if_not_found` is false,
    /// or an error status on allocation failure.
    pub fn label(
        &self,
        before_or_after: bool,
        create_if_not_found: bool,
        start_position: Option<EfiS3BootScriptPosition>,
        label_text: &str,
    ) -> Result<Option<EfiS3BootScriptPosition>, EfiStatus> {
        let mut pos: EfiS3BootScriptPosition = start_position.unwrap_or(core::ptr::null_mut());
        let mut label_buf = [0u8; 64];
        let bytes = label_text.as_bytes();
        if bytes.len() >= label_buf.len() {
            return Err(EfiStatus::INVALID_PARAMETER);
        }
        label_buf[..bytes.len()].copy_from_slice(bytes);

        let status = unsafe {
            (self.proto.label)(
                self.proto,
                before_or_after,
                create_if_not_found,
                core::ptr::from_mut(&mut pos),
                label_buf.as_ptr(),
            )
        };

        if status == EfiStatus::SUCCESS {
            Ok(Some(pos))
        } else {
            Err(status)
        }
    }

    /// Compares two positions in the boot script table.
    ///
    /// Returns:
    /// - -1 if `pos1` is before `pos2`
    /// - 0 if `pos1` equals `pos2`
    /// - 1 if `pos1` is after `pos2`
    ///
    /// # Safety
    ///
    /// `pos1` and `pos2` must be valid `EfiS3BootScriptPosition` pointers.
    ///
    /// # Errors
    ///
    /// Returns an error status if positions are invalid or comparison fails.
    #[allow(
        clippy::not_unsafe_ptr_arg_deref,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap
    )]
    pub unsafe fn compare(
        &self,
        pos1: EfiS3BootScriptPosition,
        pos2: EfiS3BootScriptPosition,
    ) -> Result<i32, EfiStatus> {
        let mut rel_pos: UIntN = 0;
        let status = unsafe {
            (self.proto.compare)(self.proto, pos1, pos2, core::ptr::from_mut(&mut rel_pos))
        };

        if status == EfiStatus::SUCCESS {
            Ok(rel_pos as i32)
        } else {
            Err(status)
        }
    }
}

// =========================================================================
// Native Rust S3 Boot Script Record Representation & Interpreter
// =========================================================================

/// Strongly-typed S3 boot script record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3BootScriptRecord {
    /// I/O write operation.
    IoWrite {
        /// Data width.
        width: EfiBootScriptWidth,
        /// I/O port address.
        address: u64,
        /// Data value.
        val: u64,
    },
    /// I/O read-modify-write operation (`new = (old & mask) | data`).
    IoReadWrite {
        /// Data width.
        width: EfiBootScriptWidth,
        /// I/O port address.
        address: u64,
        /// Data OR-mask.
        data: u64,
        /// Data AND-mask.
        mask: u64,
    },
    /// Memory write operation.
    MemWrite {
        /// Data width.
        width: EfiBootScriptWidth,
        /// Physical memory address.
        address: u64,
        /// Data value.
        val: u64,
    },
    /// Memory read-modify-write operation (`new = (old & mask) | data`).
    MemReadWrite {
        /// Data width.
        width: EfiBootScriptWidth,
        /// Physical memory address.
        address: u64,
        /// Data OR-mask.
        data: u64,
        /// Data AND-mask.
        mask: u64,
    },
    /// PCI configuration write operation.
    PciConfigWrite {
        /// Data width.
        width: EfiBootScriptWidth,
        /// Encoded PCI address.
        address: u64,
        /// Data value.
        val: u64,
    },
    /// PCI configuration read-modify-write operation.
    PciConfigReadWrite {
        /// Data width.
        width: EfiBootScriptWidth,
        /// Encoded PCI address.
        address: u64,
        /// Data OR-mask.
        data: u64,
        /// Data AND-mask.
        mask: u64,
    },
    /// PCI configuration 2 (segment-aware) write operation.
    PciConfig2Write {
        /// Data width.
        width: EfiBootScriptWidth,
        /// PCI segment number.
        segment: u32,
        /// Encoded PCI address.
        address: u64,
        /// Data value.
        val: u64,
    },
    /// PCI configuration 2 (segment-aware) read-modify-write operation.
    PciConfig2ReadWrite {
        /// Data width.
        width: EfiBootScriptWidth,
        /// PCI segment number.
        segment: u32,
        /// Encoded PCI address.
        address: u64,
        /// Data OR-mask.
        data: u64,
        /// Data AND-mask.
        mask: u64,
    },
    /// Execution stall (delay in microseconds).
    Stall {
        /// Microseconds to delay.
        duration_us: u64,
    },
    /// Dispatches a C/firmware function entry point.
    Dispatch {
        /// Code entry point address.
        entry_point: usize,
    },
    /// Dispatches a C/firmware function entry point with context.
    Dispatch2 {
        /// Code entry point address.
        entry_point: usize,
        /// Context parameter.
        context: usize,
    },
    /// Informational or telemetry marker.
    Information {
        /// Informational identifier or tag.
        tag: u32,
        /// Informational data value.
        val: u64,
    },
    /// I/O polling operation.
    IoPoll {
        /// Data width.
        width: EfiBootScriptWidth,
        /// I/O port address.
        address: u64,
        /// Target data value after masking.
        data: u64,
        /// Polling bitmask.
        mask: u64,
        /// Delay in microseconds between attempts.
        delay_us: u64,
    },
    /// Memory polling operation.
    MemPoll {
        /// Data width.
        width: EfiBootScriptWidth,
        /// Physical memory address.
        address: u64,
        /// Target data value after masking.
        data: u64,
        /// Polling bitmask.
        mask: u64,
        /// Delay in microseconds between attempts.
        delay_us: u64,
        /// Maximum loop iterations.
        loop_times: u64,
    },
    /// PCI configuration polling operation.
    PciConfigPoll {
        /// Data width.
        width: EfiBootScriptWidth,
        /// Encoded PCI address.
        address: u64,
        /// Target data value after masking.
        data: u64,
        /// Polling bitmask.
        mask: u64,
        /// Delay in microseconds between attempts.
        delay_us: u64,
    },
    /// PCI configuration 2 polling operation.
    PciConfig2Poll {
        /// Data width.
        width: EfiBootScriptWidth,
        /// PCI segment.
        segment: u32,
        /// Encoded PCI address.
        address: u64,
        /// Target data value after masking.
        data: u64,
        /// Polling bitmask.
        mask: u64,
        /// Delay in microseconds between attempts.
        delay_us: u64,
    },
}

/// Abstract hardware target interface used during S3 boot script execution/replay.
pub trait S3BootScriptTarget {
    /// Writes an I/O port value.
    ///
    /// # Errors
    ///
    /// Returns an error status if the I/O write fails.
    fn io_write(
        &mut self,
        width: EfiBootScriptWidth,
        address: u64,
        val: u64,
    ) -> Result<(), EfiStatus>;

    /// Reads an I/O port value.
    ///
    /// # Errors
    ///
    /// Returns an error status if the I/O read fails.
    fn io_read(&mut self, width: EfiBootScriptWidth, address: u64) -> Result<u64, EfiStatus>;

    /// Writes a physical memory address.
    ///
    /// # Errors
    ///
    /// Returns an error status if the memory write fails.
    fn mem_write(
        &mut self,
        width: EfiBootScriptWidth,
        address: u64,
        val: u64,
    ) -> Result<(), EfiStatus>;

    /// Reads a physical memory address.
    ///
    /// # Errors
    ///
    /// Returns an error status if the memory read fails.
    fn mem_read(&mut self, width: EfiBootScriptWidth, address: u64) -> Result<u64, EfiStatus>;

    /// Writes a PCI configuration register.
    ///
    /// # Errors
    ///
    /// Returns an error status if the PCI write fails.
    fn pci_write(
        &mut self,
        width: EfiBootScriptWidth,
        segment: u32,
        address: u64,
        val: u64,
    ) -> Result<(), EfiStatus>;

    /// Reads a PCI configuration register.
    ///
    /// # Errors
    ///
    /// Returns an error status if the PCI read fails.
    fn pci_read(
        &mut self,
        width: EfiBootScriptWidth,
        segment: u32,
        address: u64,
    ) -> Result<u64, EfiStatus>;

    /// Stalls execution for the specified microseconds.
    ///
    /// # Errors
    ///
    /// Returns an error status if the stall fails.
    fn stall(&mut self, microseconds: u64) -> Result<(), EfiStatus>;
}

/// In-memory S3 boot script table with fixed capacity for `#![no_std]` environments.
pub struct S3BootScriptTable<const CAPACITY: usize> {
    records: [Option<S3BootScriptRecord>; CAPACITY],
    count: usize,
}

impl<const CAPACITY: usize> S3BootScriptTable<CAPACITY> {
    /// Creates an empty boot script table.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            records: [None; CAPACITY],
            count: 0,
        }
    }

    /// Returns the number of recorded boot script operations.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Returns whether the table contains no recorded operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clears all recorded operations.
    pub fn clear(&mut self) {
        self.records.fill(None);
        self.count = 0;
    }

    /// Appends a new operation to the boot script.
    ///
    /// # Errors
    ///
    /// Returns `OUT_OF_RESOURCES` if capacity is reached.
    pub fn add(&mut self, record: S3BootScriptRecord) -> Result<usize, EfiStatus> {
        if self.count >= CAPACITY {
            return Err(EfiStatus::OUT_OF_RESOURCES);
        }
        let index = self.count;
        self.records[index] = Some(record);
        self.count += 1;
        Ok(index)
    }

    /// Retrieves an operation record by index.
    #[must_use]
    pub const fn get(&self, index: usize) -> Option<&S3BootScriptRecord> {
        if index < self.count {
            self.records[index].as_ref()
        } else {
            None
        }
    }

    /// Replays and executes all recorded boot script operations against the target.
    ///
    /// Returns the total number of operations successfully executed.
    ///
    /// # Errors
    ///
    /// Returns an error status if any boot script operation fails on the target.
    #[allow(clippy::too_many_lines)]
    pub fn execute<T: S3BootScriptTarget>(&self, target: &mut T) -> Result<usize, EfiStatus> {
        let mut executed = 0;
        for record in self.records[..self.count].iter().flatten() {
            match *record {
                S3BootScriptRecord::IoWrite {
                    width,
                    address,
                    val,
                } => {
                    target.io_write(width, address, val)?;
                }
                S3BootScriptRecord::IoReadWrite {
                    width,
                    address,
                    data,
                    mask,
                } => {
                    let old = target.io_read(width, address)?;
                    let new = (old & mask) | data;
                    target.io_write(width, address, new)?;
                }
                S3BootScriptRecord::MemWrite {
                    width,
                    address,
                    val,
                } => {
                    target.mem_write(width, address, val)?;
                }
                S3BootScriptRecord::MemReadWrite {
                    width,
                    address,
                    data,
                    mask,
                } => {
                    let old = target.mem_read(width, address)?;
                    let new = (old & mask) | data;
                    target.mem_write(width, address, new)?;
                }
                S3BootScriptRecord::PciConfigWrite {
                    width,
                    address,
                    val,
                } => {
                    target.pci_write(width, 0, address, val)?;
                }
                S3BootScriptRecord::PciConfigReadWrite {
                    width,
                    address,
                    data,
                    mask,
                } => {
                    let old = target.pci_read(width, 0, address)?;
                    let new = (old & mask) | data;
                    target.pci_write(width, 0, address, new)?;
                }
                S3BootScriptRecord::PciConfig2Write {
                    width,
                    segment,
                    address,
                    val,
                } => {
                    target.pci_write(width, segment, address, val)?;
                }
                S3BootScriptRecord::PciConfig2ReadWrite {
                    width,
                    segment,
                    address,
                    data,
                    mask,
                } => {
                    let old = target.pci_read(width, segment, address)?;
                    let new = (old & mask) | data;
                    target.pci_write(width, segment, address, new)?;
                }
                S3BootScriptRecord::Stall { duration_us } => {
                    target.stall(duration_us)?;
                }
                S3BootScriptRecord::Dispatch { entry_point } => {
                    if entry_point != 0 {
                        let func: unsafe extern "efiapi" fn() -> EfiStatus =
                            unsafe { core::mem::transmute(entry_point) };
                        let status = unsafe { func() };
                        if status != EfiStatus::SUCCESS {
                            return Err(status);
                        }
                    }
                }
                S3BootScriptRecord::Dispatch2 {
                    entry_point,
                    context,
                } => {
                    if entry_point != 0 {
                        let func: unsafe extern "efiapi" fn(ctx: usize) -> EfiStatus =
                            unsafe { core::mem::transmute(entry_point) };
                        let status = unsafe { func(context) };
                        if status != EfiStatus::SUCCESS {
                            return Err(status);
                        }
                    }
                }
                S3BootScriptRecord::Information { .. } => {
                    // Informational marker, no-op for execution.
                }
                S3BootScriptRecord::IoPoll {
                    width,
                    address,
                    data,
                    mask,
                    delay_us,
                } => {
                    let mut attempts = 0;
                    loop {
                        let val = target.io_read(width, address)?;
                        if (val & mask) == data {
                            break;
                        }
                        attempts += 1;
                        if attempts > 10_000 {
                            return Err(EfiStatus::TIMEOUT);
                        }
                        if delay_us > 0 {
                            target.stall(delay_us)?;
                        }
                    }
                }
                S3BootScriptRecord::MemPoll {
                    width,
                    address,
                    data,
                    mask,
                    delay_us,
                    loop_times,
                } => {
                    let max_loops = if loop_times == 0 { 10_000 } else { loop_times };
                    let mut attempts = 0;
                    loop {
                        let val = target.mem_read(width, address)?;
                        if (val & mask) == data {
                            break;
                        }
                        attempts += 1;
                        if attempts >= max_loops {
                            return Err(EfiStatus::TIMEOUT);
                        }
                        if delay_us > 0 {
                            target.stall(delay_us)?;
                        }
                    }
                }
                S3BootScriptRecord::PciConfigPoll {
                    width,
                    address,
                    data,
                    mask,
                    delay_us,
                } => {
                    let mut attempts = 0;
                    loop {
                        let val = target.pci_read(width, 0, address)?;
                        if (val & mask) == data {
                            break;
                        }
                        attempts += 1;
                        if attempts > 10_000 {
                            return Err(EfiStatus::TIMEOUT);
                        }
                        if delay_us > 0 {
                            target.stall(delay_us)?;
                        }
                    }
                }
                S3BootScriptRecord::PciConfig2Poll {
                    width,
                    segment,
                    address,
                    data,
                    mask,
                    delay_us,
                } => {
                    let mut attempts = 0;
                    loop {
                        let val = target.pci_read(width, segment, address)?;
                        if (val & mask) == data {
                            break;
                        }
                        attempts += 1;
                        if attempts > 10_000 {
                            return Err(EfiStatus::TIMEOUT);
                        }
                        if delay_us > 0 {
                            target.stall(delay_us)?;
                        }
                    }
                }
            }
            executed += 1;
        }
        Ok(executed)
    }
}

impl<const CAPACITY: usize> Default for S3BootScriptTable<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}
