pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }
    pub fn add_text(&mut self, text:&str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        // as_ref because we want a references to the value inside
        // Rather than ownership Option<&Box<dyn State>>
        // and unwraps to &Box<dyn State>
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        // We use the take method to take the Some value out of state and leave
        // None in its place. This lets us move the state value out of Post
        // We need to set state to None temporarily
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

#[allow(unused_variables)]
trait State {
    // This method is only valid when called on a Box holding the type
    // This syntax takes ownership of Box<Self>
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
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
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}