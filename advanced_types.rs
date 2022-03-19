fn main() {
    type Kilometers = i32; //Type alias
    let x : i32 = 8;
    let y: Kilometers = 5;

    println!("{}", x + y);
    type My_Result<T> = std::result::Result<T, std::io::Error>;
    let res : My_Result<String> = Ok(String::from("Hey!"));
}
