//! # Spinlock Synchronization Primitive
//!
//! Provides a mutual exclusion lock based on atomic test-and-test-and-set spinning
//! with processor pause hints during contention. Suitable for `#![no_std]` firmware
//! environments and early boot stages before scheduler or thread infrastructure exists.

use core::cell::UnsafeCell;
use core::fmt;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

use super::barrier::cpu_pause;

/// A mutual exclusion lock that spins until the resource is acquired.
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: `SpinLock` can be shared across execution contexts if `T: Send`.
unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

/// An RAII implementation of a "scoped lock" of a spinlock.
///
/// When dropped, the lock is released.
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

// Safety: `SpinLockGuard` can be transferred across execution contexts if `T: Send`.
unsafe impl<T: Sync> Sync for SpinLockGuard<'_, T> {}

impl<T> SpinLock<T> {
    /// Creates a new spinlock wrapping the provided data in an unlocked state.
    #[must_use]
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the lock, spinning until it is available.
    ///
    /// Performs an exponential backoff with [`cpu_pause`] hints when the lock
    /// is contended to avoid cache line bouncing on multiprocessor platforms.
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        let mut backoff = 1usize;
        loop {
            // First check without atomic write to avoid bus locks when already locked
            while self.locked.load(Ordering::Relaxed) {
                for _ in 0..backoff {
                    cpu_pause();
                }
                backoff = (backoff << 1).min(1024);
            }

            // Attempt to acquire
            if !self.locked.swap(true, Ordering::Acquire) {
                return SpinLockGuard { lock: self };
            }
        }
    }

    /// Attempts to acquire the lock without blocking.
    ///
    /// Returns `Some(SpinLockGuard)` if successfully acquired, or `None` if
    /// the lock was already held.
    #[must_use]
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if self.locked.swap(true, Ordering::Acquire) {
            None
        } else {
            Some(SpinLockGuard { lock: self })
        }
    }

    /// Attempts to acquire the lock with a maximum spin loop iteration count.
    ///
    /// Returns `Some(SpinLockGuard)` if acquired within `max_spins`, or `None`
    /// if the timeout elapsed.
    pub fn lock_with_timeout(&self, max_spins: usize) -> Option<SpinLockGuard<'_, T>> {
        for _ in 0..max_spins {
            if !self.locked.load(Ordering::Relaxed) && !self.locked.swap(true, Ordering::Acquire) {
                return Some(SpinLockGuard { lock: self });
            }
            cpu_pause();
        }
        None
    }

    /// Checks whether the lock is currently held.
    #[must_use]
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }

    /// Consumes the spinlock, returning the underlying data.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }

    /// Returns a mutable reference to the underlying data without locking.
    ///
    /// This is safe because the mutable reference guarantees unique access.
    #[must_use]
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }
}

impl<T: Default> Default for SpinLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: fmt::Debug> fmt::Debug for SpinLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("SpinLock");
        if let Some(guard) = self.try_lock() {
            d.field("data", &&*guard);
        } else {
            d.field("data", &"<locked>");
        }
        d.finish()
    }
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: the lock is held by this guard
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: the lock is held exclusively by this guard
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}
