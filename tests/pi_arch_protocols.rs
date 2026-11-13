//! Tests for PI architectural protocols.

use rover_uefi::protocols::firmware::arch_protocols::*;

#[test]
fn test_pi_arch_protocol_guids() {
    assert_ne!(EFI_RESET_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_REAL_TIME_CLOCK_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_WATCHDOG_TIMER_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_MONOTONIC_COUNTER_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_METRONOME_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_TIMER_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_VARIABLE_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_VARIABLE_WRITE_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_RUNTIME_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_SECURITY_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_SECURITY2_ARCH_PROTOCOL_GUID.data1, 0);
    assert_ne!(EFI_CPU_ARCH_PROTOCOL_GUID.data1, 0);
}

#[test]
fn test_metronome_and_timer_struct_layouts() {
    assert_eq!(
        core::mem::size_of::<EfiMetronomeArchProtocol>(),
        core::mem::size_of::<usize>()
            + 4
            + if core::mem::size_of::<usize>() == 8 {
                4
            } else {
                0
            }
    );
    assert_eq!(
        core::mem::size_of::<EfiTimerArchProtocol>(),
        4 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiWatchdogTimerArchProtocol>(),
        3 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiSecurityArchProtocol>(),
        core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiSecurity2ArchProtocol>(),
        core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiCpuArchProtocol>(),
        8 * core::mem::size_of::<usize>() + 8
    );
}

#[test]
fn test_cpu_flush_and_init_enums() {
    assert_eq!(EfiCpuFlushType::WriteBackInvalidate as u32, 0);
    assert_eq!(EfiCpuFlushType::WriteBack as u32, 1);
    assert_eq!(EfiCpuFlushType::Invalidate as u32, 2);
    assert_eq!(EfiCpuInitType::Init as u32, 0);
}
