use std::sync::Mutex;

fn main() {
    // new mutex
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // attempt to unwrap, could panic which would kill the thread
        // .lock() returns a mutexGuard, which unlocks when the var goes out of scope
        *num = 6; // num is now a mutable reference to the data in the mutex, this updates that data
    } // mutex is unlocked at the end of scope

    println!("m = {:?}", m);
}
