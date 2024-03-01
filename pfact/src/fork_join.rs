use num::{bigint::BigUint, One};
use std::thread;

pub fn compute(num: usize) -> BigUint {
    let cpus = num_cpus::get();
    let mut total_prod: BigUint = One::one();
    let mut handles = vec![];

    for i in 1..=cpus { 
        handles.push(
            thread::spawn(move || {
                let mut prod: BigUint = One::one();
                for n in (i..=num).step_by(cpus) {
                    prod *= n;
                }
                prod
            })
        ); 
    }
    
    for handle in handles {
        total_prod *= handle.join().unwrap();
    }

    total_prod
}
