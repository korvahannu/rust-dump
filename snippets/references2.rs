fn main() {
    let mut x : u8 = 5;
    lps(&mut x);
    println!("{}", x);

    let mut str = String::from("hey");
    strp(&mut str);
    println!("{}", str);
}

fn lps(b : &mut u8) {
    *b = 20;
}

fn strp(stra : &mut String) {
    *stra = String::from("Hello world");
}
