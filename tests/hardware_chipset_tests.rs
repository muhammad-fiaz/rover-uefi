//! Integration tests for hardware chipset controllers, ARM firmware protocols,
//! UEFI Shell dynamic commands, and platform security mitigation interfaces:
//! Dual 8259 PIC, 8254 PIT, MC146818 CMOS RTC, ARM SMCCC, ARM PSCI,
//! Shell Dynamic Command, TCG MOR, and TCG Physical Presence.

use core::mem::size_of;
use rover_uefi::arch::arm64::{
    PSCI_AFFINITY_OFF, PSCI_AFFINITY_ON, PSCI_CPU_OFF, PSCI_CPU_ON_SMC64, PSCI_SYSTEM_OFF,
    PSCI_SYSTEM_RESET, PSCI_VERSION, Pl011Uart, PsciStatus, SMCCC_ARCH_FEATURES, SMCCC_ARCH_SOC_ID,
    SMCCC_RET_INVALID_PARAMETER, SMCCC_RET_NOT_REQUIRED, SMCCC_RET_NOT_SUPPORTED,
    SMCCC_RET_SUCCESS, SMCCC_VERSION, SmcccFunctionId, SmcccOwner, SmcccResult,
    compute_pl011_divisors, parse_psci_version, pl011_fr, psci_target_cpu64,
};
use rover_uefi::arch::x86::{
    ACPI_PM_TIMER_FREQUENCY_HZ, CMOS_ADDRESS_PORT, CMOS_DATA_PORT, CMOS_NMI_DISABLE_BIT,
    CMOS_REG_CENTURY, CMOS_REG_HOURS, CMOS_REG_MINUTES, CMOS_REG_MONTH, CMOS_REG_SECONDS,
    CMOS_REG_YEAR, ChainedPic, CmosDateTime, HPET_DEFAULT_BASE_ADDRESS, HpetController, ICW1_INIT,
    ICW4_8086, PIC_EOI, PIC_MASK_ALL, PIC1_COMMAND, PIC1_DATA, PIC2_COMMAND, PIC2_DATA,
    PIT_BASE_FREQUENCY_HZ, PIT_CHANNEL0_DATA, PIT_COMMAND_PORT, PitAccessMode, PitChannel,
    PitOperatingMode, STATUS_A_UPDATE_IN_PROGRESS, STATUS_B_24_HOUR_MODE, STATUS_B_BINARY_MODE,
    UART_BASE_CLOCK_HZ, UART_MAX_BAUD, Uart16550, bcd_to_binary, binary_to_bcd, build_pit_command,
    compute_pit_divisor, compute_uart_baud_divisor, microseconds_to_pm_timer_ticks,
    pm_timer_elapsed_microseconds, uart_com_port, uart_lsr,
};
use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::console::shell::*;
use rover_uefi::protocols::security::mor::*;
use rover_uefi::protocols::security::tcg_pp::*;

#[test]
fn test_dual_8259_pic_controller() {
    assert_eq!(PIC1_COMMAND, 0x20);
    assert_eq!(PIC1_DATA, 0x21);
    assert_eq!(PIC2_COMMAND, 0xA0);
    assert_eq!(PIC2_DATA, 0xA1);
    assert_eq!(PIC_EOI, 0x20);
    assert_eq!(PIC_MASK_ALL, 0xFF);
    assert_eq!(ICW1_INIT, 0x10);
    assert_eq!(ICW4_8086, 0x01);

    let pic = ChainedPic::new(0x20, 0x28);
    assert!(pic.handles_interrupt(0x20));
    assert!(pic.handles_interrupt(0x27));
    assert!(pic.handles_interrupt(0x28));
    assert!(pic.handles_interrupt(0x2F));
    assert!(!pic.handles_interrupt(0x19));
    assert!(!pic.handles_interrupt(0x30));

    // Master IRQ EOI
    assert_eq!(pic.compute_eoi(0x23), (true, false));
    // Slave IRQ EOI (requires EOI to both Master and Slave)
    assert_eq!(pic.compute_eoi(0x2A), (true, true));
    // Non-handled vector
    assert_eq!(pic.compute_eoi(0x80), (false, false));
}

#[test]
fn test_8254_pit_timer() {
    assert_eq!(PIT_BASE_FREQUENCY_HZ, 1_193_182);
    assert_eq!(PIT_CHANNEL0_DATA, 0x40);
    assert_eq!(PIT_COMMAND_PORT, 0x43);

    let cmd = build_pit_command(
        PitChannel::Channel0,
        PitAccessMode::LowHighByte,
        PitOperatingMode::SquareWave,
        false,
    );
    // Channel 0 (0 << 6) | LowHighByte (3 << 4 = 0x30) | SquareWave (3 << 1 = 0x06) | binary (0) = 0x36
    assert_eq!(cmd, 0x36);

    // Divisors
    assert_eq!(compute_pit_divisor(1000), 1193); // ~1 kHz
    assert_eq!(compute_pit_divisor(100), 11931); // ~100 Hz
    assert_eq!(compute_pit_divisor(18), 0); // 65536 wrap
    assert_eq!(compute_pit_divisor(2_000_000), 1); // Clamp max
}

#[test]
fn test_cmos_rtc_controller() {
    assert_eq!(CMOS_ADDRESS_PORT, 0x70);
    assert_eq!(CMOS_DATA_PORT, 0x71);
    assert_eq!(CMOS_NMI_DISABLE_BIT, 0x80);
    assert_eq!(CMOS_REG_SECONDS, 0x00);
    assert_eq!(CMOS_REG_MINUTES, 0x02);
    assert_eq!(CMOS_REG_HOURS, 0x04);
    assert_eq!(CMOS_REG_MONTH, 0x08);
    assert_eq!(CMOS_REG_YEAR, 0x09);
    assert_eq!(CMOS_REG_CENTURY, 0x32);
    assert_eq!(STATUS_A_UPDATE_IN_PROGRESS, 0x80);
    assert_eq!(STATUS_B_24_HOUR_MODE, 0x02);
    assert_eq!(STATUS_B_BINARY_MODE, 0x04);

    // BCD conversion roundtrip
    for val in 0..=99 {
        let bcd = binary_to_bcd(val);
        assert_eq!(bcd_to_binary(bcd), val);
    }
    assert_eq!(bcd_to_binary(0x59), 59);
    assert_eq!(binary_to_bcd(59), 0x59);

    let dt = CmosDateTime {
        year: 2026,
        month: 11,
        day: 13,
        hour: 18,
        minute: 45,
        second: 30,
    };
    assert!(dt.is_valid());

    let invalid_dt = CmosDateTime {
        year: 2026,
        month: 13,
        day: 32,
        hour: 25,
        minute: 60,
        second: 60,
    };
    assert!(!invalid_dt.is_valid());
}

#[test]
fn test_arm_smccc_encoding_and_results() {
    assert_eq!(SMCCC_VERSION, 0x8000_0000);
    assert_eq!(SMCCC_ARCH_FEATURES, 0x8000_0001);
    assert_eq!(SMCCC_ARCH_SOC_ID, 0x8000_0002);
    assert_eq!(SMCCC_RET_SUCCESS, 0);
    assert_eq!(SMCCC_RET_NOT_SUPPORTED, -1);
    assert_eq!(SMCCC_RET_NOT_REQUIRED, -2);
    assert_eq!(SMCCC_RET_INVALID_PARAMETER, -3);

    // Encode a 64-bit fast call to Standard Secure Service (PSCI CPU_ON = 0xC400_0003)
    let fid = SmcccFunctionId {
        is_fast_call: true,
        is_smc64: true,
        owner: SmcccOwner::StandardSecure,
        function_number: 0x0003,
    };
    assert_eq!(fid.to_raw(), 0xC400_0003);

    // Decode roundtrip
    let parsed = SmcccFunctionId::from_raw(0xC400_0003);
    assert_eq!(parsed, fid);

    let res = SmcccResult {
        x0: 0,
        x1: 42,
        x2: 100,
        x3: 200,
    };
    assert_eq!(res.x0, 0);
    assert_eq!(res.x1, 42);
}

#[test]
fn test_arm_psci_functions_and_affinity() {
    assert_eq!(PSCI_VERSION, 0x8400_0000);
    assert_eq!(PSCI_CPU_OFF, 0x8400_0002);
    assert_eq!(PSCI_CPU_ON_SMC64, 0xC400_0003);
    assert_eq!(PSCI_SYSTEM_OFF, 0x8400_0008);
    assert_eq!(PSCI_SYSTEM_RESET, 0x8400_0009);

    assert_eq!(PSCI_AFFINITY_ON, 0);
    assert_eq!(PSCI_AFFINITY_OFF, 1);

    // Status mapping
    assert_eq!(PsciStatus::from_raw(0), PsciStatus::Success);
    assert_eq!(PsciStatus::from_raw(-1), PsciStatus::NotSupported);
    assert_eq!(PsciStatus::from_raw(-2), PsciStatus::InvalidParameters);
    assert_eq!(PsciStatus::from_raw(-3), PsciStatus::Denied);
    assert_eq!(PsciStatus::from_raw(-4), PsciStatus::AlreadyOn);

    // Version parsing: 0x00010002 -> (1, 2)
    assert_eq!(parse_psci_version(0x0001_0002), (1, 2));

    // Target CPU64 MPIDR affinity pack
    let target = psci_target_cpu64(0x01, 0x02, 0x03, 0x04);
    assert_eq!(target, 0x0000_0001_0002_0304);
}

#[test]
fn test_uefi_shell_protocols_and_guid() {
    assert_eq!(
        EFI_SHELL_PROTOCOL_GUID,
        EfiGuid::new(
            0x6302_D008,
            0x7F9B,
            0x4F30,
            [0x87, 0xAC, 0x60, 0xC9, 0xFE, 0xF5, 0xDA, 0x4E]
        )
    );
    assert_eq!(
        EFI_SHELL_PARAMETERS_PROTOCOL_GUID,
        EfiGuid::new(
            0x752F_3136,
            0x4E16,
            0x4FDC,
            [0xA2, 0x2A, 0xE5, 0xF4, 0x68, 0x12, 0xF4, 0xCA]
        )
    );
    assert_eq!(
        EFI_SHELL_DYNAMIC_COMMAND_PROTOCOL_GUID,
        EfiGuid::new(
            0x3C72_00E9,
            0x005F,
            0x4EA4,
            [0x87, 0xDE, 0xA3, 0xDF, 0xAC, 0x8A, 0x27, 0xC3]
        )
    );

    assert_eq!(
        size_of::<EfiShellDynamicCommandProtocol>(),
        size_of::<[usize; 3]>()
    );

    let params_raw = EfiShellParametersProtocol {
        argv: core::ptr::null_mut(),
        argc: 0,
        std_in: core::ptr::null_mut(),
        std_out: core::ptr::null_mut(),
        std_err: core::ptr::null_mut(),
    };
    let params = ShellParameters::new(&params_raw);
    assert_eq!(params.argc(), 0);
    assert_eq!(params.arg(0), None);
}

#[test]
fn test_tcg_mor_and_physical_presence() {
    assert_eq!(
        MEMORY_ONLY_RESET_CONTROL_GUID,
        EfiGuid::new(
            0xE209_39BE,
            0x32D4,
            0x41BE,
            [0xA1, 0x50, 0x89, 0x7F, 0x85, 0xD4, 0x98, 0x29]
        )
    );
    assert_eq!(
        MEMORY_OVERWRITE_REQUEST_CONTROL_LOCK_GUID,
        EfiGuid::new(
            0xBB98_3CCF,
            0x151D,
            0x40E1,
            [0xA0, 0x7B, 0x4A, 0x17, 0xBE, 0x16, 0x82, 0x92]
        )
    );
    assert_eq!(
        TCG_PHYSICAL_PRESENCE_DATA_GUID,
        EfiGuid::new(
            0xF646_1488,
            0x89A0,
            0x4305,
            [0xAC, 0x57, 0xAB, 0x19, 0x4F, 0x10, 0x88, 0xF5]
        )
    );

    // MOR flags encoding/decoding
    let mor_byte = MemoryOverwriteControl::encode(true, true);
    assert_eq!(mor_byte, 0x11);
    assert!(MemoryOverwriteControl::should_clear_memory(mor_byte));
    assert!(MemoryOverwriteControl::disable_autodetect(mor_byte));

    let mor_clear_only = MemoryOverwriteControl::encode(true, false);
    assert_eq!(mor_clear_only, 0x01);
    assert!(MemoryOverwriteControl::should_clear_memory(mor_clear_only));
    assert!(!MemoryOverwriteControl::disable_autodetect(mor_clear_only));

    // Physical presence operations
    assert!(TcgPhysicalPresence::is_valid_operation(
        TCG_PHYSICAL_PRESENCE_ENABLE
    ));
    assert!(TcgPhysicalPresence::is_valid_operation(
        TCG_PHYSICAL_PRESENCE_CLEAR
    ));
    assert!(!TcgPhysicalPresence::is_valid_operation(99));
    assert!(TcgPhysicalPresence::is_destructive(
        TCG_PHYSICAL_PRESENCE_CLEAR
    ));
    assert!(!TcgPhysicalPresence::is_destructive(
        TCG_PHYSICAL_PRESENCE_ENABLE
    ));

    assert_eq!(TCG_PP_RETURN_TPM_SUCCESS, 0);
    assert_eq!(TCG_PP_RETURN_TPM_FAILURE, 1);
    assert_eq!(TCG_PP_RETURN_TPM_BLOCKED, 2);
    assert_eq!(TCG_PP_RETURN_USER_ABORT, 3);
}

#[test]
fn test_uart16550_controller() {
    assert_eq!(UART_BASE_CLOCK_HZ, 1_843_200);
    assert_eq!(UART_MAX_BAUD, 115_200);

    assert_eq!(compute_uart_baud_divisor(115_200), 1);
    assert_eq!(compute_uart_baud_divisor(9_600), 12);
    assert_eq!(compute_uart_baud_divisor(38_400), 3);
    assert_eq!(compute_uart_baud_divisor(0), 1);

    let com1 = Uart16550::new(uart_com_port::COM1);
    assert_eq!(com1.base_port(), 0x03F8);
    assert_eq!(com1.port_for(0), 0x03F8);
    assert_eq!(com1.port_for(5), 0x03FD); // LSR

    assert!(Uart16550::is_tx_ready(uart_lsr::TX_HOLDING_EMPTY));
    assert!(!Uart16550::is_tx_ready(0));
    assert!(Uart16550::is_rx_ready(uart_lsr::DATA_READY));
    assert!(!Uart16550::is_rx_ready(0));
}

#[test]
fn test_pl011_uart_controller() {
    // Standard 24 MHz clock at 115200 baud
    // Divisor = 24_000_000 / (16 * 115200) = 13.020833...
    // Int = 13, Frac = round(0.020833 * 64) = round(1.33) = 1
    let divisors = compute_pl011_divisors(24_000_000, 115_200);
    assert_eq!(divisors.integer_divisor, 13);
    assert_eq!(divisors.fractional_divisor, 1);

    let uart = Pl011Uart::new(0x0900_0000);
    assert_eq!(uart.base_address(), 0x0900_0000);
    assert_eq!(uart.reg_address(0x18), 0x0900_0018); // UARTFR

    assert!(Pl011Uart::is_tx_full(pl011_fr::TXFF));
    assert!(!Pl011Uart::is_tx_full(0));
    assert!(Pl011Uart::is_rx_empty(pl011_fr::RXFE));
    assert!(!Pl011Uart::is_rx_empty(0));
}

#[test]
fn test_acpi_pm_timer() {
    assert_eq!(ACPI_PM_TIMER_FREQUENCY_HZ, 3_579_545);

    // 3,579,545 ticks = exactly 1,000,000 microseconds (1 second)
    let elapsed = pm_timer_elapsed_microseconds(0, 3_579_545);
    assert_eq!(elapsed, 1_000_000);

    // Rollover test: start at 0x00FF_FFF0, end at 10
    let start = 0x00FF_FFF0;
    let end = 10;
    let elapsed_roll = pm_timer_elapsed_microseconds(start, end);
    assert!(elapsed_roll > 0);

    let ticks_1ms = microseconds_to_pm_timer_ticks(1_000);
    assert_eq!(ticks_1ms, 3_579);
}

#[test]
fn test_hpet_controller() {
    assert_eq!(HPET_DEFAULT_BASE_ADDRESS, 0xFED0_0000);

    let hpet = HpetController::new(HPET_DEFAULT_BASE_ADDRESS);
    assert_eq!(hpet.base_address(), 0xFED0_0000);

    // Cap register mock: 64-bit counter (bit 13), 3 timers (count_minus_one = 2 at bits 12:8), period 10000 fs (at bits 63:32)
    let cap_reg: u64 = (10_000_u64 << 32) | (1_u64 << 13) | (2_u64 << 8) | 1;
    assert_eq!(HpetController::tick_period_fs(cap_reg), 10_000);
    assert_eq!(HpetController::timer_count(cap_reg), 3);
    assert!(HpetController::is_64bit_counter(cap_reg));
}
