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
use rover_uefi::arch::loongarch64::{CSR_CRMD, Crmd, LoongArch64ExceptionContext};
use rover_uefi::arch::riscv::{
    CSR_SATP, RiscvPte, RiscvPteFlags, Satp, SatpMode, Sstatus, sv39_page_offset, sv39_vpn0_index,
    sv39_vpn1_index, sv39_vpn2_index,
};
use rover_uefi::arch::x86::{
    ApicBase, CpuMicrocodeDate, CpuMicrocodeHeader, CpuMicrocodeProcessorSignature, Cr0Flags, Cr3,
    Cr4Flags, DescriptorTablePointer, EferFlags, Idt, IdtEntry, PageTable, PageTableEntry,
    PageTableFlags, PatRegister, SegmentDescriptor, SegmentSelector, TaskStateSegment64, Xcr0Flags,
    calculate_microcode_checksum, canonicalize_48, canonicalize_57, page_offset_1g, page_offset_2m,
    page_offset_4k, pd_index, pdpt_index, pml4_index, pml5_index, pt_index,
};
use rover_uefi::base::{
    MemoryRange, compare_bytes, compare_bytes_raw, copy_bytes, fill_bytes, move_bytes,
    secure_zero_memory, zero_bytes,
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

#[test]
fn test_x86_segment_selector_and_gdt() {
    let selector = SegmentSelector::new(2, true, 3);
    assert_eq!(selector.index(), 2);
    assert_eq!(selector.rpl(), 3);
    assert!(selector.is_ldt());

    assert_eq!(SegmentSelector::KERNEL_CODE64.bits(), 0x08);
    assert_eq!(SegmentSelector::KERNEL_CODE64.index(), 1);
    assert_eq!(SegmentSelector::KERNEL_CODE64.rpl(), 0);
    assert!(!SegmentSelector::KERNEL_CODE64.is_ldt());

    assert_eq!(SegmentSelector::KERNEL_DATA.bits(), 0x10);
    assert_eq!(SegmentSelector::USER_CODE64.rpl(), 3);
    assert_eq!(SegmentSelector::USER_DATA.rpl(), 3);

    let (tss_low, tss_high) = SegmentDescriptor::new_tss(0x1234_5678_9ABC_DEF0, 0x67);
    assert_eq!(tss_low.0 & 0xFFFF, 0x67);
    assert_eq!(tss_high.0, 0x1234_5678);

    let tss = TaskStateSegment64::new();
    assert_eq!(tss.rsp0(), 0);

    let ptr = DescriptorTablePointer::new(0xFFFF_8000_0000_1000, 0x1F);
    assert_eq!(ptr.base(), 0xFFFF_8000_0000_1000);
    assert_eq!(ptr.limit(), 0x1F);
}

#[test]
fn test_x86_idt_entry_and_idt() {
    let entry =
        IdtEntry::new_interrupt(0x1234_5678_9ABC_DEF0, SegmentSelector::KERNEL_CODE64, 0, 1);
    assert!(entry.is_present());
    assert_eq!(entry.selector(), SegmentSelector::KERNEL_CODE64);
    assert_eq!(entry.ist(), 1);
    assert_eq!(entry.handler_address(), 0x1234_5678_9ABC_DEF0);

    let mut idt = Idt::new();
    assert!(!idt.get(0x20).is_present());

    idt.set_handler(
        0x20,
        0xDEAD_BEEF_CAFE_BABE,
        SegmentSelector::KERNEL_CODE64,
        0,
    );
    assert!(idt.get(0x20).is_present());
    assert_eq!(idt.get(0x20).handler_address(), 0xDEAD_BEEF_CAFE_BABE);

    let idt_ptr = idt.pointer();
    assert_ne!(idt_ptr.base(), 0);
    let Ok(expected_limit) = u16::try_from(core::mem::size_of::<Idt>() - 1) else {
        panic!("IDT limit fits in u16");
    };
    assert_eq!(idt_ptr.limit(), expected_limit);
}

#[test]
fn test_riscv_satp_and_sstatus() {
    let satp = Satp::new(SatpMode::Sv39, 42, 0x8020_0000 >> 12);
    assert_eq!(satp.mode(), SatpMode::Sv39);
    assert_eq!(satp.asid(), 42);
    assert_eq!(satp.ppn(), 0x8020_0000 >> 12);

    let sstatus = Sstatus::SIE | Sstatus::SUM;
    assert!(sstatus.contains(Sstatus::SIE));
    assert!(sstatus.contains(Sstatus::SUM));
    assert!(!sstatus.contains(Sstatus::SPP));
    assert_eq!(CSR_SATP, 0x180);
}

#[test]
fn test_riscv_paging_sv39() {
    let vaddr = 0x0000_007F_FFFF_F123;
    assert_eq!(sv39_vpn2_index(vaddr), 511);
    assert_eq!(sv39_vpn1_index(vaddr), 511);
    assert_eq!(sv39_vpn0_index(vaddr), 511);
    assert_eq!(sv39_page_offset(vaddr), 0x123);

    let table_pte = RiscvPte::new_table(0x8000_1000);
    assert!(table_pte.is_valid());
    assert!(table_pte.is_branch());
    assert!(!table_pte.is_leaf());
    assert_eq!(table_pte.physical_address(), 0x8000_1000);

    let leaf_flags = RiscvPteFlags::READABLE
        | RiscvPteFlags::WRITABLE
        | RiscvPteFlags::ACCESSED
        | RiscvPteFlags::DIRTY;
    let leaf_pte = RiscvPte::new_leaf(0x8000_2000, leaf_flags);
    assert!(leaf_pte.is_valid());
    assert!(leaf_pte.is_leaf());
    assert!(!leaf_pte.is_branch());
    assert_eq!(leaf_pte.physical_address(), 0x8000_2000);
}

#[test]
fn test_loongarch64_crmd_and_context() {
    let crmd = Crmd::PG | Crmd::IE;
    assert!(crmd.is_paging_enabled());
    assert!(crmd.is_interrupts_enabled());
    assert_eq!(crmd.privilege_level(), 0);
    assert_eq!(CSR_CRMD, 0x0000);

    let ctx = LoongArch64ExceptionContext {
        estat: (0x0Bu64 << 16) | (0x02u64 << 22),
        ..Default::default()
    };
    assert_eq!(ctx.exception_code(), 0x0B);
    assert_eq!(ctx.sub_exception_code(), 0x02);
}

#[test]
fn test_memory_range_abstractions() {
    let range = MemoryRange::new(0x1000, 0x2000);
    assert_eq!(range.start(), 0x1000);
    assert_eq!(range.length(), 0x2000);
    assert_eq!(range.end(), 0x3000);
    assert!(!range.is_empty());

    assert!(range.contains_address(0x1000));
    assert!(range.contains_address(0x2FFF));
    assert!(!range.contains_address(0x3000));
    assert!(!range.contains_address(0x0FFF));

    let sub = MemoryRange::new(0x1500, 0x0500);
    assert!(range.contains_range(&sub));

    let overlapping = MemoryRange::new(0x2500, 0x1000);
    assert!(range.overlaps(&overlapping));

    let disjoint = MemoryRange::new(0x4000, 0x1000);
    assert!(!range.overlaps(&disjoint));

    let Some((left, right)) = range.split_at(0x800) else {
        panic!("split failed");
    };
    assert_eq!(left.start(), 0x1000);
    assert_eq!(left.length(), 0x800);
    assert_eq!(right.start(), 0x1800);
    assert_eq!(right.length(), 0x1800);

    assert!(range.is_aligned(0x1000));
    assert!(!range.is_aligned(0x2000));

    let unaligned = MemoryRange::new(0x1005, 0x1000);
    let aligned = unaligned.align_outward(0x1000);
    assert_eq!(aligned.start(), 0x1000);
    assert_eq!(aligned.end(), 0x3000);
}

#[test]
fn test_memory_primitives_copy_move_fill_zero() {
    let src = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let mut dst = [0u8; 8];

    unsafe {
        copy_bytes(dst.as_mut_ptr(), src.as_ptr(), 8);
    }
    assert_eq!(dst, src);

    assert_eq!(
        unsafe { compare_bytes(dst.as_ptr(), src.as_ptr(), 8) },
        core::cmp::Ordering::Equal
    );
    assert_eq!(
        unsafe { compare_bytes_raw(dst.as_ptr(), src.as_ptr(), 8) },
        0
    );

    // Overlapping move backwards
    let mut buffer = [1u8, 2, 3, 4, 5, 6, 7, 8];
    unsafe {
        move_bytes(buffer.as_mut_ptr().add(2), buffer.as_ptr(), 4);
    }
    assert_eq!(buffer, [1, 2, 1, 2, 3, 4, 7, 8]);

    // Overlapping move forwards
    let mut buffer2 = [1u8, 2, 3, 4, 5, 6, 7, 8];
    unsafe {
        move_bytes(buffer2.as_mut_ptr(), buffer2.as_ptr().add(2), 4);
    }
    assert_eq!(buffer2, [3, 4, 5, 6, 5, 6, 7, 8]);

    // Fill bytes
    unsafe {
        fill_bytes(dst.as_mut_ptr(), 0xAA, 8);
    }
    assert_eq!(dst, [0xAA; 8]);

    // Zero bytes
    unsafe {
        zero_bytes(dst.as_mut_ptr(), 8);
    }
    assert_eq!(dst, [0u8; 8]);

    // Secure zero memory
    let mut secret = [0x55u8; 16];
    unsafe {
        secure_zero_memory(secret.as_mut_ptr(), 16);
    }
    assert_eq!(secret, [0u8; 16]);
}

#[test]
fn test_x86_microcode_patch_structures() {
    let date = CpuMicrocodeDate::new(0x2024, 0x07, 0x15);
    assert_eq!(date.decoded_year(), 2024);
    assert_eq!(date.decoded_month(), 7);
    assert_eq!(date.decoded_day(), 15);

    // Signature for Intel Core i7 (Haswell / Broadwell / Skylake example)
    let sig = CpuMicrocodeProcessorSignature::from_raw(0x0008_06E9);
    assert_eq!(sig.stepping(), 0x9);
    assert_eq!(sig.model(), 0xE);
    assert_eq!(sig.family(), 0x6);
    assert_eq!(sig.extended_model(), 0x8);
    assert_eq!(sig.extended_family(), 0x0);
    assert_eq!(sig.effective_family(), 6);
    assert_eq!(sig.effective_model(), 0x8E);

    let dwords = [0x1234_5678, 0x9ABC_DEF0, 0x1122_3344];
    let chk = calculate_microcode_checksum(&dwords);
    let mut all_dwords = [0u32; 4];
    all_dwords[0..3].copy_from_slice(&dwords);
    all_dwords[3] = chk;

    let mut sum: u32 = 0;
    for &dw in &all_dwords {
        sum = sum.wrapping_add(dw);
    }
    assert_eq!(sum, 0);

    let mut raw_header = [0u8; 2048];
    // Populate header
    let header_ptr = raw_header.as_mut_ptr().cast::<CpuMicrocodeHeader>();
    unsafe {
        (*header_ptr).header_version = 1;
        (*header_ptr).update_revision = 0x10;
        (*header_ptr).date = date;
        (*header_ptr).processor_signature = sig;
        (*header_ptr).loader_revision = 1;
        (*header_ptr).processor_flags = 0x02; // Platform ID bit 1
        (*header_ptr).data_size = 2000;
        (*header_ptr).total_size = 2048;
    }

    // Calculate whole patch checksum
    let mut sum: u32 = 0;
    let mut offset = 0;
    while offset < 2048 {
        if offset != 16 {
            // Checksum field offset is 16
            let chunk = [
                raw_header[offset],
                raw_header[offset + 1],
                raw_header[offset + 2],
                raw_header[offset + 3],
            ];
            sum = sum.wrapping_add(u32::from_le_bytes(chunk));
        }
        offset += 4;
    }
    let patch_chk = 0u32.wrapping_sub(sum);
    unsafe {
        (*header_ptr).checksum = patch_chk;
    }

    let header = unsafe { &*header_ptr };
    assert!(header.verify_checksum(&raw_header));
    assert!(header.matches_processor(0x0008_06E9, 0x02));
    assert!(!header.matches_processor(0x0008_06E9, 0x04));
    assert!(!header.matches_processor(0x0009_06EA, 0x02));
}
