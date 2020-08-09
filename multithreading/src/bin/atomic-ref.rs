use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let my_val = AtomicUsize::new(0);
    let main_pointer = Arc::new(my_val);
    let thread_pointer = main_pointer.clone();

    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            for _ in 0..250_000 {
                let cur_value = thread_pointer.load(Ordering::Relaxed);
                let sum = cur_value + 1;
                thread_pointer.store(sum, Ordering::Relaxed);
            }
        })
        .expect("could not create the thread");

    for _ in 0..250_000 {
        let cur_value = main_pointer.load(Ordering::Relaxed);
        let sum = cur_value + 1;
        // When we save the integer, we don't check whether
        // it has changed, so we are overriding whatever was written there
        main_pointer.store(sum, Ordering::Relaxed);
    }

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }

    let a_int = Arc::try_unwrap(main_pointer).unwrap();
    println!("Final number: {}", a_int.into_inner());
}
