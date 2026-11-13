//! # Core protocol support
//!
//! Shared support for all protocol modules.
//!
//! The module provides protocol `GUID`
//! constants, lifetime rules, and open and
//! close helpers. Concrete protocols provide
//! their own wrappers that borrow the interface
//! for an explicit lifetime.

pub mod arch;
pub mod cpu;
pub mod cpu_io2;
pub mod guid;
pub mod interface;
pub mod memory_attribute;
pub mod mp_services;
pub mod open;
pub mod regex;
pub mod reset_notification;

pub use crate::arch::riscv::{
    RISCV_BOOT_PROTOCOL_GUID, RISCV_BOOT_PROTOCOL_REVISION, RiscvBoot, RiscvBootProtocol,
    RiscvGetBootHartId,
};

pub use arch::{
    MONOTONIC_COUNTER_ARCH_PROTOCOL_GUID, RESET_ARCH_PROTOCOL_GUID, VARIABLE_ARCH_PROTOCOL_GUID,
    VARIABLE_WRITE_ARCH_PROTOCOL_GUID,
};
pub use cpu::{
    CpuArch, EFI_CPU_ARCH_PROTOCOL_GUID, EfiCpuArchProtocol, EfiCpuDisableInterrupt,
    EfiCpuEnableInterrupt, EfiCpuFlushDataCache, EfiCpuFlushType, EfiCpuGetInterruptState,
    EfiCpuGetTimerValue, EfiCpuInit, EfiCpuInitType, EfiCpuInterruptHandler,
    EfiCpuRegisterInterruptHandler, EfiCpuSetMemoryAttributes,
};
pub use cpu_io2::{
    CpuIo2, EFI_CPU_IO2_PROTOCOL_GUID, EfiCpuIo2Access, EfiCpuIo2Protocol, EfiCpuIo2ProtocolAccess,
    EfiCpuIoProtocolWidth,
};
pub use guid::{DEVICE_PATH_GUID, SIMPLE_TEXT_INPUT_GUID, SIMPLE_TEXT_OUTPUT_GUID};
pub use interface::{is_handle_present, is_interface_present, locate_protocol};
pub use memory_attribute::{
    EFI_MEMORY_ATTRIBUTE_MASK, EFI_MEMORY_RO, EFI_MEMORY_RP, EFI_MEMORY_XP,
    EfiClearMemoryAttributes, EfiGetMemoryAttributes, EfiMemoryAttributeProtocol,
    EfiSetMemoryAttributes, MEMORY_ATTRIBUTE_PROTOCOL_GUID, MemoryAttributeProtocol,
};
pub use mp_services::{
    CORE_TYPE_EFFICIENT, CORE_TYPE_PERFORMANCE, CORE_TYPE_STANDARD, EFI_MP_SERVICES_PROTOCOL_GUID,
    EfiApProcedure, EfiCpuPhysicalLocation, EfiCpuPhysicalLocation2, EfiMpServicesEnableDisableAP,
    EfiMpServicesGetNumberOfProcessors, EfiMpServicesGetProcessorInfo, EfiMpServicesProtocol,
    EfiMpServicesStartupAllAPs, EfiMpServicesStartupThisAP, EfiMpServicesSwitchBsp,
    EfiMpServicesWhoAmI, EfiProcessorInformation, MP_INFORMATION2_GUID,
    MP_INFORMATION2_HOB_REVISION, MpInformation2Entry, MpInformation2HobData, MpServices,
    PROCESSOR_AS_BSP_BIT, PROCESSOR_ENABLED_BIT, PROCESSOR_HEALTH_STATUS_BIT,
};
pub use open::{close, open};
pub use regex::{
    CompiledRegex, EFI_REGEX_SYNTAX_TYPE_ECMA_262_GUID, EFI_REGEX_SYNTAX_TYPE_PERL_GUID,
    EFI_REGEX_SYNTAX_TYPE_POSIX_EXTENDED_GUID, EFI_REGULAR_EXPRESSION_PROTOCOL_GUID,
    EfiRegexCapture, EfiRegularExpressionGetInfo, EfiRegularExpressionMatch,
    EfiRegularExpressionProtocol, MAX_REGEX_ELEMENTS, RegularExpression, regex_match,
    regex_match_str,
};
pub use reset_notification::{
    EfiRegisterResetNotify, EfiResetNotificationProtocol, EfiUnregisterResetNotify,
    PLATFORM_SPECIFIC_RESET_FILTER_PROTOCOL_GUID, PLATFORM_SPECIFIC_RESET_HANDLER_PROTOCOL_GUID,
    RESET_NOTIFICATION_PROTOCOL_GUID, ResetNotificationProtocol,
};
