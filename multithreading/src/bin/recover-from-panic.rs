use std::thread;

// cargo run --bin recover-from-panic
fn main() {
    println!("Before the thread spawn!");

    let handle = thread::Builder::new()
        .name("bad thread".to_owned())
        .spawn(|| {
            panic!("Panicking inside the thread!");
        })
        .expect("could not create the thread");
    println!("After thread spawn!");

    // you should never just call `expect()` or `unwrap()`
    // when joining a thread, since it could make your whole program fail
    if handle.join().is_err() {
        println!("Something bad happened :(");
    }
    println!("After everything!");
}

// Before the thread spawn!
// After thread spawn!
// thread 'bad thread' panicked at 'Panicking inside the thread!', multithreading/src/bin/recover-from-panic.rs:10:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// Something bad happened :(
// After everything!
