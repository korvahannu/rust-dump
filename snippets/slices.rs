fn main() {
    let s = String::from("hello world");
    sup = "Hello".to_string();
    let _hello = &s[0..5]; // Hello is a reference to a slice of a string
    let _world = &s[6..11]; // world is a reference to the 6th byte of string
    let _hello2 = &s[..5];
    let _world2 = &s[6..];
    let _world3 = &s[6..s.len()];
    let _helloworld = &s[..]; // This is a reference to the entire string
    let example = String::from("Youre an allstar");
    let example = first_word_again(&example);
    println!("{}", example);

    let _s: &str = "Hello world"; // String literal is type &str, it is an immutable slice
    let _s2 = String::from(_s);

    let a = [1, 2, 3, 4, 5];

    let slice : &[i32] = &a[1..3];  // Slice type of &[i32]

    assert_eq!(slice, &[2, 3]);
}

// Returns the index of the first space it encounters
fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // turn string to bytes

    for (i, &item) in bytes.iter().enumerate() {
        //iter().enumerate() returns an index and a immutable references as a tuple
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn _first_word_(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_again(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
