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

	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text);
	}

	pub fn content(&self) -> &str {
		self.state.as_ref().unwrap().content(&self)
	}

	pub fn request_review(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.request_review())
		}
	}

	pub fn approve(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.approve())
		}
	}
}

trait State {
	fn request_review(self: Box<Self>) -> Box<dyn State>;
	fn approve(self: Box<Self>) -> Box<dyn State>;
	// default implementation
	fn content<'a>(&self, post: &'a Post) -> &'a str {
		""
	}
}

struct Draft {}

impl State for Draft {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		Box::new(PendingReview {})
	}
	fn approve(self: Box<Self>) -> Box<dyn State>{
		self
	}
}

struct PendingReview {}

impl State for PendingReview {
	fn request_review(self: Box<Self>) -> Box<dyn State>{
		self
	}
	fn approve(self: Box<Self>) -> Box<dyn State> {
		Box::new(Published {})
	}
}

struct Published {}

impl State for Published {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn approve(self: Box<Self>) -> Box<dyn State> {
		self
	}

	// we have to have a lifetime since we are taking a reference to a post and we need to return a part of that reference
	fn content<'a>(&self, post: &'a Post) -> &'a str {
		&post.content
	}
}
