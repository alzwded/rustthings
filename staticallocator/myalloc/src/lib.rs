// don't use std, since we are masochists
#![no_std]

mod myallocimpl;

use crate::myallocimpl::MyAllocImpl;

use core::alloc::{
    GlobalAlloc, // trait for the global allocator
    Layout, // allocation request
};
use core::cell::RefCell; // needed to have mutable data in the GlobalAlloc implementation
use core::sync::atomic::{
    AtomicBool,
    Ordering::{
        Acquire,
        Relaxed,
        SeqCst, // sequential memory access for all threads; the tightest; 
                // these are the same as C++'s memory_order_* types
    }
};


pub struct MyAlloc {
    // I would use a mutex, but that requires allocations, so we have a bit of
    // a chicken and egg problem; an atomic int and extensive use of 
    // AtomicCompareAndSwap should do the job
    pub poor_mans_lock: AtomicBool,
    data: RefCell<MyAllocImpl>
}

impl MyAlloc {
    // token constructor because why not
    pub const fn new() -> Self {
        MyAlloc { poor_mans_lock: AtomicBool::new(false), data: RefCell::new(MyAllocImpl::new()) }
    }

    fn threadyield() {
        // TODO, find a way to yield platform independently;
        //       rust's yield_now is in std
    }

    fn spin(&self) {
        loop {
            if self.poor_mans_lock.compare_exchange_weak(false,
                                                        true,
                                                        Acquire,
                                                        Relaxed)
                .is_ok()
            {
                break;
            }
            Self::threadyield();
        }
    }

    fn unspin(&self) {
        self.poor_mans_lock.store(false, SeqCst);
    }

    pub fn available(&self) -> usize {
        let rval;
        self.spin();
        {
            rval = self.data.borrow().available();
        }
        self.unspin();
        return rval;
    }

    pub fn used(&self) -> usize {
        let rval;
        self.spin();
        {
            rval = self.data.borrow().used();
        }
        self.unspin();
        return rval;
    }

}

// Actually implement the allocator
unsafe impl GlobalAlloc for MyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // acquire lock
        self.spin();
        // go into a method for the borrow_mut(), which must return before we spin down
        // otherwise we get a panic
        let rval = {
            let mut allocimpl = self.data.borrow_mut();
            allocimpl.alloc(layout)
        };
        // release lock
        self.unspin();
        return rval;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr == core::ptr::null_mut() {
            // nothing to do
            return;
        }

        // acquire lock
        self.spin();
        // go into a method for the borrow_mut(), which must return before we spin down
        // otherwise we get a panic
        {
            let mut allocimpl = self.data.borrow_mut();
            allocimpl.dealloc(ptr, layout);
        }
        // release lock
        self.unspin();
    }
}

// Send denotes that a type is safe to send to another thread
// Sync denotes that a type is safe to be shared across threads
// Our allocator only has an atomic int accessed with the tightest
// memory ordering requirements, and the System allocator has
// this trait already.
unsafe impl Sync for MyAlloc {}

