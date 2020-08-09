use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel();

    // five threads get created with an iterator,
    // and their handles are collected in a vector
    let handles: Vec<_> = (1..6)
        .map(|i| {
            let t_sender = sender.clone();
            thread::Builder::new()
                .name(format!("sender-{}", i))
                .spawn(move || {
                    t_sender.send(format!("Hello from sender {}!", i)).unwrap();
                })
                .expect("could not create the thread")
        })
        .collect();

    // a while loop will check with a 1-second timeout for the messages
    // In the case that no message gets received for one second
    // (or all the senders get out of scope), the while loop will
    // top printing the messages and the threads will be joined
    while let Ok(message) = receiver.recv_timeout(Duration::from_secs(1)) {
        // the threads are not executed in any particular order.
        // They will send the message to the receiver, and the
        // receiver will read them in the received order
        println!("{}", message);
    }

    // this is asynchronous, so we don't need to wait for the
    // receiver to empty the buffer to send new messages
    // we could join the threads before reading any messages from the receiver
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finished");
}

// output
// Hello from sender 3!
// Hello from sender 1!
// Hello from sender 2!
// Hello from sender 5!
// Hello from sender 4!
// Finished
