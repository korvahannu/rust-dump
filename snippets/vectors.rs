#![allow(dead_code, unused_variables)]

enum Cell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let mut vector: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    vector.push(5);
    vector.push(3);

    let second : &i32 = &vector[1];
    println!("Second number is {}", second);

    match v.get(3) {    // .get returns Option<&T>
        Some(val) => println!("{}", val),
        None => println!("It is empty"),
    };

    let does_it_exist = vector.get(5);

    if let None = does_it_exist {
        println!("It din does not exist");
    }

    for i in &mut vector {
        *i += 5;
        println!("{}", i);
    }

    let spreadsheet : Vec<Cell> = vec![Cell::Float(10.0), Cell::Text(String::from("Hello")), Cell::Int(5)];

    for element in &spreadsheet {
        match element {
            Cell::Float(v) => println!("Float {}", v),
            Cell::Text(v) => println!("Text {}", v),
            Cell::Int(v) => println!("Integer {}", v)
        }
    }
}
