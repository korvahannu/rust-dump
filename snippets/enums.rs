#![allow(dead_code)]
#![allow(unused_variables)]

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        if let Message::Write(msg) = self {
            println!("{}", msg);
        }

        // tai
        match self {
            Message::Write(msg) => {
                println!("{}", msg);
            },
            _ => ()
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::0"));

    let message : Message = Message::Write(String::from("Hello world!"));
    message.call();
}