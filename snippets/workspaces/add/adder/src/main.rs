use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("Hello, world! {}. And again {}", num, add_one::add_one(num));
    let num = 10;
    println!("Hello world {}. and again {}", num, add_two::add_two(num));
}
