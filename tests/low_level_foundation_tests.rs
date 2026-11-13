//! # Integration and Unit Tests for Low-Level Foundation and Hardware Abstraction

use core::sync::atomic::{AtomicI32, AtomicI64, AtomicU16, AtomicU32, AtomicU64, AtomicUsize};
use rover_uefi::arch::arm64::{
    ArmDescriptorFlags, ArmTableDescriptor, CpacrEl1, CurrentEl, Daif, MAIR_ATTR_DEVICE_NGNRE,
    MAIR_ATTR_DEVICE_NGNRNE, MAIR_ATTR_NORMAL_NON_CACHEABLE, MAIR_ATTR_NORMAL_WRITE_BACK, MairEl1,
    MidrEl1, MpidrEl1, SctlrEl1, TcrEl1,
};
use rover_uefi::arch::common::cache::{
    CACHE_LINE_SIZE_DEFAULT, cache_line_align_down, cache_line_align_up, cache_line_span,
};
use rover_uefi::arch::x86::{
    ApicBase, Cr0Flags, Cr3, Cr4Flags, EferFlags, PageTable, PageTableEntry, PageTableFlags,
    PatRegister, Xcr0Flags, canonicalize_48, canonicalize_57, page_offset_1g, page_offset_2m,
    page_offset_4k, pd_index, pdpt_index, pml4_index, pml5_index, pt_index,
};
use rover_uefi::sync::{
    InterruptState, IrqSpinLock, SpinLock, TicketLock, atomic_add_u64, atomic_add_usize,
    atomic_and_u32, atomic_compare_exchange_u16, atomic_compare_exchange_u32,
    atomic_compare_exchange_u64, atomic_decrement_i32, atomic_decrement_i64, atomic_decrement_u32,
    atomic_decrement_u64, atomic_increment_i32, atomic_increment_i64, atomic_increment_u32,
    atomic_increment_u64, atomic_or_u32, atomic_sub_u64, atomic_xor_u32, compiler_fence, cpu_pause,
    cpu_relax, disable_interrupts, instruction_barrier, memory_barrier, read_memory_barrier,
    restore_interrupts, without_interrupts, write_memory_barrier,
};

#[test]
fn test_spinlock_basic() {
    let lock = SpinLock::new(42);
    assert!(!lock.is_locked());

    {
        let mut guard = lock.lock();
        assert!(lock.is_locked());
        assert_eq!(*guard, 42);
        *guard = 100;
    }

    assert!(!lock.is_locked());
    let Some(guard) = lock.try_lock() else {
        panic!("must acquire unlocked spinlock");
    };
    assert_eq!(*guard, 100);
}

#[test]
fn test_spinlock_try_lock_and_timeout() {
    let lock = SpinLock::new("rover");
    let guard = lock.lock();

    assert!(lock.try_lock().is_none());
    assert!(lock.lock_with_timeout(50).is_none());

    drop(guard);
    assert!(lock.try_lock().is_some());
}

#[test]
fn test_spinlock_into_inner_and_get_mut() {
    let mut lock = SpinLock::new(10);
    *lock.get_mut() += 5;
    assert_eq!(lock.into_inner(), 15);
}

#[test]
fn test_ticket_lock_basic() {
    let lock = TicketLock::new(1234);
    assert!(!lock.is_locked());

    {
        let mut guard = lock.lock();
        assert!(lock.is_locked());
        assert_eq!(*guard, 1234);
        *guard = 5678;
    }

    assert!(!lock.is_locked());
    let Some(guard) = lock.try_lock() else {
        panic!("must acquire unlocked ticket lock");
    };
    assert_eq!(*guard, 5678);
}

#[test]
fn test_irq_spinlock_and_critical_section() {
    let lock = IrqSpinLock::new(vec![1, 2, 3]);
    assert!(!lock.is_locked());

    {
        let mut guard = lock.lock();
        assert!(lock.is_locked());
        guard.push(4);
    }

    assert!(!lock.is_locked());
    let Some(guard) = lock.try_lock() else {
        panic!("must acquire irq lock");
    };
    assert_eq!(*guard, vec![1, 2, 3, 4]);

    let res = without_interrupts(|| {
        let state = disable_interrupts();
        restore_interrupts(state);
        "inside_critical_section"
    });
    assert_eq!(res, "inside_critical_section");
}

#[test]
fn test_interrupt_state_struct() {
    let s1 = InterruptState::new(true);
    assert!(s1.was_enabled);
    let s2 = InterruptState::new(false);
    assert!(!s2.was_enabled);
}

#[test]
fn test_barriers_and_relax() {
    compiler_fence();
    memory_barrier();
    read_memory_barrier();
    write_memory_barrier();
    instruction_barrier();
    cpu_pause();
    cpu_relax();
}

#[test]
fn test_atomic_helpers() {
    let u32_val = AtomicU32::new(10);
    assert_eq!(atomic_increment_u32(&u32_val), 11);
    assert_eq!(atomic_decrement_u32(&u32_val), 10);
    assert_eq!(atomic_and_u32(&u32_val, 0b1010), 10);
    assert_eq!(atomic_or_u32(&u32_val, 0b0101), 15);
    assert_eq!(atomic_xor_u32(&u32_val, 0b1111), 0);

    let u64_val = AtomicU64::new(100);
    assert_eq!(atomic_increment_u64(&u64_val), 101);
    assert_eq!(atomic_decrement_u64(&u64_val), 100);
    assert_eq!(atomic_add_u64(&u64_val, 50), 150);
    assert_eq!(atomic_sub_u64(&u64_val, 25), 125);
    assert_eq!(atomic_compare_exchange_u64(&u64_val, 125, 200), 125);
    assert_eq!(u64_val.load(core::sync::atomic::Ordering::Relaxed), 200);

    let u16_val = AtomicU16::new(0xABCD);
    assert_eq!(
        atomic_compare_exchange_u16(&u16_val, 0xABCD, 0x1234),
        0xABCD
    );
    assert_eq!(
        atomic_compare_exchange_u16(&u16_val, 0x9999, 0x5678),
        0x1234
    );

    let i32_val = AtomicI32::new(-5);
    assert_eq!(atomic_increment_i32(&i32_val), -4);
    assert_eq!(atomic_decrement_i32(&i32_val), -5);

    let i64_val = AtomicI64::new(-100);
    assert_eq!(atomic_increment_i64(&i64_val), -99);
    assert_eq!(atomic_decrement_i64(&i64_val), -100);

    let usize_val = AtomicUsize::new(500);
    assert_eq!(atomic_add_usize(&usize_val, 100), 600);
    assert_eq!(atomic_compare_exchange_u32(&u32_val, 0, 99), 0);
}

#[test]
fn test_cache_line_span_and_alignment() {
    assert_eq!(cache_line_align_down(0x103F, 64), 0x1000);
    assert_eq!(cache_line_align_up(0x1001, 64), 0x1040);
    assert_eq!(cache_line_align_up(0x1040, 64), 0x1040);

    let (span_base, span_len) = cache_line_span(0x1010, 100, CACHE_LINE_SIZE_DEFAULT);
    assert_eq!(span_base, 0x1000);
    // 0x1010 + 100 = 0x1074 -> rounded up to 64 is 0x1080 -> len is 0x80 (128 bytes)
    assert_eq!(span_len, 128);

    let (zero_base, zero_len) = cache_line_span(0x1010, 0, 64);
    assert_eq!(zero_base, 0x1010);
    assert_eq!(zero_len, 0);
}

#[test]
fn test_x86_control_registers() {
    let cr0 = Cr0Flags::PROTECTION_ENABLE
        .union(Cr0Flags::WRITE_PROTECT)
        .union(Cr0Flags::PAGING);
    assert!(cr0.contains(Cr0Flags::PROTECTION_ENABLE));
    assert!(cr0.contains(Cr0Flags::WRITE_PROTECT));
    assert!(cr0.contains(Cr0Flags::PAGING));
    assert!(!cr0.contains(Cr0Flags::CACHE_DISABLE));

    let cr3 = Cr3::new(0x0010_0000, true, false);
    assert_eq!(cr3.address(), 0x0010_0000);
    assert!(cr3.is_page_write_through());
    assert!(!cr3.is_page_cache_disabled());

    let cr3_pcid = Cr3::new_with_pcid(0x0020_0000, 0x0123);
    assert_eq!(cr3_pcid.address(), 0x0020_0000);
    assert_eq!(cr3_pcid.pcid(), 0x0123);

    let cr4 = Cr4Flags::PHYSICAL_ADDRESS_EXTENSION
        .union(Cr4Flags::PAGE_GLOBAL_ENABLE)
        .union(Cr4Flags::OS_FXSR);
    assert!(cr4.contains(Cr4Flags::PHYSICAL_ADDRESS_EXTENSION));
    assert!(cr4.contains(Cr4Flags::PAGE_GLOBAL_ENABLE));
    assert!(cr4.contains(Cr4Flags::OS_FXSR));
    assert!(!cr4.contains(Cr4Flags::LINEAR_ADDRESSES_57_BIT));

    let xcr0 = Xcr0Flags::X87.union(Xcr0Flags::SSE).union(Xcr0Flags::AVX);
    assert!(xcr0.contains(Xcr0Flags::X87));
    assert!(xcr0.contains(Xcr0Flags::SSE));
    assert!(xcr0.contains(Xcr0Flags::AVX));
    assert!(!xcr0.contains(Xcr0Flags::OPMASK));
}

#[test]
fn test_x86_msr_structures() {
    let efer = EferFlags::SYSTEM_CALL_EXTENSIONS
        .union(EferFlags::LONG_MODE_ENABLE)
        .union(EferFlags::NO_EXECUTE_ENABLE);
    assert!(efer.contains(EferFlags::SYSTEM_CALL_EXTENSIONS));
    assert!(efer.contains(EferFlags::LONG_MODE_ENABLE));
    assert!(efer.contains(EferFlags::NO_EXECUTE_ENABLE));
    assert!(!efer.contains(EferFlags::SECURE_VIRTUAL_MACHINE_ENABLE));

    let apic = ApicBase {
        is_bsp: true,
        is_global_enabled: true,
        is_x2apic_enabled: true,
        base_address: 0xFEE0_0000,
    };
    let raw = apic.to_raw();
    let decoded = ApicBase::from_raw(raw);
    assert_eq!(decoded, apic);

    let pat = PatRegister::RESET_DEFAULT;
    let raw_pat = pat.to_raw();
    let Some(decoded_pat) = PatRegister::from_raw(raw_pat) else {
        panic!("must parse default PAT register");
    };
    assert_eq!(decoded_pat, pat);
}

#[test]
fn test_x86_paging_structures() {
    let vaddr = 0xFFFF_8000_1234_5678u64;
    assert_eq!(pml5_index(vaddr), 0x1FF);
    assert_eq!(pml4_index(vaddr), 0x100);
    assert_eq!(pdpt_index(vaddr), 0x000);
    assert_eq!(pd_index(vaddr), 0x091);
    assert_eq!(pt_index(vaddr), 0x145);
    assert_eq!(page_offset_4k(vaddr), 0x678);
    assert_eq!(page_offset_2m(vaddr), 0x14_5678);
    assert_eq!(page_offset_1g(vaddr), 0x1234_5678);

    assert_eq!(
        canonicalize_48(0x0000_7FFF_FFFF_FFFF),
        0x0000_7FFF_FFFF_FFFF
    );
    assert_eq!(
        canonicalize_48(0x0000_8000_0000_0000),
        0xFFFF_8000_0000_0000
    );
    assert_eq!(
        canonicalize_57(0x0100_0000_0000_0000),
        0xFF00_0000_0000_0000
    );

    let flags = PageTableFlags::PRESENT
        .union(PageTableFlags::WRITABLE)
        .union(PageTableFlags::GLOBAL);
    let mut entry = PageTableEntry::new(0x0010_0000, flags);
    assert!(entry.is_present());
    assert!(entry.is_writable());
    assert!(!entry.is_huge());
    assert_eq!(entry.address(), 0x0010_0000);

    entry.set_address(0x0020_0000);
    assert_eq!(entry.address(), 0x0020_0000);
    assert!(entry.is_present());

    let mut table = PageTable::empty();
    table[0] = entry;
    assert_eq!(table[0].address(), 0x0020_0000);
    table.clear();
    assert_eq!(table[0].address(), 0);
    assert!(!table[0].is_present());
}

#[test]
fn test_arm64_sysregs() {
    assert_eq!(CurrentEl::from_raw(0x00).level(), 0);
    assert_eq!(CurrentEl::from_raw(0x04).level(), 1);
    assert_eq!(CurrentEl::from_raw(0x08).level(), 2);
    assert_eq!(CurrentEl::from_raw(0x0C).level(), 3);

    let daif = Daif(0);
    assert!(daif.are_irqs_enabled());
    assert!(daif.are_fiqs_enabled());
    assert!(daif.are_serrors_enabled());

    let daif_masked = Daif::IRQ_MASK.union(Daif::FIQ_MASK);
    assert!(!daif_masked.are_irqs_enabled());
    assert!(!daif_masked.are_fiqs_enabled());
    assert!(daif_masked.are_serrors_enabled());

    let sctlr = SctlrEl1::MMU_ENABLE
        .union(SctlrEl1::DATA_CACHE_ENABLE)
        .union(SctlrEl1::INSTRUCTION_CACHE_ENABLE);
    assert!(sctlr.contains(SctlrEl1::MMU_ENABLE));
    assert!(sctlr.contains(SctlrEl1::DATA_CACHE_ENABLE));
    assert!(sctlr.contains(SctlrEl1::INSTRUCTION_CACHE_ENABLE));
    assert!(!sctlr.contains(SctlrEl1::ALIGNMENT_CHECK));

    let affinity_reg = MpidrEl1::from_raw(0x0000_0001_0002_0304);
    assert_eq!(affinity_reg.aff0, 4);
    assert_eq!(affinity_reg.aff1, 3);
    assert_eq!(affinity_reg.aff2, 2);
    assert_eq!(affinity_reg.aff3, 1);
    assert_eq!(affinity_reg.linear_core_id(), 0x0102_0304);

    let model_id_reg = MidrEl1::from_raw(0x410F_D034);
    assert_eq!(model_id_reg.implementer, 0x41);
    assert_eq!(model_id_reg.part_num, 0xD03);
    assert_eq!(model_id_reg.revision, 4);

    let cpacr = CpacrEl1::FP_SIMD_ENABLE;
    assert!(cpacr.contains(CpacrEl1::FP_SIMD_ENABLE));
}

#[test]
fn test_arm64_mmu_descriptors_and_registers() {
    let table = ArmTableDescriptor::new_table(0x4000_1000);
    assert!(table.is_valid());
    assert!(table.is_table());
    assert_eq!(table.output_address(), 0x4000_1000);

    let flags = ArmDescriptorFlags::AP_RW_PRIVILEGED
        .union(ArmDescriptorFlags::SH_INNER_SHAREABLE)
        .union(ArmDescriptorFlags::ACCESS_FLAG)
        .with_attr_index(MAIR_ATTR_NORMAL_WRITE_BACK);

    let block = ArmTableDescriptor::new_block(0x8000_0000, flags);
    assert!(block.is_valid());
    assert!(!block.is_table());
    assert_eq!(block.output_address(), 0x8000_0000);

    let page = ArmTableDescriptor::new_page(0x8000_4000, flags);
    assert!(page.is_valid());
    assert_eq!(page.output_address(), 0x8000_4000);

    let mair = MairEl1::DEFAULT;
    assert_eq!((mair.0 >> (MAIR_ATTR_DEVICE_NGNRNE * 8)) & 0xFF, 0x00);
    assert_eq!((mair.0 >> (MAIR_ATTR_DEVICE_NGNRE * 8)) & 0xFF, 0x04);
    assert_eq!(
        (mair.0 >> (MAIR_ATTR_NORMAL_NON_CACHEABLE * 8)) & 0xFF,
        0x44
    );
    assert_eq!((mair.0 >> (MAIR_ATTR_NORMAL_WRITE_BACK * 8)) & 0xFF, 0xFF);

    let tcr = TcrEl1::DEFAULT_48BIT;
    assert_eq!(tcr.0 & 0x3F, 16); // T0SZ
}
