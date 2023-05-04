use std::alloc::{
    GlobalAlloc, // trait for the global allocator
    Layout, // allocation request
    System, // the normal global allocator
};
use std::ptr::null_mut; // similar to std::ptr::null(), but mutable :-)
use std::sync::atomic::{
    AtomicUsize,
    Ordering::SeqCst, // sequential memory access for all threads; the tightest; 
                      // these are the same as C++'s memory_order_* types
};

#[derive(Default)]
pub struct MyAlloc {
    // Keep track of allocations
    pub used: AtomicUsize,
}

impl MyAlloc {
    // token constructor because why not
    pub const fn new() -> Self {
        MyAlloc { used: AtomicUsize::new(0) }
    }
}

// Send denotes that a type is safe to send to another thread
// Sync denotes that a type is safe to be shared across threads
// Our allocator only has an atomic int accessed with the tightest
// memory ordering requirements, and the System allocator has
// this trait already.
unsafe impl Sync for MyAlloc {}

// Actually implement the allocator
unsafe impl GlobalAlloc for MyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        // atomically update our tracking number
        if self
            .used
            .fetch_update(SeqCst, SeqCst, |mut used| {
                used += size;
                Some(used)
            })
            .is_err()
        {
            return null_mut();
        };
        // let the system allocator deal with it
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        // atomically update our tracking number
        if self
            .used
            .fetch_update(SeqCst, SeqCst, |mut used| {
                used -= size;
                Some(used)
            })
            .is_err()
        {
            panic!("something's wrong");
        };
        // let the system allocator deal with it
        System.dealloc(ptr, layout);
    }
}
