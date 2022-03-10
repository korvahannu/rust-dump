pub enum BookType {
    PaperBack,
    HardCover
}

pub struct Color(pub u8, pub u8, pub u8);

pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: usize,
    pub booktype: BookType,
    pub color: Color
}

impl Book {

    pub fn print(&self) {
        println!("{} by {}. This book has {} pages.", self.title, self.author, self.pages);

        match self.booktype {
            BookType::PaperBack => println!("This is book is a paperback."),
            BookType::HardCover => println!("This book has a hard cover.")
        }

        println!("The RGB of this book is {} {} {}", self.color.0, self.color.1, self.color.2);
    }

    pub fn copy(b : Book) -> Book {
        Book {
            ..b
        }
    }

}