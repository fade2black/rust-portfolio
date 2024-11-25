use stress::config;
use rand::{thread_rng, Rng};
use tokio::{time::Duration, sync::oneshot::{self, error::TryRecvError, Receiver}};

fn compute(mut rx: Receiver<()>){ 
    let mut rng = thread_rng();
    let num = rng.gen_range(1975..2005) as f64;

    while let Err(TryRecvError::Empty) = rx.try_recv() {
          let _ = f64::sqrt(num);
    }
}

#[tokio::main()]
async fn main() {
    let config = config::get_args();

    let mut handles = vec![];
    let mut channels = vec![];

    println!("stress: {} cpu(s), for {} seconds", config.cpus, config.timeout);

    for _ in 0..config.cpus {
        let (tx, rx) = oneshot::channel(); 
        channels.push(tx);
        handles.push(std::thread::spawn(|| compute(rx)));
    }
    
    tokio::time::sleep(Duration::from_secs(config.timeout)).await;

    for tx in channels {
        tx.send(()).unwrap();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
