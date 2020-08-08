use std::thread;

struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        if thread::panicking() {
            println!(
                "The thread is panicking with the {} struct (from drop)!",
                self.name
            );
        } else {
            println!("The {} struct is dropped", self.name);
        }
    }
}

// cargo run --bin recover-from-panic
fn main() {
    let _main_fn_struct = MyStruct {
        name: "main_fn_struct".to_owned(),
    };

    {
        let _inner_scoped_struct = MyStruct {
            name: "inner_scoped_struct".to_owned(),
        };
    }

    println!("Before the thread spawn!");

    let handle = thread::Builder::new()
        .name("bad thread".to_owned())
        .spawn(|| {
            let _thread_struct = MyStruct {
                name: "thread_struct".to_owned(),
            };

            panic!("Panicking inside the thread!");
        })
        .expect("could not create the thread");
    println!("After thread spawn!");

    // you should never just call `expect()` or `unwrap()`
    // when joining a thread, since it could make your whole program fail
    if handle.join().is_err() {
        println!("Something bad happened when joining :(");
    }

    println!("After everything!");
}

// output
// The inner_scoped_struct struct is dropped
// Before the thread spawn!
// After thread spawn!
// thread 'bad thread' panicked at 'Panicking inside the thread!', multithreading/src/bin/recover-from-panic.rs:41:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// The thread is panicking with the thread_struct struct (from drop)!
// Something bad happened when joining :(
// After everything!
// The main_fn_struct struct is dropped
