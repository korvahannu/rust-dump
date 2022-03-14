// Multiple owners of mutable data
// Remember: Rc for read-only access
// RefCell for read-only or read-and-write access

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Item(Rc<RefCell<i32>>, Rc<List>),
    None
}

use self::List::{Item, None};

fn main() {
    let value = Rc::new(RefCell::new(5)); // Holds a value inside read-access pointer that points to a read-and-write pointer

    let a = Rc::new(Item(Rc::clone(&value), Rc::new(None)));
    let b = Item(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Item(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
