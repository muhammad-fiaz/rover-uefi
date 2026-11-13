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
}
