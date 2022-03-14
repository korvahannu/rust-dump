#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Box is a pointed which is a size Rust knows
    Nil                    // To have list inside a list it needs to know how much memory to allocate
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);    // 5 is now allocated to heap instead of stack
    println!("{}", b);
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))));
    println!("{:?}", list);
}
