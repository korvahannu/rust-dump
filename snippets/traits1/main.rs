#![allow(unused_imports, unused_variables, unused_mut, dead_code)]

mod aggregator;

use std::fmt::Display;
use crate::aggregator::{Tweet, NewsArticle, Summary};

fn main() {
    let tweet : Tweet = Tweet {
        username: "Hannu".to_string(),
        content: "Horse busman".to_string(),
        reply: false,
        retweet: false
    };

    println!("{}", tweet.summarize());
    tweet.unimplemented();

    let fake_news : NewsArticle = NewsArticle::new();
    fake_news.unimplemented();

    notify(&fake_news);
    println!("{}", returns_summraizable().summarize());
}

fn notify(poop : &impl Summary) {   // Instead of specifying a type, this function accepts anything that implements Summary -trait
    poop.unimplemented();
}

// T:Summary means that T is trait bound to Summary -trait. This is same thing as above
pub fn notify2<T:Summary>(item: &T) {

}

pub fn notify3<T:Summary>(item: &T, item2: &T) {

}
// The code up and below are nearly the same, but the one above forces type T, the one below item and item2 can have different types
pub fn notify4(item: &impl Summary, item2:&impl Summary) {

}

// Multiple trait bounds with the + syntax
pub fn notify5(item: &(impl Summary + Display)) { // Must implement summary and display
}
// Multiple trait bounds
pub fn notify6<T: Summary+Display>(item: &T) { } // Same as above but with generic
pub fn notify7<T: Summary + Display, U : Summary>(t: &T, b: &U) {}

// This is the same as notify7 above but with WHERE syntax
fn notify7_redone<T, U>(t: &T, u: &U)
    where T: Summary + Display,
        U: Display {}

fn returns_summraizable() -> impl Summary {
    NewsArticle::new()
} // note: because of the way rust compiler handles the impl Trait syntax, you
// cant have a function return different types of Summary
/* This wouldnt work
fn ret(b:bool) -> impl Summary {
    if b {
        NewsArticle::new()
    }
    else {
        Tweet {
            username: "Hannu".to_string(),
            content: "Horse busman".to_string(),
            reply: false,
            retweet: false
        }
    }
}
*/