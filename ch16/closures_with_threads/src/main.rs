use std::thread;

fn main() {
    let v = vec![1,2,3];

    // the closure will try to borrow v, but there is no telling how long v will live
    // or how long the thread might run, so we have to use move to make the closure to own v
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
