//! # Interrupt-Safe Synchronization
//!
//! Provides processor interrupt management and interrupt-safe spinlocks.
//! Prevents deadlock conditions where an interrupt handler executes on the same
//! processor core that is currently holding a spinlock.

use core::cell::UnsafeCell;
use core::fmt;
use core::ops::{Deref, DerefMut};
#[cfg(not(any(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        any(target_os = "uefi", target_os = "none")
    ),
    all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none"))
)))]
use core::sync::atomic::{AtomicBool, Ordering};

use super::spin::{SpinLock, SpinLockGuard};

#[cfg(not(any(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        any(target_os = "uefi", target_os = "none")
    ),
    all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none"))
)))]
static HOST_INTERRUPT_ENABLED: AtomicBool = AtomicBool::new(true);

/// Holds the saved processor interrupt state before interrupts were disabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InterruptState {
    /// True if hardware interrupts were enabled prior to disabling them.
    pub was_enabled: bool,
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        any(target_os = "uefi", target_os = "none")
    ))]
    raw_flags: usize,
    #[cfg(all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none")))]
    raw_daif: u64,
}

impl InterruptState {
    /// Constructs an interrupt state descriptor.
    #[must_use]
    pub const fn new(was_enabled: bool) -> Self {
        Self {
            was_enabled,
            #[cfg(all(
                any(target_arch = "x86", target_arch = "x86_64"),
                any(target_os = "uefi", target_os = "none")
            ))]
            raw_flags: if was_enabled { 0x200 } else { 0 },
            #[cfg(all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none")))]
            raw_daif: if was_enabled { 0 } else { 0x80 },
        }
    }
}

/// Checks whether hardware interrupts are currently enabled on the calling processor.
#[must_use]
pub fn interrupts_enabled() -> bool {
    #[cfg(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        let flags: usize;
        core::arch::asm!("pushfq", "pop {}", out(reg) flags, options(nomem, preserves_flags));
        (flags & (1 << 9)) != 0
    }

    #[cfg(all(target_arch = "x86", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        let flags: usize;
        core::arch::asm!("pushfd", "pop {}", out(reg) flags, options(nomem, preserves_flags));
        (flags & (1 << 9)) != 0
    }

    #[cfg(all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        let daif: u64;
        core::arch::asm!("mrs {}, daif", out(reg) daif, options(nomem, nostack, preserves_flags));
        // Bit 7 is the IRQ mask bit; 0 means IRQs are enabled/unmasked.
        (daif & (1 << 7)) == 0
    }

    #[cfg(not(any(
        all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")),
        all(target_arch = "x86", any(target_os = "uefi", target_os = "none")),
        all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none"))
    )))]
    {
        HOST_INTERRUPT_ENABLED.load(Ordering::SeqCst)
    }
}

/// Disables hardware interrupts on the calling processor and returns the previous state.
#[must_use]
pub fn disable_interrupts() -> InterruptState {
    #[cfg(all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        let flags: usize;
        core::arch::asm!("pushfq", "pop {}", out(reg) flags, options(nomem, preserves_flags));
        core::arch::asm!("cli", options(nomem, nostack));
        InterruptState {
            was_enabled: (flags & (1 << 9)) != 0,
            raw_flags: flags,
        }
    }

    #[cfg(all(target_arch = "x86", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        let flags: usize;
        core::arch::asm!("pushfd", "pop {}", out(reg) flags, options(nomem, preserves_flags));
        core::arch::asm!("cli", options(nomem, nostack));
        InterruptState {
            was_enabled: (flags & (1 << 9)) != 0,
            raw_flags: flags,
        }
    }

    #[cfg(all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        let daif: u64;
        core::arch::asm!("mrs {}, daif", out(reg) daif, options(nomem, nostack, preserves_flags));
        // Mask IRQ (I bit, bit 7 -> daifset #2)
        core::arch::asm!("msr daifset, #2", options(nomem, nostack, preserves_flags));
        InterruptState {
            was_enabled: (daif & (1 << 7)) == 0,
            raw_daif: daif,
        }
    }

    #[cfg(not(any(
        all(target_arch = "x86_64", any(target_os = "uefi", target_os = "none")),
        all(target_arch = "x86", any(target_os = "uefi", target_os = "none")),
        all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none"))
    )))]
    {
        let prev = HOST_INTERRUPT_ENABLED.swap(false, Ordering::SeqCst);
        InterruptState::new(prev)
    }
}

/// Enables hardware interrupts on the calling processor.
///
/// # Safety
///
/// Caller must ensure that enabling interrupts does not violate any critical section
/// or lock invariants currently active on this core.
pub unsafe fn enable_interrupts() {
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        any(target_os = "uefi", target_os = "none")
    ))]
    unsafe {
        core::arch::asm!("sti", options(nomem, nostack));
    }

    #[cfg(all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none")))]
    unsafe {
        // Unmask IRQ (I bit, bit 7 -> daifclr #2)
        core::arch::asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags));
    }

    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            any(target_os = "uefi", target_os = "none")
        ),
        all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none"))
    )))]
    {
        HOST_INTERRUPT_ENABLED.store(true, Ordering::SeqCst);
    }
}

/// Restores processor interrupts to the state captured in `state`.
pub fn restore_interrupts(state: InterruptState) {
    if state.was_enabled {
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            any(target_os = "uefi", target_os = "none")
        ))]
        unsafe {
            core::arch::asm!("sti", options(nomem, nostack));
        }

        #[cfg(all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none")))]
        unsafe {
            core::arch::asm!("msr daif, {}", in(reg) state.raw_daif, options(nomem, nostack, preserves_flags));
        }

        #[cfg(not(any(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                any(target_os = "uefi", target_os = "none")
            ),
            all(target_arch = "aarch64", any(target_os = "uefi", target_os = "none"))
        )))]
        {
            HOST_INTERRUPT_ENABLED.store(true, Ordering::SeqCst);
        }
    }
}

struct IrqGuard(InterruptState);

impl Drop for IrqGuard {
    fn drop(&mut self) {
        restore_interrupts(self.0);
    }
}

/// Executes a closure within a critical section with hardware interrupts disabled.
///
/// The previous interrupt state is restored when the closure returns.
pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let state = disable_interrupts();
    let _guard = IrqGuard(state);
    f()
}

/// An interrupt-safe spinlock.
///
/// Disables interrupts on the local processor before acquiring the spinlock,
/// and restores the previous interrupt state upon releasing the lock.
pub struct IrqSpinLock<T> {
    inner: SpinLock<()>,
    data: UnsafeCell<T>,
}

// Safety: `IrqSpinLock` can be shared across execution contexts if `T: Send`.
unsafe impl<T: Send> Sync for IrqSpinLock<T> {}
unsafe impl<T: Send> Send for IrqSpinLock<T> {}

/// An RAII guard for [`IrqSpinLock`].
///
/// Automatically unlocks the spinlock and restores hardware interrupts when dropped.
pub struct IrqSpinLockGuard<'a, T> {
    _guard: SpinLockGuard<'a, ()>,
    lock: &'a IrqSpinLock<T>,
    saved_state: InterruptState,
}

// Safety: `IrqSpinLockGuard` can be transferred across execution contexts if `T: Send`.
unsafe impl<T: Sync> Sync for IrqSpinLockGuard<'_, T> {}

impl<T> IrqSpinLock<T> {
    /// Creates a new interrupt-safe spinlock.
    #[must_use]
    pub const fn new(data: T) -> Self {
        Self {
            inner: SpinLock::new(()),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the lock with interrupts disabled on the calling processor.
    pub fn lock(&self) -> IrqSpinLockGuard<'_, T> {
        let saved_state = disable_interrupts();
        let guard = self.inner.lock();
        IrqSpinLockGuard {
            _guard: guard,
            lock: self,
            saved_state,
        }
    }

    /// Attempts to acquire the lock without spinning.
    ///
    /// If the lock cannot be acquired immediately, interrupts are restored to their
    /// prior state and `None` is returned.
    #[must_use]
    pub fn try_lock(&self) -> Option<IrqSpinLockGuard<'_, T>> {
        let saved_state = disable_interrupts();
        if let Some(guard) = self.inner.try_lock() {
            Some(IrqSpinLockGuard {
                _guard: guard,
                lock: self,
                saved_state,
            })
        } else {
            restore_interrupts(saved_state);
            None
        }
    }

    /// Checks whether the lock is currently held.
    #[must_use]
    pub fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }

    /// Consumes the lock and returns the protected data.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }

    /// Returns a mutable reference to the underlying data without locking.
    #[must_use]
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }
}

impl<T: Default> Default for IrqSpinLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: fmt::Debug> fmt::Debug for IrqSpinLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("IrqSpinLock");
        if let Some(guard) = self.try_lock() {
            d.field("data", &&*guard);
        } else {
            d.field("data", &"<locked>");
        }
        d.finish()
    }
}

impl<T> Deref for IrqSpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: the lock is held exclusively by this guard
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for IrqSpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: the lock is held exclusively by this guard
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for IrqSpinLockGuard<'_, T> {
    fn drop(&mut self) {
        // SpinLockGuard is dropped when _guard field goes out of scope, releasing the lock.
        restore_interrupts(self.saved_state);
    }
}
