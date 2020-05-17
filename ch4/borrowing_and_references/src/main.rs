fn main() {
    let s1 = String::from("hello"); // create String

    let len = calculate_length(&s1); // calculate length by passing reference, but retaining ownership

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    // code will not compile 👇 meant to explain shared mutation
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time

    println!("{}, {}", r1, r2);

    // we can correct it by adding scope
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    let mut s = String::from("hello");

    // this code does not compile
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}, {}, and {}", r1, r2, r3);


    // correct the above with this
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Observe the dangle() function to see to create a dangling pointer, would not compile
    // fix dangle with no_dangle()


}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

  fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
