// SPDX-License-Identifier: Apache-2.0
//! Unit tests for ARM32, IA32, and `LoongArch64` architectural contexts.

use core::mem::size_of;
use rover_uefi::arch::arm32::{Arm32Registers, CPSR_IRQ_DISABLED, CPSR_THUMB_STATE};
use rover_uefi::arch::ia32::{EFLAGS_IF, Ia32ExceptionContext};
use rover_uefi::arch::loongarch64::LoongArch64ExceptionContext;

#[test]
fn arm32_registers_and_helpers() {
    assert!(size_of::<Arm32Registers>() >= 68);

    let mut regs = Arm32Registers::default();
    assert!(!regs.is_thumb());
    assert!(!regs.are_irqs_disabled());

    regs.cpsr |= CPSR_THUMB_STATE;
    assert!(regs.is_thumb());

    regs.cpsr |= CPSR_IRQ_DISABLED;
    assert!(regs.are_irqs_disabled());
}

#[test]
fn ia32_context_and_helpers() {
    assert!(size_of::<Ia32ExceptionContext>() >= 56);

    let mut ctx = Ia32ExceptionContext::default();
    assert!(!ctx.are_interrupts_enabled());

    ctx.eflags |= EFLAGS_IF;
    assert!(ctx.are_interrupts_enabled());
}

#[test]
fn loongarch64_context_and_helpers() {
    assert!(size_of::<LoongArch64ExceptionContext>() >= 320);

    let ctx = LoongArch64ExceptionContext {
        estat: (0x0C << 16) | (0x01 << 22), // Syscall / breakpoint
        ..Default::default()
    };
    assert_eq!(ctx.exception_code(), 0x0C);
    assert_eq!(ctx.sub_exception_code(), 0x01);
}
