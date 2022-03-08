fn main() {
    hello_world();
    printmsg("Hello world!");

    let ika = {
        let t : i32 = 1994;
        t - 2022                // Note: No ; -at the end, also note that while this returns t - 2022, you can not do return t - 2022
    };

    println!("{}", ika);
    println!("{}", five());
    println!("{}", six());
    println!("{}", seven());
    println!("{}", square(25));
}

fn hello_world() {
    println!("Hello world!");
}

fn printmsg(message : &str) {
    println!("{}", message);
}

fn five() -> u8 {
    5
}

fn six() -> u8 {
    let b = 6;
    b
}

fn seven() -> u8 {
    let b = 7;
    return b    // semicolon is optional
}

fn square(number : usize) -> usize {
    number * number
}