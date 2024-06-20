use core::{
    cell::Cell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use wdk_sys::ntddk::PsGetCurrentThreadId;

pub struct ReentrantSpinLock {
    lock: AtomicBool,
    owner: AtomicUsize,
    recursion_count: Cell<usize>,
}

pub struct ReentrantSpinGuard<'a> {
    spinlock: &'a ReentrantSpinLock,
}

impl ReentrantSpinLock {
    pub fn new() -> Self {
        ReentrantSpinLock {
            lock: AtomicBool::new(false),
            owner: AtomicUsize::new(0),
            recursion_count: Cell::new(0),
        }
    }

    pub fn lock(&self) -> ReentrantSpinGuard {
        let current_thread_id = unsafe { PsGetCurrentThreadId() } as usize;

        if self.owner.load(Ordering::Relaxed) == current_thread_id {
            // 如果当前线程已经持有锁，则递增递归计数
            self.recursion_count.set(self.recursion_count.get() + 1);
        } else {
            // 否则尝试获取锁
            while self
                .lock
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_err()
            {
                // Busy-wait (spin)
            }
            // 锁定后设置所有者和递归计数
            self.owner.store(current_thread_id, Ordering::Relaxed);
            self.recursion_count.set(1);
        }

        ReentrantSpinGuard { spinlock: self }
    }
}

impl Drop for ReentrantSpinGuard<'_> {
    fn drop(&mut self) {
        // 减少递归计数
        let count = self.spinlock.recursion_count.get();
        if count > 1 {
            self.spinlock.recursion_count.set(count - 1);
        } else {
            // 如果递归计数为1，则释放锁
            self.spinlock.recursion_count.set(0);
            self.spinlock.owner.store(0, Ordering::Relaxed);
            self.spinlock.lock.store(false, Ordering::Release);
        }
    }
}

impl core::ops::Deref for ReentrantSpinGuard<'_> {
    type Target = ReentrantSpinLock;

    fn deref(&self) -> &Self::Target {
        self.spinlock
    }
}
