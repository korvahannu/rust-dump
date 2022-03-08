fn main() {
    'my_loop: loop {
        let mut index: u32 = 0;
        loop {
            index += 1;
            println!("Forever");
            if index > 100 {
                break 'my_loop;
            }
        }
    }

    let mut counter : u32 = 0;

    let result = loop {
        counter += 1;

        if counter >= 100 {
            break counter * 2;
        }
    };

    println!("{}", result);

    let mut number : u8 = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    let array = ["Hannu", "Nea", "Maisa", "Jenina"];

    for element in array {
        println!("{}", element);
    }

    for number in (1..=10).rev() {
        println!("{}", number);
    }
}
