//! Tests for the architecture and hardware abstraction layer.
//!
//! The tests exercise the register layouts, the offset
//! computations and the feature decoding of the three supported
//! architectures. The processor identification is tested with a
//! recorded source, so that the decoding can be checked without
//! depending on the processor that runs the test.

use core::mem::{align_of, offset_of, size_of};

use rover_uefi::arch::arm64::context::{
    ELR_EL1, ESR_EL1, EXCEPTION_VECTOR_ENTRY_SIZE, ExceptionClass, ExceptionContext,
    ExceptionSource, FAR_EL1, PAR_EL1, SP_EL0, SP_EL1, SP_ELX, SPSR_EL1, TPIDR_EL0, TPIDRRO_EL0,
    VBAR_EL1, exception_vector_index, exception_vector_offset,
};
use rover_uefi::arch::arm64::gic::{
    GICC_BPR, GICC_CTLR, GICC_DIR, GICC_EOIR, GICC_HPPIR, GICC_IAR, GICC_IIDR, GICC_PMR, GICC_RPR,
    GICD_CTLR, GICD_ICENABLER, GICD_ICFGR_V2, GICD_ICFGR_V3, GICD_ICPENDR, GICD_IGROUPR,
    GICD_IPRIORITYR, GICD_ISENABLER, GICD_ISPENDR, GICD_ITARGETSR, GICD_SGIR, GICD_TYPER,
    GicCpuInterface, GicDistributor, GicVersion, ICC_BPR1_EL1, ICC_CTLR_EL1, ICC_DIR_EL1,
    ICC_EOIR_EL1, ICC_HPPIR_EL1, ICC_PRIO_MASK_EL1, ICC_RPR_EL1, ICC_SGI1R_EL1, ICC_SRE_EL1,
};
use rover_uefi::arch::common::dma::{
    DMA_MAX_SEGMENTS, DMA_MIN_ALIGNMENT, DMA_PAGE_ALIGNMENT, DmaBuffer, DmaDirection, DmaError,
    DmaSegment, DmaSegmentList, contiguous_buffer, contiguous_segment,
};
use rover_uefi::arch::common::timer::{
    MILLIS_PER_SEC, MonotonicTimer, NANOS_PER_SEC, TickAccumulator, Timer, TimerError,
    millis_to_nanos, nanos_to_millis, nanos_to_ticks, secs_to_nanos, ticks_to_nanos,
};
use rover_uefi::arch::riscv::context::{
    SCAUSE_ILLEGAL_INSTRUCTION, SCAUSE_INTERRUPT_BIT, SCAUSE_LOAD_PAGE_FAULT,
    SCAUSE_SUPERVISOR_ECALL, TrapCause, TrapContext, exception_code, is_interrupt,
};
use rover_uefi::arch::riscv::sbi::{
    SBI_ERR_DENIED, SBI_ERR_INVALID_PARAM, SBI_ERR_NOT_SUPPORTED, SBI_EXTENSION_BASE,
    SBI_EXTENSION_TIME, SBI_SUCCESS, SBI_TIME_FN_SET_TIMER_STIME, SbiEcall, SbiError, SbiRet,
};
use rover_uefi::arch::x86::apic::{
    APIC_EOI, APIC_ICR_HIGH, APIC_ICR_LOW, APIC_ID, APIC_LVT_TIMER, APIC_SVR, APIC_TIMER_CURCNT,
    APIC_TIMER_DIV, APIC_TIMER_INITCNT, APIC_TPR, DeliveryMode, DestinationMode,
    IO_APIC_DATA_OFFSET, IO_APIC_DELIVERY_MODE_MASK, IO_APIC_DELIVERY_MODE_SHIFT,
    IO_APIC_DESTINATION_MASK, IO_APIC_DESTINATION_SHIFT, IO_APIC_INDEX_OFFSET, IO_APIC_MASKED_MASK,
    IO_APIC_POLARITY_MASK, IO_APIC_REDIRECTION_INDEX, IO_APIC_TRIGGER_MODE_MASK,
    IO_APIC_VECTOR_MASK, IoApic, IoApicRedirection, IpiRequest, Polarity, TriggerMode,
};
use rover_uefi::arch::x86::context::{
    EXCEPTION_VECTOR_BREAKPOINT, EXCEPTION_VECTOR_DEBUG, EXCEPTION_VECTOR_DIVIDE_ERROR,
    EXCEPTION_VECTOR_DOUBLE_FAULT, EXCEPTION_VECTOR_INVALID_OPCODE, EXCEPTION_VECTOR_PAGE_FAULT,
    EXCEPTION_VECTOR_SPURIOUS, GeneralRegisters, InterruptContext, SegmentSelectors, SystemContext,
    has_error_code, is_exception,
};
use rover_uefi::arch::x86::cpuid::{
    CpuInfo, Cpuid, CpuidResult, HostCpuid, Leaf1Features, Leaf7Features, Leaf7Subleaf1Features,
    StubCpuid,
};

/// A recorded source of processor identification leaves.
struct FakeCpuid {
    leaves: Vec<(u32, u32, CpuidResult)>,
}

impl FakeCpuid {
    /// Creates a source without any recorded leaf.
    fn new() -> Self {
        Self { leaves: Vec::new() }
    }

    /// Records the result of a leaf.
    fn set(mut self, leaf: u32, subleaf: u32, result: CpuidResult) -> Self {
        self.leaves.push((leaf, subleaf, result));
        self
    }
}

impl Cpuid for FakeCpuid {
    fn leaf(&self, leaf: u32, subleaf: u32) -> CpuidResult {
        for (known, known_subleaf, result) in &self.leaves {
            if *known == leaf && *known_subleaf == subleaf {
                return *result;
            }
        }
        CpuidResult::default()
    }
}

/// Builds a 32-bit register image from a slice of a longer text.
fn le_chunk(text: &[u8], offset: usize) -> u32 {
    let mut bytes = [0u8; 4];
    for (index, slot) in bytes.iter_mut().enumerate() {
        if let Some(byte) = text.get(offset.saturating_add(index)) {
            *slot = *byte;
        }
    }
    u32::from_le_bytes(bytes)
}

#[test]
fn cpuid_leaf_one_features() {
    // The feature register holds the time stamp counter, the local
    // interrupt controller and the processor identification bits.
    let edx = (1 << 4) | (1 << 9) | (1 << 21);
    // The extended feature register holds the instruction set
    // extensions and the state save support.
    let ecx = (1 << 9)
        | (1 << 12)
        | (1 << 19)
        | (1 << 20)
        | (1 << 25)
        | (1 << 26)
        | (1 << 27)
        | (1 << 28)
        | (1 << 30);

    let features = Leaf1Features::new(edx, ecx);
    assert!(features.tsc());
    assert!(features.apic());
    assert!(features.sse2());
    assert!(!features.fpu());
    assert!(!features.pse());
    assert!(features.ssse3());
    assert!(features.fma());
    assert!(features.sse41());
    assert!(features.sse42());
    assert!(features.aes());
    assert!(features.xsave());
    assert!(features.osxsave());
    assert!(features.avx());
    assert!(features.rdrand());
    assert!(!features.hypervisor());
    assert_eq!(features.raw_edx(), edx);
    assert_eq!(features.raw_ecx(), ecx);
}

#[test]
fn cpuid_structured_features() {
    // The first register of the structured leaf holds the bit
    // manipulation, the vector, the execution prevention and the
    // hash algorithm extensions.
    let ebx = (1 << 0)
        | (1 << 3)
        | (1 << 5)
        | (1 << 7)
        | (1 << 8)
        | (1 << 9)
        | (1 << 10)
        | (1 << 11)
        | (1 << 14)
        | (1 << 16)
        | (1 << 17)
        | (1 << 27)
        | (1 << 29);
    let edx = 1 << 18;

    let features = Leaf7Features::new(ebx, edx);
    assert!(features.fsgsbase());
    assert!(features.bmi1());
    assert!(features.avx2());
    assert!(features.smep());
    assert!(features.bmi2());
    assert!(features.erms());
    assert!(features.invpcid());
    assert!(features.fsrm());
    assert!(features.rdseed());
    assert!(features.smap());
    assert!(features.clflushopt());
    assert!(features.sha());
    assert!(features.cet_ss());
    assert!(features.cet_ift());
    assert!(features.cet());
    assert!(!features.rtm());
    assert!(!features.mpx());

    let subleaf = Leaf7Subleaf1Features::new((1 << 0) | (1 << 1));
    assert!(subleaf.sha512());
    assert!(subleaf.sm3());
    assert!(!subleaf.sm4());
}

#[test]
fn cpuid_vendor_and_brand_strings() {
    let brand = b"Rover Test";
    let cpuid = FakeCpuid::new()
        .set(
            0,
            0,
            CpuidResult::new(0x16, 0x756E_6547, 0x6C65_746E, 0x4965_6E69),
        )
        .set(1, 0, CpuidResult::new(0x0000_0F00, 0, 0, 0))
        .set(0x8000_0000, 0, CpuidResult::new(0x8000_0004, 0, 0, 0))
        .set(
            0x8000_0002,
            0,
            CpuidResult::new(
                le_chunk(brand, 0),
                le_chunk(brand, 4),
                le_chunk(brand, 8),
                le_chunk(brand, 12),
            ),
        )
        .set(0x8000_0003, 0, CpuidResult::new(0, 0, 0, 0))
        .set(0x8000_0004, 0, CpuidResult::new(0, 0, 0, 0));

    let info = CpuInfo::collect(&cpuid);
    assert_eq!(info.max_basic_leaf, 0x16);
    assert_eq!(info.max_extended_leaf, 0x8000_0004);
    assert_eq!(info.vendor_string(), Some("GenuineIntel"));
    assert_eq!(info.brand_string(), Some("Rover Test"));
    assert_eq!(info.vendor.as_bytes()[0], b'G');
    assert_eq!(info.stepping, 0);
    assert_eq!(info.family, 0x0F);
    assert_eq!(info.model, 0x00);
}

#[test]
fn cpuid_max_leaf_limits_the_query() {
    // A processor that only implements the identification leaf
    // must not report any feature and must not read the brand.
    let cpuid = FakeCpuid::new().set(
        0,
        0,
        CpuidResult::new(0, 0x756E_6547, 0x6C65_746E, 0x4965_6E69),
    );

    let info = CpuInfo::collect(&cpuid);
    assert_eq!(info.max_basic_leaf, 0);
    assert_eq!(info.max_extended_leaf, 0);
    assert_eq!(info.vendor_string(), Some("GenuineIntel"));
    assert_eq!(info.brand_string(), Some(""));
    assert!(!info.leaf1.tsc());
    assert!(!info.leaf1.apic());
    assert!(!info.leaf7.bmi2());
    assert!(!info.extended.lm());
    assert!(!info.extended.nx());
    assert!(!info.xsave.xsaveopt());
    assert_eq!(info.xsave.xcr0_supported(), 0);
}

#[test]
fn cpuid_extended_features_and_state() {
    let cpuid = FakeCpuid::new()
        .set(0, 0, CpuidResult::new(0x0000_000D, 0, 0, 0))
        .set(1, 0, CpuidResult::new(0, 0, 0x0000_0001, 0))
        .set(7, 0, CpuidResult::new(1, 1 << 8, 0, 0))
        .set(7, 1, CpuidResult::new(1 << 0, 0, 0, 0))
        .set(
            0xD,
            0,
            CpuidResult::new(0x0000_0007, 0x0000_00E7, 0x0000_02A0, 0),
        )
        .set(0xD, 1, CpuidResult::new(0, 0, 1, 0))
        .set(0xD, 2, CpuidResult::new(0, 0, 1, 0))
        .set(0x8000_0000, 0, CpuidResult::new(0x8000_0001, 0, 0, 0))
        .set(
            0x8000_0001,
            0,
            CpuidResult::new(0, 0, 0, (1 << 0) | (1 << 20) | (1 << 29)),
        );

    let info = CpuInfo::collect(&cpuid);
    assert_eq!(info.max_basic_leaf, 0x0D);
    assert!(info.leaf7.bmi2());
    assert!(info.leaf7_subleaf1.sha512());
    assert!(info.extended.syscall());
    assert!(info.extended.nx());
    assert!(info.extended.lm());
    assert!(!info.extended.rdtscp());
    assert!(info.xsave.xsaveopt());
    assert!(info.xsave.xsaves());
    assert_eq!(info.xsave.xcr0_valid(), 0x0000_0007);
    assert_eq!(info.xsave.xcr0_supported(), 0x0000_00E7);
    assert_eq!(info.xsave.xsave_area_size(), 0x02A0);
    assert!(info.xsave.ymm_state());
    assert!(info.xsave.opmask_state());
    assert!(info.xsave.zmm_hi256_state());
    assert!(!info.xsave.pkru_state());
    assert!(!info.xsave.pasid_state());
}

#[test]
fn cpuid_stub_and_host_sources() {
    let stub = StubCpuid;
    assert!(stub.leaf(1, 0).is_zero());
    assert!(stub.leaf(7, 0).is_zero());

    let host = HostCpuid;
    let identification = host.leaf(0, 0);

    // On x86_64 hosts, HostCpuid executes real cpuid instructions.
    // On non-x86_64 architectures (such as Apple Silicon aarch64 on macos-latest),
    // HostCpuid falls back to StubCpuid and returns zeros.
    #[cfg(target_arch = "x86_64")]
    {
        assert!(identification.eax >= 1);
        let version = host.leaf(1, 0);
        assert!(version.edx & (1 << 4) != 0);
        assert!(version.edx & (1 << 5) != 0);
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        assert!(identification.is_zero());
    }
}

#[test]
fn apic_register_offsets() {
    assert_eq!(APIC_ID, 0x020);
    assert_eq!(APIC_TPR, 0x080);
    assert_eq!(APIC_EOI, 0x0B0);
    assert_eq!(APIC_SVR, 0x0F0);
    assert_eq!(APIC_ICR_LOW, 0x300);
    assert_eq!(APIC_ICR_HIGH, 0x310);
    assert_eq!(APIC_LVT_TIMER, 0x320);
    assert_eq!(APIC_TIMER_INITCNT, 0x380);
    assert_eq!(APIC_TIMER_CURCNT, 0x390);
    assert_eq!(APIC_TIMER_DIV, 0x3E0);
}

#[test]
fn io_apic_register_offsets() {
    assert_eq!(IO_APIC_INDEX_OFFSET, 0x010);
    assert_eq!(IO_APIC_DATA_OFFSET, 0x011);
    assert_eq!(IO_APIC_REDIRECTION_INDEX, 0x010);
    assert_eq!(IoApic::redirection_index(0), 0x010);
    assert_eq!(IoApic::redirection_index(1), 0x012);
    assert_eq!(IoApic::redirection_index(16), 0x030);
    assert_eq!(IoApic::redirection_index(23), 0x03E);
}

#[test]
fn io_apic_redirection_round_trip() {
    let entry = IoApicRedirection {
        vector: 0x2A,
        delivery_mode: DeliveryMode::LowestPriority,
        destination_mode: DestinationMode::Physical,
        polarity: Polarity::ActiveLow,
        trigger_mode: TriggerMode::Level,
        masked: true,
        destination: 0x05,
    };

    let encoded = entry.encode();
    assert_eq!(encoded & IO_APIC_VECTOR_MASK, 0x2A);
    assert_eq!(
        (encoded & IO_APIC_DELIVERY_MODE_MASK) >> IO_APIC_DELIVERY_MODE_SHIFT,
        1
    );
    assert_ne!(encoded & IO_APIC_POLARITY_MASK, 0);
    assert_ne!(encoded & IO_APIC_TRIGGER_MODE_MASK, 0);
    assert_ne!(encoded & IO_APIC_MASKED_MASK, 0);
    assert_eq!(
        (encoded & IO_APIC_DESTINATION_MASK) >> IO_APIC_DESTINATION_SHIFT,
        0x05
    );

    let decoded = IoApicRedirection::decode(encoded);
    assert_eq!(decoded, entry);
    assert_eq!(decoded.vector, 0x2A);
    assert_eq!(decoded.delivery_mode, DeliveryMode::LowestPriority);
    assert_eq!(decoded.destination_mode, DestinationMode::Physical);
    assert_eq!(decoded.polarity, Polarity::ActiveLow);
    assert_eq!(decoded.trigger_mode, TriggerMode::Level);
    assert!(decoded.masked);
    assert_eq!(decoded.destination, 0x05);
}

#[test]
fn io_apic_redirection_defaults() {
    let entry = IoApicRedirection::new(0x20, 0x01, DeliveryMode::Fixed);
    assert_eq!(entry.delivery_mode, DeliveryMode::Fixed);
    assert_eq!(entry.destination_mode, DestinationMode::Physical);
    assert_eq!(entry.polarity, Polarity::ActiveHigh);
    assert_eq!(entry.trigger_mode, TriggerMode::Edge);
    assert!(!entry.masked);

    let decoded = IoApicRedirection::decode(entry.encode());
    assert_eq!(decoded, entry);
}

#[test]
fn ipi_command_encoding() {
    let request = IpiRequest::new(0x30, 0x02, DeliveryMode::Fixed);
    let low = request.encode_low();
    let high = request.encode_high();
    assert_eq!(low & 0xFF, 0x30);
    assert_eq!((low >> 8) & 0x07, 0);
    assert_eq!((high >> 24) & 0xFF, 0x02);
}

#[test]
fn x86_interrupt_context_layout() {
    assert_eq!(size_of::<SegmentSelectors>(), 12);
    assert_eq!(align_of::<SegmentSelectors>(), 2);
    assert_eq!(size_of::<GeneralRegisters>(), 32);
    assert_eq!(align_of::<GeneralRegisters>(), 4);

    assert_eq!(size_of::<InterruptContext>(), 64);
    assert_eq!(align_of::<InterruptContext>(), 4);
    assert_eq!(offset_of!(InterruptContext, segments), 0);
    assert_eq!(offset_of!(InterruptContext, general), 12);
    assert_eq!(offset_of!(InterruptContext, eip), 44);
    assert_eq!(offset_of!(InterruptContext, eflags), 48);
    assert_eq!(offset_of!(InterruptContext, error_code), 52);
    assert_eq!(offset_of!(InterruptContext, trap_number), 56);
    assert_eq!(offset_of!(InterruptContext, cr2), 60);

    assert_eq!(offset_of!(SegmentSelectors, cs), 0);
    assert_eq!(offset_of!(SegmentSelectors, ss), 10);
    assert_eq!(offset_of!(GeneralRegisters, eax), 0);
    assert_eq!(offset_of!(GeneralRegisters, edi), 28);
}

#[test]
fn x86_system_context_layout() {
    assert_eq!(size_of::<SystemContext>(), 144);
    assert_eq!(align_of::<SystemContext>(), 8);
    assert_eq!(offset_of!(SystemContext, rsp), 0);
    assert_eq!(offset_of!(SystemContext, rip), 8);
    assert_eq!(offset_of!(SystemContext, rflags), 16);
    assert_eq!(offset_of!(SystemContext, rbp), 24);
    assert_eq!(offset_of!(SystemContext, r8), 32);
    assert_eq!(offset_of!(SystemContext, r15), 88);
    assert_eq!(offset_of!(SystemContext, cr2), 96);
    assert_eq!(offset_of!(SystemContext, gs_base), 104);
    assert_eq!(offset_of!(SystemContext, fs_base), 112);
    assert_eq!(offset_of!(SystemContext, cs), 120);
    assert_eq!(offset_of!(SystemContext, ss), 130);
    assert_eq!(offset_of!(SystemContext, error_code), 132);
    assert_eq!(offset_of!(SystemContext, vector), 136);
}

#[test]
fn x86_exception_vectors() {
    assert_eq!(EXCEPTION_VECTOR_DIVIDE_ERROR, 0);
    assert_eq!(EXCEPTION_VECTOR_DEBUG, 1);
    assert_eq!(EXCEPTION_VECTOR_BREAKPOINT, 3);
    assert_eq!(EXCEPTION_VECTOR_INVALID_OPCODE, 6);
    assert_eq!(EXCEPTION_VECTOR_DOUBLE_FAULT, 8);
    assert_eq!(EXCEPTION_VECTOR_PAGE_FAULT, 14);
    assert_eq!(EXCEPTION_VECTOR_SPURIOUS, 255);

    assert!(is_exception(EXCEPTION_VECTOR_PAGE_FAULT));
    assert!(!is_exception(32));
    assert!(has_error_code(EXCEPTION_VECTOR_PAGE_FAULT));
    assert!(has_error_code(EXCEPTION_VECTOR_DOUBLE_FAULT));
    assert!(!has_error_code(EXCEPTION_VECTOR_BREAKPOINT));
}

#[test]
fn gic_distributor_offsets() {
    assert_eq!(GICD_CTLR, 0x000);
    assert_eq!(GICD_TYPER, 0x004);
    assert_eq!(GICD_IGROUPR, 0x080);
    assert_eq!(GICD_ISENABLER, 0x100);
    assert_eq!(GICD_ICENABLER, 0x180);
    assert_eq!(GICD_ISPENDR, 0x200);
    assert_eq!(GICD_ICPENDR, 0x280);
    assert_eq!(GICD_IPRIORITYR, 0x400);
    assert_eq!(GICD_ITARGETSR, 0x800);
    assert_eq!(GICD_ICFGR_V2, 0xC00);
    assert_eq!(GICD_ICFGR_V3, 0x800);
    assert_eq!(GICD_SGIR, 0xF00);
}

#[test]
fn gic_processor_interface_offsets() {
    assert_eq!(GICC_CTLR, 0x000);
    assert_eq!(GICC_PMR, 0x004);
    assert_eq!(GICC_BPR, 0x008);
    assert_eq!(GICC_IAR, 0x00C);
    assert_eq!(GICC_EOIR, 0x010);
    assert_eq!(GICC_RPR, 0x014);
    assert_eq!(GICC_HPPIR, 0x018);
    assert_eq!(GICC_IIDR, 0x0FC);
    assert_eq!(GICC_DIR, 0x1000);
}

#[test]
fn gic_system_register_encodings() {
    assert_eq!(ICC_PRIO_MASK_EL1.encode(), 0x0018_4040);
    assert_eq!(ICC_EOIR_EL1.encode(), 0x0018_C100);
    assert_eq!(ICC_DIR_EL1.encode(), 0x0018_C120);
    assert_eq!(ICC_SRE_EL1.encode(), 0x0018_C160);
    assert_eq!(ICC_HPPIR_EL1.encode(), 0x0018_C200);
    assert_eq!(ICC_RPR_EL1.encode(), 0x0018_C220);
    assert_eq!(ICC_BPR1_EL1.encode(), 0x0018_C360);
    assert_eq!(ICC_SGI1R_EL1.encode(), 0x0018_C1A0);
    assert_eq!(ICC_CTLR_EL1.encode(), 0x0018_CD80);
}

#[test]
fn gic_exception_system_register_encodings() {
    assert_eq!(SPSR_EL1.encode(), 0x0018_4040);
    assert_eq!(ELR_EL1.encode(), 0x0018_4020);
    assert_eq!(ESR_EL1.encode(), 0x0018_5200);
    assert_eq!(FAR_EL1.encode(), 0x0018_6000);
    assert_eq!(PAR_EL1.encode(), 0x0018_7000);
    assert_eq!(VBAR_EL1.encode(), 0x0018_C000);
    assert_eq!(TPIDR_EL0.encode(), 0x001B_D040);
    assert_eq!(TPIDRRO_EL0.encode(), 0x001B_D060);
    assert_eq!(SP_EL0.encode(), 0x0018_D000);
    assert_eq!(SP_EL1.encode(), 0x0018_D080);
    assert_eq!(SP_ELX.encode(), 0x0018_D0C0);
}

#[test]
fn gic_views_expose_the_offsets() {
    let distributor = GicDistributor::new(0x8000_0000, GicVersion::Third);
    assert_eq!(distributor.base(), 0x8000_0000);
    assert_eq!(distributor.version(), GicVersion::Third);
    assert_eq!(distributor.configuration_offset(), GICD_ICFGR_V3);
    assert_eq!(distributor.address(GICD_TYPER), 0x8000_0004);

    let legacy = GicDistributor::new(0x8000_0000, GicVersion::Second);
    assert_eq!(legacy.configuration_offset(), GICD_ICFGR_V2);

    let (offset, bit) = GicDistributor::register_and_bit(GICD_ISENABLER, 33);
    assert_eq!(offset, 0x104);
    assert_eq!(bit, 1);

    let interface = GicCpuInterface::new(0x8000_2000);
    assert_eq!(interface.base(), 0x8000_2000);
    assert_eq!(interface.address(GICC_EOIR), 0x8000_2010);
}

#[test]
fn arm64_exception_context_layout() {
    assert_eq!(size_of::<ExceptionContext>(), 80);
    assert_eq!(align_of::<ExceptionContext>(), 8);
    assert_eq!(offset_of!(ExceptionContext, elr_el1), 0);
    assert_eq!(offset_of!(ExceptionContext, sp_el0), 8);
    assert_eq!(offset_of!(ExceptionContext, sp_el1), 16);
    assert_eq!(offset_of!(ExceptionContext, elr_el0), 24);
    assert_eq!(offset_of!(ExceptionContext, esr_el1), 32);
    assert_eq!(offset_of!(ExceptionContext, far_el1), 40);
    assert_eq!(offset_of!(ExceptionContext, par_el1), 48);
    assert_eq!(offset_of!(ExceptionContext, tpidr_el0), 56);
    assert_eq!(offset_of!(ExceptionContext, tpidrro_el0), 64);
    assert_eq!(offset_of!(ExceptionContext, sp_elx), 72);
}

#[test]
fn arm64_exception_vector_table() {
    assert_eq!(EXCEPTION_VECTOR_ENTRY_SIZE, 0x200);

    assert_eq!(
        exception_vector_offset(
            ExceptionSource::CurrentLevelStackLower,
            ExceptionClass::Synchronous
        ),
        0x000
    );
    assert_eq!(
        exception_vector_offset(
            ExceptionSource::CurrentLevelStackLower,
            ExceptionClass::Interrupt
        ),
        0x200
    );
    assert_eq!(
        exception_vector_offset(
            ExceptionSource::CurrentLevelStackCurrent,
            ExceptionClass::AsynchronousError
        ),
        0xE00
    );
    assert_eq!(
        exception_vector_offset(
            ExceptionSource::LowerLevelSixtyFour,
            ExceptionClass::Synchronous
        ),
        0x1000
    );
    assert_eq!(
        exception_vector_offset(
            ExceptionSource::LowerLevelThirtyTwo,
            ExceptionClass::AsynchronousError
        ),
        0x1E00
    );

    assert_eq!(
        exception_vector_index(
            ExceptionSource::LowerLevelThirtyTwo,
            ExceptionClass::AsynchronousError
        ),
        15
    );
    assert_eq!(
        exception_vector_index(
            ExceptionSource::CurrentLevelStackLower,
            ExceptionClass::Synchronous
        ),
        0
    );
}

#[test]
fn riscv_trap_context_layout() {
    let word = size_of::<usize>();
    assert_eq!(size_of::<TrapContext>(), 38 * word);
    assert_eq!(align_of::<TrapContext>(), word);
    assert_eq!(offset_of!(TrapContext, ra), 0);
    assert_eq!(offset_of!(TrapContext, sp), word);
    assert_eq!(offset_of!(TrapContext, gp), 2 * word);
    assert_eq!(offset_of!(TrapContext, tp), 3 * word);
    assert_eq!(offset_of!(TrapContext, t0), 4 * word);
    assert_eq!(offset_of!(TrapContext, s0), 7 * word);
    assert_eq!(offset_of!(TrapContext, a0), 9 * word);
    assert_eq!(offset_of!(TrapContext, a7), 16 * word);
    assert_eq!(offset_of!(TrapContext, s2), 17 * word);
    assert_eq!(offset_of!(TrapContext, s11), 26 * word);
    assert_eq!(offset_of!(TrapContext, t3), 27 * word);
    assert_eq!(offset_of!(TrapContext, t6), 30 * word);
    assert_eq!(offset_of!(TrapContext, sstatus), 31 * word);
    assert_eq!(offset_of!(TrapContext, sepc), 32 * word);
    assert_eq!(offset_of!(TrapContext, scause), 33 * word);
    assert_eq!(offset_of!(TrapContext, stval), 34 * word);
    assert_eq!(offset_of!(TrapContext, stvec), 35 * word);
    assert_eq!(offset_of!(TrapContext, sie), 36 * word);
    assert_eq!(offset_of!(TrapContext, sip), 37 * word);
}

#[test]
fn riscv_scause_decoding() {
    assert_eq!(
        TrapCause::decode(0),
        TrapCause::InstructionAddressMisaligned
    );
    assert_eq!(
        TrapCause::decode(SCAUSE_ILLEGAL_INSTRUCTION),
        TrapCause::IllegalInstruction
    );
    assert_eq!(
        TrapCause::decode(SCAUSE_SUPERVISOR_ECALL),
        TrapCause::EnvironmentCallFromSupervisor
    );
    assert_eq!(
        TrapCause::decode(SCAUSE_LOAD_PAGE_FAULT),
        TrapCause::LoadPageFault
    );
    assert_eq!(TrapCause::decode(42), TrapCause::Unknown(42));

    let interrupt = SCAUSE_INTERRUPT_BIT | 5;
    assert!(is_interrupt(interrupt));
    assert_eq!(TrapCause::decode(interrupt), TrapCause::Interrupt(5));
    assert!(TrapCause::decode(interrupt).is_interrupt());
    assert_eq!(exception_code(interrupt), 5);
    assert!(!is_interrupt(5));
    assert_eq!(exception_code(5), 5);
}

#[test]
fn sbi_error_decoding() {
    assert_eq!(SbiError::from_code(SBI_SUCCESS), None);
    assert_eq!(
        SbiError::from_code(SBI_ERR_NOT_SUPPORTED),
        Some(SbiError::NotSupported)
    );
    assert_eq!(
        SbiError::from_code(SBI_ERR_INVALID_PARAM),
        Some(SbiError::InvalidParameter)
    );
    assert_eq!(SbiError::from_code(SBI_ERR_DENIED), Some(SbiError::Denied));
    assert_eq!(SbiError::from_code(-99), Some(SbiError::Unknown(-99)));

    assert_eq!(SbiError::Failed.code(), -1);
    assert_eq!(SbiError::NotSupported.code(), -2);
    assert_eq!(SbiError::InvalidParameter.code(), -3);
    assert_eq!(SbiError::Denied.code(), -4);

    let success = SbiRet::success(7);
    assert!(success.is_success());
    assert!(!success.is_error());
    assert_eq!(success.error_code(), None);
    assert_eq!(success.into_result(), Ok(7));

    let failure = SbiRet::new(SBI_ERR_INVALID_PARAM, 0);
    assert!(failure.is_error());
    assert_eq!(failure.error_code(), Some(SbiError::InvalidParameter));
    assert_eq!(failure.into_result(), Err(SbiError::InvalidParameter));
}

#[test]
fn sbi_ecall_argument_packing() {
    let call = SbiEcall::with_args(
        SBI_EXTENSION_TIME,
        SBI_TIME_FN_SET_TIMER_STIME,
        [1, 2, 3, 4, 5, 6],
    );
    let registers = call.encode();
    assert_eq!(registers[0], 1);
    assert_eq!(registers[1], 2);
    assert_eq!(registers[2], 3);
    assert_eq!(registers[3], 4);
    assert_eq!(registers[4], 5);
    assert_eq!(registers[5], 6);
    assert_eq!(registers[6], SBI_TIME_FN_SET_TIMER_STIME);
    assert_eq!(registers[7], SBI_EXTENSION_TIME);

    let decoded = SbiEcall::decode(registers);
    assert_eq!(decoded, call);
    assert_eq!(decoded.extension, SBI_EXTENSION_TIME);
    assert_eq!(decoded.function, SBI_TIME_FN_SET_TIMER_STIME);
    assert_eq!(decoded.args, [1, 2, 3, 4, 5, 6]);

    let probe = SbiEcall::probe(SBI_EXTENSION_TIME);
    assert_eq!(probe.extension, SBI_EXTENSION_BASE);
    assert_eq!(probe.args[0], SBI_EXTENSION_TIME);
}

#[test]
fn timer_conversions() {
    assert_eq!(NANOS_PER_SEC, 1_000_000_000);
    assert_eq!(MILLIS_PER_SEC, 1_000);
    assert_eq!(millis_to_nanos(1), 1_000_000);
    assert_eq!(nanos_to_millis(1_000_000), 1);
    assert_eq!(secs_to_nanos(2), 2_000_000_000);
    assert_eq!(millis_to_nanos(u64::MAX), u64::MAX);
    assert_eq!(nanos_to_ticks(1_000_000_000, 1_000_000), 1_000_000);
    assert_eq!(nanos_to_ticks(1_000_000_000, 0), 0);
    assert_eq!(ticks_to_nanos(1_000_000, 1_000_000), 1_000_000_000);
    assert_eq!(ticks_to_nanos(1_000, 0), 0);
}

#[test]
fn tick_accumulator_counts_the_periods() {
    let mut accumulator = TickAccumulator::new(1_000, 0);
    assert_eq!(accumulator.period_nanos(), 1_000);
    assert_eq!(accumulator.next_deadline_nanos(), 1_000);
    assert_eq!(accumulator.advance(500), 0);
    assert_eq!(accumulator.advance(1_000), 1);
    assert_eq!(accumulator.next_deadline_nanos(), 2_000);
    assert_eq!(accumulator.advance(3_500), 2);
    assert_eq!(accumulator.next_deadline_nanos(), 4_000);

    assert_eq!(TickAccumulator::ticks_in(3_500, 1_000), 3);
    assert_eq!(TickAccumulator::ticks_in(3_500, 0), 0);
}

#[test]
fn monotonic_timer_arms_a_deadline() {
    static TICKS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

    fn read_ticks() -> u64 {
        TICKS.load(core::sync::atomic::Ordering::Relaxed)
    }

    TICKS.store(1_000, core::sync::atomic::Ordering::Relaxed);
    let mut timer = MonotonicTimer::new(1_000_000_000, read_ticks);
    assert_eq!(timer.frequency_hz(), 1_000_000_000);
    assert_eq!(timer.now_nanos(), 1_000);
    assert!(!timer.is_armed());
    assert_eq!(timer.set_deadline(5_000), Ok(()));
    assert!(timer.is_armed());
    assert_eq!(timer.deadline_nanos(), Some(5_000));
    assert_eq!(timer.set_deadline(6_000), Err(TimerError::AlreadyArmed));
    assert_eq!(timer.cancel(), Ok(()));
    assert_eq!(timer.cancel(), Err(TimerError::NotArmed));
}

#[test]
fn dma_descriptors() {
    assert_eq!(DMA_MIN_ALIGNMENT, 16);
    assert_eq!(DMA_PAGE_ALIGNMENT, 4096);
    assert_eq!(DMA_MAX_SEGMENTS, 64);

    let segment = contiguous_segment(0x1000, 0x200);
    assert_eq!(segment.address(), 0x1000);
    assert_eq!(segment.length(), 0x200);
    assert!(segment.is_aligned(DMA_MIN_ALIGNMENT));

    let buffer = contiguous_buffer(0x1000, 0x200, DmaDirection::ToDevice);
    assert_eq!(buffer.address(), 0x1000);
    assert_eq!(buffer.length(), 0x200);
    assert_eq!(buffer.direction(), DmaDirection::ToDevice);
    assert!(buffer.is_aligned());
    assert_eq!(buffer.validate(), Ok(()));
    assert_eq!(buffer.as_segment(), segment);

    let empty = DmaBuffer::new(0x1000, 0, DmaDirection::FromDevice);
    assert_eq!(empty.validate(), Err(DmaError::ZeroLength));

    let misaligned = DmaBuffer::new(0x1001, 0x10, DmaDirection::Bidirectional);
    assert_eq!(misaligned.validate(), Err(DmaError::Misaligned));
}

#[test]
fn dma_segment_list_is_bounded() {
    let mut list: DmaSegmentList<2> = DmaSegmentList::new();
    assert!(list.is_empty());
    assert_eq!(list.capacity(), 2);
    assert_eq!(list.push(DmaSegment::new(0x1000, 0x10)), Ok(()));
    assert_eq!(list.push(DmaSegment::new(0x2000, 0x10)), Ok(()));
    assert_eq!(
        list.push(DmaSegment::new(0x3000, 0x10)),
        Err(DmaError::TooManySegments)
    );
    assert_eq!(
        list.push(DmaSegment::new(0x4000, 0)),
        Err(DmaError::ZeroLength)
    );
    assert_eq!(list.len(), 2);
    assert_eq!(list.total_length(), 0x20);
    assert_eq!(list.get(0), Some(DmaSegment::new(0x1000, 0x10)));
    assert_eq!(list.get(2), None);
    assert!(list.is_aligned(DMA_MIN_ALIGNMENT));

    let mut default_list: DmaSegmentList<4> = DmaSegmentList::default();
    assert!(default_list.is_empty());
    assert_eq!(default_list.push(DmaSegment::new(0x10, 0x10)), Ok(()));
    assert_eq!(default_list.len(), 1);
}
