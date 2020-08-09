use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let main_vec = Arc::new(Mutex::new(Vec::new()));
    let thread_vec = main_vec.clone();

    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            for i in 0..10 {
                let mut inside_vec = thread_vec.lock().unwrap();
                inside_vec.push(i);
                panic!("Panicking the secondary thread");
            }
        })
        .expect("could not create the thread");

    // small 1-second sleep in the main thread to make sure that
    //  the secondary thread would execute before the main one
    thread::sleep(Duration::from_secs(1));

    for i in 10..20 {
        let mut outside_vec = match main_vec.lock() {
            Ok(g) => g,
            Err(e) => {
                println!(
                    "The secondary thread panicked, recovering from inside the second thread…"
                );
                // When a Mutex is poisoned because a thread panicked
                // while having it locked, the error result of calling
                // the lock() method will return the poisoning error.
                // We can recover from it by calling the into_inner() method
                e.into_inner()
            }
        };
        outside_vec.push(i);
    }

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }

    let vec_mutex = Arc::try_unwrap(main_vec).unwrap();

    //Consumes this mutex, returning the underlying data.
    let final_vec = match vec_mutex.into_inner() {
        Ok(g) => g,
        Err(e) => {
            println!("The secondary thread panicked, recovering from the main thread…");
            e.into_inner()
        }
    };

    // once a Mutex has been poisoned, it will stay poisoned for all of its life
    // In any case, you can still use it and as you can see, the final vector
    // will contain values from both threads; only the 0 from the secondary
    // thread, until the panic, and then the rest from the main thread
    println!("Final vector: {:?}", final_vec);
}

// thread 'my thread' panicked at 'Panicking the secondary thread', multithreading/src/bin/my-mutex-poisoning.rs:15:17
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// The secondary thread panicked, recovering from inside the second thread…
// Something bad happened :(
// The secondary thread panicked, recovering from the main thread…
// Final vector: [0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
