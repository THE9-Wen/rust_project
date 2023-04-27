use std::arch::x86_64::_mm_loadl_epi64;

fn main() {
    let mut post = Post::new();

    post.add_text("Hello world!");
    post.request_review();
    post.content();
    post.content();

}

pub struct Post {
    content: String,
    state: Option<Box<dyn State>>
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, content: &str) {
        if let Some(s) = self.state.take() {
            self.content.push_str(s.add_text(content));
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn add_text<'a>(&self, content: &'a str) -> &'a str {
        ""
    }
    fn content<'a>(&self, post:&'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {
            approve_time: 0,
        })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, content: &'a str) -> &'a str {
        content
    }
}

struct PendingReview {
    approve_time: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        match self.approve_time {
            0 => Box::new(PendingReview{
                approve_time: 1
            }),
            _ => Box::new(Published {}),
        }
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
