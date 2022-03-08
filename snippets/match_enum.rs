#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum Shine {
    Shiny,
    NotShiny,
}

enum Coin {
    Penny,
    Nickel(Shine),
    Dime,
    Quarter,
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter));
    println!("{}", value_in_cents(Coin::Nickel(Shine::Shiny)));
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    let dice_roll : u8 = 9;

    match dice_roll {
        3 => println!("Nice!"),
        7 => println!("Super nice!"),
        _ => println!("Akward"),
    };
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel(sh) => {
            println!("{:?}", sh);
            5
        }
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("Wow rich boy!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            Some(i + 1) // Returns Option::Some<i32>
        }
    }
}
