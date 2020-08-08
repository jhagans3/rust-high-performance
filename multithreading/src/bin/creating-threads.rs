use std::thread;

// cargo run --bin creating-threads
fn main() {
    // the main thread, running the main() function
    println!("Before typing thread spawn!");

    {
        let inside = || { println!("Inside the thread!"); };
        let handle = thread::spawn(inside);

        // The println!() macro will lock stdout while it 
        // writes the message, and if a new message wants 
        // to be written, it will have to wait until 
        // the first write finishes
        println!("After typing thread spawn!");

        // This will make the current thread wait for the other one to finish
        handle.join().expect("the thread panicked");
    }// drop inside here


    println!("After everything (thread join handle)!");
}