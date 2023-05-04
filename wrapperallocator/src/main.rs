mod myalloc;

use crate::myalloc::MyAlloc;
use std::sync::atomic::Ordering::Acquire;

use rayon::prelude::*;

#[global_allocator]
static ALLOCATOR: MyAlloc = MyAlloc::new();

fn stats(msg: &str) {
    println!("{} -- used: {}", msg, ALLOCATOR.used.load(Acquire));
}

fn someotherfunc() {
    let _s = format!("Allocating another string!");
    stats("allocated in someotherfunc");
}


fn somefunc() {
    let _s = format!("Allocating a string!");
    stats("allocated in somefunc");
    someotherfunc();
    stats("allocated after someotherfunc");
}

fn threadedfunc() {
    let sum = {
        let col: Vec<i32> = {
            let big: Vec<Vec<i32>> = (1 ..=150) // this is about as big as we can go without overflowing our static pool
                .collect::<Vec<_>>()
                .par_iter()
                .map(|i| -> Vec<i32> {
                    (0..*i).collect::<Vec<_>>()
                })
                .collect()
                ;
            stats("After initial allocation");
            big
        }.into_iter().flatten().collect();
        stats("After running par_iter");
        col.into_iter().sum::<i32>()
    };
    stats("After getting rid of collection");
    println!("Threaded sum: {}", sum);
}

fn main() {
    stats("allocated");
    somefunc();
    stats("allocated after somefunc");
    let mut v = Vec::<u8>::with_capacity(16);
    stats("allocated vec");
    v.resize(64, 0);
    stats("resized vec");
    v.resize(0, 0);
    stats("resized vec to 0");
    v.shrink_to(0);
    stats("shrunk vec to 0");
    threadedfunc();
    threadedfunc();
    stats("after threadedfunc");
}
