use std::sync::Arc;
use std::thread;

use crossbeam_queue::SegQueue;

// cargo run --bin crossbeam-queue
fn main() {
    // we still needed to use an Arc to control the multiple references
    //  to the queue. This is needed because the queue itself cannot be
    // duplicated and shared, it has no reference count
    let queue = Arc::new(SegQueue::new());

    // It will iterate 1,000,000 times in 5 different threads,
    // and each time it will push a 10 to a queue
    let handles: Vec<_> = (1..6)
        .map(|_| {
            let t_queue = queue.clone();
            thread::spawn(move || {
                for _ in 0..1_000_000 {
                    t_queue.push(10);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let final_queue = Arc::try_unwrap(queue).unwrap();
    let mut sum = 0;
    while let Ok(i) = final_queue.pop() {
        sum += i;
    }

    println!("Final sum: {}", sum);
}
