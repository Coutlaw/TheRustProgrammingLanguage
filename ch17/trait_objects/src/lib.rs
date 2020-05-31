// using trait objects for a GUI library with a screen that can have multiple things draw on it
pub trait Draw { 
	fn draw(&self);
}

pub struct Screen { 
	pub components: Vec<Box<dyn Draw>> // a trait object is the trait wrapped in a dynamic smart pointer
	// this allows for substitution more complex than generics
	// a generic value can only be substituted for one concrete type at a time
	// this can substitute any type that implements the Draw trait
	// generics would be the solution if all the items in the collection were homogenous
	// like Vec<Button> or Vec<TextBox>
}

impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}

impl Draw for Button { 
	fn draw(&self) {
		// some actual code to draw a button, this would vary for everything that uses our trait
		// but all structs that impl draw would work for screen because of the trait object
	}
}

// THE GENERIC WAY for screen, bad for this example but including as a reference
pub struct Screen2<T: Draw> {
	pub components: Vec<T>,
}

impl<T> Screen2<T>
	where T: Draw {
		pub fn run(&self) {
			for component in self.components.iter() {
				component.draw();
			}
		}
	}

