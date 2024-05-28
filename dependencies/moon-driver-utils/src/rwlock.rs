use core::{cell::UnsafeCell, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};

extern crate alloc;

pub struct ReadWriteLock<T> {
    lock: AtomicBool,
    readers: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for ReadWriteLock<T> {}

impl<T> ReadWriteLock<T> {
    pub const fn new(data: T) -> Self {
        ReadWriteLock {
            lock: AtomicBool::new(false),
            readers: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        loop {
            while self.lock.load(Ordering::Acquire) {}
            self.readers.fetch_add(1, Ordering::Acquire);
            if !self.lock.load(Ordering::Acquire) {
                break;
            }
            self.readers.fetch_sub(1, Ordering::Release);
        }
        ReadGuard { lock: self }
    }

    pub fn write(&self) -> WriteGuard<T> {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {}
        while self.readers.load(Ordering::Acquire) != 0 {}
        WriteGuard { lock: self }
    }
}

pub struct ReadGuard<'a, T> {
    lock: &'a ReadWriteLock<T>,
}

impl<'a, T> core::ops::Deref for ReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> Drop for ReadGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.readers.fetch_sub(1, Ordering::Release);
    }
}

pub struct WriteGuard<'a, T> {
    lock: &'a ReadWriteLock<T>,
}

impl<'a, T> core::ops::Deref for WriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for WriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for WriteGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.lock.store(false, Ordering::Release);
    }
}