//! # Processor identification
//!
//! A portable model of the `CPUID` instruction.
//!
//! The instruction cannot be executed in a portable way: it is
//! only meaningful on an Intel compatible processor, and the
//! result depends on the machine that runs the code. The module
//! therefore separates the *model* from the *execution*.
//!
//! [`Cpuid`] describes the query, [`HostCpuid`] executes the
//! real instruction where the target allows it, and
//! [`StubCpuid`] answers with zeros everywhere else. The
//! decoding helpers only consume [`CpuidResult`] values, so
//! they can be exercised with recorded data on any host.
//!
//! The leaf and bit assignments follow the architecture manuals
//! of the Intel and AMD processor families.

/// The result of a single `CPUID` query.
///
/// The four registers are kept in the order in which the
/// instruction returns them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CpuidResult {
    /// The value of the accumulator register.
    pub eax: u32,
    /// The value of the first general purpose register.
    pub ebx: u32,
    /// The value of the second general purpose register.
    pub ecx: u32,
    /// The value of the third general purpose register.
    pub edx: u32,
}

impl CpuidResult {
    /// Creates a result from the four register values.
    #[must_use]
    pub const fn new(eax: u32, ebx: u32, ecx: u32, edx: u32) -> Self {
        Self { eax, ebx, ecx, edx }
    }

    /// Returns whether every register holds zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.eax == 0 && self.ebx == 0 && self.ecx == 0 && self.edx == 0
    }
}

/// A source of `CPUID` leaf values.
///
/// The trait is the seam between the decoding helpers and the
/// processor. A test can implement it with recorded values, and
/// the architecture module implements it with the real
/// instruction.
pub trait Cpuid {
    /// Returns the result of the given leaf and sub-leaf.
    ///
    /// Leaves that the source does not implement answer with
    /// zero in every register.
    fn leaf(&self, leaf: u32, subleaf: u32) -> CpuidResult;
}

/// The highest basic leaf number.
pub const CPUID_MAX_BASIC_LEAF: u32 = 0x0000_0000;

/// The leaf that reports the processor identification.
pub const CPUID_LEAF_IDENTIFICATION: u32 = 0x0000_0000;

/// The leaf that reports the version and the basic feature
/// flags.
pub const CPUID_LEAF_VERSION_AND_FEATURES: u32 = 0x0000_0001;

/// The leaf that reports the structured extended feature flags.
pub const CPUID_LEAF_STRUCTURED_FEATURES: u32 = 0x0000_0007;

/// The leaf that reports the extended state enumeration.
pub const CPUID_LEAF_XSAVE: u32 = 0x0000_000D;

/// The leaf that reports the highest extended leaf number.
pub const CPUID_EXTENDED_LEAF_IDENTIFICATION: u32 = 0x8000_0000;

/// The leaf that reports the extended feature flags.
pub const CPUID_EXTENDED_LEAF_FEATURES: u32 = 0x8000_0001;

/// The first leaf that reports the processor brand string.
pub const CPUID_EXTENDED_LEAF_BRAND_START: u32 = 0x8000_0002;

/// The last leaf that reports the processor brand string.
pub const CPUID_EXTENDED_LEAF_BRAND_END: u32 = 0x8000_0004;

/// A `CPUID` source that executes the real instruction.
///
/// The instruction is only available on Intel compatible
/// targets. On every other target the queries answer with
/// zeros, which keeps the crate buildable for the firmware
/// targets of the other architectures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HostCpuid;

impl Cpuid for HostCpuid {
    fn leaf(&self, leaf: u32, subleaf: u32) -> CpuidResult {
        #[cfg(target_arch = "x86_64")]
        {
            // The intrinsic reads the leaf and the sub-leaf from
            // the accumulator and the second general purpose
            // register. It is available on every processor of
            // this target, it has no memory side effect and it
            // cannot fault.
            let result = core::arch::x86_64::__cpuid_count(leaf, subleaf);
            CpuidResult::new(result.eax, result.ebx, result.ecx, result.edx)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            StubCpuid.leaf(leaf, subleaf)
        }
    }
}

/// A `CPUID` source that answers with zeros.
///
/// The stub is used on targets that cannot execute the
/// instruction, and by callers that only need the shape of the
/// data.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StubCpuid;

impl Cpuid for StubCpuid {
    fn leaf(&self, _leaf: u32, _subleaf: u32) -> CpuidResult {
        CpuidResult::default()
    }
}

/// The decoded feature flags of the version leaf.
///
/// The flags are stored as the raw register images so that the
/// structure stays compact and so that unknown bits are
/// preserved.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Leaf1Features {
    edx: u32,
    ecx: u32,
}

impl Leaf1Features {
    /// Creates the feature set from the raw register images.
    #[must_use]
    pub const fn new(edx: u32, ecx: u32) -> Self {
        Self { edx, ecx }
    }

    /// Returns the raw image of the feature register.
    #[must_use]
    pub const fn raw_edx(&self) -> u32 {
        self.edx
    }

    /// Returns the raw image of the extended feature register.
    #[must_use]
    pub const fn raw_ecx(&self) -> u32 {
        self.ecx
    }

    /// Returns whether the processor has a coprocessor.
    #[must_use]
    pub const fn fpu(&self) -> bool {
        self.edx & (1 << 0) != 0
    }

    /// Returns whether the processor supports virtual mode
    /// extensions.
    #[must_use]
    pub const fn vme(&self) -> bool {
        self.edx & (1 << 1) != 0
    }

    /// Returns whether the processor supports debugging
    /// extensions.
    #[must_use]
    pub const fn de(&self) -> bool {
        self.edx & (1 << 2) != 0
    }

    /// Returns whether the processor supports the protected mode
    /// system call extension.
    #[must_use]
    pub const fn pse(&self) -> bool {
        self.edx & (1 << 3) != 0
    }

    /// Returns whether the processor has a time stamp counter.
    #[must_use]
    pub const fn tsc(&self) -> bool {
        self.edx & (1 << 4) != 0
    }

    /// Returns whether the processor supports the model specific
    /// register extension.
    #[must_use]
    pub const fn msr(&self) -> bool {
        self.edx & (1 << 5) != 0
    }

    /// Returns whether the processor supports physical address
    /// extension.
    #[must_use]
    pub const fn pae(&self) -> bool {
        self.edx & (1 << 6) != 0
    }

    /// Returns whether the processor supports machine check
    /// exception handling.
    #[must_use]
    pub const fn mce(&self) -> bool {
        self.edx & (1 << 7) != 0
    }

    /// Returns whether the processor supports the compare and
    /// exchange instruction on 8 bytes.
    #[must_use]
    pub const fn cx8(&self) -> bool {
        self.edx & (1 << 8) != 0
    }

    /// Returns whether the processor contains a local advanced
    /// programmable interrupt controller.
    #[must_use]
    pub const fn apic(&self) -> bool {
        self.edx & (1 << 9) != 0
    }

    /// Returns whether the processor supports the fast system
    /// call extension.
    #[must_use]
    pub const fn sep(&self) -> bool {
        self.edx & (1 << 10) != 0
    }

    /// Returns whether the processor supports the memory type
    /// range register extension.
    #[must_use]
    pub const fn mtrr(&self) -> bool {
        self.edx & (1 << 11) != 0
    }

    /// Returns whether the processor supports page global
    /// extension.
    #[must_use]
    pub const fn pge(&self) -> bool {
        self.edx & (1 << 12) != 0
    }

    /// Returns whether the processor supports the machine check
    /// architecture extension.
    #[must_use]
    pub const fn mca(&self) -> bool {
        self.edx & (1 << 13) != 0
    }

    /// Returns whether the processor supports the conditional
    /// move instruction.
    #[must_use]
    pub const fn cmov(&self) -> bool {
        self.edx & (1 << 14) != 0
    }

    /// Returns whether the processor supports the page attribute
    /// table.
    #[must_use]
    pub const fn pat(&self) -> bool {
        self.edx & (1 << 15) != 0
    }

    /// Returns whether the processor supports 36-bit page size
    /// extension.
    #[must_use]
    pub const fn pse36(&self) -> bool {
        self.edx & (1 << 16) != 0
    }

    /// Returns whether the processor supports the flush
    /// instruction.
    #[must_use]
    pub const fn clfsh(&self) -> bool {
        self.edx & (1 << 17) != 0
    }

    /// Returns whether the processor supports the media
    /// instruction.
    #[must_use]
    pub const fn mmx(&self) -> bool {
        self.edx & (1 << 18) != 0
    }

    /// Returns whether the processor supports the extended
    /// floating point register save and restore instructions.
    #[must_use]
    pub const fn fxsr(&self) -> bool {
        self.edx & (1 << 19) != 0
    }

    /// Returns whether the processor supports the streaming
    /// single instruction stream.
    #[must_use]
    pub const fn sse(&self) -> bool {
        self.edx & (1 << 20) != 0
    }

    /// Returns whether the processor supports the streaming
    /// single instruction stream, version two.
    #[must_use]
    pub const fn sse2(&self) -> bool {
        self.edx & (1 << 21) != 0
    }

    /// Returns whether the processor supports the streaming
    /// single instruction stream, version three.
    #[must_use]
    pub const fn sse3(&self) -> bool {
        self.ecx & (1 << 0) != 0
    }

    /// Returns whether the processor supports the carry-less
    /// multiplication instruction.
    #[must_use]
    pub const fn pclmulqdq(&self) -> bool {
        self.ecx & (1 << 1) != 0
    }

    /// Returns whether the processor supports the monitor and
    /// wait instructions.
    #[must_use]
    pub const fn monitor(&self) -> bool {
        self.ecx & (1 << 3) != 0
    }

    /// Returns whether the processor supports the extended
    /// streaming single instruction stream.
    #[must_use]
    pub const fn ssse3(&self) -> bool {
        self.ecx & (1 << 9) != 0
    }

    /// Returns whether the processor supports the fused multiply
    /// add instruction.
    #[must_use]
    pub const fn fma(&self) -> bool {
        self.ecx & (1 << 12) != 0
    }

    /// Returns whether the processor supports the compare and
    /// exchange instruction on 16 bytes.
    #[must_use]
    pub const fn cmpxchg16b(&self) -> bool {
        self.ecx & (1 << 13) != 0
    }

    /// Returns whether the processor supports the process
    /// context identifiers.
    #[must_use]
    pub const fn pcid(&self) -> bool {
        self.ecx & (1 << 17) != 0
    }

    /// Returns whether the processor supports the streaming
    /// single instruction stream, version four, part one.
    #[must_use]
    pub const fn sse41(&self) -> bool {
        self.ecx & (1 << 19) != 0
    }

    /// Returns whether the processor supports the streaming
    /// single instruction stream, version four, part two.
    #[must_use]
    pub const fn sse42(&self) -> bool {
        self.ecx & (1 << 20) != 0
    }

    /// Returns whether the processor supports the extended
    /// application programming interface.
    #[must_use]
    pub const fn x2apic(&self) -> bool {
        self.ecx & (1 << 21) != 0
    }

    /// Returns whether the processor supports the move
    /// instruction on 16 bytes.
    #[must_use]
    pub const fn movbe(&self) -> bool {
        self.ecx & (1 << 22) != 0
    }

    /// Returns whether the processor supports the population
    /// count instruction.
    #[must_use]
    pub const fn popcnt(&self) -> bool {
        self.ecx & (1 << 23) != 0
    }

    /// Returns whether the processor supports the time stamp
    /// counter deadline.
    #[must_use]
    pub const fn tsc_deadline(&self) -> bool {
        self.ecx & (1 << 24) != 0
    }

    /// Returns whether the processor supports the advanced
    /// encryption standard instruction set.
    #[must_use]
    pub const fn aes(&self) -> bool {
        self.ecx & (1 << 25) != 0
    }

    /// Returns whether the processor supports the extended state
    /// save and restore area.
    #[must_use]
    pub const fn xsave(&self) -> bool {
        self.ecx & (1 << 26) != 0
    }

    /// Returns whether the operating system enabled the extended
    /// state save and restore area.
    #[must_use]
    pub const fn osxsave(&self) -> bool {
        self.ecx & (1 << 27) != 0
    }

    /// Returns whether the processor supports the advanced
    /// vector extensions.
    #[must_use]
    pub const fn avx(&self) -> bool {
        self.ecx & (1 << 28) != 0
    }

    /// Returns whether the processor supports the half precision
    /// conversion instructions.
    #[must_use]
    pub const fn f16c(&self) -> bool {
        self.ecx & (1 << 29) != 0
    }

    /// Returns whether the processor supports the read random
    /// instruction.
    #[must_use]
    pub const fn rdrand(&self) -> bool {
        self.ecx & (1 << 30) != 0
    }

    /// Returns whether the processor runs under a
    /// hypervisor.
    #[must_use]
    pub const fn hypervisor(&self) -> bool {
        self.ecx & (1 << 31) != 0
    }
}

/// The decoded feature flags of the structured extended leaf.
///
/// The structure covers sub-leaf zero, which reports the
/// features in the first register, and the sub-leaf that reports
/// the control flow enforcement features.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Leaf7Features {
    ebx: u32,
    edx: u32,
}

impl Leaf7Features {
    /// Creates the feature set from the raw register images.
    #[must_use]
    pub const fn new(ebx: u32, edx: u32) -> Self {
        Self { ebx, edx }
    }

    /// Returns the raw image of the first register.
    #[must_use]
    pub const fn raw_ebx(&self) -> u32 {
        self.ebx
    }

    /// Returns the raw image of the second register.
    #[must_use]
    pub const fn raw_edx(&self) -> u32 {
        self.edx
    }

    /// Returns whether the processor supports the base and
    /// limit register access instructions.
    #[must_use]
    pub const fn fsgsbase(&self) -> bool {
        self.ebx & (1 << 0) != 0
    }

    /// Returns whether the processor supports bit manipulation
    /// instruction set, version one.
    #[must_use]
    pub const fn bmi1(&self) -> bool {
        self.ebx & (1 << 3) != 0
    }

    /// Returns whether the processor supports the hardware
    /// lock execution environment.
    #[must_use]
    pub const fn hle(&self) -> bool {
        self.ebx & (1 << 4) != 0
    }

    /// Returns whether the processor supports the advanced
    /// vector extensions, version two.
    #[must_use]
    pub const fn avx2(&self) -> bool {
        self.ebx & (1 << 5) != 0
    }

    /// Returns whether the processor supports supervisor mode
    /// execution prevention.
    #[must_use]
    pub const fn smep(&self) -> bool {
        self.ebx & (1 << 7) != 0
    }

    /// Returns whether the processor supports bit manipulation
    /// instruction set, version two.
    #[must_use]
    pub const fn bmi2(&self) -> bool {
        self.ebx & (1 << 8) != 0
    }

    /// Returns whether the processor supports the enhanced
    /// memory management capability.
    #[must_use]
    pub const fn erms(&self) -> bool {
        self.ebx & (1 << 9) != 0
    }

    /// Returns whether the processor supports the virtual
    /// processor identifiers.
    #[must_use]
    pub const fn invpcid(&self) -> bool {
        self.ebx & (1 << 10) != 0
    }

    /// Returns whether the processor supports the fast short
    /// rep movsb, also known as the resource director
    /// technology.
    #[must_use]
    pub const fn fsrm(&self) -> bool {
        self.ebx & (1 << 11) != 0
    }

    /// Returns whether the processor supports restricted
    /// transactional memory.
    #[must_use]
    pub const fn rtm(&self) -> bool {
        self.ebx & (1 << 12) != 0
    }

    /// Returns whether the processor supports the memory
    /// protection extensions.
    #[must_use]
    pub const fn mpx(&self) -> bool {
        self.ebx & (1 << 13) != 0
    }

    /// Returns whether the processor supports the read seed
    /// instruction.
    #[must_use]
    pub const fn rdseed(&self) -> bool {
        self.ebx & (1 << 14) != 0
    }

    /// Returns whether the processor supports the address
    /// generation instruction.
    #[must_use]
    pub const fn adx(&self) -> bool {
        self.ebx & (1 << 15) != 0
    }

    /// Returns whether the processor supports supervisor mode
    /// access prevention.
    #[must_use]
    pub const fn smap(&self) -> bool {
        self.ebx & (1 << 16) != 0
    }

    /// Returns whether the processor supports the cache line
    /// flush optimization instruction.
    #[must_use]
    pub const fn clflushopt(&self) -> bool {
        self.ebx & (1 << 17) != 0
    }

    /// Returns whether the processor supports the cache line
    /// write back instruction.
    #[must_use]
    pub const fn clwb(&self) -> bool {
        self.ebx & (1 << 18) != 0
    }

    /// Returns whether the processor supports the secure hash
    /// algorithm extensions.
    #[must_use]
    pub const fn sha(&self) -> bool {
        self.ebx & (1 << 29) != 0
    }

    /// Returns whether the processor supports the control flow
    /// enforcement technology shadow stack.
    #[must_use]
    pub const fn cet_ss(&self) -> bool {
        self.ebx & (1 << 27) != 0
    }

    /// Returns whether the processor supports the control flow
    /// enforcement technology indirect branch tracking.
    #[must_use]
    pub const fn cet_ift(&self) -> bool {
        self.edx & (1 << 18) != 0
    }

    /// Returns whether the processor supports any control flow
    /// enforcement feature.
    #[must_use]
    pub const fn cet(&self) -> bool {
        self.cet_ss() || self.cet_ift()
    }
}

/// The decoded feature flags of the second sub-leaf of the
/// structured extended leaf.
///
/// The sub-leaf reports the hash algorithm extensions that were
/// introduced after the first sub-leaf was defined.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Leaf7Subleaf1Features {
    eax: u32,
}

impl Leaf7Subleaf1Features {
    /// Creates the feature set from the raw register image.
    #[must_use]
    pub const fn new(eax: u32) -> Self {
        Self { eax }
    }

    /// Returns the raw image of the register.
    #[must_use]
    pub const fn raw_eax(&self) -> u32 {
        self.eax
    }

    /// Returns whether the processor supports the secure hash
    /// algorithm instructions for the 512 bit digest.
    #[must_use]
    pub const fn sha512(&self) -> bool {
        self.eax & (1 << 0) != 0
    }

    /// Returns whether the processor supports the secure hash
    /// algorithm three instructions.
    #[must_use]
    pub const fn sm3(&self) -> bool {
        self.eax & (1 << 1) != 0
    }

    /// Returns whether the processor supports the secure hash
    /// algorithm four instructions.
    #[must_use]
    pub const fn sm4(&self) -> bool {
        self.eax & (1 << 2) != 0
    }

    /// Returns whether the processor supports the vector neural
    /// network instructions.
    #[must_use]
    pub const fn avx_vnni(&self) -> bool {
        self.eax & (1 << 4) != 0
    }

    /// Returns whether the processor supports the brain float
    /// instructions.
    #[must_use]
    pub const fn avx512_bf16(&self) -> bool {
        self.eax & (1 << 5) != 0
    }
}

/// The decoded feature flags of the extended leaf.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ExtendedFeatures {
    edx: u32,
    ecx: u32,
}

impl ExtendedFeatures {
    /// Creates the feature set from the raw register images.
    #[must_use]
    pub const fn new(edx: u32, ecx: u32) -> Self {
        Self { edx, ecx }
    }

    /// Returns the raw image of the feature register.
    #[must_use]
    pub const fn raw_edx(&self) -> u32 {
        self.edx
    }

    /// Returns the raw image of the extended feature register.
    #[must_use]
    pub const fn raw_ecx(&self) -> u32 {
        self.ecx
    }

    /// Returns whether the processor supports the system call
    /// instruction.
    #[must_use]
    pub const fn syscall(&self) -> bool {
        self.edx & (1 << 0) != 0
    }

    /// Returns whether the processor supports the system call and
    /// system entry model specific registers.
    #[must_use]
    pub const fn syscall_msr(&self) -> bool {
        self.edx & (1 << 11) != 0
    }

    /// Returns whether the processor supports the no execute page
    /// protection.
    #[must_use]
    pub const fn nx(&self) -> bool {
        self.edx & (1 << 20) != 0
    }

    /// Returns whether the processor supports the read time
    /// stamp counter processor identifier instruction.
    #[must_use]
    pub const fn rdtscp(&self) -> bool {
        self.edx & (1 << 21) != 0
    }

    /// Returns whether the processor supports one gigabyte
    /// pages.
    #[must_use]
    pub const fn one_gb_pages(&self) -> bool {
        self.edx & (1 << 26) != 0
    }

    /// Returns whether the processor supports the long mode
    /// architecture.
    #[must_use]
    pub const fn lm(&self) -> bool {
        self.edx & (1 << 29) != 0
    }

    /// Returns whether the processor supports the load status and
    /// load instructions in long mode.
    #[must_use]
    pub const fn lahf_lm(&self) -> bool {
        self.ecx & (1 << 0) != 0
    }

    /// Returns whether the processor supports the leading zero
    /// count and population count instructions.
    #[must_use]
    pub const fn lzcnt(&self) -> bool {
        self.ecx & (1 << 5) != 0
    }

    /// Returns whether the processor supports the extended
    /// floating point register save and restore area.
    #[must_use]
    pub const fn prefetchw(&self) -> bool {
        self.ecx & (1 << 8) != 0
    }
}

/// The state components that the extended state save and restore
/// area can hold.
pub const XCR0_X87: u64 = 1 << 0;

/// The state component that holds the legacy registers.
pub const XCR0_SSE: u64 = 1 << 1;

/// The state component that holds the upper half of the vector
/// registers.
pub const XCR0_AVX: u64 = 1 << 2;

/// The state component that holds the bounds registers.
pub const XCR0_BNDREGS: u64 = 1 << 3;

/// The state component that holds the bounds configuration
/// registers.
pub const XCR0_BNDCSR: u64 = 1 << 4;

/// The state component that holds the operation mask registers.
pub const XCR0_OPMASK: u64 = 1 << 5;

/// The state component that holds the upper half of the extended
/// vector registers.
pub const XCR0_ZMM_HI256: u64 = 1 << 6;

/// The state component that holds the upper sixteen vector
/// registers.
pub const XCR0_HI16_ZMM: u64 = 1 << 7;

/// The state component that holds the processor trace state.
pub const XCR0_PT: u64 = 1 << 8;

/// The state component that holds the protection keys.
pub const XCR0_PKRU: u64 = 1 << 9;

/// The state component that holds the process address space
/// identifiers.
pub const XCR0_PASID: u64 = 1 << 10;

/// The state component that holds the control flow enforcement
/// state of the user mode.
pub const XCR0_CET_U: u64 = 1 << 11;

/// The decoded extended state enumeration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct XsaveFeatures {
    xcr0_supported: u64,
    xcr0_valid: u64,
    xsave_area_size: u32,
    xsaveopt: bool,
    xsaves: bool,
}

impl XsaveFeatures {
    /// Creates the feature set from the raw register images.
    #[must_use]
    pub const fn new(
        xcr0_supported: u64,
        xcr0_valid: u64,
        xsave_area_size: u32,
        xsaveopt: bool,
        xsaves: bool,
    ) -> Self {
        Self {
            xcr0_supported,
            xcr0_valid,
            xsave_area_size,
            xsaveopt,
            xsaves,
        }
    }

    /// Returns the state components that the processor supports.
    #[must_use]
    pub const fn xcr0_supported(&self) -> u64 {
        self.xcr0_supported
    }

    /// Returns the state components that the processor can
    /// currently hold.
    #[must_use]
    pub const fn xcr0_valid(&self) -> u64 {
        self.xcr0_valid
    }

    /// Returns the size of the extended state save and restore
    /// area in bytes.
    #[must_use]
    pub const fn xsave_area_size(&self) -> u32 {
        self.xsave_area_size
    }

    /// Returns whether the processor supports the optimised
    /// extended state save instruction.
    #[must_use]
    pub const fn xsaveopt(&self) -> bool {
        self.xsaveopt
    }

    /// Returns whether the processor supports the supervisor
    /// state save instruction.
    #[must_use]
    pub const fn xsaves(&self) -> bool {
        self.xsaves
    }

    /// Returns whether a state component is supported.
    #[must_use]
    pub const fn supports(&self, component: u64) -> bool {
        self.xcr0_supported & component == component
    }

    /// Returns whether the upper half of the vector registers is
    /// a supported state component.
    #[must_use]
    pub const fn ymm_state(&self) -> bool {
        self.supports(XCR0_AVX)
    }

    /// Returns whether the operation mask registers are a
    /// supported state component.
    #[must_use]
    pub const fn opmask_state(&self) -> bool {
        self.supports(XCR0_OPMASK)
    }

    /// Returns whether the upper half of the extended vector
    /// registers is a supported state component.
    #[must_use]
    pub const fn zmm_hi256_state(&self) -> bool {
        self.supports(XCR0_ZMM_HI256)
    }

    /// Returns whether the upper sixteen vector registers are a
    /// supported state component.
    #[must_use]
    pub const fn hi16_zmm_state(&self) -> bool {
        self.supports(XCR0_HI16_ZMM)
    }

    /// Returns whether the processor trace state is a supported
    /// state component.
    #[must_use]
    pub const fn pt_state(&self) -> bool {
        self.supports(XCR0_PT)
    }

    /// Returns whether the protection keys are a supported state
    /// component.
    #[must_use]
    pub const fn pkru_state(&self) -> bool {
        self.supports(XCR0_PKRU)
    }

    /// Returns whether the process address space identifiers are a
    /// supported state component.
    #[must_use]
    pub const fn pasid_state(&self) -> bool {
        self.supports(XCR0_PASID)
    }

    /// Returns whether the control flow enforcement state of the
    /// user mode is a supported state component.
    #[must_use]
    pub const fn cet_u_state(&self) -> bool {
        self.supports(XCR0_CET_U)
    }
}

/// A fixed size string that holds the identification text of the
/// processor.
///
/// The text is stored as bytes because the processor reports it
/// without a terminator and without a guarantee that it is
/// printable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AsciiString<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> Default for AsciiString<N> {
    fn default() -> Self {
        Self::from_bytes([0; N])
    }
}

impl<const N: usize> AsciiString<N> {
    /// Creates a string from its raw bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; N]) -> Self {
        Self { bytes }
    }

    /// Returns the raw bytes of the string.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; N] {
        &self.bytes
    }

    /// Returns the bytes up to the first terminator.
    #[must_use]
    pub fn trimmed(&self) -> &[u8] {
        let end = self.bytes.iter().position(|byte| *byte == 0).unwrap_or(N);
        self.bytes.get(..end).unwrap_or(&[])
    }

    /// Returns the text of the string.
    ///
    /// The result is `None` when the string holds bytes that are
    /// not printable text.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        let trimmed = self.trimmed();
        let text = core::str::from_utf8(trimmed).ok()?;
        if text.is_ascii() { Some(text) } else { None }
    }
}

/// The description of a processor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuInfo {
    /// The highest basic leaf that the processor implements.
    pub max_basic_leaf: u32,
    /// The highest extended leaf that the processor implements.
    pub max_extended_leaf: u32,
    /// The identification text of the processor vendor.
    pub vendor: AsciiString<12>,
    /// The brand text of the processor.
    pub brand: AsciiString<48>,
    /// The base family of the processor.
    pub family: u8,
    /// The model of the processor.
    pub model: u8,
    /// The stepping of the processor.
    pub stepping: u8,
    /// The features of the version leaf.
    pub leaf1: Leaf1Features,
    /// The features of the structured extended leaf.
    pub leaf7: Leaf7Features,
    /// The features of the second sub-leaf of the structured
    /// extended leaf.
    pub leaf7_subleaf1: Leaf7Subleaf1Features,
    /// The features of the extended leaf.
    pub extended: ExtendedFeatures,
    /// The features of the extended state enumeration.
    pub xsave: XsaveFeatures,
}

impl CpuInfo {
    /// Collects the description of a processor.
    ///
    /// Leaves that the processor does not implement are left
    /// empty, so the result is well defined even when the
    /// source only answers with zeros.
    #[must_use]
    pub fn collect<C: Cpuid + ?Sized>(cpuid: &C) -> Self {
        let identification = cpuid.leaf(CPUID_LEAF_IDENTIFICATION, 0);
        let max_basic_leaf = identification.eax;

        let mut vendor_bytes = [0u8; 12];
        vendor_bytes[0..4].copy_from_slice(&identification.ebx.to_le_bytes());
        vendor_bytes[4..8].copy_from_slice(&identification.edx.to_le_bytes());
        vendor_bytes[8..12].copy_from_slice(&identification.ecx.to_le_bytes());
        let vendor = AsciiString::<12>::from_bytes(vendor_bytes);

        let extended_identification = cpuid.leaf(CPUID_EXTENDED_LEAF_IDENTIFICATION, 0);
        let max_extended_leaf = extended_identification.eax;

        let mut brand_bytes = [0u8; 48];
        if max_extended_leaf >= CPUID_EXTENDED_LEAF_BRAND_END {
            let leaves = [
                CPUID_EXTENDED_LEAF_BRAND_START,
                CPUID_EXTENDED_LEAF_BRAND_START + 1,
                CPUID_EXTENDED_LEAF_BRAND_END,
            ];
            for (chunk, leaf) in brand_bytes.chunks_mut(16).zip(leaves) {
                let result = cpuid.leaf(leaf, 0);
                chunk[0..4].copy_from_slice(&result.eax.to_le_bytes());
                chunk[4..8].copy_from_slice(&result.ebx.to_le_bytes());
                chunk[8..12].copy_from_slice(&result.ecx.to_le_bytes());
                chunk[12..16].copy_from_slice(&result.edx.to_le_bytes());
            }
        }

        let version = if max_basic_leaf >= CPUID_LEAF_VERSION_AND_FEATURES {
            cpuid.leaf(CPUID_LEAF_VERSION_AND_FEATURES, 0)
        } else {
            CpuidResult::default()
        };

        let (leaf7, leaf7_subleaf1) = if max_basic_leaf >= CPUID_LEAF_STRUCTURED_FEATURES {
            let structured = cpuid.leaf(CPUID_LEAF_STRUCTURED_FEATURES, 0);
            let subleaf1 = if structured.eax >= 1 {
                cpuid.leaf(CPUID_LEAF_STRUCTURED_FEATURES, 1)
            } else {
                CpuidResult::default()
            };
            (
                Leaf7Features::new(structured.ebx, structured.edx),
                Leaf7Subleaf1Features::new(subleaf1.eax),
            )
        } else {
            (Leaf7Features::default(), Leaf7Subleaf1Features::default())
        };

        let xsave = if max_basic_leaf >= CPUID_LEAF_XSAVE {
            let base = cpuid.leaf(CPUID_LEAF_XSAVE, 0);
            let opt = if base.eax >= 1 {
                cpuid.leaf(CPUID_LEAF_XSAVE, 1)
            } else {
                CpuidResult::default()
            };
            let saves = if base.eax >= 2 {
                cpuid.leaf(CPUID_LEAF_XSAVE, 2)
            } else {
                CpuidResult::default()
            };
            XsaveFeatures::new(
                u64::from(base.ebx),
                u64::from(base.eax),
                base.ecx,
                opt.ecx & 1 != 0,
                saves.ecx & 1 != 0,
            )
        } else {
            XsaveFeatures::default()
        };

        let extended = if max_extended_leaf >= CPUID_EXTENDED_LEAF_FEATURES {
            let result = cpuid.leaf(CPUID_EXTENDED_LEAF_FEATURES, 0);
            ExtendedFeatures::new(result.edx, result.ecx)
        } else {
            ExtendedFeatures::default()
        };

        let (family, model, stepping) = decode_version(version.eax);

        Self {
            max_basic_leaf,
            max_extended_leaf,
            vendor,
            brand: AsciiString::from_bytes(brand_bytes),
            family,
            model,
            stepping,
            leaf1: Leaf1Features::new(version.edx, version.ecx),
            leaf7,
            leaf7_subleaf1,
            extended,
            xsave,
        }
    }

    /// Returns the identification text of the processor vendor.
    #[must_use]
    pub fn vendor_string(&self) -> Option<&str> {
        self.vendor.as_str()
    }

    /// Returns the brand text of the processor.
    #[must_use]
    pub fn brand_string(&self) -> Option<&str> {
        self.brand.as_str()
    }
}

/// Decodes the family, the model and the stepping of a processor.
///
/// The version register encodes the extended family and the
/// extended model in separate fields that are only meaningful
/// for particular base values.
#[must_use]
pub fn decode_version(version: u32) -> (u8, u8, u8) {
    let base_family = u8::try_from((version >> 8) & 0x0F).unwrap_or_default();
    let extended_family = u8::try_from((version >> 20) & 0xFF).unwrap_or_default();
    let base_model = u8::try_from((version >> 4) & 0x0F).unwrap_or_default();
    let extended_model = u8::try_from((version >> 16) & 0x0F).unwrap_or_default();
    let stepping = u8::try_from(version & 0x0F).unwrap_or_default();

    let family = if base_family == 0x0F {
        base_family.saturating_add(extended_family)
    } else {
        base_family
    };
    let model = if base_family == 0x06 || base_family == 0x0F {
        base_model.saturating_add(extended_model << 4)
    } else {
        base_model
    };
    (family, model, stepping)
}
