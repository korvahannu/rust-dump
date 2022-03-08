fn main() {
    let mut s = String::from("example");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; This is invalid, as there are immutable references to s
    println!("{}{}", r1, r2);

    let r3 = &mut s; // This is valid, because r1 and r2 are not used here
    println!("{}", r3); // This is called Non-Lexical Lifetime
}

/* Consider the following function
    String d gets out of scope and is dropped
    after function has done its thing
    what does d point to? Rust does not allow this so this results in error

fn dangle() -> &String {
    let d = String::from("This dangles");
    &d
}
*/

/*
At any given time, you can have either one mutable reference or any number of immutable references.
*/