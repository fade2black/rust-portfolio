# Shared State Concurrency
In the shared memory concurrency model multiple
threads can access the same memory location at the same time.

Suppose we have a vector and we want to access/share
that vector in several threads.
In particular, assume that each
thread takes an index `i`, the vector,
and prints to `stdout` element located at index `i`.

A simple solution is to use `Arc<T>` like the following
```rust
use std::sync::Arc;
use std::thread;

pub fn main() {
    let ptr1 = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..ptr1.len() {
        let ptr2 = Arc::clone(&ptr1);

        let handle = thread::spawn(move || {
            println!("{}", ptr2[i]);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

Next, suppose we want to mutate the vector
inside the threads. We could implement it
using `Mutex<T>`.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    let ptr1 = Arc::new(Mutex::new(vec![0, 1, 2]));
    let mut handles = vec![];

    println!("Before increasing: {:?}", *ptr1);

    for _ in 0..6 {
        let ptr2 = Arc::clone(&ptr1);

        let handle = thread::spawn(move || {
            let mut vptr = ptr2.lock().unwrap();
            // mutate data
            vptr[0] += 1;
            vptr[1] += 1;
            vptr[2] += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("After increasing: {:?}", *ptr1);
}
```
However, `Mutex<T>` comes with the risk of creating deadlocks.
In the following piece of code we have two vectors and we
try to mutually access these vectors inside two different
threads in different order which results in a deadlock.
```rust
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub fn main() {
    let ptr1 = Arc::new(Mutex::new(vec![0, 1, 2]));
    let ptr2 = Arc::new(Mutex::new(vec![3, 4, 5]));
    let mut handles = vec![];

    for i in 0..2 {
        let ptr3 = Arc::clone(&ptr1);
        let ptr4 = Arc::clone(&ptr2);

        let handle = thread::spawn(move || {
            let one_sec = time::Duration::from_secs(1);
            if i & 1 == 0 {
                let _lock1 = ptr3.lock().unwrap();
                thread::sleep(one_sec);
                let _lock2 = ptr4.lock().unwrap();
            } else {
                let _lock1 = ptr4.lock().unwrap();
                thread::sleep(one_sec);
                let _lock2 = ptr3.lock().unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // This line is unreachable
    println!("Done.");
}
```
