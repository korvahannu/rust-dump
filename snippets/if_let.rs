fn main() {
    let config_max : Option<u8> = Some(200u8);

    match config_max {
        Option::Some(max) => {
            println!("Max is {}", max);
        },
        _ => (),
    }

    if let Option::Some(max) = config_max {
        println!("Max is {}", max);
    }
}
