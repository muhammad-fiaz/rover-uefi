//! # x86 Port-Mapped I/O (PMIO) Primitives
//!
//! Provides hardware-level x86 `IN` and `OUT` processor port instructions and
//! strongly-typed port accessors for legacy chipsets, PCI configuration spaces,
//! UART serial controllers, and debug consoles.

/// Reads an 8-bit byte from an x86 I/O port (`in al, dx`).
///
/// # Safety
///
/// Accessing arbitrary hardware I/O ports can affect system state, bus controllers,
/// or cause hardware faults if an invalid port address is accessed.
#[must_use]
#[inline]
pub unsafe fn in_u8(port: u16) -> u8 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        let value: u8;
        core::arch::asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack, preserves_flags));
        value
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = port;
        0
    }
}

/// Writes an 8-bit byte to an x86 I/O port (`out dx, al`).
///
/// # Safety
///
/// Direct port writes alter chipset registers and device controllers.
#[inline]
pub unsafe fn out_u8(port: u16, value: u8) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, value);
    }
}

/// Reads a 16-bit word from an x86 I/O port (`in ax, dx`).
///
/// # Safety
///
/// Direct port reads can trigger hardware side effects on peripheral buses.
#[must_use]
#[inline]
pub unsafe fn in_u16(port: u16) -> u16 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        let value: u16;
        core::arch::asm!("in ax, dx", in("dx") port, out("ax") value, options(nomem, nostack, preserves_flags));
        value
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = port;
        0
    }
}

/// Writes a 16-bit word to an x86 I/O port (`out dx, ax`).
///
/// # Safety
///
/// Direct port writes alter device state.
#[inline]
pub unsafe fn out_u16(port: u16, value: u16) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, value);
    }
}

/// Reads a 32-bit double-word from an x86 I/O port (`in eax, dx`).
///
/// # Safety
///
/// Direct port reads can trigger hardware side effects on peripheral buses.
#[must_use]
#[inline]
pub unsafe fn in_u32(port: u16) -> u32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        let value: u32;
        core::arch::asm!("in eax, dx", in("dx") port, out("eax") value, options(nomem, nostack, preserves_flags));
        value
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = port;
        0
    }
}

/// Writes a 32-bit double-word to an x86 I/O port (`out dx, eax`).
///
/// # Safety
///
/// Direct port writes alter device state.
#[inline]
pub unsafe fn out_u32(port: u16, value: u32) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        core::arch::asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, value);
    }
}

/// Small delay for slow legacy I/O buses by writing to port `0x80` (POST manufacturing port).
#[inline]
pub fn io_wait() {
    // Safety: Writing zero to diagnostic port 0x80 is an architectural idiom for an ~1 microsecond bus delay.
    unsafe {
        out_u8(0x80, 0);
    }
}

/// Reads a stream of 8-bit bytes from an x86 I/O port into `buffer` (`rep insb`).
///
/// # Safety
///
/// Direct port reads can trigger hardware side effects on peripheral buses.
/// The destination buffer must be valid for writes.
#[inline]
pub unsafe fn ins_u8(port: u16, buffer: &mut [u8]) {
    if buffer.is_empty() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep insb",
            in("dx") port,
            inout("rcx") buffer.len() => _,
            inout("rdi") buffer.as_mut_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep insb",
            in("dx") port,
            inout("ecx") buffer.len() => _,
            inout("edi") buffer.as_mut_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, buffer);
    }
}

/// Writes a stream of 8-bit bytes from `buffer` to an x86 I/O port (`rep outsb`).
///
/// # Safety
///
/// Direct port writes alter chipset registers and device controllers.
#[inline]
pub unsafe fn outs_u8(port: u16, buffer: &[u8]) {
    if buffer.is_empty() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep outsb",
            in("dx") port,
            inout("rcx") buffer.len() => _,
            inout("rsi") buffer.as_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let ptr = buffer.as_ptr();
        let len = buffer.len();
        core::arch::asm!(
            "push esi",
            "mov esi, {ptr}",
            "cld",
            "rep outsb",
            "pop esi",
            ptr = in(reg) ptr,
            in("dx") port,
            inout("ecx") len => _,
        );
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, buffer);
    }
}

/// Reads a stream of 16-bit words from an x86 I/O port into `buffer` (`rep insw`).
///
/// # Safety
///
/// Direct port reads can trigger hardware side effects on peripheral buses.
/// The destination buffer must be valid for writes.
#[inline]
pub unsafe fn ins_u16(port: u16, buffer: &mut [u16]) {
    if buffer.is_empty() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep insw",
            in("dx") port,
            inout("rcx") buffer.len() => _,
            inout("rdi") buffer.as_mut_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep insw",
            in("dx") port,
            inout("ecx") buffer.len() => _,
            inout("edi") buffer.as_mut_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, buffer);
    }
}

/// Writes a stream of 16-bit words from `buffer` to an x86 I/O port (`rep outsw`).
///
/// # Safety
///
/// Direct port writes alter chipset registers and device controllers.
#[inline]
pub unsafe fn outs_u16(port: u16, buffer: &[u16]) {
    if buffer.is_empty() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep outsw",
            in("dx") port,
            inout("rcx") buffer.len() => _,
            inout("rsi") buffer.as_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let ptr = buffer.as_ptr();
        let len = buffer.len();
        core::arch::asm!(
            "push esi",
            "mov esi, {ptr}",
            "cld",
            "rep outsw",
            "pop esi",
            ptr = in(reg) ptr,
            in("dx") port,
            inout("ecx") len => _,
        );
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, buffer);
    }
}

/// Reads a stream of 32-bit double words from an x86 I/O port into `buffer` (`rep insd`).
///
/// # Safety
///
/// Direct port reads can trigger hardware side effects on peripheral buses.
/// The destination buffer must be valid for writes.
#[inline]
pub unsafe fn ins_u32(port: u16, buffer: &mut [u32]) {
    if buffer.is_empty() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep insd",
            in("dx") port,
            inout("rcx") buffer.len() => _,
            inout("rdi") buffer.as_mut_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep insd",
            in("dx") port,
            inout("ecx") buffer.len() => _,
            inout("edi") buffer.as_mut_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, buffer);
    }
}

/// Writes a stream of 32-bit double words from `buffer` to an x86 I/O port (`rep outsd`).
///
/// # Safety
///
/// Direct port writes alter chipset registers and device controllers.
#[inline]
pub unsafe fn outs_u32(port: u16, buffer: &[u32]) {
    if buffer.is_empty() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "cld",
            "rep outsd",
            in("dx") port,
            inout("rcx") buffer.len() => _,
            inout("rsi") buffer.as_ptr() => _,
            options(nostack)
        );
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        let ptr = buffer.as_ptr();
        let len = buffer.len();
        core::arch::asm!(
            "push esi",
            "mov esi, {ptr}",
            "cld",
            "rep outsd",
            "pop esi",
            ptr = in(reg) ptr,
            in("dx") port,
            inout("ecx") len => _,
        );
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = (port, buffer);
    }
}

/// Strongly typed wrapper for an 8-bit I/O port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Port8 {
    port: u16,
}

impl Port8 {
    /// Creates a new 8-bit I/O port accessor.
    #[must_use]
    pub const fn new(port: u16) -> Self {
        Self { port }
    }

    /// Returns the port number.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Reads an 8-bit value from this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that accessing this port is safe in current hardware context.
    #[must_use]
    #[inline]
    pub unsafe fn read(&self) -> u8 {
        unsafe { in_u8(self.port) }
    }

    /// Writes an 8-bit value to this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that writing to this port does not violate hardware contracts.
    #[inline]
    pub unsafe fn write(&self, value: u8) {
        unsafe { out_u8(self.port, value) }
    }

    /// Reads multiple 8-bit bytes from this port into `buffer`.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that reading from this port is safe in current hardware context.
    #[inline]
    pub unsafe fn read_buffer(&self, buffer: &mut [u8]) {
        unsafe { ins_u8(self.port, buffer) }
    }

    /// Writes multiple 8-bit bytes from `buffer` to this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that writing to this port does not violate hardware contracts.
    #[inline]
    pub unsafe fn write_buffer(&self, buffer: &[u8]) {
        unsafe { outs_u8(self.port, buffer) }
    }
}

/// Strongly typed wrapper for a 16-bit I/O port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Port16 {
    port: u16,
}

impl Port16 {
    /// Creates a new 16-bit I/O port accessor.
    #[must_use]
    pub const fn new(port: u16) -> Self {
        Self { port }
    }

    /// Returns the port number.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Reads a 16-bit value from this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that accessing this port is safe in current hardware context.
    #[must_use]
    #[inline]
    pub unsafe fn read(&self) -> u16 {
        unsafe { in_u16(self.port) }
    }

    /// Writes a 16-bit value to this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that writing to this port does not violate hardware contracts.
    #[inline]
    pub unsafe fn write(&self, value: u16) {
        unsafe { out_u16(self.port, value) }
    }

    /// Reads multiple 16-bit words from this port into `buffer`.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that reading from this port is safe in current hardware context.
    #[inline]
    pub unsafe fn read_buffer(&self, buffer: &mut [u16]) {
        unsafe { ins_u16(self.port, buffer) }
    }

    /// Writes multiple 16-bit words from `buffer` to this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that writing to this port does not violate hardware contracts.
    #[inline]
    pub unsafe fn write_buffer(&self, buffer: &[u16]) {
        unsafe { outs_u16(self.port, buffer) }
    }
}

/// Strongly typed wrapper for a 32-bit I/O port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Port32 {
    port: u16,
}

impl Port32 {
    /// Creates a new 32-bit I/O port accessor.
    #[must_use]
    pub const fn new(port: u16) -> Self {
        Self { port }
    }

    /// Returns the port number.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Reads a 32-bit value from this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that accessing this port is safe in current hardware context.
    #[must_use]
    #[inline]
    pub unsafe fn read(&self) -> u32 {
        unsafe { in_u32(self.port) }
    }

    /// Writes a 32-bit value to this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that writing to this port does not violate hardware contracts.
    #[inline]
    pub unsafe fn write(&self, value: u32) {
        unsafe { out_u32(self.port, value) }
    }

    /// Reads multiple 32-bit double words from this port into `buffer`.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that reading from this port is safe in current hardware context.
    #[inline]
    pub unsafe fn read_buffer(&self, buffer: &mut [u32]) {
        unsafe { ins_u32(self.port, buffer) }
    }

    /// Writes multiple 32-bit double words from `buffer` to this port.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that writing to this port does not violate hardware contracts.
    #[inline]
    pub unsafe fn write_buffer(&self, buffer: &[u32]) {
        unsafe { outs_u32(self.port, buffer) }
    }
}
