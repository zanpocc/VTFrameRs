use core::sync::atomic::{AtomicBool, Ordering};


pub struct SpinLock {
    lock: AtomicBool, 
}

impl SpinLock {
    pub fn new() -> Self {
        SpinLock { 
            lock: AtomicBool::new(false),
        }
    }

    pub fn acquire(&self) -> bool {
        // let r = self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed);
        // match r {
        //     Ok(_) => {
        //         return true;
        //     }
        //     Err(_) => {
        //         return false;
        //     }
        // }

        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {}
        true
    }

    pub fn release(&self) {
        if self.lock.load(Ordering::SeqCst){
            self.lock.store(false, Ordering::Release);
        }
    }
}

impl Drop for SpinLock{
    fn drop(&mut self) {
        self.release();
    }
}