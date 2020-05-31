// just an example if someone were to consume our lib and make a new type of thing to draw on screen
use trait_objects::{Draw, Screen, Button};

fn main() {
    // now a user can consume our screen and add their new type to be drawn on the screen without issue
    // thanks to trait objects, also known as "duck typing"
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// A user wants to add a new field to draw on the screen from our library
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code that would draw it on the screen
    }
}
