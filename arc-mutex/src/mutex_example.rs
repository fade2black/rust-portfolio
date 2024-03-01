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
