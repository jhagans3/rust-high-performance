use std::sync::Arc;
use std::thread;

// cargo run --bin move-ref
fn main() {
    let my_vec = vec![10, 33, 54];

    // The behavior of `Arc`, being able to be
    // shared among threads, is only thanks to implementing the
    // `Sync` trait, and it will only implement it if the inner
    // value implements `Sync` and `Send`
    let main_thread_pointer = Arc::new(my_vec);

    let second_thread_pointer = main_thread_pointer.clone();
    let handle = thread::Builder::new()
        .name("my vector thread".to_owned())
        .spawn(move || {
            println!("Vector in second thread: {:?}", second_thread_pointer);
        })
        .expect("could not create the thread");

    println!("Vector in main thread: {:?}", main_thread_pointer);

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }
}
