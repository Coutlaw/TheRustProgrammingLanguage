fn main() {
    // string literal
    let s = "hello";
    // String (stored on heap)
    let mut dynamic_string = String::from(s); // dynamically allocated at runtime vs a string literal at compile time

    dynamic_string.push_str(", world!");
    // can mutate string.From

    // s is stored on the stack, hence its imutability, while dynamic_string is stored on the heap for growth

    

}
