// Multiple ownerships with Rc<T>. Works only in single-threaded scenarios
use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {
    // This enables multiple owners of A
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // Use Rc::clone to pass a reference

    {
        let x = Cons(3, Rc::clone(&a)); // this gets dropped asap
    }

    let c = Cons(4, Rc::clone(&a)); // Rc::clone does not make a deep copy,
    println!("count after creating {}", Rc::strong_count(&a));
}
