fn main() {
    // constant that remains in scope for the lifetime of the scope its declared, cannot be reassigned
    const MAX_VALUE: u32 = 100;
    println!("{}", MAX_VALUE);

    //Shadowing
    let x = 5;
    println!("{}", x);

    let x = x + 1; // shadows the previous x, effectivly creating a new variable vs mutating
    println!("{}", x);

    let x = " "; // changes the type of x from an int to string.alloc

    let spaces = "     ";
    let spaces = spaces.len(); //  changes spaces to a number, we cannot do this with a mut
    println!("{}", spaces);

    // below code does not work, compiler error because you can't change types of a mut variable, have to use shadowing for this
    // let mut spaces1 = "   ";
    // spaces1 = spaces1.len();

    // Types
    // Rust has scalar and compound primitive types

    // Scalar types
    // represents a singular value
    // integers, floating point number, booleans and characters

    // ints
    let num: u32 = 4; // default rust value for a number is u32, 2^n - 1 numbers
    let signed: i32 = -5; // - 2^(n-1) to 2(n-1) - 1 numbers
    println!("{} {}", num, signed);

    // FP + operations + ints
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 32;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);

    // bools
    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);

    // chars
    let c = 'z';
    // represents more than just ASCII, is Unicode scalar
    println!("{}", c);


    // Compound types
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructur tuple
    let (thing1, thing2, thing3) = tup;
    println!("{} {} {}", thing1, thing2, thing3);
    // could also do tup.0, tup.1 etc etc

    // Array type, Has fixed length!! and can put collections of data on the stack over the heap
    // Vectors are like arrays but can grow
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // create an array with all the same intial values
    let empt_arr = [3; 5];
    // array index
    let _ = arr[1]; // gets thing at position 1
    let _ = empt_arr[2];

    // Call out to another function
    another_function();

    // another function with arguments
    another_function2(4, 5);

    // variable with a function body
    let _ = {
        // can assign other vars
        let x = 3;
        // expression == return value
        x + 1
    };

    // call a function with a return type
    println!("function outputs {}", plus_one(4));


    // conditionals
    let number = 5;

    if number == 0 {
        println!("first arm of condition");
    } else if number < 4 {
        println!("second arm of condition");
    } else {
        println!("final arm of condition");
    }

    // conditional var assignment
    let conditional_var = if number < 4 {
        7
    } else {
        8
    };

    println!("{}", conditional_var);

    //Loops
    /*
    loop{
        println!("all the loop");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    }

    // while loop
    while number != 0 {
        number = number - 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        // do something with element
        element.foo()
    }

    for number in (1..4).rev() {
        println!("{}". number)
    }

    */
}



// Another function in rust
fn another_function() {
    println!("Another Function!");
}

fn another_function2(x: i32, y: i32) {
    println!("Got {} and {} as an argument", x, y);
}

// function with return type
fn plus_one(x: i32) -> i32 {
    // expression, meaning the returning value
    x + 1
}
