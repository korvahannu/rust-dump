#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter takes ownership
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}

fn main() {
    let v1 = vec![1,2,3];
    for x in &v1 {
        println!("{}", x);
    }

    let v1_iter = v1.iter();
    for x in v1_iter {
        println!("{}", x);
    }

    let v1 = vec![1,2,3];
    let iter = v1.iter();
    let total : i32 = iter.sum();
    println!("{}", total);
    println!("{:?}", v1);
    // let total2 : i32 = iter.sum(); This wouldnt work as iter is already consumed by previous sum

    let v1  = vec![1, 2, 3, 4, 5, 6];
    // By itself map doesnt do anything because it is lazy
    // We use collect to collect the results of iterating over
    // the iterator thats returned from the call to map into a vector
    let v2 : Vec<u32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}{:?}", v1, v2);

    let b = v2.iter().map(|x| x * 5);
    println!("{:?}", b);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("{:?}", in_my_size);
}