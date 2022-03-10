#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    let mut s = String::new();
    let data = "foo";
    let mut s: String = data.to_string();
    s.push_str("bar");
    println!("{}", s);
    let concated: String = s + &data + "lol"; // Notice how s does not have & but &data has. s is no longer valid after this
    println!("{}", concated);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = "".to_string() + &s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1]; Each of characters inside hello are length of 2 bytes. We cant slice it to be half a character
    let hello = "Hello";
    let s = &hello[0..1]; // But here this works fine because these letters are a length of 1 byte

    // Iterate strings
    // Per char
    for c in "Здравствуйте".chars() {
        print!("{}", c);
    }
    println!();
    // Bytes
    for b in "hello Здравствуйте".bytes() {
        print!("{}-", b); // Unicode scalar values may be made up of more than 1 byte
    }
    println!();
}
