use core::alloc::{
    Layout, // allocation request
};

// arbitrarily decide on a pool of data;
// the quality of this allocator is irrelevant to the discussion,
// so pretend this is some smart and highly efficient arena allocator;
// for this excercise, we'll use a dump bitmap of 640 cells;
// we won't even use it as a true bitmap, we'll use a full 8 bits to
// mark a single cell; all cells are 1024 bytes large;
// again, irrelevant; if you want to implement a proper good allocator,
// you can still use this framework, but, you know, actually implement
// a good allocator :-)
const BITMAP_SIZE: usize = 640;
const BLOCK_SIZE: usize = 1024;
const TOTAL_MEM: usize = BITMAP_SIZE * BLOCK_SIZE + BITMAP_SIZE;

// Keep track of allocations
pub struct MyAllocImpl {
    // hint where an empty block might be
    pub hint: usize,
    // the first BITMAP_SIZE bytes are our allocation bitmap;
    // the remaining memory is what we allocate out of
    pub mem: [u8; 640 + 640 * 1024],
}

unsafe impl Sync for MyAllocImpl {}

impl MyAllocImpl {
    // token constructor because why not
    pub const fn new() -> Self {
        MyAllocImpl { hint: 0, mem: [0; TOTAL_MEM] }
    }

    pub fn available(&self) -> usize {
        TOTAL_MEM - self.used()
    }

    pub fn used(&self) -> usize {
        let numblocks = (0..BITMAP_SIZE-1)
            .map(|i| -> usize {
                self.mem[i] as usize
            })
            .sum::<usize>();
        numblocks * BLOCK_SIZE
    }

    fn size_to_blocks(sz: usize) -> usize {
        match sz % BLOCK_SIZE {
            0 => sz / BLOCK_SIZE,
            _ => sz / BLOCK_SIZE + 1,
        }
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        // out allocator isn't really alignment aware, so
        // we need to make it; we can do multiple things,
        // like overallocate by align-1 and adjust later,
        // plus one more byte for bookeeping
        // let's do the actual allocation
        let size = layout.size();
        let align = layout.align();
        let align_mask = align-1;
        let oversize = size + align - 1 + 1;

        // space left next to our hint?
        if (self.hint .. self.hint + Self::size_to_blocks(oversize))
            .all(|e| e < BITMAP_SIZE && self.mem[e] == 0)
        {
            (self.hint .. self.hint + Self::size_to_blocks(oversize))
                .for_each(|i| self.mem[i] = 1)
                ;
            let mut ptr = self.hint * BLOCK_SIZE + BITMAP_SIZE;
            // adjust ptr
            if ptr & align_mask > 0 {
                let iptr = ((ptr | align_mask) + 1) - ptr;
                ptr = (ptr | align_mask) + 1;
                self.mem[ptr-1] = iptr as u8;
            } else {
                let iptr = 1;
                ptr = ptr + align;
                self.mem[ptr - 1] = iptr as u8;
            }
            // update hint
            self.hint = self.hint + Self::size_to_blocks(oversize);
            // return pointer
            return &mut self.mem[ptr] as *mut u8;
        }

        // can we find empty space?
        let mut i = 0;
        loop {
            if i+Self::size_to_blocks(oversize) >= BITMAP_SIZE {
                return core::ptr::null_mut();
            }
            if self.mem[i] != 0 {
                i = i + 1;
                continue;
            }
            let mut j = i;
            loop {
                j = j + 1;
                if j - i > Self::size_to_blocks(oversize) {
                    // we found space
                    self.hint = j;
                    // mark blocks as used
                    (i .. j)
                        .for_each(|k| self.mem[k] = 1)
                        ;
                    let mut ptr = i * BLOCK_SIZE + BITMAP_SIZE;
                    // adjust ptr
                    if ptr & align_mask > 0 {
                        let iptr = ptr;
                        ptr = (ptr | align_mask) + 1;
                        self.mem[ptr-1] = iptr as u8;
                    } else {
                        let iptr = ptr;
                        ptr = ptr + align;
                        self.mem[ptr - 1] = iptr as u8;
                    }
                    // mark blocks as used
                    // return pointer
                    return &mut self.mem[ptr] as *mut u8;
                }
                if self.mem[j] != 0 {
                    i = j + 1;
                    break;
                }
            } // loop j
        } // loop i
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        // we need to determine where in our static memory our
        // pointer actually lies; first, we need to readjust it
        let ptr_minus_one = ((ptr as usize) - 1) as *mut u8;
        let adjustment = *ptr_minus_one as usize;
        // next, do pointer math
        let ptrdiff = (ptr as usize) - (((&mut self.mem[BITMAP_SIZE]) as *mut u8) as usize);
        let adj_ptrdiff = ptrdiff - adjustment;
        let base = adj_ptrdiff / BLOCK_SIZE;
        // redetermine sizes and whatnot
        let size = layout.size();
        let align = layout.align();
        let oversize = size + align - 1 + 1;

        // mark blocks as free
        (base .. base + Self::size_to_blocks(oversize))
            .for_each(|i| self.mem[i] = 0)
            ;

        // update hint if necessary
        if self.hint >= BITMAP_SIZE || self.mem[self.hint] > 0 {
            self.hint = base;
        }
    }
}

