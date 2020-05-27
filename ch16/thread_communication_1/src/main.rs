use std::thread;
use std::sync::mpsc;
// multiple producer single consumer

fn main() {
    // transmitter and receiver from the channel
    let (tx, rx) = mpsc::channel();

    // create a thread that will send a message
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // val is no longer available since it went into the channel
    });

    // receive the message sent by the thread with the main thread
    let received = rx.recv().unwrap(); // recv() is blocking, could use try_recv() if we needed to do more work
    // print the value that you received from the channel
    println!("Got: {}", received);

}
