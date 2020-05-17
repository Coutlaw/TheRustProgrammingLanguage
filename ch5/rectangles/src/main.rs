fn main() {
    // old way
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        old_area(width1, height1)
    );

    // tuple way
    println!(
        "The area of the rectangle is {} square pixels.",
        tuple_area((width1, height1))
    );

    // struct way

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // using debug trait
    println!("rect1 is {:?}", rect1);

    // now using the rectangle method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // using some class methods
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // using an associated function
    let _sqr = Rectangle::square(3);


}

// old function
fn old_area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// rectangle struct with the derived debug trait for printing it out
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {

    // impl methods that are not associated functions always take a reference to self as the first param
    // rectangle method 
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // rectangle method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associated function, did not have to be in separate impl block
impl Rectangle {
    // we know its an associated function because it doesn't talk a reference to self
    // these are typically used as constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
