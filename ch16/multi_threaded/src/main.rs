use std::thread;
use std::time::Duration;

fn main() {
    // this thread will die when the main thread has ended
    // but this spawned thread will start printing outputs while the main thread continues on
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // this is just to allow the OS to do work on another thread
            // threads will probably take turns, but this is not guaranteed
            // thats why the output of this program will change every time
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}
