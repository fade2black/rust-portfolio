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
