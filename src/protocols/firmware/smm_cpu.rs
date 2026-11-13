//! # SMM / MM CPU Protocol
//!
//! Provides access to CPU register save states for processors executing within
//! System Management Mode (SMM) or Management Mode (MM).

use crate::base::{EfiGuid, EfiStatus};
use core::ffi::c_void;

/// SMM/MM CPU Protocol GUID: `{eb346704-eb3c-4b31-1e24-99882200a658}`.
pub const SMM_CPU_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xeb34_6704,
    0xeb3c,
    0x4b31,
    [0x1e, 0x24, 0x99, 0x88, 0x22, 0x00, 0xa6, 0x58],
);

/// SMM Save State Register enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiSmmSaveStateRegister {
    /// GDT Base address.
    GdtBase = 4,
    /// IDT Base address.
    IdtBase = 5,
    /// LDT Base address.
    LdtBase = 6,
    /// GDT Limit.
    GdtLimit = 7,
    /// IDT Limit.
    IdtLimit = 8,
    /// LDT Limit.
    LdtLimit = 9,
    /// LDT Info.
    LdtInfo = 10,
    /// ES Segment Selector.
    Es = 20,
    /// CS Segment Selector.
    Cs = 21,
    /// SS Segment Selector.
    Ss = 22,
    /// DS Segment Selector.
    Ds = 23,
    /// FS Segment Selector.
    Fs = 24,
    /// GS Segment Selector.
    Gs = 25,
    /// LDTR Selector.
    LdtrSel = 26,
    /// TR Selector.
    TrSel = 27,
    /// DR7 Debug Register.
    Dr7 = 28,
    /// DR6 Debug Register.
    Dr6 = 29,
    /// R8 General Purpose Register.
    R8 = 30,
    /// R9 General Purpose Register.
    R9 = 31,
    /// R10 General Purpose Register.
    R10 = 32,
    /// R11 General Purpose Register.
    R11 = 33,
    /// R12 General Purpose Register.
    R12 = 34,
    /// R13 General Purpose Register.
    R13 = 35,
    /// R14 General Purpose Register.
    R14 = 36,
    /// R15 General Purpose Register.
    R15 = 37,
    /// RAX / EAX General Purpose Register.
    Rax = 38,
    /// RBX / EBX General Purpose Register.
    Rbx = 39,
    /// RCX / ECX General Purpose Register.
    Rcx = 40,
    /// RDX / EDX General Purpose Register.
    Rdx = 41,
    /// RSP / ESP Stack Pointer.
    Rsp = 42,
    /// RBP / EBP Base Pointer.
    Rbp = 43,
    /// RSI / ESI Source Index.
    Rsi = 44,
    /// RDI / EDI Destination Index.
    Rdi = 45,
    /// RIP / EIP Instruction Pointer.
    Rip = 46,
    /// RFLAGS / EFLAGS Status Register.
    Rflags = 47,
    /// CR0 Control Register.
    Cr0 = 48,
    /// CR3 Control Register.
    Cr3 = 49,
    /// CR4 Control Register.
    Cr4 = 50,
    /// I/O Trapped Information pseudo-register.
    Io = 512,
    /// Long Mode Active pseudo-register.
    Lma = 513,
    /// Processor ID pseudo-register.
    ProcessorId = 514,
}

/// SMM Save State I/O Data Width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiSmmSaveStateIoWidth {
    /// 8-bit I/O width.
    Uint8 = 0,
    /// 16-bit I/O width.
    Uint16 = 1,
    /// 32-bit I/O width.
    Uint32 = 2,
    /// 64-bit I/O width.
    Uint64 = 3,
}

/// SMM Save State I/O Instruction Type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EfiSmmSaveStateIoType {
    /// IN instruction.
    Input = 1,
    /// OUT instruction.
    Output = 2,
    /// String I/O instruction (INS / OUTS).
    String = 4,
    /// REP prefix with string I/O instruction.
    RepPrefix = 8,
}

/// Information returned when querying `EfiSmmSaveStateRegister::Io`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiSmmSaveStateIoInfo {
    /// Data transferred during trapped I/O instruction.
    pub io_data: u64,
    /// I/O port address accessed.
    pub io_port: u16,
    /// Data width.
    pub io_width: EfiSmmSaveStateIoWidth,
    /// Instruction type.
    pub io_type: EfiSmmSaveStateIoType,
}

/// Function signature for `EFI_MM_READ_SAVE_STATE`.
pub type EfiSmmReadSaveState = unsafe extern "efiapi" fn(
    this: *const EfiSmmCpuProtocol,
    width: usize,
    register: EfiSmmSaveStateRegister,
    cpu_index: usize,
    buffer: *mut c_void,
) -> EfiStatus;

/// Function signature for `EFI_MM_WRITE_SAVE_STATE`.
pub type EfiSmmWriteSaveState = unsafe extern "efiapi" fn(
    this: *const EfiSmmCpuProtocol,
    width: usize,
    register: EfiSmmSaveStateRegister,
    cpu_index: usize,
    buffer: *const c_void,
) -> EfiStatus;

/// Low-level EFI SMM CPU Protocol ABI table.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSmmCpuProtocol {
    /// Reads a register from processor save state.
    pub read_save_state: EfiSmmReadSaveState,
    /// Writes a register to processor save state.
    pub write_save_state: EfiSmmWriteSaveState,
}

/// Safe client wrapper around the SMM CPU Protocol.
#[derive(Debug, Clone, Copy)]
pub struct SmmCpu {
    proto: *const EfiSmmCpuProtocol,
}

impl SmmCpu {
    /// Wraps an SMM CPU protocol interface pointer.
    ///
    /// # Safety
    ///
    /// `proto` must point to a valid, live `EfiSmmCpuProtocol` structure.
    #[must_use]
    pub const unsafe fn from_raw(proto: *const EfiSmmCpuProtocol) -> Self {
        Self { proto }
    }

    /// Reads an architecture register from the CPU save-state area.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if register is not supported by processor.
    /// Returns `EfiStatus::INVALID_PARAMETER` if width or CPU index is invalid.
    pub fn read_save_state<T: Copy>(
        &self,
        cpu_index: usize,
        register: EfiSmmSaveStateRegister,
    ) -> Result<T, EfiStatus> {
        let mut value = core::mem::MaybeUninit::<T>::uninit();
        let status = unsafe {
            let func = (*self.proto).read_save_state;
            func(
                self.proto,
                core::mem::size_of::<T>(),
                register,
                cpu_index,
                value.as_mut_ptr().cast::<c_void>(),
            )
        };
        if status == EfiStatus::SUCCESS {
            unsafe { Ok(value.assume_init()) }
        } else {
            Err(status)
        }
    }

    /// Writes an architecture register to the CPU save-state area.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if writing to register is not supported.
    /// Returns `EfiStatus::INVALID_PARAMETER` if CPU index or register width is invalid.
    pub fn write_save_state<T: Copy>(
        &self,
        cpu_index: usize,
        register: EfiSmmSaveStateRegister,
        value: &T,
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            let func = (*self.proto).write_save_state;
            func(
                self.proto,
                core::mem::size_of::<T>(),
                register,
                cpu_index,
                core::ptr::from_ref(value).cast::<c_void>(),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Reads I/O trapped information from the CPU save-state area.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::NOT_FOUND` if SMI was not triggered by an I/O instruction.
    pub fn read_io_info(&self, cpu_index: usize) -> Result<EfiSmmSaveStateIoInfo, EfiStatus> {
        self.read_save_state::<EfiSmmSaveStateIoInfo>(cpu_index, EfiSmmSaveStateRegister::Io)
    }
}
