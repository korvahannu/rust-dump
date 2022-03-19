trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("I am a pilot");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("I am a wizard");
    }
}
impl Human {
    fn fly(&self) {
        println!("I am a human");
    }
}
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

use std::fmt;
// In the following example we specify that OutlinePrint trait will work only on types that also implement Display -type
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct Point {
    x: i32,
    y: i32
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}

// According to orphan rule we can not implement a trait to a type if both the trait and type are not local scripts
// For example, we cant implement display for vec<t>
// Heres a workaround
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    println!("{}", Dog::baby_name());
    println!("{}", <Dog as Animal>::baby_name()); // We need to tell Rust that we want to use the implementation of animal for dog
}
