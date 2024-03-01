use std::{thread, sync::{Arc, Mutex}};
pub mod config;

pub fn compute_sum(integers: &Vec<usize>) -> usize {
    let len = integers.len();
    let cpus = num_cpus::get();
    let total_sum = Arc::new(Mutex::new(0));

    thread::scope(|s| {
        for i in 0..cpus { 
            let total = total_sum.clone();
            s.spawn(move || {
                for k in (i..len).step_by(cpus) {
                    *total.lock().unwrap() += integers[k];
                }
            });
        }
    });

    let guard = total_sum.lock().unwrap();
    *guard
}