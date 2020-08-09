use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// cargo run --bin atomic-lock
fn main() {
    let my_val = AtomicUsize::new(0);
    let main_thread_pointer = Arc::new(my_val);
    let lock = Arc::new(AtomicBool::new(false));

    let thread_pointer = main_thread_pointer.clone();
    let thread_lock = lock.clone();
    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            for _ in 0..250_000 {
                // make sure that all your threads only change values
                //  when the lock is set to true by them,
                // by using some memory ordering
                while thread_lock.compare_and_swap(false, true, Ordering::Relaxed) {}
                let cur_value = thread_pointer.load(Ordering::Relaxed);
                let sum = cur_value + 1;
                thread_pointer.store(sum, Ordering::Relaxed);
                thread_lock.store(false, Ordering::Relaxed);
            }
        })
        .expect("could not create my thread");

    for _ in 0..250_000 {
        // compare_and_swap() and compare_exchange()
        // functions, which can be used to create locks
        while lock.compare_and_swap(false, true, Ordering::Relaxed) {}
        let cur_value = main_thread_pointer.load(Ordering::Relaxed);
        let sum = cur_value + 1;
        main_thread_pointer.store(sum, Ordering::Relaxed);
        lock.store(false, Ordering::Relaxed);
    }

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }

    let a_int = Arc::try_unwrap(main_thread_pointer).unwrap();
    println!("Final number: {}", a_int.into_inner());
}
