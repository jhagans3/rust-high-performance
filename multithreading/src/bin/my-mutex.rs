use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let my_vec = Arc::new(Mutex::new(Vec::new()));

    let t_vec = my_vec.clone();
    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            for i in 0..50 {
                t_vec.lock().unwrap().push(i);
            }
        })
        .expect("could not create the thread");

    for i in 0..50 {
        my_vec.lock().unwrap().push(i);
    }

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }

    let vec_mutex = Arc::try_unwrap(my_vec).unwrap();
    let f_vec = vec_mutex.into_inner().unwrap();
    println!("Final vector: {:?}", f_vec);
}

// output
// Final vector:
// [0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
// 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
// 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
// 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
// 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,

// The main issue with sharing information
// between threads is that when the Mutex locks,
// it requires synchronization from both threads

// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
// 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
// 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
// 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
// 40, 41, 42, 43, 44, 45, 46, 47, 48, 49]
