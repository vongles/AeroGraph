use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

#[repr(align(64))]
struct CachePaddedAtomic(AtomicUsize);

pub struct LockFreeSPSCQueue<T, const CAPACITY: usize> {
    head: CachePaddedAtomic,
    tail: CachePaddedAtomic,
    buffer: [UnsafeCell<MaybeUninit<T>>; CAPACITY],
}

unsafe impl<T: Send, const CAPACITY: usize> Sync for LockFreeSPSCQueue<T, CAPACITY> {}
unsafe impl<T: Send, const CAPACITY: usize> Send for LockFreeSPSCQueue<T, CAPACITY> {}

impl<T, const CAPACITY: usize> LockFreeSPSCQueue<T, CAPACITY> {
    pub const fn new() -> Self {
        assert!(CAPACITY.is_power_of_two(), "Capacity must be a power of 2");
        let buffer = unsafe { MaybeUninit::uninit().assume_init() };
        Self {
            head: CachePaddedAtomic(AtomicUsize::new(0)),
            tail: CachePaddedAtomic(AtomicUsize::new(0)),
            buffer,
        }
    }

    pub fn push(&self, value: T) -> Result<(), T> {
        let current_tail = self.tail.0.load(Ordering::Relaxed);
        let current_head = self.head.0.load(Ordering::Acquire);

        if current_tail.wrapping_sub(current_head) >= CAPACITY {
            return Err(value);
        }

        let index = current_tail & (CAPACITY - 1);
        unsafe {
            let slot = self.buffer[index].get();
            (*slot).write(value);
        }

        self.tail.0.store(current_tail.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let current_head = self.head.0.load(Ordering::Relaxed);
        let current_tail = self.tail.0.load(Ordering::Acquire);

        if current_head == current_tail {
            return None;
        }

        let index = current_head & (CAPACITY - 1);
        let value = unsafe {
            let slot = self.buffer[index].get();
            (*slot).assume_init_read()
        };

        self.head.0.store(current_head.wrapping_add(1), Ordering::Release);
        Some(value)
    }
}
