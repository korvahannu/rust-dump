use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Unless we implement Deref for MyBox we can not use deference * operator for it
impl<T> Deref for MyBox<T> {
    type Target = T; // Associated type

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);  // We must use * operator to follow the reference the value is pointing to

    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // This line of code is the same as below
    assert_eq!(5, *(y.deref()));

    // Because we implemented deref, Rust can turn &MyBox<String> to &String
    // Standard libarry implements Defer on String that returns a string slice
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m).as_str()); // Same as above except without defer coercion

    let mut x = 5;
    dereferencing(&mut x);
    assert_eq!(10, x);
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn dereferencing(x: &mut i32) {
    *x += 5;
}