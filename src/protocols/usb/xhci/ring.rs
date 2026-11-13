//! # xHCI rings
//!
//! Command rings, transfer rings and event ring segments used by the
//! eXtensible Host Controller Interface (`xHCI`).
//!
//! A ring is an array of 16-byte Transfer Request Blocks. The producer
//! writes TRBs into the ring and advances the enqueue pointer; the
//! controller consumes them and advances the dequeue pointer. The cycle
//! bit of the last TRB of a lap tells the reader whether the entry it
//! looks at belongs to the lap that is currently being produced.

use crate::protocols::usb::xhci::trb::{TRB_SIZE, Trb};

/// A ring of Transfer Request Blocks.
///
/// The ring is a view over memory the caller owns. The cycle bit of
/// every TRB written through [`TransferRing::enqueue`] is set to the
/// cycle state of the lap the ring is currently on.
pub struct TransferRing<'a> {
    bytes: &'a mut [u8],
    enqueue: usize,
    dequeue: usize,
    cycle: bool,
}

impl<'a> TransferRing<'a> {
    /// Creates a ring over `bytes`.
    ///
    /// Returns `None` when the buffer is empty or is not a whole
    /// number of TRBs.
    #[must_use]
    pub fn new(bytes: &'a mut [u8], cycle: bool) -> Option<Self> {
        if bytes.is_empty() || bytes.len() % TRB_SIZE != 0 {
            return None;
        }
        Some(Self {
            bytes,
            enqueue: 0,
            dequeue: 0,
            cycle,
        })
    }

    /// Returns the number of TRBs in the ring.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.bytes.len() / TRB_SIZE
    }

    /// Returns the number of TRBs waiting to be consumed.
    #[must_use]
    pub fn len(&self) -> usize {
        if self.enqueue >= self.dequeue {
            self.enqueue - self.dequeue
        } else {
            self.capacity() - self.dequeue + self.enqueue
        }
    }

    /// Returns whether the ring holds no unconsumed TRB.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.enqueue == self.dequeue
    }

    /// Returns whether the ring is full.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Returns the index of the TRB the producer writes next.
    #[must_use]
    pub fn enqueue_index(&self) -> usize {
        self.enqueue
    }

    /// Returns the index of the TRB the consumer reads next.
    #[must_use]
    pub fn dequeue_index(&self) -> usize {
        self.dequeue
    }

    /// Returns the cycle state of the lap being produced.
    #[must_use]
    pub fn cycle(&self) -> bool {
        self.cycle
    }

    /// Writes `trb` into the ring and advances the enqueue pointer.
    ///
    /// Returns the index of the TRB that was written, or `None` when
    /// the ring is full.
    pub fn enqueue(&mut self, trb: &Trb) -> Option<usize> {
        if self.is_full() {
            return None;
        }
        let index = self.enqueue;
        let mut staged = trb.clone();
        staged.set_cycle(self.cycle);
        let start = index * TRB_SIZE;
        let chunk = self.bytes.get_mut(start..start + TRB_SIZE)?;
        if !staged.encode(chunk) {
            return None;
        }
        self.enqueue += 1;
        if self.enqueue == self.capacity() {
            self.enqueue = 0;
            self.cycle = !self.cycle;
        }
        Some(index)
    }

    /// Reads the next TRB and advances the dequeue pointer.
    pub fn dequeue(&mut self) -> Option<Trb> {
        if self.is_empty() {
            return None;
        }
        let index = self.dequeue;
        let start = index * TRB_SIZE;
        let chunk = self.bytes.get(start..start + TRB_SIZE)?;
        let trb = Trb::decode(chunk)?;
        self.dequeue += 1;
        if self.dequeue == self.capacity() {
            self.dequeue = 0;
        }
        Some(trb)
    }
}

/// A command ring, the source of the commands the controller executes.
pub struct CommandRing<'a> {
    ring: TransferRing<'a>,
}

impl<'a> CommandRing<'a> {
    /// Creates a command ring over `bytes`.
    #[must_use]
    pub fn new(bytes: &'a mut [u8], cycle: bool) -> Option<Self> {
        TransferRing::new(bytes, cycle).map(|ring| Self { ring })
    }

    /// Returns the number of commands waiting to be executed.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ring.len()
    }

    /// Returns whether the ring holds no pending command.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }

    /// Writes a command into the ring.
    pub fn enqueue(&mut self, trb: &Trb) -> Option<usize> {
        self.ring.enqueue(trb)
    }

    /// Reads the next command.
    pub fn dequeue(&mut self) -> Option<Trb> {
        self.ring.dequeue()
    }
}

/// A segment of an event ring.
///
/// The controller fills the segment with event TRBs. Every entry carries
/// the cycle bit of the lap that produced it, so an entry whose cycle bit
/// does not match the expected state has not been written yet.
pub struct EventRingSegment<'a> {
    bytes: &'a [u8],
    index: usize,
    cycle: bool,
}

impl<'a> EventRingSegment<'a> {
    /// Creates a cursor over an event ring segment.
    ///
    /// Returns `None` when the buffer is empty or is not a whole number
    /// of TRBs.
    #[must_use]
    pub fn new(bytes: &'a [u8], cycle: bool) -> Option<Self> {
        if bytes.is_empty() || bytes.len() % TRB_SIZE != 0 {
            return None;
        }
        Some(Self {
            bytes,
            index: 0,
            cycle,
        })
    }

    /// Returns the index of the entry the cursor points at.
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the byte offset of the entry the cursor points at.
    #[must_use]
    pub fn offset(&self) -> usize {
        self.index * TRB_SIZE
    }

    /// Returns the expected cycle state of the entries.
    #[must_use]
    pub fn cycle(&self) -> bool {
        self.cycle
    }

    /// Returns the entry the cursor points at, without consuming it.
    #[must_use]
    pub fn peek(&self) -> Option<Trb> {
        let start = self.offset();
        let chunk = self.bytes.get(start..start + TRB_SIZE)?;
        Trb::decode(chunk)
    }
}

impl Iterator for EventRingSegment<'_> {
    type Item = Trb;

    /// Returns the next event that the controller has produced.
    fn next(&mut self) -> Option<Self::Item> {
        let trb = self.peek()?;
        if trb.cycle() != self.cycle {
            return None;
        }
        if !trb.trb_type().is_event() {
            return None;
        }
        self.index += 1;
        if self.index * TRB_SIZE == self.bytes.len() {
            self.index = 0;
            self.cycle = !self.cycle;
        }
        Some(trb)
    }
}
