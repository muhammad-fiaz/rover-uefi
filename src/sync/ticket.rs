//! # FIFO Ticket Spinlock
//!
//! Provides a fair, first-in-first-out (FIFO) spinlock. Cores take a sequential
//! ticket number and spin until their turn is served, ensuring no processor core
//! starves under heavy multiprocessor contention.

use core::cell::UnsafeCell;
use core::fmt;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

use super::barrier::cpu_pause;

/// A fair FIFO spinlock ensuring starvation-free mutual exclusion.
pub struct TicketLock<T> {
    next_ticket: AtomicUsize,
    now_serving: AtomicUsize,
    data: UnsafeCell<T>,
}

// Safety: `TicketLock` can be shared across execution contexts if `T: Send`.
unsafe impl<T: Send> Sync for TicketLock<T> {}
unsafe impl<T: Send> Send for TicketLock<T> {}

/// An RAII guard for the [`TicketLock`]. Releases the lock and serves the next ticket on drop.
pub struct TicketLockGuard<'a, T> {
    lock: &'a TicketLock<T>,
    ticket: usize,
}

// Safety: `TicketLockGuard` can be transferred across execution contexts if `T: Send`.
unsafe impl<T: Sync> Sync for TicketLockGuard<'_, T> {}

impl<T> TicketLock<T> {
    /// Creates a new ticket lock wrapping `data` in an unlocked state.
    #[must_use]
    pub const fn new(data: T) -> Self {
        Self {
            next_ticket: AtomicUsize::new(0),
            now_serving: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the lock, spinning until this core's ticket is served.
    pub fn lock(&self) -> TicketLockGuard<'_, T> {
        let ticket = self.next_ticket.fetch_add(1, Ordering::Relaxed);
        while self.now_serving.load(Ordering::Acquire) != ticket {
            cpu_pause();
        }
        TicketLockGuard { lock: self, ticket }
    }

    /// Attempts to acquire the lock immediately without waiting in line.
    ///
    /// Returns `Some(TicketLockGuard)` if no tickets were pending, or `None`
    /// if the lock was already held.
    #[must_use]
    pub fn try_lock(&self) -> Option<TicketLockGuard<'_, T>> {
        let current = self.now_serving.load(Ordering::Relaxed);
        let result = self.next_ticket.compare_exchange(
            current,
            current.wrapping_add(1),
            Ordering::Acquire,
            Ordering::Relaxed,
        );

        match result {
            Ok(ticket) => Some(TicketLockGuard { lock: self, ticket }),
            Err(_) => None,
        }
    }

    /// Checks whether the ticket lock is currently held.
    #[must_use]
    pub fn is_locked(&self) -> bool {
        self.now_serving.load(Ordering::Relaxed) != self.next_ticket.load(Ordering::Relaxed)
    }

    /// Consumes the lock, returning the underlying data.
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

impl<T: Default> Default for TicketLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: fmt::Debug> fmt::Debug for TicketLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("TicketLock");
        if let Some(guard) = self.try_lock() {
            d.field("data", &&*guard);
        } else {
            d.field("data", &"<locked>");
        }
        d.finish()
    }
}

impl<T> Deref for TicketLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: the lock is held exclusively by the owner of this ticket
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for TicketLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: the lock is held exclusively by the owner of this ticket
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for TicketLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock
            .now_serving
            .store(self.ticket.wrapping_add(1), Ordering::Release);
    }
}
