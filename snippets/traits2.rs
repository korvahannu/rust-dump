#![allow(dead_code, unused_variables)]
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> { // Only if the Pair is type T that implements traits Display and PartialOld is this implemented
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is x = {}", self.y);
        }
    }
}

fn main() {
    let list = vec![5, 4, 12, 8];
    println!("{}", largest(&list));
}

// Now this function only allows generics that implement Copy and PartialOrd -traits
fn largest<T:PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
