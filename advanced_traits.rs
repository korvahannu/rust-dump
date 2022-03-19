struct Counter {
    value: u32
}

struct Counter2 {
    value: u32
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterators<T> {
    fn next(&mut self) -> Option<T>;
}

// We can implement the trait for a struct only once
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        Some(self.value)
    }
}

// If we use generics, we can implement the trait many times for a struct
impl Iterators<u32> for Counter2 {
    fn next(&mut self) -> Option<u32> {
        self.value += 1;
        Some(self.value)
    }
}

impl Iterators<String> for Counter2 {
    fn next(&mut self) -> Option<String> {
        self.value += 1;
        Some(format!("{}", self.value))
    }
}

use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    // We can override the + operator by implementing Add trait
    fn add(self, other:Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// by default the add trait is impl Add<Rhs=Self>, we can overwrite this
// fn add(self, rhs: Rhs) -> to fn add(self, meters: Meters)
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let p = Point { x: 5, y: 10};
    let p2 = Point { x: 12, y: 9};
    println!("{:?}", p + p2);
}
