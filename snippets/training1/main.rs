mod readables;
use readables::{Magazine, Book, Message, Readable};

fn main() {
    let mag : Magazine = Magazine::new(
        String::from("Helsingin sanomat"),
        String::from("Venäjä hyökkää!"));
    mag.print();

    let book : Book = Book::new(String::from("Kauhujen yö"), String::from("Hannu Korvala"), 216);
    book.print();

    let msg : Message = Message::new(String::from("Important!"), String::from("This is a message"));
    msg.print();

    print_readable(&msg);

    let mut n = 0;

    while n < 1000000 {
        println!("Hello!");
        n += 1;
    }
}

fn print_readable<T:Readable>(readable : &T) {
    readable.print();
}