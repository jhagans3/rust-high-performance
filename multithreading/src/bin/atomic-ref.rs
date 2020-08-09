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
                thread_pointer.fetch_add(1, Ordering::Relaxed);
            }
        })
        .expect("could not create the thread");

    for _ in 0..250_000 {
        // atomics have the great fetch_add() function and its friends fetch_sub(),
        // fetch_and(), fetch_or(), and fetch_xor().
        // They will perform the complete operation atomically
        main_pointer.fetch_add(1, Ordering::Relaxed);
    }

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }

    let a_int = Arc::try_unwrap(main_pointer).unwrap();
    println!("Final number: {}", a_int.into_inner());
}
