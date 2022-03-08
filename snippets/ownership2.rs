fn main() {
    // since s1 is moved at the calculate -function
    // we can use tuples to get it back (example)
    // This is redundant as we need to return the string every time we call it
    let s1 = String::from("My string");
    let (s2, len) = calculate(s1);
    println!("{} length {}", s2, len);

    // We can use references
    let s1 = String::from("Hello");
    let len = calculate_better(&s1); // & lets us create a reference, but does not own it
    println!("{} length {}", s1, len);  // because it is not owned, the value it points to will not be dropped when out of scope

    let mut s = String::from("Mutable reference");
    change(&mut s);
    println!("{}", s);

    let _r1 = &mut s;   // You can only borrow one mutable references at a time
    // let _r2 = &mut s; this would hence be invalid

    let mut weird_mutable = String::from("weird");

    // This works because _w1 is out of scope of _w2
    // Note how these brackets just capsulate
    {
        let _w1 = &mut weird_mutable;
    }

    let _w2 = &mut weird_mutable;

    // NOTE: Multiple immutable references are allowed
    // But if a immutable references points to string X
    // You cannot make a mutable reference to string X
    // Because immutable references do not expect it to change
}

fn calculate(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_better(s: &String) -> usize {
    s.len()
    // s.push_str("This would not work, as references are immutable");
}   // Here s goes out of scope. But it does not own what it refers to, it will not drop it

fn change(s : &mut String) {
    s.push_str(", world!");
}