//! # EFI Byte Code (EBC) Protocols and Virtual Machine Abstractions
//!
//! Provides protocol definitions, virtual machine contexts, debugger interfaces,
//! and opcode definitions for the EFI Byte Code interpreter.

use crate::abi::types::{Char16, UIntN};
use crate::base::{EfiGuid, EfiHandle, EfiPhysicalAddress, EfiStatus};
use crate::tables::system::EfiSystemTable;
use core::ffi::c_void;

// ---------------------------------------------------------------------------
// Protocol GUIDs
// ---------------------------------------------------------------------------

/// EFI Byte Code Interpreter Protocol GUID: `{13ac6dd1-73d0-11d4-b06b-00aa00bd6de7}`.
pub const EFI_EBC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x13AC_6DD1,
    0x73D0,
    0x11D4,
    [0xB0, 0x6B, 0x00, 0xAA, 0x00, 0xBD, 0x6D, 0xE7],
);

/// EFI Byte Code Interpreter Protocol GUID alias.
pub const EFI_EBC_INTERPRETER_PROTOCOL_GUID: EfiGuid = EFI_EBC_PROTOCOL_GUID;

/// EFI Byte Code VM Test Protocol GUID: `{aaeaccfd-f27b-4c17-b610-75ca1f2dfb52}`.
pub const EFI_EBC_VM_TEST_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xAAEA_CCFD,
    0xF27B,
    0x4C17,
    [0xB6, 0x10, 0x75, 0xCA, 0x1F, 0x2D, 0xFB, 0x52],
);

/// EFI Byte Code Simple Debugger Protocol GUID: `{2a72d11e-7376-40f6-9c68-23fa2fe363f1}`.
pub const EFI_EBC_SIMPLE_DEBUGGER_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x2A72_D11E,
    0x7376,
    0x40F6,
    [0x9C, 0x68, 0x23, 0xFA, 0x2F, 0xE3, 0x63, 0xF1],
);

// ---------------------------------------------------------------------------
// EBC Opcodes
// ---------------------------------------------------------------------------

/// Standard EBC virtual machine instruction opcodes.
pub mod opcodes {
    /// Breakpoint instruction.
    pub const BREAK: u8 = 0x00;
    /// Direct jump.
    pub const JMP: u8 = 0x01;
    /// 8-bit relative jump.
    pub const JMP8: u8 = 0x02;
    /// Subroutine call.
    pub const CALL: u8 = 0x03;
    /// Return from subroutine.
    pub const RET: u8 = 0x04;
    /// Compare equal.
    pub const CMPEQ: u8 = 0x05;
    /// Compare signed less than or equal.
    pub const CMPLTE: u8 = 0x06;
    /// Compare signed greater than or equal.
    pub const CMPGTE: u8 = 0x07;
    /// Compare unsigned less than or equal.
    pub const CMPULTE: u8 = 0x08;
    /// Compare unsigned greater than or equal.
    pub const CMPUGTE: u8 = 0x09;
    /// Bitwise NOT.
    pub const NOT: u8 = 0x0A;
    /// Arithmetic negate.
    pub const NEG: u8 = 0x0B;
    /// Add.
    pub const ADD: u8 = 0x0C;
    /// Subtract.
    pub const SUB: u8 = 0x0D;
    /// Multiply signed.
    pub const MUL: u8 = 0x0E;
    /// Multiply unsigned.
    pub const MULU: u8 = 0x0F;
    /// Divide signed.
    pub const DIV: u8 = 0x10;
    /// Divide unsigned.
    pub const DIVU: u8 = 0x11;
    /// Modulo signed.
    pub const MOD: u8 = 0x12;
    /// Modulo unsigned.
    pub const MODU: u8 = 0x13;
    /// Bitwise AND.
    pub const AND: u8 = 0x14;
    /// Bitwise OR.
    pub const OR: u8 = 0x15;
    /// Bitwise XOR.
    pub const XOR: u8 = 0x16;
    /// Shift left.
    pub const SHL: u8 = 0x17;
    /// Logical shift right.
    pub const SHR: u8 = 0x18;
    /// Arithmetic shift right.
    pub const ASHR: u8 = 0x19;
    /// Sign extend byte to natural width.
    pub const EXTNDB: u8 = 0x1A;
    /// Sign extend word to natural width.
    pub const EXTNDW: u8 = 0x1B;
    /// Sign extend double-word to natural width.
    pub const EXTNDD: u8 = 0x1C;
    /// Push register/value onto stack.
    pub const PUSH: u8 = 0x2B;
    /// Pop register/value from stack.
    pub const POP: u8 = 0x2C;
    /// Move immediate.
    pub const MOVI: u8 = 0x37;
    /// Move relative.
    pub const MOVREL: u8 = 0x39;
}

// ---------------------------------------------------------------------------
// VM Flags and Context
// ---------------------------------------------------------------------------

/// Execution stop flags for the EBC virtual machine.
pub mod stop_flags {
    /// Application has completed execution.
    pub const APP_DONE: u32 = 0x0001;
    /// VM encountered a breakpoint instruction.
    pub const BREAKPOINT: u32 = 0x0002;
    /// VM encountered an invalid breakpoint.
    pub const INVALID_BREAK: u32 = 0x0004;
    /// Breakpoint triggered on external native call.
    pub const BREAK_ON_CALLEX: u32 = 0x0008;
}

/// EBC VM condition and execution mode flags.
pub mod vm_flags {
    /// Condition code flag (set if last comparison was true).
    pub const CC: u64 = 0x0001;
    /// Single step instruction execution mode.
    pub const STEP: u64 = 0x0002;
}

/// Exception severity flags for EBC VM.
pub mod exception_flags {
    /// Fatal exception, execution cannot continue.
    pub const FATAL: u32 = 0x8000_0000;
    /// Error condition, VM may attempt recovery.
    pub const ERROR: u32 = 0x4000_0000;
    /// Warning condition, non-fatal.
    pub const WARNING: u32 = 0x2000_0000;
    /// Normal return, no exception pending.
    pub const NONE: u32 = 0x0000_0000;
}

/// EBC Virtual Machine execution context.
#[repr(C)]
#[derive(Debug)]
pub struct EbcVmContext {
    /// General purpose 64-bit registers R0 through R7.
    pub gpr: [i64; 8],
    /// Flags register (bit 0 = CC, bit 1 = Step).
    pub flags: u64,
    /// Instruction pointer.
    pub ip: *mut u8,
    /// Last exception identifier.
    pub last_exception: UIntN,
    /// Exception severity flags.
    pub exception_flags: u32,
    /// VM execution stop flags.
    pub stop_flags: u32,
    /// Compiler version information.
    pub compiler_version: u32,
    /// Bottom address of upper stack region.
    pub high_stack_bottom: UIntN,
    /// Top address of lower stack region.
    pub low_stack_top: UIntN,
    /// Location of final return address on stack.
    pub stack_ret_addr: u64,
    /// Magic value pointer on stack for corruption detection.
    pub stack_magic_ptr: *mut UIntN,
    /// Image handle for the active EBC driver.
    pub image_handle: EfiHandle,
    /// System table pointer for debug inspection.
    pub system_table: *mut EfiSystemTable,
    /// Last address converted for debug inspection.
    pub last_addr_converted: UIntN,
    /// Last value converted for debug inspection.
    pub last_addr_converted_value: UIntN,
    /// Current frame pointer.
    pub frame_ptr: *mut c_void,
    /// Entry point of EBC image.
    pub entry_point: *mut c_void,
    /// Base address of loaded image in memory.
    pub image_base: UIntN,
    /// Base of stack memory pool.
    pub stack_pool: *mut c_void,
    /// Top of stack memory pool.
    pub stack_top: *mut c_void,
}

// ---------------------------------------------------------------------------
// EBC Interpreter Protocol
// ---------------------------------------------------------------------------

/// Prototype for instruction cache flush callback.
pub type EbcIcacheFlush =
    unsafe extern "efiapi" fn(start: EfiPhysicalAddress, length: u64) -> EfiStatus;

/// Creates a native thunk for an EBC entry point.
pub type EfiEbcCreateThunk = unsafe extern "efiapi" fn(
    this: *mut EfiEbcProtocol,
    image_handle: EfiHandle,
    ebc_entry_point: *const c_void,
    thunk: *mut *mut c_void,
) -> EfiStatus;

/// Performs cleanup prior to unloading an EBC image.
pub type EfiEbcUnloadImage =
    unsafe extern "efiapi" fn(this: *mut EfiEbcProtocol, image_handle: EfiHandle) -> EfiStatus;

/// Registers a processor instruction cache flush callback.
pub type EfiEbcRegisterIcacheFlush = unsafe extern "efiapi" fn(
    this: *mut EfiEbcProtocol,
    flush: Option<EbcIcacheFlush>,
) -> EfiStatus;

/// Queries the interpreter version.
pub type EfiEbcGetVersion =
    unsafe extern "efiapi" fn(this: *mut EfiEbcProtocol, version: *mut u64) -> EfiStatus;

/// Low-level EFI Byte Code Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiEbcProtocol {
    /// Creates a native execution thunk for EBC entry point.
    pub create_thunk: Option<EfiEbcCreateThunk>,
    /// Unloads an EBC image and performs interpreter cleanup.
    pub unload_image: Option<EfiEbcUnloadImage>,
    /// Registers an instruction cache flush callback.
    pub register_icache_flush: Option<EfiEbcRegisterIcacheFlush>,
    /// Retrieves the interpreter version.
    pub get_version: Option<EfiEbcGetVersion>,
}

/// Safe client wrapper around [`EfiEbcProtocol`].
#[derive(Debug)]
pub struct Ebc<'a> {
    proto: &'a mut EfiEbcProtocol,
}

impl<'a> Ebc<'a> {
    /// Creates a safe wrapper around a mutable reference to [`EfiEbcProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiEbcProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiEbcProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *mut EfiEbcProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Creates a native processor thunk to execute an EBC image entry point.
    ///
    /// # Safety
    ///
    /// `ebc_entry_point` must point to valid EBC machine instructions or entry point.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub unsafe fn create_thunk(
        &mut self,
        image_handle: EfiHandle,
        ebc_entry_point: *const c_void,
    ) -> Result<*mut c_void, EfiStatus> {
        let Some(func) = self.proto.create_thunk else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let mut thunk = core::ptr::null_mut();
        let status = unsafe { func(self.proto, image_handle, ebc_entry_point, &raw mut thunk) };
        if status == EfiStatus::SUCCESS {
            Ok(thunk)
        } else {
            Err(status)
        }
    }

    /// Cleans up state prior to unloading an EBC image.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub fn unload_image(&mut self, image_handle: EfiHandle) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.unload_image else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { func(self.proto, image_handle) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Queries the loaded interpreter version.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub fn version(&mut self) -> Result<u64, EfiStatus> {
        let Some(func) = self.proto.get_version else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let mut ver: u64 = 0;
        let status = unsafe { func(self.proto, &raw mut ver) };
        if status == EfiStatus::SUCCESS {
            Ok(ver)
        } else {
            Err(status)
        }
    }
}

// ---------------------------------------------------------------------------
// EBC VM Test Protocol
// ---------------------------------------------------------------------------

/// Executes instructions in an EBC virtual machine context for test purposes.
pub type EbcVmTestExecute = unsafe extern "efiapi" fn(
    this: *mut EfiEbcVmTestProtocol,
    vm_ptr: *mut EbcVmContext,
    instruction_count: *mut UIntN,
) -> EfiStatus;

/// Assembles textual EBC assembly into machine bytecode.
pub type EbcVmTestAsm = unsafe extern "efiapi" fn(
    this: *mut EfiEbcVmTestProtocol,
    asm_text: *const Char16,
    buffer: *mut i8,
    buffer_len: *mut UIntN,
) -> EfiStatus;

/// Disassembles machine bytecode into text.
pub type EbcVmTestDasm = unsafe extern "efiapi" fn(
    this: *mut EfiEbcVmTestProtocol,
    asm_text: *mut Char16,
    buffer: *mut i8,
    len: *mut UIntN,
) -> EfiStatus;

/// Low-level EBC VM Test Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiEbcVmTestProtocol {
    /// Executes one or more instructions within VM context.
    pub execute: Option<EbcVmTestExecute>,
    /// Assembles textual assembly code.
    pub assemble: Option<EbcVmTestAsm>,
    /// Disassembles machine bytecode.
    pub disassemble: Option<EbcVmTestDasm>,
}

/// Safe client wrapper around [`EfiEbcVmTestProtocol`].
#[derive(Debug)]
pub struct EbcVmTest<'a> {
    proto: &'a mut EfiEbcVmTestProtocol,
}

impl<'a> EbcVmTest<'a> {
    /// Creates a safe wrapper around a mutable reference to [`EfiEbcVmTestProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiEbcVmTestProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiEbcVmTestProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *mut EfiEbcVmTestProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Executes instructions in the given VM context.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub fn execute(
        &mut self,
        vm: &mut EbcVmContext,
        instruction_count: &mut UIntN,
    ) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.execute else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe {
            func(
                self.proto,
                core::ptr::from_mut(vm),
                core::ptr::from_mut(instruction_count),
            )
        };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}

// ---------------------------------------------------------------------------
// EBC Simple Debugger Protocol
// ---------------------------------------------------------------------------

/// Debugs instructions within VM context.
pub type EbcDebuggerDebug =
    unsafe extern "efiapi" fn(this: *mut EfiEbcSimpleDebuggerProtocol, vm_ptr: *mut EbcVmContext);

/// Signals an exception on the EBC virtual machine.
pub type EbcDebuggerSignalException = unsafe extern "efiapi" fn(
    this: *mut EfiEbcSimpleDebuggerProtocol,
    vm_ptr: *mut EbcVmContext,
    exception_type: u32,
) -> EfiStatus;

/// Disassembles instructions to a string buffer.
pub type EbcDebuggerDasm = unsafe extern "efiapi" fn(
    this: *mut EfiEbcSimpleDebuggerProtocol,
    vm_ptr: *mut EbcVmContext,
    dasm_string: *mut u16,
    dasm_string_size: u32,
) -> u32;

/// Configures EBC debugger settings.
pub type EbcDebuggerConfigure = unsafe extern "efiapi" fn(
    this: *mut EfiEbcSimpleDebuggerProtocol,
    config_id: u32,
    config_value: UIntN,
) -> EfiStatus;

/// Low-level EBC Simple Debugger Protocol interface.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiEbcSimpleDebuggerProtocol {
    /// Debugs one or more instructions.
    pub debugger: Option<EbcDebuggerDebug>,
    /// Triggers an exception in the VM.
    pub signal_exception: Option<EbcDebuggerSignalException>,
    /// Dumps disassembled instructions.
    pub dasm: Option<EbcDebuggerDasm>,
    /// Configures debugger settings.
    pub configure: Option<EbcDebuggerConfigure>,
}

/// Safe client wrapper around [`EfiEbcSimpleDebuggerProtocol`].
#[derive(Debug)]
pub struct EbcSimpleDebugger<'a> {
    proto: &'a mut EfiEbcSimpleDebuggerProtocol,
}

impl<'a> EbcSimpleDebugger<'a> {
    /// Creates a safe wrapper around a mutable reference to [`EfiEbcSimpleDebuggerProtocol`].
    #[must_use]
    pub fn new(proto: &'a mut EfiEbcSimpleDebuggerProtocol) -> Self {
        Self { proto }
    }

    /// Wraps a raw pointer to [`EfiEbcSimpleDebuggerProtocol`].
    ///
    /// # Safety
    ///
    /// `proto` must point to an initialized and valid protocol instance.
    #[must_use]
    pub const unsafe fn from_raw(proto: *mut EfiEbcSimpleDebuggerProtocol) -> Option<Self> {
        if proto.is_null() {
            None
        } else {
            Some(Self {
                proto: unsafe { &mut *proto },
            })
        }
    }

    /// Configures debugger settings.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub fn configure(&mut self, config_id: u32, config_value: UIntN) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.configure else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { func(self.proto, config_id, config_value) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Signals an exception on the EBC virtual machine.
    ///
    /// # Errors
    ///
    /// Returns `EfiStatus::UNSUPPORTED` if unimplemented, or status returned by firmware.
    pub fn signal_exception(
        &mut self,
        vm: &mut EbcVmContext,
        exception_type: u32,
    ) -> Result<(), EfiStatus> {
        let Some(func) = self.proto.signal_exception else {
            return Err(EfiStatus::UNSUPPORTED);
        };
        let status = unsafe { func(self.proto, core::ptr::from_mut(vm), exception_type) };
        if status == EfiStatus::SUCCESS {
            Ok(())
        } else {
            Err(status)
        }
    }
}
