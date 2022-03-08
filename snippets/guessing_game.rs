use std::io::stdin;    // Standard library, input output, stdin
use rand::Rng;  // Rng is a trait
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is: {}", secret_number);

    println!("Guess the number!");
    loop {
        println!("Please input your guess: ");

        // "let" is used to define variables
        // variables are constants by default
        // adding "mut" makes the variable mutable
        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)  // Instead of a reference &guess which would be an immutable reference
                                    // we use &mut guess
            .expect("Failed to read line"); // read_line also returns a type of Result
        
        println!("You guessed: {}", guess);

        let guess :u32 = match guess.trim().parse() {   // Rust allows foreshadowing
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {   // .cmp() returns type of Ordering
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
