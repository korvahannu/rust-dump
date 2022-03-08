fn main() {
    let _s = "This is a string literal"; // String literals are fast and efficient, but not mutable
                                         // String literals are known at compile time

    // Mutabel String -> memory must be requested from the memory allocator at runtime
    let mut s = String::from("This is another type of string");
    s.push_str(", which you can add new text to.");
    println!("{}", s);

    // x and y are simple integers which are stored to stack
    // Most simple types implement the "Copy" -trait which
    // means that x is copied to _y and x is still valid after copying
    let x = 5;
    let _y = x; // value of 5 is copied over to y

    // String -type is stored to heap
    // It would be very expensive to copy s1 to s2
    // Instead Rust copies the POINTER
    // Meaning after the follow, both s1 and s2 point to the same string
    // However! To consider double free errors, Rust considers
    // s1 to be invalid after _s2 = _s1 and drops it
    let _s1 = String::from("This is stored to heap");
    let _s2 = _s1;
    // let _s3 = _s1; this is invalid, as _s1 has been dropped
    // This line would not work, because _s1 is considered invalid ==> println!("{}", _s1);
    // Technically, _s1 was moved to _s2

    // Here is a work around. Should be avoided, because it is expensive!
    let _mystring1 = String::from("This is stored to heap");
    let _mystring2 = _mystring1.clone();

    takes_ownership(_mystring2);
    // takes_ownership(_mystring2); This is invalid, because _mystring2 has already been moved and is no longer valid

    // i32 inherits Copy -trait and is saved to stack
    makes_copy(x);
    makes_copy(x);

    let mut hey : String = String::from("Hello");
    let hey2 = &mut hey;
    hey2.push_str("Hello");
    println!("{}", &hey2);
    println!("{}", &hey);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
