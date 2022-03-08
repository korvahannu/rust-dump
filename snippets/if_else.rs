fn main() {
    let number : u8 = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number == 0 {
        println!("Number was 0");
    }
    else if number == 7 {
        println!("Number was 7");
    }

    let condition : bool = true;

    let number : i16 = if condition { 10 } else { -10 };

    println!("{}", number);
}
