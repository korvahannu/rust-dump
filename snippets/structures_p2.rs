
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = (30, 50);
    println!("{}", area(rect1));

    let rect2 = Rectangle {
        width: dbg!(500),   // dgb is the debug macro and returns the value
        height: 500
    };

    println!("{:#?}, with area of {}", rect2, area2(&rect2));

    dbg!(&rect2);
}

fn area (dimension : (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area2 ( rect : &Rectangle) -> u32 {
    rect.width * rect.height
}