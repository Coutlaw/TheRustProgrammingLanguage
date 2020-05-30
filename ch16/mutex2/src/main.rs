use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Arc is like RC but safe for concurrency
    let counter = Arc::new(Mutex::new(0)); // we have to use an atomic reference counter here because a mutex can only have a single owner
    // Mutex gives us the interior mutability pattern 
    
    // handles is to aggregate the threads so we can join to await them
    let mut handles = vec![];

    // spawn 10 threads that all take the mutex lock to mutate the data inside the lock
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn( move || {
            // take the mutex lock by the thread
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // complete all the threads
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
