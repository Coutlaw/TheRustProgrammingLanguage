use std::thread;
use std::sync::mpsc;
// multiple producer single consumer
use std::time::Duration;

fn main() {
    // transmitter and receiver from the channel
    let (tx, rx) = mpsc::channel();

    // create a thread that will send a message
    thread::spawn(move || {
        let vals = vec![
            String::from("hi!"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            // giving time between each sent message
            thread::sleep(Duration::from_secs(1))
        }
    });

    // this will block the main thread until things have stopped transmitting (The TX is closed)
    for received in rx {
        println!("Got: {}", received);
    }

}
