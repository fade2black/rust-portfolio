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
