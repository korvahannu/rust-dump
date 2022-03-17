mod mymod;

use oop::Post;

use crate::mymod::Vehicle;

fn main() {
    let mut post = Post::new();

    let car = mymod::Car::new();
    println!("{}", car.getInfo());

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut wrapper : Wrapper<&str> = Wrapper::new("Hello world!");

    println!("{}", wrapper.borrow());

    let c = wrapper.borrow_mut();
    *c = "Hello again!";

    println!("{}", wrapper.borrow());

    let mut x : i32 = 50;
    let mut z : i32 = 10;

    {
        let mut y : &mut i32= &mut x;
        *y = 100;
        y = &mut z;
        *y = 20;
    }

    println!("{} {}", x, z);

}

struct Wrapper<T> {
    content: T
}

impl<T> Wrapper<T> {
    fn new(content : T) -> Wrapper<T> {
        Wrapper {
            content
        }
    }
    fn borrow(&self) -> &T {
        &self.content
    }
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.content
    }
}