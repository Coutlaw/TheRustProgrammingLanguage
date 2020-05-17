fn main() {
    let s = String::from("hello world");

    // slices are imutable
    // string literals are slices

    let _hello = &s[0..5];
    let _world = &s[6..11];

    // these are equal for getting the 0 through 1st element [inclusive..exclusive]
    let _slice = &s[0..2];
    let _slice = &s[..2];

    let len = s.len();

    // these are equal for getting the third element to the end of the string
    let _slice = &s[3..len];
    let _slice = &s[3..];

    // these are equal
    let _slice = &s[0..len];
    let _slice = &s[..];

    // see _first_word() to see how to find the first word of a string

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
