
pub trait Summary {
    fn summarize(&self) -> String;
    fn unimplemented(&self) {
        println!("You could implement this");
        println!("{}", &self.summarize()); // We can call a function inside a trait
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl NewsArticle {
    pub fn new() -> NewsArticle {
        NewsArticle {
            headline: "Test headline".to_string(),
            location: "Test location".to_string(),
            author: "Test author".to_string(),
            content: "Test content".to_string()
        }
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} at {}", self.headline, self.author, self.location)
    }
    fn unimplemented(&self) { // This overrides the method unlike Tweet that uses default implementation
        println!("Oh nice!");
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}