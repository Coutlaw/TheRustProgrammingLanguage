use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // rand crate using the number generator on the current os thread
    // gen_range is inclusive of lower bound, exclusive of upper
    let secret_number = rand::thread_rng().gen_range(1, 11);

    println!("Guess the number 1-10!");

    loop {
        // variables are immutable by default in rust
        // mut makes the mutable
        // the :: calls the static new function (doesn't need an instance)
        let mut guess = String::new();

        println!("Enter a guess: ");

        // without the using std::io this would have to be std::io::stdin()
        // & is obviously a pointer reference as to not pass around copies of data
        // &mut guess makes guess mutable, &guess would not be mutable
        io::stdin()
            .read_line(&mut guess)
            // .expect is a method on io::Result that will run if Result is an instance of Err
            .expect("Failed to read line!");

        // same variable name is called a shadow
        // shadowing allows us to convert the variable instead of making two instances
        // trim takes off the new line character on the input string
        // parse parses as the type on the other side of equals
        // (refactored to match) parse returns Result, and we can reuse the same .expect as before if Result == Err
        // Implemented a Match to match on if parse is OK or Err without crashing the program
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the {} will substitute variables in the order they were supplied
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // exit loop on win
                break;
            }
        }
    }
}
