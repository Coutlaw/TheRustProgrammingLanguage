fn main() {
    // string literal
    let sl = "hello";
    // String (stored on heap)
    let mut dynamic_string = String::from(sl); // dynamically allocated at runtime vs a string literal at compile time

    dynamic_string.push_str(", world!");
    // can mutate string.From

    // s is stored on the stack, hence its imutability, while dynamic_string is stored on the heap for growth

    // Trace the program
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    // DOES NOT COMPILE 
    let s1 = String::from("hello");
    let s2 = s1; // s1 becomes invalid

    println!("{}, world!", s1); // compiler error
    /*
    $ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
    error[E0382]: borrow of moved value: `s1`
    --> src/main.rs:5:28
    |
    2 |     let s1 = String::from("hello");
    |         -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
    3 |     let s2 = s1;
    |              -- value moved here
    4 | 
    5 |     println!("{}, world!", s1);
    |                            ^^ value borrowed here after move

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0382`.
    error: could not compile `ownership`.

    To learn more, run the command again with --verbose.
    */

    // working way to do the above 👆
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // primitives that have a known size at compilation, don't need clone and are put on the stack
    // these implement the copy trait
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // See fake_main() and fake_main2() for ownership walk through

    // fake_main3() shows how to return tuples and return ownership



}

fn fake_main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn fake_main2() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn fake_main3() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
