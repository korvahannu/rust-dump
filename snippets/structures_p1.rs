#![allow(dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32); // This is a tuple struct

struct AlwaysEqual;

fn main() {
    let mut user1 : User = build_user("hannu", "hannu.s.korvala@gmail.com");

    user1.email = String::from("hannu.korvala@cgi.com");
    println!("{}", user1.email);

    let _user2 : User = build_user2(String::from("neav"), String::from("nea.vau@gmail.com"));
    let _user3 = User {
        email: String::from("hannu.s.korvala@gmail.com"),
        ..user1 // This must come last. This MOVES so user1 becomes invalid. 
        // If username would have been defined aswell user1 would have remained valid, because active and sign_in_count are primitives with COPY trait
    };

    let _color : Color = Color(255, 255, 255);
    let Color(r, g, b) = _color;    // How to destruct tuple struct
    println!("{} {} {}", r, g, b);
    let _test = AlwaysEqual;
}

fn build_user (username: &str, email: &str) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        active: true,
        sign_in_count: 1
    }
}

fn build_user2 ( username : String, email : String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}