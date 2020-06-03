pub struct Post {
	state: Option<Box<dyn State>>,
	content: String,
}

impl Post {
	pub fn new() -> Post {
		Post {
			// initilize state as a reference to a draft with an empty content
			state: Some(Box::new(Draft {})),
			content: String::new(),
		}
	}
}

trait State {}

struct Draft {}

impl State for Draft {}
