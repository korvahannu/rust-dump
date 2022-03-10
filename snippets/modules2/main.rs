mod hello;
mod square;

use square::Square as Rectangle;

fn main() {
    hello::hello_world();
    let rect : Rectangle = Rectangle {
        width: 50,
        height: 50
    };

    println!("{}", rect.area());
}
