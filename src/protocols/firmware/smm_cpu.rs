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

/// SMM CPU Service Protocol GUID: `{1d202cab-c8ab-4d5c-94f7-3cfcc0d3d335}`.
pub const SMM_CPU_SERVICE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x1D20_2CAB,
    0xC8AB,
    0x4D5C,
    [0x94, 0xF7, 0x3C, 0xFC, 0xC0, 0xD3, 0xD3, 0x35],
);

/// SMM CPU Rendezvous Protocol GUID: `{aa00d50b-4911-428f-b91a-a59ddb13e24c}`.
pub const SMM_CPU_RENDEZVOUS_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xAA00_D50B,
    0x4911,
    0x428F,
    [0xB9, 0x1A, 0xA5, 0x9D, 0xDB, 0x13, 0xE2, 0x4C],
);

/// SMI Transfer Monitor (STM) Initialization Protocol GUID: `{228f344d-b3de-43bb-a4d7-ea200b1b1482}`.
pub const SM_MONITOR_INIT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x228F_344D,
    0xB3DE,
    0x43BB,
    [0xA4, 0xD7, 0xEA, 0x20, 0x0B, 0x1B, 0x14, 0x82],
);

/// Function pointer to get processor information from SMM.
pub type SmmGetProcessorInfoFn = unsafe extern "efiapi" fn(
    this: *const SmmCpuServiceProtocol,
    processor_number: usize,
    processor_info: *mut crate::protocols::core::mp_services::EfiProcessorInformation,
) -> EfiStatus;

/// Function pointer to switch the bootstrap processor (BSP) within SMM.
pub type SmmSwitchBspFn = unsafe extern "efiapi" fn(
    this: *const SmmCpuServiceProtocol,
    processor_number: usize,
) -> EfiStatus;

/// Function pointer to notify that a new processor has been added to the system in SMM.
pub type SmmAddProcessorFn = unsafe extern "efiapi" fn(
    this: *const SmmCpuServiceProtocol,
    processor_id: u64,
    processor_number: *mut usize,
) -> EfiStatus;

/// Function pointer to notify that a processor has been removed from the system in SMM.
pub type SmmRemoveProcessorFn = unsafe extern "efiapi" fn(
    this: *const SmmCpuServiceProtocol,
    processor_number: usize,
) -> EfiStatus;

/// Function pointer to query the logical processor number of the calling core within SMM.
pub type SmmWhoAmIFn = unsafe extern "efiapi" fn(
    this: *const SmmCpuServiceProtocol,
    processor_number: *mut usize,
) -> EfiStatus;

/// Function pointer to register or unregister an SMM processor exception handler.
pub type SmmRegisterExceptionHandlerFn = unsafe extern "efiapi" fn(
    this: *mut SmmCpuServiceProtocol,
    exception_type: isize,
    interrupt_handler: Option<crate::protocols::core::cpu::EfiCpuInterruptHandler>,
) -> EfiStatus;

/// Raw SMM CPU Service Protocol table.
#[repr(C)]
pub struct SmmCpuServiceProtocol {
    /// Retrieves processor info for a logical core.
    pub get_processor_info: SmmGetProcessorInfoFn,
    /// Switches the BSP to a target processor.
    pub switch_bsp: SmmSwitchBspFn,
    /// Hot-adds a processor to SMM tracking.
    pub add_processor: SmmAddProcessorFn,
    /// Hot-removes a processor from SMM tracking.
    pub remove_processor: SmmRemoveProcessorFn,
    /// Identifies the calling processor index.
    pub who_am_i: SmmWhoAmIFn,
    /// Registers an SMM exception or interrupt handler.
    pub register_exception_handler: SmmRegisterExceptionHandlerFn,
}

/// Safe client wrapper around [`SmmCpuServiceProtocol`].
pub struct SmmCpuService<'a> {
    raw: &'a SmmCpuServiceProtocol,
}

impl SmmCpuService<'_> {
    /// Wraps a raw [`SmmCpuServiceProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *const SmmCpuServiceProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &*raw },
            })
        }
    }

    /// Queries the logical processor number of the executing processor.
    ///
    /// # Errors
    ///
    /// Returns an error status if processor determination fails.
    pub fn who_am_i(&self) -> Result<usize, EfiStatus> {
        let mut proc_num = 0usize;
        let status = unsafe { (self.raw.who_am_i)(self.raw, &raw mut proc_num) };
        if status.is_success() {
            Ok(proc_num)
        } else {
            Err(status)
        }
    }

    /// Retrieves detailed topology and state information for a given processor number.
    ///
    /// # Errors
    ///
    /// Returns `NOT_FOUND` if `processor_number` is invalid, or `DEVICE_ERROR` if not BSP.
    pub fn get_processor_info(
        &self,
        processor_number: usize,
    ) -> Result<crate::protocols::core::mp_services::EfiProcessorInformation, EfiStatus> {
        let mut info = crate::protocols::core::mp_services::EfiProcessorInformation {
            processor_id: 0,
            status_flag: 0,
            location: crate::protocols::core::mp_services::EfiCpuPhysicalLocation {
                package: 0,
                core: 0,
                thread: 0,
            },
            extended_information: crate::protocols::core::mp_services::EfiCpuPhysicalLocation2 {
                package: 0,
                die: 0,
                tile: 0,
                module: 0,
                core: 0,
                thread: 0,
            },
        };
        let status =
            unsafe { (self.raw.get_processor_info)(self.raw, processor_number, &raw mut info) };
        if status.is_success() {
            Ok(info)
        } else {
            Err(status)
        }
    }

    /// Switches the SMM Bootstrap Processor (BSP) to the designated processor.
    ///
    /// # Errors
    ///
    /// Returns `INVALID_PARAMETER` if the target is already BSP or disabled, or `UNSUPPORTED`.
    pub fn switch_bsp(&self, processor_number: usize) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.switch_bsp)(self.raw, processor_number) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Function pointer to wait for all APs to arrive in SMM mode within a timeout constraint.
pub type SmmWaitForAllProcessorFn = unsafe extern "efiapi" fn(
    this: *mut SmmCpuRendezvousProtocol,
    blocking_mode: bool,
) -> EfiStatus;

/// Raw SMM CPU Rendezvous Protocol table.
#[repr(C)]
pub struct SmmCpuRendezvousProtocol {
    /// Synchronizes APs arriving in SMM mode.
    pub wait_for_all_processor: SmmWaitForAllProcessorFn,
}

/// Safe client wrapper around [`SmmCpuRendezvousProtocol`].
pub struct SmmCpuRendezvous<'a> {
    raw: &'a mut SmmCpuRendezvousProtocol,
}

impl SmmCpuRendezvous<'_> {
    /// Wraps a raw [`SmmCpuRendezvousProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut SmmCpuRendezvousProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Waits for all APs to arrive in SMM mode under the specified blocking mode.
    ///
    /// # Errors
    ///
    /// Returns `TIMEOUT` if APs did not rendezvous within the timeout constraint.
    pub fn wait_for_all_processor(&mut self, blocking_mode: bool) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.wait_for_all_processor)(self.raw, blocking_mode) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }
}

/// Function pointer to load STM monitor image into MSEG.
pub type SmMonitorLoadMonitorFn =
    unsafe extern "efiapi" fn(stm_image: u64, stm_image_size: usize) -> EfiStatus;

/// Function pointer to add resources to the STM database.
pub type SmMonitorAddPiResourceFn =
    unsafe extern "efiapi" fn(resource_list: *mut c_void, num_entries: u32) -> EfiStatus;

/// Function pointer to delete resources from the STM database.
pub type SmMonitorDeletePiResourceFn =
    unsafe extern "efiapi" fn(resource_list: *mut c_void, num_entries: u32) -> EfiStatus;

/// Function pointer to retrieve resources from the STM database.
pub type SmMonitorGetPiResourceFn =
    unsafe extern "efiapi" fn(resource_list: *mut c_void, resource_size: *mut u32) -> EfiStatus;

/// Function pointer to query STM monitor state flags.
pub type SmMonitorGetMonitorStateFn = unsafe extern "efiapi" fn() -> u32;

/// State flag indicating STM monitor is enabled in MSEG.
pub const SM_MONITOR_STATE_ENABLED: u32 = 0x1;
/// State flag indicating STM monitor is activated and enforcing isolation.
pub const SM_MONITOR_STATE_ACTIVATED: u32 = 0x2;

/// Raw SMI Transfer Monitor (STM) Initialization Protocol table.
#[repr(C)]
pub struct SmMonitorInitProtocol {
    /// Loads the STM image into MSEG.
    pub load_monitor: SmMonitorLoadMonitorFn,
    /// Adds protected resources to the STM registry.
    pub add_pi_resource: SmMonitorAddPiResourceFn,
    /// Deletes protected resources from the STM registry.
    pub delete_pi_resource: SmMonitorDeletePiResourceFn,
    /// Retrieves BIOS resources configured in STM.
    pub get_pi_resource: SmMonitorGetPiResourceFn,
    /// Queries the STM activation and enablement state.
    pub get_monitor_state: SmMonitorGetMonitorStateFn,
}

/// Safe client wrapper around [`SmMonitorInitProtocol`].
pub struct SmMonitorInit<'a> {
    raw: &'a mut SmMonitorInitProtocol,
}

impl SmMonitorInit<'_> {
    /// Wraps a raw [`SmMonitorInitProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut SmMonitorInitProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Loads the STM image into MSEG.
    ///
    /// # Errors
    ///
    /// Returns `BUFFER_TOO_SMALL`, `ALREADY_STARTED`, or `UNSUPPORTED` if MSEG is disabled.
    pub fn load_monitor(&mut self, stm_image: u64, image_size: usize) -> Result<(), EfiStatus> {
        let status = unsafe { (self.raw.load_monitor)(stm_image, image_size) };
        if status.is_success() {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Returns the current state flags of the STM monitor.
    #[must_use]
    pub fn get_monitor_state(&self) -> u32 {
        unsafe { (self.raw.get_monitor_state)() }
    }

    /// Returns true if the STM monitor is active.
    #[must_use]
    pub fn is_activated(&self) -> bool {
        (self.get_monitor_state() & SM_MONITOR_STATE_ACTIVATED) != 0
    }
}
