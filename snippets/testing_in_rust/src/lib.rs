#![allow(dead_code, unused_mut, unused_variables)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    pub fn new(width: u32, height:u32) -> Rectangle {
        Rectangle { width, height }
    }
}

pub fn add_two(a : i32) -> i32 {
    a +2
}

fn greeting (name : &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32 // value is private
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration() {
        assert_eq!(2 +2 , 4);
        let b : bool = true;
        assert!(b);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rect : Rectangle = Rectangle::new(25, 30);
        let rect2 : Rectangle = Rectangle::new(22, 23);
        assert!(rect.can_hold(&rect2));
        assert!(!rect2.can_hold(&rect));
    }

    #[test]
    #[ignore]
    fn test_twos() {
        assert_eq!(add_two(5), 7);
    }

    #[test]
    fn greetings_test() {
        let result = greeting("Hannu");
        assert!(
            result.contains("Hannu"),
            "Greeting did not contain name, value was {}", // After result.contains the paramters go inside format! macro
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 150.")] // This code is expected to panic
    fn guess_test() {
        let q : Guess = Guess::new(150);
    }

    #[test] // Err means test failed, Ok means test was ok!
    fn it_works_with_results() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_string())
        }
    }
}
