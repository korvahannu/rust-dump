#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    joker: u32
}

impl Rectangle {    // Implementation block
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    fn joker (&self) -> bool {
        if self.joker > 50 {
            return true;
        }
        false
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size : u32) -> Rectangle { // This does not use &self and therefor is an associative function
        Rectangle {
            width: size,
            height: size,
            joker: 0
        }
    }
}

impl Rectangle {    // We can have multiple impl blocks, even tho there is no reason for it
    fn new (width : u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
            joker: 0
        }
    }
}

fn main() {
    let mut rectangle : Rectangle = Rectangle {
        width: 30,
        height: 50,
        joker: 69
    };

    println!("{}", rectangle.area());
    rectangle.double();
    println!("{}", rectangle.area());
    if rectangle.joker() {
        println!("{}", rectangle.joker);
    }
    println!("{}", (&rectangle).area());    // This is the same as in line 35, just written expticitly
    println!("{}", Rectangle::square(5).area());
    println!("{}", Rectangle::new(9, 9).area());
}
