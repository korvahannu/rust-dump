mod book;

use book::{Book, BookType, Color};

fn main() {
    let mybook : Book = Book {
        title: String::from("Gulligans Travels"),
        author: String::from("Gulligan himself"),
        pages: 500,
        booktype: BookType::HardCover,
        color: Color(255, 0, 0)
    };

    mybook.print();

    let mybook2 : Book = Book {
        title: String::from("Haunted house on a hill"),
        pages: 325,
        ..mybook
    };

    mybook2.print();

    let mut mybook3 : Book = Book::copy(mybook2);

    mybook3.title = "REMOVED TITLE".to_string();
    mybook3.print();
}
